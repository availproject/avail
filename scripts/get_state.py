from concurrent.futures import ThreadPoolExecutor
from substrateinterface import SubstrateInterface
import subprocess
import json
import threading
import xxhash
import os
import argparse
import yaml

FORK_SPEC, RAW_STORAGE = "./output/fork.json", "./output/raw_storage.json"


def fetch_storage_keys_task(hash, keys, prefixes, lock, url):
    substrate = SubstrateInterface(url=url)
    rpc_result = []

    while True:
        lock.acquire()
        if rpc_result:
            keys += rpc_result

        if not prefixes:
            lock.release()
            return

        prefix = prefixes.pop()
        lock.release()

        rpc_result = substrate.rpc_request(method='state_getKeys', params={
            "prefix": prefix, "at": hash})['result']


def fetch_storage_keys(hash, url):
    keys = []
    prefixes = [f'0x{hex(i)[2:].zfill(2)}' for i in range(256)]
    lock = threading.Lock()

    with ThreadPoolExecutor(max_workers=20) as executor:
        for _ in range(20):
            executor.submit(fetch_storage_keys_task,
                            hash, keys, prefixes, lock, url)

    return keys


def fetch_storage_values_task(hash, lock, keys, key_values, url):
    substrate = SubstrateInterface(url=url)

    while True:
        lock.acquire()
        keys_to_fetch = [keys.pop() for _ in range(min(2000, len(keys)))]
        if not keys_to_fetch:
            lock.release()
            return
        lock.release()

        key_values_result = substrate.rpc_request(method='state_queryStorageAt', params={
            "keys": keys_to_fetch, "at": hash})['result'][0]['changes']

        for i, kv in enumerate(key_values_result):
            if kv[1] is None:
                kv[1] = substrate.rpc_request(method='state_getStorage', params={
                    "key": kv[0], "at": hash})['result']

        lock.acquire()
        key_values += key_values_result
        lock.release()


def fetch_storage_values(hash, keys, url):
    key_values = []
    lock = threading.Lock()

    with ThreadPoolExecutor(max_workers=10) as executor:
        for _ in range(10):
            executor.submit(fetch_storage_values_task,
                            hash, lock, keys, key_values, url)

    with open(RAW_STORAGE, 'w') as outfile:
        json.dump(key_values, outfile, indent=2)

    return key_values


def xxh6464(x):
    o1 = bytearray(xxhash.xxh64(x, seed=0).digest())
    o1.reverse()
    o2 = bytearray(xxhash.xxh64(x, seed=1).digest())
    o2.reverse()
    return "0x{}{}".format(o1.hex(), o2.hex())


def list_of_prefixes_to_migrate(substrate):
    # Importing these modules will cause the chain to not work correctly
    skip_modules = ['System', 'Session', 'Babe', 'Grandpa',
                    'GrandpaFinality', 'FinalityTracker', 'Authorship']

    # We definitely want to keep System.Account data and the Runtime :)
    enabled_prefixes = [
        '0x26aa394eea5630e07c48ae0c9558cef7b99d880ec681799c0cf30e8886371da9', '0x3a636f6465']

    module_list = substrate.get_metadata_modules()

    for module in module_list:
        name = module['name']
        if name not in skip_modules:
            enabled_prefixes.append(xxh6464(name))

    return enabled_prefixes


def allowed_to_migrate(key: str, allow_list):
    for prefix in allow_list:
        if key.startswith(prefix):
            return True
    return False


def populate_dev_chain(substrate, forked_storage, chain_name):
    # Read base chain specification. This will be populated with new storage.
    with open(FORK_SPEC) as in_file:
        base_chain = json.load(in_file)
        base_storage = base_chain['genesis']['raw']['top']

    base_chain['name'] = chain_name + " Fork"

    allowed_prefixes: list[str] = list_of_prefixes_to_migrate(substrate)

    # Dev Sudo Key
    sudo_key_prefix = "0x5c0d1176a568c1f92944340dbfed9e9c530ebca703c85910e7164cb7d1c9e47b"
    sudo_key = base_storage[sudo_key_prefix]

    # Migrate storage from Copied to Base storage
    key: str
    for (key, value) in forked_storage:
        if allowed_to_migrate(key, allowed_prefixes):
            base_storage[key] = value

    # Let's change the sudo key to be Alice :)
    base_storage[sudo_key_prefix] = sudo_key

    # Delete System.LastRuntimeUpgrade to ensure that the on_runtime_upgrade event is triggered
    base_storage.pop(
        '0x26aa394eea5630e07c48ae0c9558cef7f9cce9c888469bb1a0dceaa129672ef8')

    # To prevent the validator set from changing mid-test, set Staking.ForceEra to ForceNone ('0x02')
    base_storage['0x5f3e4907f716ac89b6347d15ececedcaf7dad0317324aecae8744b87fc95f2f3'] = '0x02'

    # Write the updated base chain specification to a file
    with open(FORK_SPEC, 'w') as outfile:
        json.dump(base_chain, outfile, indent=2)


def read_command_line_params():
    parser = argparse.ArgumentParser()
    parser.add_argument('--config', '-c')
    parser.add_argument('--binary', '-b')
    args = parser.parse_args()

    f = open(args.config, "r")
    configuration = yaml.safe_load(f)
    return (configuration, args.binary)


def connect_to_remote_chain(url) -> SubstrateInterface:
    substrate = SubstrateInterface(url=url)
    substrate.init_runtime()
    chain_name = substrate.rpc_request('system_chain', None)['result']

    return (substrate, chain_name)


def main():
    (configuration, binary_file) = read_command_line_params()
    url = configuration['endpoint']

    (substrate, chain_name) = connect_to_remote_chain(url)
    hash = configuration.get('at') if configuration.get(
        'at') is not None else substrate.block_hash
    print(
        f"Connected to remote chain: Url: {url}, Chain Name: {chain_name}, Hash: {hash}")

    if not os.path.exists('./output'):
        os.mkdir('./output')
        print("Created output directory: ./output")

    print("Fetching storage keys... ", end=None)
    keys = fetch_storage_keys(hash, url)
    print(f"Fetched {len(keys)} keys")

    print("Fetching storage values... ", end=None)
    forked_storage = fetch_storage_values(hash, keys, url)
    print(f"Fetched {len(forked_storage)} values")

    if (not binary_file):
        print('Building node')
        cmd = 'cargo build --release --locked --features try-runtime'
        subprocess.run(cmd, shell=True, text=True, check=True)

    # Create Chain Snapshot
    # print('Creating Chain Snapshot')
    # cmd = f'./target/release/data-avail try-runtime on-runtime-upgrade live -s {SNAPSHOT} -u {URL}'
    # subprocess.run(cmd, shell=True, text=True, check=True)

    print('Creating Dev Chain Specification. location: ./output/fork.json')
    cmd = f'./target/release/data-avail build-spec --chain dev --raw --disable-default-bootnode > {FORK_SPEC}'
    if (binary_file):
        cmd = f'{binary_file} build-spec --chain dev --raw --disable-default-bootnode > {FORK_SPEC}'

    subprocess.run(cmd, shell=True, text=True, check=True)

    print('Populating Dev Specification. location: ./output/fork.json')
    populate_dev_chain(substrate, forked_storage, chain_name)

    print("Success :)")


if __name__ == "__main__":
    main()

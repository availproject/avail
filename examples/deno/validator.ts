import { ApiPromise, Keyring, WsProvider } from 'https://deno.land/x/polkadot@0.2.42/api/mod.ts';
import { BN } from 'https://deno.land/x/polkadot@0.2.42/util/mod.ts';
import { API_RPC, API_TYPES, API_EXTENSIONS } from './api_options.ts'
import { parse } from "https://deno.land/std@0.202.0/flags/mod.ts";

const flags = parse(Deno.args, {
    string: ["endpoint", "sessionKeys", "controller", "stash"]
});

function display_menu(title: string, choices: string[]): number | undefined {
    while (true) {
        console.log(title);
        choices.forEach((val, i) => {
            console.log(i + ": " + val);
        })
        const choice = prompt(">");
        if (choice == undefined) {
            return undefined;
        }
        const number = Number(choice);
        if (!isNaN(number) && number < choices.length) {
            return number;
        }

        console.log("Unknown option " + choice + ". Try again");
    }
}


// First Let get the api
let endpoint = "ws://127.0.0.1:9944"
if (flags.endpoint == undefined) {
    const options = ["Local Host (ws://127.0.0.1:9944) (default)", "Goldberg (wss://goldberg.avail.tools/ws)", "Custom"];
    const endpoints = ["ws://127.0.0.1:9944", "wss://goldberg.avail.tools/ws", "Custom"];
    const selected_option = display_menu("Pick Endpoint", options)
    if (selected_option != undefined) {
        endpoint = endpoints[selected_option]
    }
} else {
    endpoint = flags.endpoint;
}

console.log("Selected Endpoint: " + endpoint);
const localApi = await ApiPromise.create({ provider: new WsProvider("ws://127.0.0.1:9944"), rpc: API_RPC, types: API_TYPES, signedExtensions: API_EXTENSIONS  })
const networkApi = await ApiPromise.create({ provider: new WsProvider(endpoint), rpc: API_RPC, types: API_TYPES, signedExtensions: API_EXTENSIONS  })
// Retrieve the chain and node information via rpc calls
const [chain, nodeName, nodeVersion, runtimeVersion] = await Promise.all([
    networkApi.rpc.system.chain(),
    networkApi.rpc.system.name(),
    networkApi.rpc.system.version(),
    networkApi.rpc.state.getRuntimeVersion(),
]);


console.log(`Connected to chain ${chain} using ${nodeName}, node version ${nodeVersion} and spec version ${runtimeVersion.specVersion}`);


let controller_mnemonic = "bottom drive obey lake curtain smoke basket hold race lonely fit walk//Bob";
if (flags.controller == undefined) {
    const options = ["Alice", "Bob", "Dave", "Charlie", "Custom"];
    const selected_option = display_menu("Controller account mnemonic: ", options)
    if (selected_option == undefined) {
        controller_mnemonic = "bottom drive obey lake curtain smoke basket hold race lonely fit walk//Bob";
    }
} else {
    controller_mnemonic = flags.controller;
}

let stash_mnemonic = "bottom drive obey lake curtain smoke basket hold race lonely fit walk//Bob";
if (flags.stash == undefined) {
    const options = ["Alice", "Bob", "Dave", "Charlie", "Same as Controller", "Custom"];
    const selected_option = display_menu("Controller account mnemonic: ", options)
    if (selected_option == undefined) {
        stash_mnemonic = "bottom drive obey lake curtain smoke basket hold race lonely fit walk//Bob";
    }
} else {
    stash_mnemonic = flags.stash;
}

const controller = new Keyring({type: 'sr25519'}).addFromMnemonic(controller_mnemonic);
const stash = new Keyring({type: 'sr25519'}).addFromMnemonic(stash_mnemonic);

console.log("Controller: " + controller.address);
console.log("Stash: " + stash.address);


const a = await networkApi.query.staking.minValidatorBond();
const minimum_stash_balance = new BN(a.toString());
console.log("Minimum validator bond: " + minimum_stash_balance.toString());

const ab: any = await networkApi.query.system.account(controller.address);
const ac: any = await networkApi.query.system.account(stash.address);
const controller_balance = new BN(ab["data"]["free"].toString());
const stash_balance = new BN(ac["data"]["free"].toString());

if (stash_balance.lt(minimum_stash_balance)) {
    console.log("Stash Account doesn't have enough funds!");
    // Deno.exit(0);
}





// Now generate the sessions keys. Maybe
let session_keys = "";
if (flags.sessionKeys == undefined) {
    const options = ["Generate new ones", "Use Custom"];
    const selected_option = display_menu("Sessions keys: ", options)
    if (selected_option == undefined) {
        session_keys = (await localApi.rpc.author.rotateKeys()).toJSON();
    } else {
        session_keys = (await localApi.rpc.author.rotateKeys()).toJSON();
    }
} else {
    session_keys = flags.sessionKeys;
}

console.log("Selected Session Keys: " + session_keys);








localApi.disconnect()
networkApi.disconnect()
Deno.exit(0);



/* const api = await ApiPromise.create({ provider: new WsProvider("ws://127.0.0.1:9944"), rpc: API_RPC, types: API_TYPES, signedExtensions: API_EXTENSIONS  });
const alice = new Keyring({type: 'sr25519'}).addFromUri("//Alice");
const bobAddress = "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty";

const oneAvl = api.registry.createType('Compact<u128>', new BN("1000000000000000000"));
const hash = await api.tx.balances.transfer(bobAddress, oneAvl).signAndSend(alice);
console.log("Transfer sent with hash: " + hash.toHuman())

 */

import { ApiPromise, Keyring, WsProvider } from 'https://deno.land/x/polkadot@0.2.42/api/mod.ts';
import { BN } from 'https://deno.land/x/polkadot@0.2.42/util/mod.ts';
import { API_RPC, API_TYPES, API_EXTENSIONS } from './api_options.ts'
import { parse } from "https://deno.land/std@0.202.0/flags/mod.ts";
import { assert } from 'https://deno.land/std@0.202.0/assert/assert.ts';
import { KeyringPair } from 'https://deno.land/x/polkadot@0.2.42/keyring/types.ts';

const ONE_AVL = new BN("1000000000000000000");
const FLAGS = parse(Deno.args, {
    string: ["endpoint", "sessionKeys", "controller", "stash"]
});


interface State {
    run(machine: StateMachine): Promise<void>;
    next_state(machine: StateMachine): State | undefined,
    debug(): void;
}

function display_menu(title: string, choices: string[], values: string[], default_value: string): string {
    assert(choices.length == values.length);

    while (true) {
        console.log(title);
        choices.forEach((val, i) => {
            console.log(i + ": " + val);
        })
        const choice = prompt(">");
        if (choice == undefined) {
            return default_value;
        }
        const index = Number(choice);
        if (!isNaN(index) && index < choices.length) {
            return values[index];
        }

        console.log("Unknown option " + choice + ". Try again");
    }
}

async function establish_api_connection(endpoint: string): Promise<[ApiPromise, string]> {
    const api = await ApiPromise.create({ provider: new WsProvider(endpoint), rpc: API_RPC, types: API_TYPES, signedExtensions: API_EXTENSIONS  });
    const [chain, nodeName, nodeVersion, runtimeVersion] = await Promise.all([
        api.rpc.system.chain(),
        api.rpc.system.name(),
        api.rpc.system.version(),
        api.rpc.state.getRuntimeVersion(),
    ]);
    const info = `Chain Name: ${chain} | Node Name: ${nodeName} | Node Version: ${nodeVersion} | Spec Version: ${runtimeVersion.specVersion} | Endpoint: ${endpoint}`;
    return [api, info];
}


class LocalNodeConnectionState implements State {
    public api: ApiPromise | undefined = undefined;

    private info = ""
    private initialized = false;

    public async run(_machine: StateMachine) {
        await this.reset();

        const endpoint = this.endpoint_menu();
        [this.api, this.info] = await establish_api_connection(endpoint);
        this.initialized = true;
    }

    public next_state(machine: StateMachine): State | undefined {
        return machine.network_state;
    }

    public debug(): void {
        if (this.initialized == false) {
            return;
        }
        console.log(`Node:\t\t${this.info}`);
    }

    private async reset() {
        if (this.api) {
            await this.api.disconnect();
        }
        this.initialized = false;
        this.api = undefined;
    }

    private endpoint_menu(): string {
        if (FLAGS.endpoint) {
            return FLAGS.endpoint;
        }

        const options = ["Local Host (ws://127.0.0.1:9944) (default)", "Read from Config", "Manually Enter"];
        const values = ["ws://127.0.0.1:9944", "0",  "1"];
        const selected_option = display_menu("Step 1/5: Pick Node Endpoint", options, values, "ws://127.0.0.1:9944")
        if (selected_option == "0") {
            // Read from Config
            return "ws://127.0.0.1:9944";
        }

        if (selected_option == "1") {
            // Enter it manually
            return "ws://127.0.0.1:9944";
        }

        return selected_option;
    }
}

class NetworkConnectionState implements State {
    public api: ApiPromise | undefined = undefined;

    private info = ""
    private go_back = false;
    private initialized = false;

    public async run(_machine: StateMachine) {
        await this.reset();

        const endpoint = this.endpoint_menu();
        if (endpoint) {
            [this.api, this.info] = await establish_api_connection(endpoint);
        }
        this.initialized = true;
    }

    public next_state(machine: StateMachine): State | undefined {
        if (this.go_back) {
            return machine.local_node_state;
        }

        return machine.controller_state;
    }

    public debug(): void {
        if (this.initialized == false) {
            return;
        }
        console.log(`Network:\t${this.info}`);
    }

    private async reset() {
        if (this.api) {
            await this.api.disconnect();
        }
        this.initialized = false;
        this.api = undefined;
        this.go_back = false;
    }

    private endpoint_menu(): string | undefined {
        if (FLAGS.endpoint) {
            return FLAGS.endpoint;
        }

        const options = ["Local Host (ws://127.0.0.1:9944) (default)", "Goldberg (wss://goldberg.avail.tools/ws)", "Read from Config", "Manually Enter", "Return"];
        const values = ["ws://127.0.0.1:9944", "wss://goldberg.avail.tools/ws", "0", "1", "-1"];
        const selected_option = display_menu("Step 2/5: Pick Network Endpoint", options, values, "ws://127.0.0.1:9944");

        if (selected_option == "0") {
            // Read from Config
            return "ws://127.0.0.1:9944";
        }

        if (selected_option == "1") {
            // Enter it manually
            return "ws://127.0.0.1:9944";
        }

        if (selected_option == "-1") {
            this.go_back = true;
            return undefined;
        }

        return selected_option;
    }
}

class ControllerAccountState implements State {
    public account: KeyringPair | undefined = undefined;
    public address = ""
    public balance = new BN("0");

    private go_back = false;
    private initialized = false;

    public async run(machine: StateMachine) {
        const api = machine.network_state.api;
        if (api == undefined) {
            return;
        }
        this.reset();

        const mnemonic = this.mnemonic_menu();
        if (mnemonic == undefined) {
            return;
        }

        this.account = new Keyring({type: 'sr25519'}).addFromMnemonic(mnemonic);
        this.address = this.account.address;

        const ab: any = await api.query.system.account(this.address);
        this.balance = new BN(ab["data"]["free"].toString());
        this.initialized = true;
    }

    public next_state(machine: StateMachine): State | undefined {
        if (this.go_back) {
            return machine.network_state;
        }
        return machine.stash_state;
    }

    public debug(): void {
        if (this.initialized == false) {
            return;
        }

        console.log(`Controller:\tAddress: ${this.address} | Balance: ${this.balance.div(ONE_AVL).toString()} AVL`);
    }

    private reset() {
        this.initialized = false;
        this.account = undefined;
        this.go_back = false;
    }

    private mnemonic_menu(): string | undefined {
        if (FLAGS.controller) {
            return FLAGS.controller;
        }

        const default_value = "bottom drive obey lake curtain smoke basket hold race lonely fit walk//Bob";
        const options = ["Alice", "Bob (Default)", "Dave", "Charlie", "Read from Config", "Manually Enter", "Return"];
        const values = ["Alice", "Bob", "Dave", "Charlie", "0", "1", "-1"];
        const selected_option = display_menu("Step 3/5: Select Controller", options, values, default_value)

        if (selected_option == "0") {
            // Read from Config
            return default_value;
        }

        if (selected_option == "1") {
            // Enter it manually
            return default_value;
        }

        if (selected_option == "-1") {
            this.go_back = true;
            return undefined;
        }

        return selected_option;
    }
}

class StashAccountState implements State {
    public account: KeyringPair | undefined = undefined;
    public address = ""
    public balance = new BN("0");

    private go_back = false;
    private initialized = false;

    public async run(machine: StateMachine) {
        const api = machine.network_state.api;
        if (api == undefined) {
            return;
        }
        this.reset();

        const mnemonic = this.mnemonic_menu();
        if (mnemonic == undefined) {
            return;
        }

        this.account = new Keyring({type: 'sr25519'}).addFromMnemonic(mnemonic);
        this.address = this.account.address;

        const ab: any = await api.query.system.account(this.address);
        this.balance = new BN(ab["data"]["free"].toString());
        this.initialized = true;
    }

    public next_state(machine: StateMachine): State | undefined {
        if (this.go_back) {
            return machine.controller_state;
        }

        return machine.session_keys_state;
    }

    public debug(): void {
        if (this.initialized == false) {
            return;
        }

        console.log(`Stash:\t\tAddress: ${this.address} | Balance: ${this.balance.div(ONE_AVL).toString()} AVL`);
    }

    private reset() {
        this.account = undefined;
        this.initialized = false;
        this.go_back = false;
    }

    private mnemonic_menu(): string | undefined {
        if (FLAGS.controller) {
            return FLAGS.controller;
        }

        const default_value = "bottom drive obey lake curtain smoke basket hold race lonely fit walk//Bob";
        const options = ["Alice", "Bob (Default)", "Dave", "Charlie", "Read from Config", "Manually Enter", "Return"];
        const values = ["Alice", "Bob", "Dave", "Charlie", "0", "1", "-1"];
        const selected_option = display_menu("Step 4/5: Select Stash", options, values, default_value)

        if (selected_option == "0") {
            // Read from Config
            return default_value;
        }

        if (selected_option == "1") {
            // Enter it manually
            return default_value;
        }

        if (selected_option == "-1") {
            this.go_back = true;
            return undefined;
        }

        return selected_option;
    }
}

class SessionKeysState implements State {
    public session_keys: string | undefined = undefined;

    private go_back = false;
    private initialized = false;

    public async run(machine: StateMachine) {
        const api = machine.local_node_state.api;
        if (api == undefined) {
            return;
        }
        this.reset();

        const options = ["Use Author.RotateKeys RPC", "Generate and Inject based on Controller and Stash", "Read from Config", "Manually Enter", "Return"];
        const values = ["0", "1", "2", "3", "-1"];
        const selected_option = display_menu("Step 5/5: Select Session Keys: ", options, values, "0");
        if (selected_option == "-1") {
            this.go_back = true;
            return;
        }


        this.session_keys = (await api.rpc.author.rotateKeys()).toJSON();
        this.initialized = true;
    }

    public next_state(machine: StateMachine): State | undefined {
        if (this.go_back) {
            return machine.stash_state;
        }

        return undefined;
    }

    public debug(): void {
        if (this.initialized == false) {
            return;
        }

        console.log(`Sessions Keys:\tValue: ${this.session_keys}`);
    }

    private reset() {
        this.initialized = false;
        this.session_keys = undefined;
        this.go_back = false;
    }
}


class StateMachine {
    current_state: State | undefined = undefined;
    local_node_state = new LocalNodeConnectionState();
    network_state = new NetworkConnectionState();
    controller_state = new ControllerAccountState();
    stash_state = new StashAccountState();
    session_keys_state = new SessionKeysState();

    constructor() {
        this.current_state = this.local_node_state;
    }

    public async run(): Promise<boolean> {
        this.debug();
        
        if (this.current_state == undefined) {
            return false;
        }
        await this.current_state.run(this);
        this.current_state = this.current_state.next_state(this);

        return true;
    }

    private debug() {
        console.clear();
        this.local_node_state.debug();
        this.network_state.debug();
        this.controller_state.debug();
        this.stash_state.debug();
        this.session_keys_state.debug();
    }
}

const machine = new StateMachine();
while (true) {
    const pending = await machine.run();
    if (!pending) {
        break;
    }
}


Deno.exit(0);

/* class ProgramState {
    networkInfo: string | undefined = undefined;
    networkApi: ApiPromise | undefined = undefined;
    localInfo: string | undefined = undefined;
    localApi:ApiPromise | undefined = undefined;
    controller_address: string | undefined = undefined;
    controller_funds: BN | undefined = undefined;
    stash_address: string | undefined = undefined;
    stash_funds: BN | undefined = undefined;

    display() {
        if (this.networkInfo) {
            console.log(`Network:\t${this.networkInfo}`);
        }
        if (this.localInfo) {
            console.log(`Node:\t\t${this.localInfo}`);
        }
        if (this.controller_address && this.controller_funds) {
            console.log(`Controller:\tAddress: ${this.controller_address} | Balance: ${this.controller_funds.div(ONE_AVL).toString()} AVL`);
        }
        if (this.stash_address && this.stash_funds) {
            console.log(`Stash:\t\tAddress: ${this.stash_address} | Balance: ${this.stash_funds.div(ONE_AVL).toString()} AVL`);
        }
    }

    async run() {
        if (this.networkApi == undefined) {
            await this.network_connect();
            return;
        }

        if (this.localApi == undefined) {
            await this.local_node_connect();
            return;
        }

        if (this.controller_address == undefined) {
            await this.read_controller_account();
            return;
        }

        if (this.stash_address == undefined) {
            await this.read_stash_account();
            return;
        }
    }

    async disconnect() { 
        if (this.localApi) {
            await this.localApi.disconnect()
        }
        if (this.networkApi) {
            await this.networkApi.disconnect();
        }
    }

    private async network_connect() {
        let endpoint = "ws://127.0.0.1:9944"
        if (FLAGS.endpoint == undefined) {
            const options = ["Local Host (ws://127.0.0.1:9944) (default)", "Goldberg (wss://goldberg.avail.tools/ws)", "Custom"];
            const endpoints = ["ws://127.0.0.1:9944", "wss://goldberg.avail.tools/ws", "Custom"];
            const selected_option = display_menu("Pick Endpoint", options)
            if (selected_option != undefined) {
                endpoint = endpoints[selected_option]
            }
        } else {
            endpoint = FLAGS.endpoint;
        }

        const api = await ApiPromise.create({ provider: new WsProvider(endpoint), rpc: API_RPC, types: API_TYPES, signedExtensions: API_EXTENSIONS  });
        const [chain, nodeName, nodeVersion, runtimeVersion] = await Promise.all([
            api.rpc.system.chain(),
            api.rpc.system.name(),
            api.rpc.system.version(),
            api.rpc.state.getRuntimeVersion(),
        ]);
        console.log(`Connected to chain ${chain} using ${nodeName}, node version ${nodeVersion} and spec version ${runtimeVersion.specVersion}`);
        this.networkInfo = `Chain Name: ${chain} | Node Name: ${nodeName} | Node Version: ${nodeVersion} | Spec Version: ${runtimeVersion.specVersion} | Endpoint: ${endpoint}`;
        this.networkApi = api;
    }

    private async local_node_connect() {
        const api = await ApiPromise.create({ provider: new WsProvider("ws://127.0.0.1:9944"), rpc: API_RPC, types: API_TYPES, signedExtensions: API_EXTENSIONS  });
        const [chain, nodeName, nodeVersion, runtimeVersion] = await Promise.all([
            api.rpc.system.chain(),
            api.rpc.system.name(),
            api.rpc.system.version(),
            api.rpc.state.getRuntimeVersion(),
        ]);
        console.log(`Connected to chain ${chain} using ${nodeName}, node version ${nodeVersion} and spec version ${runtimeVersion.specVersion}`);
        this.localInfo = `Chain Name: ${chain} | Node Name: ${nodeName} | Node Version: ${nodeVersion} | Spec Version: ${runtimeVersion.specVersion} | Endpoint: ws://127.0.0.1:9944`;
        this.localApi = api;
    }

    private async read_controller_account() {
        if (this.networkApi == undefined) {
            return;
        }

        let account_mnemonic = "bottom drive obey lake curtain smoke basket hold race lonely fit walk//Bob";
        if (FLAGS.controller == undefined) {
            const options = ["Alice", "Bob", "Dave", "Charlie", "Custom"];
            const selected_option = display_menu("Controller account mnemonic: ", options)
            if (selected_option == undefined) {
                account_mnemonic = "bottom drive obey lake curtain smoke basket hold race lonely fit walk//Bob";
            }
        } else {
            account_mnemonic = FLAGS.controller;
        }

        const account = new Keyring({type: 'sr25519'}).addFromMnemonic(account_mnemonic);
        this.controller_address = account.address;

        const ab: any = await this.networkApi.query.system.account(account.address);
        this.controller_funds = new BN(ab["data"]["free"].toString());
    }

    private async read_stash_account() {
        if (this.networkApi == undefined) {
            return;
        }

        let account_mnemonic = "bottom drive obey lake curtain smoke basket hold race lonely fit walk//Bob";
        if (FLAGS.stash == undefined) {
            const options = ["Alice", "Bob", "Dave", "Charlie", "Same as Controller", "Custom"];
            const selected_option = display_menu("Controller account mnemonic: ", options)
            if (selected_option == undefined) {
                account_mnemonic = "bottom drive obey lake curtain smoke basket hold race lonely fit walk//Bob";
            }
        } else {
            account_mnemonic = FLAGS.stash;
        }

        const account = new Keyring({type: 'sr25519'}).addFromMnemonic(account_mnemonic);
        this.stash_address = account.address;

        const ab: any = await this.networkApi.query.system.account(account.address);
        this.stash_funds = new BN(ab["data"]["free"].toString());

        {
            const a = await this.networkApi.query.staking.minValidatorBond();
            const minimum_stash_balance = new BN(a.toString());
            if (this.stash_funds.lt(minimum_stash_balance)) {
                console.log("Stash Account doesn't have enough funds!");
                // Deno.exit(0);
            }      
        }   
    }

    private async generate_session_keys() {
        if (this.localApi == undefined) {
            return;
        }

        let session_keys = "";
        if (FLAGS.sessionKeys == undefined) {
            const options = ["Generate new ones", "Use Custom"];
            const selected_option = display_menu("Sessions keys: ", options)
            if (selected_option == undefined) {
                session_keys = (await this.localApi.rpc.author.rotateKeys()).toJSON();
            } else {
                session_keys = (await this.localApi.rpc.author.rotateKeys()).toJSON();
            }
        } else {
            session_keys = FLAGS.sessionKeys;
        }

        console.log("Selected Session Keys: " + session_keys);
    }
}

const program_state = new ProgramState(); 

program_state.disconnect(); */




/* const api = await ApiPromise.create({ provider: new WsProvider("ws://127.0.0.1:9944"), rpc: API_RPC, types: API_TYPES, signedExtensions: API_EXTENSIONS  });
const alice = new Keyring({type: 'sr25519'}).addFromUri("//Alice");
const bobAddress = "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty";

const oneAvl = api.registry.createType('Compact<u128>', new BN("1000000000000000000"));
const hash = await api.tx.balances.transfer(bobAddress, oneAvl).signAndSend(alice);
console.log("Transfer sent with hash: " + hash.toHuman())

 */

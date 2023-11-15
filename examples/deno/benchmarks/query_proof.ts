import { ApiPromise, Keyring, WsProvider } from 'https://deno.land/x/polkadot@0.2.42/api/mod.ts';
import { API_RPC, API_TYPES, API_EXTENSIONS } from './../api_options.ts'
import { waitForBlockInclusion, waitForBlockFinalization } from './misc.ts';

const api = await ApiPromise.create({ provider: new WsProvider("ws://127.0.0.1:9944"), rpc: API_RPC, types: API_TYPES, signedExtensions: API_EXTENSIONS  });
const alice = new Keyring({type: 'sr25519'}).addFromUri("//Alice");


console.log("Preparing data...")
const txCount = 100;
const data: string[] = [];

const characters = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789';
const charactersLength = characters.length;
for (let i = 0; i < txCount; ++i) {
    let array = '';
    let counter = 0;
    while (counter < 16 * 1024) {
        array += characters.charAt(Math.floor(Math.random() * charactersLength));
        counter += 1;
    }
    data.push(array);
}

console.log("Waiting for a new block to appear (this might take a while)...")
const currentBlockNumber = (await api.rpc.chain.getHeader()).number.toNumber();
await waitForBlockInclusion(api, currentBlockNumber + 1);

console.log("Submitting Data...")
let nonce = (await api.rpc.system.accountNextIndex(alice.address)).toNumber();
const txs = [];
for (let i = 0; i < txCount; ++i) {
    txs.push(api.tx.dataAvailability.submitData(data[i].toString()).signAndSend(alice, {nonce: nonce}));
    nonce += 1;
}
await Promise.all(txs);

console.log("Waiting for txs to be finalized (this might take a while)...")
const targetBlockHash = await waitForBlockFinalization(api, currentBlockNumber + 2);

console.log("Defining cells...")
const count = 8500;
const cells = [];
for (let i = 0; i < 256; ++i) {
    for (let j = 0; j < 256; ++j) {
        cells.push([i, j]);
    }
}

let end = 0;
const promises = [];
console.log("Querying 8.5k cells...")
performance.mark("start");
for (let counter = 0; counter < count; counter += 30) {
    end = counter + 30;
    end = end > count ? count : end;
    promises.push(api.rpc.kate.queryProof(cells.slice(counter, end), targetBlockHash))
}

await Promise.all(promises);
performance.mark("end");
console.log(performance.measure('8.5k cell retrieval', 'start', 'end'))

Deno.exit(0);
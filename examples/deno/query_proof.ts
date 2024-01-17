import { ApiPromise, Keyring, WsProvider } from 'https://deno.land/x/polkadot@0.2.42/api/mod.ts';
import { ISubmittableResult } from "https://deno.land/x/polkadot@0.2.42/types/types/extrinsic.ts";
import { H256 } from "https://deno.land/x/polkadot@0.2.42/types/interfaces/types.ts";
import { API_RPC, API_TYPES, API_EXTENSIONS } from './api_options.ts'

const api = await ApiPromise.create({ provider: new WsProvider("ws://127.0.0.1:9944"), rpc: API_RPC, types: API_TYPES, signedExtensions: API_EXTENSIONS  });
const alice = new Keyring({type: 'sr25519'}).addFromUri("//Alice");

// submit data transaction
const data = "Hello World";
const submitData = new Promise<[H256, H256]>((res, _) => {
    api.tx.dataAvailability.submitData(data).signAndSend(alice, (result: ISubmittableResult) => {
        if (result.isInBlock) {
            console.log("Waiting for block finalization...");
        } else if (result.isFinalized) {
            res([result.status.asFinalized as H256, result.txHash as H256])
        }
    });
});

console.log("Submitting TX...");
const [blockHash, txHash] = await submitData;
console.log(`Finalized block hash ${blockHash} and tx hash ${txHash}`);

const proof = await api.rpc.kate.queryProof([{row: 0, col: 0}], blockHash);
console.log(new Uint8Array(proof))

Deno.exit(0)
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
        console.log(`Tx status: ${result.status}`);
        if (result.isInBlock) {
            console.log(`Included at block hash ${result.status.asInBlock.toHex()}`);
            console.log("Waiting for finalization...");
        } else if (result.isFinalized) {
            res([result.status.asFinalized as H256, result.txHash as H256])
        }
    });
});

const [blockHash, txHash] = await submitData;
console.log(`Finalized block hash ${blockHash} and tx hash ${txHash}`);

// Extract data
const block = await api.rpc.chain.getBlock(blockHash);
block.block.extrinsics.forEach((tx, index) => {
    if (tx.hash.toHex() != txHash.toHex()) {
        return;
    }
    
    console.log(index, tx.toHuman());
    const {method: {args, method, section}} = tx;
    const dataHex = args.map(a => a.toString()).join(', ');
    //data retrieved from the extrinsic data
    let str = ''
    for (let n = 0; n < dataHex.length; n += 2) {
        str += String.fromCharCode(parseInt(dataHex.substring(n, n + 2), 16));
    }
    console.log(`Transaction hash: ${tx.hash}, submitted data: ${str}`);
    console.log(`${section}.${method}(${args.map(a => a.toString()).join(', ')})`);
})


Deno.exit(0)
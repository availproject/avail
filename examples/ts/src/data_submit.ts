import {Keyring} from '@polkadot/api';
import {createApi} from './api';
import config from "./config";
import {H256} from "@polkadot/types/interfaces";
import {ISubmittableResult} from "@polkadot/types/types";

/**
 * Extracts data from the block.
 */
async function extractData(api: any, blockHash: H256, exHash: H256) {
    const block = await api.rpc.chain.getBlock(blockHash);
    let extrinsics = block.block.extrinsics;
    for (const ex of extrinsics) {
        if (ex.hash == exHash.toHex()) {
            const index: number = extrinsics.indexOf(ex);
            console.log(index, ex.toHuman());
            const {method: {args, method, section}} = ex;
            let dataHex = args.map((a: any) => a.toString()).join(', ');
            //data retrieved from the extrinsic data
            let str = ''
            for (let n = 0; n < dataHex.length; n += 2) {
                str += String.fromCharCode(parseInt(dataHex.substring(n, n + 2), 16));
            }
            console.log(`Extrinsic hash: ${ex.hash}, submitted data: ${str}`);
            console.log(`${section}.${method}(${args.map((a: any) => a.toString()).join(', ')})`);
        }
    }
    process.exit(0);
}

async function main() {
    // instantiate the API
    const api = await createApi()
    // construct the keyring after the API
    const keyring = new Keyring({type: 'sr25519'});
    // add Alice to our keyring with a hard-derivation path (empty phrase, so uses dev)
    const alice = keyring.addFromUri(config.mnemonic);
    // data to submit
    const data = "";

    // submit data transaction
    api.tx.dataAvailability.submitData(data)
        .signAndSend(alice, async (result: ISubmittableResult) => {
            console.log(`Tx status: ${result.status}`);
            if (result.isInBlock) {
                console.log(`Included at block hash ${result.status.asInBlock.toHex()}`);
                // uncomment this lines of code to track all events
                // console.log('Events:');
                // result.events.forEach(({event: {data, method, section}, phase}) => {
                //     console.log('\t', phase.toString(), `: ${section}.${method}`, data.toString());
                // });
            } else if (result.isFinalized) {
                const blockHash: H256 = result.status.asFinalized;
                const exHash: H256 = result.txHash;
                console.log(`Finalized block hash ${blockHash}`);
                // get data from finalized block
                await extractData(api, blockHash, exHash);
            }
        });
}

main()
    .catch((err) => {
        console.error(err);
        process.exit(1);
    })


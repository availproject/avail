import { ApiPromise, WsProvider, Keyring } from '@polkadot/api';
import { KeyringPair } from '@polkadot/keyring/types';
import type { EventRecord, ExtrinsicStatus, H256 } from '@polkadot/types/interfaces';
import type { ISubmittableResult, SignatureOptions } from '@polkadot/types/types';
import config from './config';
import { createApi } from './api';

const keyring = new Keyring({ type: 'sr25519' });

interface SignatureOptionsNew extends SignatureOptions {
    app_id: number
}

//async funtion to get the nonce    
async function getNonce(api: ApiPromise, address: string): Promise<number> {
    const nonce = (await api.rpc.system.accountNextIndex(address)).toNumber();
    return nonce;
}

async function Transfer(api: ApiPromise) {
    try {
        const acc = keyring.addFromUri(config.mnemonic);  //and its address can be used by `acc.address`
        let nonce1 = await getNonce(api, acc.address);
        let amount = 0;
        if (config.amount > 0) {
            amount = config.amount;
        } else {
            amount = 12345;
        }
        /* @note here app_id is 1,
        but if you want to have one your own then create one first before initialising here */
        const options: Partial<any> = { app_id: 0, nonce: nonce1 }
        const res = await api.tx.balances.transfer(config.receiver, amount)
            .signAndSend(
                acc,  // sender
                options, // options
                (result: ISubmittableResult) => {
                    //uncomment the below line👇 to see the whole status flow of the transaction
                    // console.log(`Tx status: ${result.status}`);
                    if (result.status.isReady) {
                        console.log(`result is ready with nonce ${nonce1}`)
                    }
                    if (result.status.isInBlock) {
                        let block_hash = result.status.asInBlock;
                        let extrinsic_hash = result.txHash;
                        console.log(`\nExtrinsic hash: ${result.txHash} with nonce ${nonce1} is in block`);
                        process.exit(0);
                    }
                });
    } catch (e) {
        console.log(e);
        process.exit(1);
    }
}

async function main() {
    const api = await createApi();
    const metadata = await api.rpc.state.getMetadata();
    console.log(config.amount);
    await Transfer(api);
}

main().catch((err) => {
    console.error(err);
    process.exit(1);
});
import { ApiPromise, Keyring } from '@polkadot/api';
import type { ISubmittableResult } from '@polkadot/types/types';
import config from './config';
import { createApi, getNonce } from './api';

const keyring = new Keyring({ type: 'sr25519' });

async function transfer(api: ApiPromise) {
    try {
        const acc = keyring.addFromUri(config.mnemonic); // and its address can be used by `acc.address`
        const nonce = await getNonce(api, acc.address);
        const amount = config.amount > 0 ? config.amount : 12345;
        /* @note here app_id is 1,
            but if you want to have one your own then create one first before initialising here */
        const options: Partial<any> = { app_id: 0, nonce };
        await api.tx.balances.transfer(config.receiver, amount)
            .signAndSend(
                acc, // sender
                options, // options
                (result: ISubmittableResult) => {
                    // uncomment the below line👇 to see the whole status flow of the transaction
                    // console.log(`Tx status: ${result.status}`);
                    if (result.status.isReady) {
                        console.log(`result is ready with nonce ${nonce}`);
                    }
                    if (result.status.isInBlock) {
                        console.log(`\nExtrinsic hash: ${result.txHash} with nonce ${nonce} is in block`);
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
    console.log(`Amount: ${config.amount}`);
    await transfer(api);
}

main().catch((err) => {
    console.error(err);
    process.exit(1);
});

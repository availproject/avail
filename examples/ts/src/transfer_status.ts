import {Keyring} from '@polkadot/api';
import type {ISubmittableResult} from '@polkadot/types/types';
import config from './config';
import {createApi} from './api';
import '@polkadot/api-augment';

const keyring = new Keyring({type: 'sr25519'});

/**
 * Example of transfer token from Alice to Bob with status end event tracking.
 */
async function main() {
    const api = await createApi();
    // account that is making a token transfer
    const sender = keyring.addFromUri(config.mnemonic);
    // account that is receiving tokens
    const receiver = config.receiver;
    // default config amount of tokens to transfer
    let amount = config.amount;

    // get available balance from the account
    const {data: {free: freeBalance}} = await api.query.system.account(sender.address);
    console.log(`Available balance of ${sender.address}: ${freeBalance}`);

    // make a transfer and track the status of the transaction
    await api.tx.balances.transfer(receiver, amount)
        .signAndSend(
            sender,
            (result: ISubmittableResult) => {
                console.log(`Tx status: ${result.status}`);
                if (result.status.isInBlock) {
                    console.log(`Extrinsic hash: ${result.txHash} is in block ${result.status.asInBlock.toHex()}`);
                    console.log("Events: ");
                    result.events.forEach(({event: {data, method, section}, phase}) => {
                        console.log('\t', phase.toString(), `: ${section}.${method}`, data.toString());
                    });
                } else if (result.status.isFinalized) {
                    console.log(`Finalized block ${result.status.asFinalized.toHex()}`);
                    process.exit(0);
                }
            });
}

main()
    .catch((err) => {
        console.error(err);
        process.exit(1);
    });

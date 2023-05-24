import {ApiPromise, WsProvider, Keyring} from '@polkadot/api';
import {KeyringPair} from '@polkadot/keyring/types';
import type {EventRecord, ExtrinsicStatus, H256} from '@polkadot/types/interfaces';
import type {ISubmittableResult, SignatureOptions} from '@polkadot/types/types';
import config from './config';
import {createApi} from './api';
import '@polkadot/api-augment';

const keyring = new Keyring({type: 'sr25519'});


/**
 * Example of transfer token from Alice to Bob with status end event tracking.
 */
async function main() {
    const api = await createApi();
    // alice is making a token transfer to bob
    const alice = keyring.addFromUri(config.mnemonic);
    const bob = config.receiver;
    // default config amount of tokens to transfer
    let amount = config.amount;

    // get available balance from the account
    const {data: {free: freeBalance}} = await api.query.system.account(alice.address);
    console.log(`Available balance of ${alice.address}: ${freeBalance}`);

    // make a transfer and track the status of the transaction
    await api.tx.balances.transfer(bob, amount)
        .signAndSend(
            alice,
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
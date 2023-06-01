import {ApiPromise, Keyring} from '@polkadot/api';
import {KeyringPair} from '@polkadot/keyring/types';
import type {ISubmittableResult} from '@polkadot/types/types';
import yargs from 'yargs/yargs';
import {createApi} from './api';


async function cli_arguments() {
    return yargs(process.argv.slice(2)).options({
        i: {
            description: 'app id to be given',
            alias: 'app_id',
            type: 'string',
            default: '1'
        }

    }).argv;
}

async function createKey(api: ApiPromise, sender: KeyringPair, id: string) {
    try {
        await api.tx.dataAvailability.createApplicationKey(id)
            .signAndSend(
                sender,
                (result: ISubmittableResult) => {
                    console.log(`Tx status: ${result.status}`);
                    if (result.status.isFinalized) {
                        let block_hash = result.status.asFinalized;
                        let extrinsic_hash = result.txHash;
                        console.log(`\nBlock finalized, extrinsic hash: ${extrinsic_hash}\nin block ${block_hash}`);
                        process.exit(0);
                    }
                });
    } catch (e) {
        console.log(e);
        process.exit(1);
    }
}

async function main() {
    const argv = await cli_arguments();
    const api = await createApi();

    const keyring = new Keyring({type: 'sr25519'});
    const bob = keyring.addFromUri('//Bob');
    // creating api key
    console.log(`Creating api key ${argv.i}`);
    await createKey(api, bob, argv.i);
}

main().catch((err) => {
    console.error(err);
    process.exit(1);
});
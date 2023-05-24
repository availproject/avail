import { ApiPromise, Keyring } from '@polkadot/api';
import { KeyringPair } from '@polkadot/keyring/types';
import type { ISubmittableResult } from '@polkadot/types/types';
import yargs from 'yargs/yargs';
import { createApi, getNonce } from './api';

const keyring = new Keyring({ type: 'sr25519' });

async function cli_arguments() {
  return yargs(process.argv.slice(2)).options({
    i: {
      description: 'app id to be given',
      alias: 'app_id',
      type: 'string',
      default: '1',
    },

  }).argv;
}

async function createKey(
  api: ApiPromise,
  sender: KeyringPair,
  nonce: number,
  id: string,
): Promise<any> {
  try {
    /* @note here app_id is 1,
        but if you want to have one your own then create one first before initialising here */
    const options: Partial<any> = { app_id: 0, nonce };
    await api.tx.dataAvailability.createApplicationKey(id)
      .signAndSend(
        sender, // sender
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
  const argv = await cli_arguments();
  const api = await createApi();
  const bob = keyring.addFromUri('//Bob');
  const nonce = await getNonce(api, bob.address);
  /* @note: here BOB test account is used.
    You can use your own account mnemonic using the below code
    // const mnemonic = 'your mneomnic';
    // const acc = keyring.addFromUri(Mnemonic, 'sr25519');
    // and its address can be used by `acc.address`
    */
  await createKey(api, bob, nonce, argv.i);
}

main().catch((err) => {
  console.error(err);
  process.exit(1);
});

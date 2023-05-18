import { ApiPromise, Keyring } from '@polkadot/api';
import { KeyringPair } from '@polkadot/keyring/types';
import type { H256 } from '@polkadot/types/interfaces';
import type { ISubmittableResult } from '@polkadot/types/types';
import config from './config';
import { createApi, getNonce } from './api';

const keyring = new Keyring({ type: 'sr25519' });

const generateData = (size: number) => {
  const buffer = Buffer.alloc(size);
  for (let i = 0; i < size; i++) {
    buffer.writeUInt8(Math.floor(Math.random() * 256), i);
  }
  return buffer.toString('hex');
};

async function sendTx(api: ApiPromise, sender: KeyringPair, nonce: number): Promise<any> {
  try {
    const data = generateData(config.size);
    const submit = api.tx.dataAvailability.submitData(data);
    /* @note here app_id is 1,
        but if you want to have one your own then create one first before initialising here */
    const options: Partial<any> = { app_id: config.app_id, nonce };
    await submit
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
            const block_hash = result.status.asInBlock;
            const extrinsic_hash = result.txHash;
            console.log(`\nExtrinsic hash: ${result.txHash} with nonce ${nonce} is in block`);
            // block(block_hash, api);
            if (config.batch <= 1 || config.batch == undefined) {
              setTimeout(() => {
                get(api, block_hash, extrinsic_hash);
              }, 5000);
            }
          }
        });
  } catch (e) {
    console.log(e);
    process.exit(1);
  }
}

const sendTxs = async (api: ApiPromise, sender: KeyringPair, nonce: number) => {
  const results = [];
  for (let i = 0; i < config.batch; i++) {
    const result = await sendTx(api, sender, nonce);
    results.push(result);
    nonce = nonce + 1;
  }
};

// function to retreive data
async function get(api: any, block_hash: H256, extrinsic_hash: H256) {
  const block = await api.rpc.chain.getBlock(block_hash);
  const extrinsics = block.block.extrinsics;
  extrinsics.forEach(async (ex: any, index: number) => {
    if (extrinsic_hash == ex.hash.toHex()) {
      console.log(index, ex.toHuman());
      const { method: { args, method, section } } = ex;
      const data_hex = args.map((a: any) => a.toString()).join(', ');
      // data retreived from the extrinsic data
      let str = '';
      for (let n = 0; n < data_hex.length; n += 2) {
        str += String.fromCharCode(parseInt(data_hex.substr(n, 2), 16));
      }
      console.log(`\n 💡 DATA_SUBMITTED : ${str}`);
      console.log(`${section}.${method}(${args.map((a: any) => a.toString()).join(', ')})`);
    }
  });
  process.exit(0);
}

async function main() {
  const api = await createApi();
  const acc = keyring.addFromUri(config.mnemonic); // and its address can be used by `acc.address`
  const nonce1 = await getNonce(api, acc.address);
  if (config.batch > 1) {
    await sendTxs(api, acc, nonce1);
  } else if (config.batch <= 1 || config.batch == undefined) {
    await sendTx(api, acc, nonce1);
  } else {
    console.log('invalid input');
  }
}

main().catch((err) => {
  console.error(err);
  process.exit(1);
});

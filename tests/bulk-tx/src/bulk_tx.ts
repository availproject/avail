import { ApiPromise, WsProvider, Keyring } from '@polkadot/api';
import { SignerResult } from '@polkadot/api/types';
import { KeyringPair } from '@polkadot/keyring/types';
import type { EventRecord, ExtrinsicStatus, H256, SignedBlock } from '@polkadot/types/interfaces';
import type { ISubmittableResult, SignatureOptions } from '@polkadot/types/types';

const ALICE = '5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY';
const keyring = new Keyring({ type: 'sr25519' });

//batch size and payload size in bytes are set here
const batch:number = 3;
const size:number = 100; //payload

async function createApi(): Promise<ApiPromise> {
  // Initialise the provider to connect to the local node
  // const provider = new WsProvider('wss://polygon-da-explorer.matic.today/ws');
  const provider = new WsProvider('ws://127.0.0.1:9944');

  // Create the API and wait until ready
  return ApiPromise.create({
    provider,
    types: {
      DataLookup: {
        size: 'u32',
        index: 'Vec<(u32,u32)>'
      },
      KateExtrinsicRoot: {
        hash: 'Hash',
        commitment: 'Vec<u8>',
        rows: 'u16',
        cols: 'u16'
      },
      KateHeader: {
        parentHash: 'Hash',
        number: 'Compact<BlockNumber>',
        stateRoot: 'Hash',
        extrinsicsRoot: 'KateExtrinsicRoot',
        digest: 'Digest',
        appDataLookup: 'DataLookup'
      },
      Header: 'KateHeader',
      AppId: 'u32',
      // CheckAppId: {
      //     extra: {
      //         appId: 'u32', 
      //     },
      //     types: {}
      // }
    },
    signedExtensions: {
      CheckAppId: {
        extrinsic: {
          appId: 'u32'
        },
        payload: {}
      },
    },
  });
}

//generating random data as per payload specified
function randomDigit() {
  return Math.floor(Math.random() * Math.floor(2));
}

function generateRandomBinary(size:number) {
  let binary = "0x";
  for(let i = 0; i < size; ++i) {
    binary += randomDigit();
  }
  console.log(binary);
  return binary;
}

async function getNonce(api: ApiPromise, address: any): Promise<number> {
  const nonce = (await api.rpc.system.accountNextIndex(address)).toNumber();
  return nonce;
}

interface SignatureOptionsNew extends SignatureOptions {
  app_id: number
}

async function sendTx(api: ApiPromise, sender: KeyringPair, nonce: number): Promise<any> {
  try {
    let data = generateRandomBinary(size);
    let submit = await api.tx.dataAvailability.submitData(data);
    /* @note here app_id is 1,
    but if you want to have one your own then create one first before initialising here */
    const options: Partial<any> = { app_id: 1, nonce: nonce }
    const res = await submit
      .signAndSend(
        sender,  // sender
        options, // options
        (result: ISubmittableResult) => {
          //uncomment the below line👇 to see the whole status flow of the transaction
          // console.log(`Tx status: ${result.status}`);
          if (result.status.isReady) {
            console.log(`result is ready with nonce ${nonce}`)
          }
          if (result.status.isInBlock) {
            let block_hash = result.status.asInBlock;
            console.log(`\nExtrinsic hash: ${result.txHash} with nonce ${nonce} is in block`);
          }

        });

    return res;
  } catch (e) {
    console.log(e);
    process.exit(0);
  }
}

const sendTxs = async (api: ApiPromise, sender: KeyringPair, nonce: number) => {

  let non = await getNonce(api, sender.address);
  const results = [];
  for (let i = 0; i < batch; i++) {
    const result = await sendTx(api, sender, non)
    results.push(result);
    non = non + 1
  }

  return results;
}

async function main() {

  const api = await createApi();
  const alice = keyring.addFromUri('//Alice');
  const bob = keyring.addFromUri('//Bob');
  const metadata = await api.rpc.state.getMetadata();
  /*@note: here ALICE test account is used.
  You can use your own account mnemonic using the below code
  const mnemonic = 'your mneomnic';
  const acc = keyring.addFromUri(Mnemonic, 'sr25519'); and its address can be used by `acc.address`
  */
  let nonce = await getNonce(api, ALICE);
  let tx = await sendTxs(api, alice, nonce);

}
main().catch(console.error)


import { ApiPromise, WsProvider, Keyring } from '@polkadot/api';
import { SignerResult } from '@polkadot/api/types';
import type { EventRecord, ExtrinsicStatus, H256, SignedBlock } from '@polkadot/types/interfaces';
import type { ISubmittableResult ,SignatureOptions } from '@polkadot/types/types';

const ALICE = '5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY';
const HERMEZ = '5CAezZ6kHGHY2ZUVvpqpJ9v7xSPm5Xhdj2F7H7k2iKyRzRDH';
const herMnemonic = 'very noise grant jealous elegant update famous own pulp tongue soccer true';

const keyring = new Keyring({ type: 'sr25519' });

const batch = 3;
const size = 100;

async function createApi():Promise<ApiPromise> {
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

function generateData(size:number):string {
    let randomChars = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789';
    let result = '';
    for ( let i = 0; i < size; i++ ) {
        result += randomChars.charAt(Math.floor(Math.random() * randomChars.length));
    }
    return result
}

async function getNonce (api:ApiPromise, address: any) {
  const nonce = (await api.rpc.system.accountNextIndex(address)).toNumber();
  return nonce;
}

interface SignatureOptionsNew extends SignatureOptions {
  app_id: number
}

async function sendTx(api:ApiPromise, sender:any, nonce:any):Promise<any> {
    try{
      let data = generateData(size);
      const nonc = await api.rpc.system.accountNextIndex(sender.address);
      let submit = await api.tx.dataAvailability.submitData(data);
      const options: Partial<any> = {app_id: 1, nonce: -1 }
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

const sendTxs = async (api:any, sender:any, nonce:any) => {

  let non = await getNonce(api, sender.address);
  const results = [];
  for (let i=0 ; i<batch; i++) {
    const result = await sendTx(api, sender, non)
    results.push(result);
    non = non + 1
  }

  return results ;
}



// async function sub(tx:any){
//   return await Promise.all(tx);
// }


async function main() {

    const api = await createApi(); 
    const alice = keyring.addFromUri('//Alice');
    const bob = keyring.addFromUri('//Bob');
    const metadata = await api.rpc.state.getMetadata();
    let nonce = await getNonce(api, ALICE);
    let tx = await sendTxs(api, alice, nonce); 

}
main().catch(console.error)


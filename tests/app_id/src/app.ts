const { ApiPromise, WsProvider, Keyring } = require('@polkadot/api');
const {mnemonicGenerate, cryptoWaitReady } = require('@polkadot/util-crypto');


import type { EventRecord, ExtrinsicStatus, H256, SignedBlock } from '@polkadot/types/interfaces';
import type { ISubmittableResult} from '@polkadot/types/types';

const ALICE = '5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY';
const BOB = '5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty';

const keyring = new Keyring({ type: 'sr25519' });

async function createApi() {
  // Initialise the provider to connect to the local node
  // const provider = new WsProvider('ws://127.0.0.1:9944');
  const provider = new WsProvider('wss://polygon-da-explorer.matic.today/ws');

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
        CheckAppId: {
            extra: {
                appId: 'u32', 
            },
            types: {}
        }
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

async function main () {
  // Create the API and wait until ready
  const api = await createApi(); 

  const keyring = new Keyring({ type: 'sr25519'});
  const mnemonic = `inject fiscal misery fiscal success weasel black tube satisfy rural sauce reveal`;

  const acc = keyring.addFromMnemonic(mnemonic, { name: 'test_pair' },'sr25519');
  const alice = keyring.addFromUri('//Alice');

  // Retrieve the chain & node information information via rpc calls
  const [chain, nodeName, nodeVersion] = await Promise.all([
    api.rpc.system.chain(),
    api.rpc.system.name(),
    api.rpc.system.version()
  ]);
  
  console.log(`You are connected to chain ${chain} using ${nodeName} v${nodeVersion}`);
    try{
    let APP_ID = 1;
    let VALUE = `iucakcbak`;
    let transfer = api.tx.dataAvailability.submitData(VALUE);
    const unsub = await transfer
        .signAndSend(
          alice, 
          { app_id: 1}, 
          ( result: ISubmittableResult ) => {
              console.log(`Tx status: ${result.status}`);
    
              if (result.status.isInBlock) {
                  console.log(`Tx included at block hash ${result.status.asInBlock}`);
              } else if (result.status.isFinalized) {
                  console.log(`Tx included at blockHash ${result.status.asFinalized}`);
    
                  result.events.forEach(({ phase, event: { data, method, section } }) => {
                      console.log(`\t' ${phase}: ${section}.${method}:: ${data}`);
                  });
    
                  process.exit(0);
              }
          });
    }catch(e){
        console.error(e);
    }
}
main().catch(console.error)

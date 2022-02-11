// Required imports
const { ApiPromise, WsProvider, Keyring } = require('@polkadot/api');

import type { EventRecord, ExtrinsicStatus, H256, SignedBlock } from '@polkadot/types/interfaces';
import type { ISubmittableResult} from '@polkadot/types/types';

const ALICE = '5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY';
const BOB = '5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty';

const keyring = new Keyring({ type: 'sr25519' });

async function createApi() {
  // Initialise the provider to connect to the local node
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
          app_data_lookup: 'DataLookup'
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

  // Retrieve the chain & node information information via rpc calls
  const [chain, nodeName, nodeVersion] = await Promise.all([
    api.rpc.system.chain(),
    api.rpc.system.name(),
    api.rpc.system.version()
  ]);

  console.log(`You are connected to chain ${chain} using ${nodeName} v${nodeVersion}`);

  // Add Alice to our keyring with a hard-deived path (empty phrase, so uses dev)
  const alice = keyring.addFromUri('//Alice');
  const bob = keyring.addFromUri('//Bob');

  const metadata = await api.rpc.state.getMetadata();

  // Get the nonce for the admin key
  const { nonce: alice_nonce, data: alice_balance } = await api.query.system.account(ALICE);
  console.log(`Pre-balance of Alice(nonce=${alice_nonce}): ${alice_balance.free}`);
  const { nonce: bob_nonce, data: bob_balance } = await api.query.system.account(BOB);
  console.log(`Pre-balance of Bob(nonce=${bob_nonce}): ${bob_balance.free}`);

  // Create a transfer to BOB using AppId = 0.
  let transfer = api.tx.balances.transfer(ALICE, 1000000);
  // const check_app_id = api.createType("CheckAppId", {})

  const lastHeader = await api.rpc.chain.getHeader();
  const finalied_head = await api.rpc.chain.getFinalizedHead();
  console.log(`Last block #${lastHeader.number} has hash ${lastHeader.hash}`);
  console.log(`Finalized head #${finalied_head}`);



  const unsub = await transfer
    .signAndSend(
      bob, 
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

              unsub();
          }
      });

  /*
  const { nonce: alice_post_nonce, data: alice_post_balance } = await api.query.system.account(ALICE);
  console.log(`Post-balance of Alice: ${alice_post_balance.free}`);
  const { nonce: bob_post_nonce, data: bob_post_balance } = await api.query.system.account(BOB);
  console.log(`Post-balance of Bob: ${bob_post_balance.free}`);
  */
}

main().catch(console.error)

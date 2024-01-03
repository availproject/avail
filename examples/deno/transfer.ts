import { ApiPromise, Keyring, WsProvider } from 'https://deno.land/x/polkadot@0.2.42/api/mod.ts';
import { BN } from 'https://deno.land/x/polkadot@0.2.42/util/mod.ts';
import { API_RPC, API_TYPES, API_EXTENSIONS } from './api_options.ts'
import config from './config.ts';

const api = await ApiPromise.create({ provider: new WsProvider(config.endpoint), rpc: API_RPC, types: API_TYPES, signedExtensions: API_EXTENSIONS  });
const keyring = new Keyring({type: 'sr25519'}).addFromUri(config.seed);
const bobAddress = config.recipient;

const oneAvl = api.registry.createType('Compact<u128>', new BN("1000000000000000000"));
const hash = await api.tx.balances.transfer(bobAddress, oneAvl).signAndSend(keyring);
console.log("Transfer sent with hash: " + hash.toHuman())

Deno.exit(0);

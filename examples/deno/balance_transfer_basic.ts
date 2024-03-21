/// TODO DOC
///
///

import {
  ApiPromise,
  Keyring,
  WsProvider,
} from "https://deno.land/x/polkadot@0.2.42/api/mod.ts";
import { BN } from "https://deno.land/x/polkadot@0.2.42/util/mod.ts";
import { API_EXTENSIONS, API_RPC, API_TYPES } from "./api_options.ts";

const api = await ApiPromise.create({
  provider: new WsProvider("ws://127.0.0.1:9944"),
  rpc: API_RPC,
  types: API_TYPES,
  signedExtensions: API_EXTENSIONS,
});
const alice = new Keyring({ type: "sr25519" }).addFromUri("//Alice");
const bobAddress = "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty";

const oneAvl = new BN("1000000000000000000");
const hash = await api.tx.balances.transferKeepAlive(bobAddress, oneAvl)
  .signAndSend(alice);
console.log("Transaction Hash: " + hash.toHuman());

Deno.exit(0);

/// TODO DOC
///
///

import {
  ApiPromise,
  Keyring,
  WsProvider,
} from "https://deno.land/x/polkadot@0.2.42/api/mod.ts";
import { BN } from "https://deno.land/x/polkadot@0.2.42/util/mod.ts";
import { ISubmittableResult } from "https://deno.land/x/polkadot@0.2.42/types/types/extrinsic.ts";
import { H256 } from "https://deno.land/x/polkadot@0.2.42/types/interfaces/types.ts";
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
const balanceTransfer = new Promise<[H256, H256]>((res, _) => {
  api.tx.balances.transferKeepAlive(bobAddress, oneAvl).signAndSend(
    alice,
    (result: ISubmittableResult) => {
      console.log(`Tx status: ${result.status}`);
      if (result.isInBlock) {
        console.log("Waiting for finalization...");
      } else if (result.isFinalized) {
        res([result.status.asFinalized as H256, result.txHash as H256]);
      }
    },
  );
});

const [blockHash, txHash] = await balanceTransfer;
console.log(`Finalized block hash ${blockHash} and tx hash ${txHash}`);

// Find and print out Balance Transfer transaction
const block = await api.rpc.chain.getBlock(blockHash);
const tx = block.block.extrinsics.find((tx) =>
  tx.hash.toHex() == txHash.toHex()
);
if (tx != undefined) {
  console.log(tx.toHuman());
  const { method: { args, method, section } } = tx;
  console.log(
    `${section}.${method}(${args.map((a) => a.toString()).join(", ")})`,
  );
}

Deno.exit(0);

/// The example showcases how to programmatically do balance transfer.
///
/// The following transactions are being called:
///   Balance.transfer
///
/// The following storage are being queried:
///   System.account
///

import { ApiPromise, Keyring, WsProvider } from "https://deno.land/x/polkadot@0.2.45/api/mod.ts";
import { BN } from "https://deno.land/x/polkadot@0.2.45/util/mod.ts";
import { ISubmittableResult } from "https://deno.land/x/polkadot@0.2.45/types/types/extrinsic.ts";
import { H256 } from "https://deno.land/x/polkadot@0.2.45/types/interfaces/types.ts";
import { API_EXTENSIONS, API_RPC, API_TYPES } from "../src/api_options.ts";

const api = await ApiPromise.create({
	provider: new WsProvider("ws://127.0.0.1:9944"),
	rpc: API_RPC,
	types: API_TYPES,
	signedExtensions: API_EXTENSIONS,
});

const alice = new Keyring({ type: "sr25519" }).addFromUri("//Alice");
const bob_address = "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty";
const one_AVAIL = new BN("1000000000000000000");

const old_bobs_state: any = await api.query.system.account(bob_address);
console.log(`Bob's balance before the transfer call: ${old_bobs_state["data"]["free"].toHuman()}`);

// Transaction call
const tx_result = await new Promise<ISubmittableResult>((res, _) => {
	api.tx.balances.transferKeepAlive(bob_address, one_AVAIL).signAndSend(alice, (result: ISubmittableResult) => {
		console.log(`Tx status: ${result.status}`);
		if (result.isFinalized || result.isError) {
			res(result);
		}
	});
});
console.log(`Tx Hash: ${tx_result.txHash as H256}, Block Hash: ${tx_result.status.asFinalized as H256}`);

// Error handling
const error = tx_result.dispatchError;
if (tx_result.isError) {
	console.log(`Transaction was not executed`);
} else if (error != undefined) {
	if (error.isModule) {
		const decoded = api.registry.findMetaError(error.asModule);
		const { docs, name, section } = decoded;
		console.log(`${section}.${name}: ${docs.join(" ")}`);
	} else {
		console.log(error.toString());
	}
	Deno.exit(1);
}

const new_bobs_state: any = await api.query.system.account(bob_address);
console.log(`Bob's balance after the transfer call: ${new_bobs_state["data"]["free"].toHuman()}`);

Deno.exit(0);

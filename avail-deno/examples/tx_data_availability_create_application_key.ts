/// The example showcases how to programmatically create application key.
///
/// The following transactions are being called:
///   DataAvailability.createApplicationKey
///

import { ApiPromise, Keyring, WsProvider } from "https://deno.land/x/polkadot@0.2.45/api/mod.ts";
import { ISubmittableResult } from "https://deno.land/x/polkadot@0.2.45/types/types/extrinsic.ts";
import { H256 } from "https://deno.land/x/polkadot@0.2.45/types/interfaces/types.ts";
import { API_EXTENSIONS, API_RPC, API_TYPES } from "../src/api_options.ts";

const api = await ApiPromise.create({
	provider: new WsProvider("ws://127.0.0.1:9944"),
	rpc: API_RPC,
	types: API_TYPES,
	signedExtensions: API_EXTENSIONS,
});

// Use your secret seed or mnemonic here
const account = new Keyring({ type: "sr25519" }).addFromUri("//Alice");

// Transaction call
const key = "My Key";
const tx_result = await new Promise<ISubmittableResult>((res, _) => {
	api.tx.dataAvailability.createApplicationKey(key).signAndSend(account, (result: ISubmittableResult) => {
		console.log(`Tx status: ${result.status}`);
		if (result.isFinalized || result.isError) {
			res(result);
		}
	});
});

// Rejected Transaction handling
if (tx_result.isError) {
	console.log(`Transaction was not executed`);
	Deno.exit(1);
}

const [tx_hash, block_hash] = [tx_result.txHash as H256, tx_result.status.asFinalized as H256];
console.log(`Tx Hash: ${tx_hash}, Block Hash: ${block_hash}`);

// Failed Transaction handling
const error = tx_result.dispatchError;
if (error != undefined) {
	if (error.isModule) {
		const decoded = api.registry.findMetaError(error.asModule);
		const { docs, name, section } = decoded;
		console.log(`${section}.${name}: ${docs.join(" ")}`);
	} else {
		console.log(error.toString());
	}
	Deno.exit(1);
}

const e = tx_result.events.find((e) => e.event.method == "ApplicationKeyCreated");
if (e == undefined) {
	console.log(`Missing ApplicationKeyCreated method.`);
	Deno.exit(1);
}
const data: any = e.event.data;
const read_key = data["key"].toString();
const read_owner = data["owner"].toString();
const read_app_id = data["id"].toString();
console.log(`
  key=${read_key},
  owner=${read_owner},
  id=${read_app_id},
`);

Deno.exit(0);

/// The example showcases how to programmatically call query block length RPC.
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
const account = new Keyring({ type: "sr25519" }).addFromUri("//Alice");
// Change App Id to something that has meaning to you or leave it at one
const options = { app_id: 1, nonce: -1 };

// submit data transaction
const tx_result = await new Promise<ISubmittableResult>((res, _) => {
	api.tx.dataAvailability.submitData("Hello World").signAndSend(account, options, (result: ISubmittableResult) => {
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

const blockLength = await api.rpc.kate.blockLength(block_hash);
console.log(blockLength.toHuman());

Deno.exit(0);

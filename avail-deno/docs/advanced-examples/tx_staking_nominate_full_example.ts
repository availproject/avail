/// The example showcases how to programmatically become a nominator.
///
/// The following transactions are being called:
///   Utility.batchAll
///   Staking.bond
///   Session.nominate
///
/// The following storage are being queried:
///   Staking.bonded
///   Staking.ledger
///   Staking.nominators
///

import { ApiPromise, Keyring, WsProvider } from "https://deno.land/x/polkadot@0.2.45/api/mod.ts";
import { BN } from "https://deno.land/x/polkadot@0.2.45/util/mod.ts";
import { H256 } from "https://deno.land/x/polkadot@0.2.45/types/interfaces/types.ts";
import { API_EXTENSIONS, API_RPC, API_TYPES } from "../src/api_options.ts";
import { ISubmittableResult } from "https://deno.land/x/polkadot@0.2.45/types/types/extrinsic.ts";

const api = await ApiPromise.create({
	provider: new WsProvider("ws://127.0.0.1:9944"),
	rpc: API_RPC,
	types: API_TYPES,
	signedExtensions: API_EXTENSIONS,
});

// Use your secret seed or mnemonic here
const account = new Keyring({ type: "sr25519" }).addFromUri("//Bob");
const min_nominator_bond = (await api.query.staking.minNominatorBond()).toString();
// You can bond any amount of tokens as long as it is at least more or equal than the minimum
// In this case we either bond the minimum amount or if there is no minimum we bond 1k AVAIL
const bond_amount = new BN(min_nominator_bond == "0" ? "1000000000000000000000" : min_nominator_bond);
// Here you can specify what targets will be nominated
const targets = ["5GNJqTPyNqANBkUVMN1LPPrxXnFouWXoe2wNSmmEoLctxiZY"];

const staking_bond = api.tx.staking.bond(bond_amount, "Staked");
const staking_nominate = api.tx.staking.nominate(targets);

// Transaction call
const tx_result = await new Promise<ISubmittableResult>((res, _) => {
	api.tx.utility.batchAll([staking_bond, staking_nominate]).signAndSend(account, (result: ISubmittableResult) => {
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
		// for module errors, we have the section indexed, lookup
		const decoded = api.registry.findMetaError(error.asModule);
		const { docs, name, section } = decoded;
		console.log(`${section}.${name}: ${docs.join(" ")}`);
	} else {
		// Other, CannotLookup, BadOrigin, no extra info
		console.log(error.toString());
	}
	Deno.exit(1);
}

// Reading Nomination related information from storage
const is_bonded = (await api.query.staking.bonded(account.address)).toHuman();
const ledger = await api.query.staking.ledger(account.address);
const nominators = await api.query.staking.nominators(account.address);
if (is_bonded == undefined) {
	console.log("Something went wrong :(");
	Deno.exit(1);
}

console.log(`Staking.ledger: ${ledger}`);
console.log(`Staking.nominators: ${nominators}`);

Deno.exit(0);

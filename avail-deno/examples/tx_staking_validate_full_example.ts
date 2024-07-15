/// The example showcases how to programmatically become a validator.
///
/// The following transactions are being called:
///   Utility.batchAll
///   Staking.bond
///   Session.set_key
///   Staking.validate
///
/// The following storage are being queried:
///   Staking.bonded
///   Staking.ledger
///   Staking.validators
///

import { ApiPromise, Keyring, WsProvider } from "https://deno.land/x/polkadot@0.2.45/api/mod.ts";
import { BN } from "https://deno.land/x/polkadot@0.2.45/util/mod.ts";
import { H256 } from "https://deno.land/x/polkadot@0.2.45/types/interfaces/types.ts";
import { API_EXTENSIONS, API_RPC, API_TYPES } from "../src/api_options.ts";
import { ISubmittableResult } from "https://deno.land/x/polkadot@0.2.45/types/types/extrinsic.ts";

function deconstruct_session_keys(deconstruct_session_keys: string) {
	const keys = deconstruct_session_keys.slice(2, undefined);
	const babe_key = "0x".concat(keys.slice(0, 64));
	const grandpa_key = "0x".concat(keys.slice(64, 128));
	const imonline_key = "0x".concat(keys.slice(128, 192));
	const authority_discovery_key = "0x".concat(keys.slice(192, 256));

	return {
		babe: babe_key,
		grandpa: grandpa_key,
		imOnline: imonline_key,
		authorityDiscover: authority_discovery_key,
	};
}

function define_validator_preference() {
	// "5" means 5 percent.
	let commission = "5".concat("0000000");
	// For some reason 0 commission is not defined as "0" but as "1".
	if (commission == "00000000") {
		commission = "1";
	}
	return { commission: commission, block: false };
}

const api = await ApiPromise.create({
	provider: new WsProvider("ws://127.0.0.1:9944"),
	rpc: API_RPC,
	types: API_TYPES,
	signedExtensions: API_EXTENSIONS,
});

// Use your secret seed or mnemonic here
const account = new Keyring({ type: "sr25519" }).addFromUri("//Bob");
const min_validator_bond = (await api.query.staking.minValidatorBond()).toString();
// You can bond any amount of tokens as long as it is at least more or equal than the minimum
// In this case we either bond the minimum amount or if there is no minimum we bond 1k AVAIL
const bond_amount = new BN(min_validator_bond == "0" ? "1000000000000000000000" : min_validator_bond);
// You need to generate the session keys yourself and put the value in here
const session_keys =
	"0xcce44c3da975792242a278a90e1557ee2059ae14a6c6104a50045e13afdaea490028ae395391cba3e7aa5219802a04a0c1833b0814ed5bfae7e5b9c453a69bbedc69835386108accc1f191b82b40d92568b5e0863243cbe0351d36d5fc823b09187d3992202265cdce9d1b95481a402c9ca39fb041615ca71992d92066841534";
const keys = deconstruct_session_keys(session_keys);
const prefs = define_validator_preference();

const staking_bond = api.tx.staking.bond(bond_amount, "Staked");
const session_set_keys = api.tx.session.setKeys(keys, undefined);
const staking_validate = api.tx.staking.validate(prefs);

// Transaction call
const tx_result = await new Promise<ISubmittableResult>((res, _) => {
	api.tx.utility.batchAll([staking_bond, session_set_keys, staking_validate]).signAndSend(account, (result: ISubmittableResult) => {
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

// Reading Validator related information from storage
const is_bonded = (await api.query.staking.bonded(account.address)).toHuman();
const ledger = await api.query.staking.ledger(account.address);
const validators = await api.query.staking.validators(account.address);
if (is_bonded == undefined) {
	console.log("Something went wrong :(");
	Deno.exit(1);
}

console.log(`Staking.ledger: ${ledger}`);
console.log(`Staking.validators: ${validators}`);

Deno.exit(0);

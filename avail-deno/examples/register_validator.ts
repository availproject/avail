import { ApiPromise, Keyring, WsProvider } from "https://deno.land/x/polkadot@0.2.45/api/mod.ts";
import { BN } from "https://deno.land/x/polkadot@0.2.45/util/mod.ts";
import { API_EXTENSIONS, API_RPC, API_TYPES } from "../src/api_options.ts";
import { ISubmittableResult } from "https://deno.land/x/polkadot@0.2.45/types/types/extrinsic.ts";

interface Config {
	session_keys: string | undefined;
	stash_account_seed_hex: string | undefined;
	bond_amount: number | undefined;
	commission: number | undefined;
	endpoint: string | undefined;
}

const config: Config = JSON.parse(
	Deno.readTextFileSync("./register_validator.json"),
);

if (
	config.stash_account_seed_hex == undefined ||
	config.bond_amount == undefined || config.endpoint == undefined ||
	config.commission == undefined
) {
	console.log("Failed 1");
	Deno.exit(0);
}

if (config.bond_amount < 1) {
	console.log("bond_amount cannot less than 1");
	Deno.exit(0);
}

if (Math.floor(config.bond_amount) != config.bond_amount) {
	console.log("bond_amount needs to be a whole number (integer)");
	Deno.exit(0);
}

if (config.commission < 0 || config.commission > 100) {
	console.log(
		"commission cannot be less than 0. commission cannot be more than 100",
	);
	Deno.exit(0);
}

if (Math.floor(config.commission) != config.commission) {
	console.log("commission needs to be a whole number (integer)");
	Deno.exit(0);
}

const api = await ApiPromise.create({
	provider: new WsProvider(config.endpoint),
	rpc: API_RPC,
	types: API_TYPES,
	signedExtensions: API_EXTENSIONS,
});
const stash = new Keyring({ type: "sr25519" }).addFromUri(
	config.stash_account_seed_hex,
);

let session_keys = config.session_keys;
if (session_keys == undefined) {
	session_keys = (await api.rpc.author.rotateKeys()).toHuman()?.toString();
}
if (session_keys == undefined) {
	console.log("Failed to generate session keys.");
	Deno.exit(0);
}

const endpoint = config.endpoint;
const stash_address = stash.address;
const bond_value = new BN(
	config.bond_amount.toString().concat("000000000000000000"),
);
let commission = config.commission.toString().concat("0000000");
const already_bonded = (await api.query.staking.bonded(stash_address)).toHuman() != undefined;

// For some reason 0 commission is not defined as "0" but as "1".
if (commission == "00000000") {
	commission = "1";
}

const keys = session_keys.slice(2, undefined);
const babe_key = "0x".concat(keys.slice(0, 64));
const grandpa_key = "0x".concat(keys.slice(64, 128));
const imonline_key = "0x".concat(keys.slice(128, 192));
const authority_discovery_key = "0x".concat(keys.slice(192, 256));

console.log(`Endpoint: ${endpoint}`);
console.log(`Stash Account: ${stash_address}`);
console.log(`Bond Value: ${bond_value}`);
console.log(`Commission: ${commission}`);
console.log(`Session Keys: ${session_keys}`);
console.log(`Babe Key (derived from Session Keys): ${babe_key}`);
console.log(`Grandpa Key (derived from Session Keys): ${grandpa_key}`);
console.log(`ImOnline Key (derived from Session Keys): ${imonline_key}`);
console.log(
	`Authority Discovery Key (derived from Session Keys): ${authority_discovery_key}`,
);
if (already_bonded) {
	console.log(
		"Stash is already bonded. No new staking.bond call will be executed",
	);
}

const tx = new Promise<string | undefined>((res, _) => {
	const keys = {
		babe: babe_key,
		grandpa: grandpa_key,
		imOnline: imonline_key,
		authorityDiscover: authority_discovery_key,
	};
	const prefs = {
		commission: commission,
		block: false,
	};

	const calls = [];
	if (!already_bonded) {
		calls.push(api.tx.staking.bond(bond_value, "Staked"));
	}
	calls.push(api.tx.session.setKeys(keys, undefined));
	calls.push(api.tx.staking.validate(prefs));

	api.tx.utility.batchAll(calls).signAndSend(
		stash,
		(result: ISubmittableResult) => {
			console.log(`Tx status: ${result.status}`);
			if (result.isInBlock) {
				res(undefined);
			} else if (result.isError) {
				res(result.toHuman()?.toString());
			}
		},
	);
});

const tx_res = await tx;
if (tx_res != undefined) {
	console.log(tx_res);
}

Deno.exit(0);

use super::get_account_id_from_seed;
use avail_core::{AppId, BLOCK_CHUNK_SIZE};
use avail_core_kate::{
	config::{MAX_BLOCK_COLUMNS, MAX_BLOCK_ROWS},
	testnet::public_params,
};
use da_control::AppKeyInfo;
use da_runtime::{wasm_binary_unwrap, AccountId, Balance, DataAvailabilityConfig, AVL};
use frame_system::limits::BlockLength;
use hex_literal::hex;
use primitive_types::H160;
use sc_telemetry::TelemetryEndpoints;
use sp_core::sr25519::Public;
use sp_runtime::Perbill;
use std::collections::HashMap;

pub const NOMAD_LOCAL_DOMAIN: u32 = 2000;
pub const NOMAD_UPDATER: H160 = H160(hex!("695dFcFc604F9b2992642BDC5b173d1a1ed60b03"));
pub const PROTOCOL_ID: Option<&str> = Some("Avail");

pub fn standard_system_configuration() -> (Vec<u8>, Vec<u8>, BlockLength) {
	let code = wasm_binary_unwrap().to_vec();
	let kc_public_params = public_params(MAX_BLOCK_COLUMNS).to_raw_var_bytes();

	let block_length = BlockLength::with_normal_ratio(
		MAX_BLOCK_ROWS,
		MAX_BLOCK_COLUMNS,
		BLOCK_CHUNK_SIZE,
		Perbill::from_percent(90),
	)
	.expect("Valid `BlockLength` genesis definition .qed");

	(code, kc_public_params, block_length)
}

pub const TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

pub fn to_telemetry_endpoint(s: String) -> TelemetryEndpoints {
	TelemetryEndpoints::new(vec![(s, 0)]).unwrap()
}

const DEFAULT_ENDOWED_SEEDS: [&str; 12] = [
	"Alice",
	"Bob",
	"Charlie",
	"Dave",
	"Eve",
	"Ferdie",
	"Alice//stash",
	"Bob//stash",
	"Charlie//stash",
	"Dave//stash",
	"Eve//stash",
	"Ferdie//stash",
];
pub const ENDOWMENT: Balance = 1_000 * AVL;
pub const STASH_BOND: Balance = ENDOWMENT / 10;

/// Generates a default endowed accounts.
pub fn dev_endowed_accounts() -> HashMap<AccountId, Balance> {
	DEFAULT_ENDOWED_SEEDS
		.iter()
		.map(|seed| (get_account_id_from_seed::<Public>(seed), ENDOWMENT))
		.collect::<HashMap<_, _>>()
}

const INIT_APP_IDS: [&str; 3] = ["Data Avail", "Ethereum", "Polygon"];

pub(crate) fn make_data_avail_config(owner: AccountId) -> DataAvailabilityConfig {
	let app_keys = INIT_APP_IDS
		.iter()
		.enumerate()
		.map(|(id, app)| {
			let info = AppKeyInfo {
				owner: owner.clone(),
				id: AppId(id as u32),
			};
			(app.as_bytes().to_vec(), info)
		})
		.collect();

	DataAvailabilityConfig {
		app_keys,
		..Default::default()
	}
}

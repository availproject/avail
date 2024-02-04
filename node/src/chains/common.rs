use super::{get_account_id_from_seed, AuthorityKeys};
use avail_core::{BLOCK_CHUNK_SIZE, NORMAL_DISPATCH_RATIO};
use kate::config::{MAX_BLOCK_COLUMNS, MAX_BLOCK_ROWS};

use da_runtime::{
	constants, AccountId, Balance, DataAvailabilityConfig, SessionKeys, StakerStatus, AVL,
};
use frame_system::limits::BlockLength;
use hex_literal::hex;
use primitive_types::H256;
use sc_telemetry::TelemetryEndpoints;
use serde_json::{json, Value};
use sp_core::crypto::AccountId32;
use sp_core::sr25519::Public;

pub const PROTOCOL_ID: &str = "Avail";
pub const TELEMETRY_URL: &str = "ws://telemetry.avail.tools:8001/submit";

// bridge init config
const BROADCASTER_DOMAIN: u32 = 2;
const BROADCASTER: H256 = H256(hex!(
	"Aa8c1bFC413e00884A7ac991851686D27b387997000000000000000000000000" // Sepolia address
));
const SLOTS_PER_PERIOD: u64 = 8192;
const FINALITY_THRESHOLD: u16 = 342;

const ENDOWMENT: Balance = 1_000_000 * AVL;
const STASH_BOND: Balance = ENDOWMENT / 100;
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
const INIT_APP_IDS: [(u32, &str); 3] = [(0, "Data Avail"), (1, "Ethereum"), (2, "Polygon")];

pub fn to_telemetry_endpoint(s: String) -> TelemetryEndpoints {
	TelemetryEndpoints::new(vec![(s, 0)]).unwrap()
}

/// Generates a default endowed accounts.
fn dev_endowed_accounts() -> Vec<(AccountId, Balance)> {
	DEFAULT_ENDOWED_SEEDS
		.iter()
		.map(|seed| (get_account_id_from_seed::<Public>(seed), ENDOWMENT))
		.collect()
}

fn make_data_avail_config(owner: AccountId) -> DataAvailabilityConfig {
	let app_keys = INIT_APP_IDS
		.iter()
		.map(|(id, app)| (app.as_bytes().to_vec(), (owner.clone(), *id)))
		.collect();

	DataAvailabilityConfig { app_keys }
}

pub fn runtime_genesis_config(
	sudo: AccountId32,
	technical_committee: Vec<AccountId32>,
	session_keys: Vec<AuthorityKeys>,
) -> Value {
	let balances = dev_endowed_accounts();
	let stakers: Vec<(AccountId, AccountId, Balance, StakerStatus<AccountId>)> = session_keys
		.iter()
		.map(|k| {
			(
				k.stash.clone(),
				k.controller.clone(),
				STASH_BOND,
				StakerStatus::Validator,
			)
		})
		.collect();
	let validator_count = session_keys.len() as u32;
	let session_keys: Vec<(AccountId, AccountId, SessionKeys)> =
		session_keys.into_iter().map(|k| k.into()).collect();
	let block_length = BlockLength::with_normal_ratio(
		MAX_BLOCK_ROWS,
		MAX_BLOCK_COLUMNS,
		BLOCK_CHUNK_SIZE,
		NORMAL_DISPATCH_RATIO,
	)
	.expect("Valid `BlockLength` genesis definition .qed");

	json!({
		"system": {
			"blockLength": block_length,
		},
		"balances": {
			"balances": balances,
		},
		"staking": {
			"validatorCount": validator_count,
			"minimumValidatorCount": 1,
			"stakers": stakers,
		},
		"babe": {
			"epochConfig": Some(da_runtime::constants::babe::GENESIS_EPOCH_CONFIG),
		},
		"session": {
			"keys": session_keys,
		},
		"sudo": {
			"key": Some(sudo.clone()),
		},
		"technicalCommittee": {
			"members": technical_committee,
		},
		"vector": {
			"slotsPerPeriod": SLOTS_PER_PERIOD,
			"finalityThreshold": FINALITY_THRESHOLD,
			"broadcasterDomain": BROADCASTER_DOMAIN,
			"broadcaster": BROADCASTER,
			"whitelistedDomains": vec![2],
		},
		"nominationPools": {
			"minCreateBond": constants::nomination_pools::MIN_CREATE_BOND,
			"minJoinBond": constants::nomination_pools::MIN_JOIN_BOND,
			"maxPools": Some(constants::nomination_pools::MAX_POOLS),
			"maxMembersPerPool": Some(constants::nomination_pools::MAX_MEMBERS_PER_POOL),
			"maxMembers": Some(constants::nomination_pools::MAX_MEMBERS),
		},
		"dataAvailability": {
			"appKeys": make_data_avail_config(sudo).app_keys
		},
	})
}

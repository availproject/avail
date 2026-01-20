//! Genesis presets for the Avail runtime, used by the new sp_genesis_builder
//! framework (e.g. for `--dev` / `--tmp`).

#![allow(clippy::vec_init_then_push)]

use serde_json::{json, Value};
use sp_std::vec;
use sp_std::vec::Vec;

use avail_core::BLOCK_CHUNK_SIZE;
use da_control::DA_DISPATCH_RATIO_PERBILL;

use crate::{
	self as da_runtime, constants, AccountId, Balance, DataAvailabilityConfig, SessionKeys,
};
use frame_system::limits::BlockLength;
use sp_staking::StakerStatus;

use pallet_vector::constants::{
	get_poseidon_hash_for_period, BROADCASTER, BROADCASTER_DOMAIN, FINALITY_THRESHOLD,
	GENESIS_TIME, GENESIS_VALIDATOR_ROOT, PERIOD, ROTATE_FUNCTION_ID, ROTATE_VK, SECONDS_PER_SLOT,
	SLOTS_PER_PERIOD, SOURCE_CHAIN_ID, STEP_FUNCTION_ID, STEP_VK,
};

use pallet_im_online::sr25519::AuthorityId as ImOnlineId;
use scale_info::prelude::format;
use sp_authority_discovery::AuthorityId as AuthorityDiscoveryId;
use sp_consensus_babe::AuthorityId as BabeId;
use sp_consensus_grandpa::AuthorityId as GrandpaId;
use sp_core::{sr25519, Pair, Public};
use sp_genesis_builder::PresetId;
use sp_runtime::traits::IdentifyAccount;

/// Copy of `AccountPublic` from your node chainspec.
type AccountPublic = <da_runtime::Signature as sp_runtime::traits::Verify>::Signer;

/// The staker type as supplied to the Staking config.
pub type Staker = (AccountId, AccountId, Balance, StakerStatus<AccountId>);

/// Helper function to generate an account ID from a well-known seed.
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Generate a crypto pair from seed.
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

/// Helper struct equivalent to your `AuthorityKeys` in node chainspec.
#[derive(Clone)]
pub struct AuthorityKeys {
	pub controller: AccountId,
	pub stash: AccountId,
	pub session_keys: SessionKeys,
}

impl AuthorityKeys {
	/// Helper function to generate stash, controller and session key from seed
	pub fn from_seed(seed: &str) -> Self {
		let controller = get_account_id_from_seed::<sr25519::Public>(seed);
		let stash = get_account_id_from_seed::<sr25519::Public>(&format!("{}//stash", seed));
		let session_keys = SessionKeys {
			babe: get_from_seed::<BabeId>(seed),
			grandpa: get_from_seed::<GrandpaId>(seed),
			im_online: get_from_seed::<ImOnlineId>(seed),
			authority_discovery: get_from_seed::<AuthorityDiscoveryId>(seed),
		};

		Self {
			controller,
			stash,
			session_keys,
		}
	}
}

impl From<AuthorityKeys> for (AccountId, AccountId, SessionKeys) {
	fn from(val: AuthorityKeys) -> (AccountId, AccountId, SessionKeys) {
		(val.stash.clone(), val.stash, val.session_keys)
	}
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

const INIT_APP_IDS: [(u32, &str); 10] = [
	(0, "Avail"),
	(1, "Reserved-1"),
	(2, "Reserved-2"),
	(3, "Reserved-3"),
	(4, "Reserved-4"),
	(5, "Reserved-5"),
	(6, "Reserved-6"),
	(7, "Reserved-7"),
	(8, "Reserved-8"),
	(9, "Reserved-9"),
];

/// Generates default endowed accounts
fn dev_endowed_accounts() -> Vec<(AccountId, Balance)> {
	DEFAULT_ENDOWED_SEEDS
		.iter()
		.map(|seed| {
			(
				get_account_id_from_seed::<sr25519::Public>(seed),
				constants::staking::MIN_VALIDATOR_BOND * 100,
			)
		})
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
	sudo: AccountId,
	technical_committee: Vec<AccountId>,
	treasury_committee: Vec<AccountId>,
	session_keys: Vec<AuthorityKeys>,
) -> Value {
	let balances = dev_endowed_accounts();

	let stakers: Vec<Staker> = session_keys
		.iter()
		.map(|k| {
			(
				k.stash.clone(),
				k.controller.clone(),
				constants::staking::MIN_VALIDATOR_BOND,
				StakerStatus::Validator,
			)
		})
		.collect();

	let validator_count = session_keys.len() as u32;

	let session_keys: Vec<(AccountId, AccountId, SessionKeys)> =
		session_keys.into_iter().map(|k| k.into()).collect();

	let block_length = BlockLength::with_normal_ratio(
		frame_system::limits::MAX_BLOCK_ROWS,
		frame_system::limits::MAX_BLOCK_COLUMNS,
		BLOCK_CHUNK_SIZE,
		DA_DISPATCH_RATIO_PERBILL,
	)
	.expect("Valid `BlockLength` genesis definition; qed");

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
			"minNominatorBond": constants::staking::MIN_NOMINATOR_BOND,
			"minValidatorBond": constants::staking::MIN_VALIDATOR_BOND,
		},
		"babe": {
			"epochConfig": da_runtime::constants::babe::GENESIS_EPOCH_CONFIG,
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
		"treasuryCommittee": {
			"members": treasury_committee,
		},
		"vector": {
			"broadcaster": BROADCASTER,
			"broadcasterDomain": BROADCASTER_DOMAIN,
			"finalityThreshold": FINALITY_THRESHOLD,
			"functionIds": (STEP_FUNCTION_ID, ROTATE_FUNCTION_ID),
			"genesisTime": GENESIS_TIME,
			"genesisValidatorRoot": GENESIS_VALIDATOR_ROOT,
			"period": PERIOD,
			"secondsPerSlot": SECONDS_PER_SLOT,
			"slotsPerPeriod": SLOTS_PER_PERIOD,
			"sourceChainId": SOURCE_CHAIN_ID,
			"syncCommitteePoseidon": format!("0x{:064x}", get_poseidon_hash_for_period()),
			"stepVerificationKey": STEP_VK.as_bytes().to_vec(),
			"rotateVerificationKey": ROTATE_VK.as_bytes().to_vec(),
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

/// Provides the JSON representation of predefined genesis config for given `id`.
pub fn get_preset(id: &PresetId) -> Option<Vec<u8>> {
	let patch = match id.as_ref() {
		sp_genesis_builder::DEV_RUNTIME_PRESET => {
			let alice = AuthorityKeys::from_seed("Alice");
			let sudo = alice.controller.clone();

			runtime_genesis_config(
				sudo.clone(),
				vec![sudo.clone()], // technical committee
				vec![sudo.clone()], // treasury committee
				vec![alice],        // 1 validator
			)
		},

		// Avail local testnet: Alice, Bob, Charlie – mirrors `dev_tri::chain_spec`.
		sp_genesis_builder::LOCAL_TESTNET_RUNTIME_PRESET => {
			let alice = AuthorityKeys::from_seed("Alice");
			let bob = AuthorityKeys::from_seed("Bob");
			let charlie = AuthorityKeys::from_seed("Charlie");
			let sudo = alice.controller.clone();

			runtime_genesis_config(
				sudo.clone(),
				vec![sudo.clone()],        // technical committee
				vec![sudo.clone()],        // treasury committee
				vec![alice, bob, charlie], // 3 validators
			)
		},

		_ => return None,
	};

	Some(
		serde_json::to_string(&patch)
			.expect("serialization to json is expected to work; qed")
			.into_bytes(),
	)
}

/// List of supported presets.
pub fn preset_names() -> Vec<PresetId> {
	vec![
		PresetId::from(sp_genesis_builder::DEV_RUNTIME_PRESET),
		PresetId::from(sp_genesis_builder::LOCAL_TESTNET_RUNTIME_PRESET),
	]
}

#![allow(clippy::identity_op)]
use core::cmp::max;
use std::collections::HashMap;

use da_runtime::{
	constants::elections::InitialMemberBond, AccountId, Balance, Block, GenesisConfig, SessionKeys,
	Signature, AVL,
};
use hex_literal::hex;
use pallet_im_online::sr25519::AuthorityId as ImOnlineId;
use primitive_types::H160;
use sc_chain_spec::ChainSpecExtension;
use sc_service::{ChainType, Properties};
use serde::{Deserialize, Serialize};
use sp_authority_discovery::AuthorityId as AuthorityDiscoveryId;
use sp_consensus_babe::AuthorityId as BabeId;
use sp_core::{crypto::UncheckedInto, sr25519, Pair, Public};
use sp_finality_grandpa::AuthorityId as GrandpaId;
use sp_runtime::traits::{IdentifyAccount, Verify};

pub const NOMAD_LOCAL_DOMAIN: u32 = 2000;
pub const NOMAD_UPDATER: H160 = H160(hex!("1563915e194d8cfba1943570603f7606a3115508"));
pub const PROTOCOL_ID: Option<&str> = Some("Avail");
pub const FORK_ID: Option<&str> = None;

type AccountPublic = <Signature as Verify>::Signer;

pub mod config;
pub mod locals;
pub mod testnets;

// The URL for the telemetry server.
// const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";
/// Node `ChainSpec` extensions.
///
/// Additional parameters for some Substrate core modules,
/// customizable from the chain spec.
#[derive(Default, Clone, Serialize, Deserialize, ChainSpecExtension)]
#[serde(rename_all = "camelCase")]
pub struct Extensions {
	/// Block numbers with known hashes.
	pub fork_blocks: sc_client_api::ForkBlocks<Block>,
	/// Known bad block hashes.
	pub bad_blocks: sc_client_api::BadBlocks<Block>,
	/// The light sync state extension used by the sync-state rpc.
	pub light_sync_state: sc_sync_state_rpc::LightSyncStateExtension,
}

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig, Extensions>;

/// Generate a crypto pair from seed.
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

/// Common properties for chains
fn chain_properties() -> Option<Properties> {
	serde_json::json!({ "tokenDecimals": 18, "tokenSymbol": "AVL" })
		.as_object()
		.cloned()
}

/// Generate an account ID from seed.
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

pub fn tech_committee_from_authorities(authorities: &[AuthorityKeys]) -> Vec<AccountId> {
	let max_members = max(1, (authorities.len() + 1) / 2);
	authorities
		.iter()
		.map(|auth| auth.controller.clone())
		.take(max_members)
		.collect::<Vec<_>>()
}

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

	pub fn from_accounts(controller: AccountId, grandpa: GrandpaId) -> Self {
		let session_keys = session_keys(controller.clone(), grandpa);
		let stash = controller.clone();

		Self {
			controller,
			stash,
			session_keys,
		}
	}
}

fn session_keys(common: AccountId, grandpa: GrandpaId) -> SessionKeys {
	let raw: [u8; 32] = common.into();
	SessionKeys {
		babe: raw.unchecked_into(),
		grandpa,
		im_online: raw.unchecked_into(),
		authority_discovery: raw.unchecked_into(),
	}
}

/// Helper function to create GenesisConfig for testing
pub(crate) fn make_genesis(
	sudo: AccountId,
	authorities: Vec<AuthorityKeys>,
	council: Vec<AccountId>,
	tech_committee_members: Vec<AccountId>,
	mut endowed_accounts: HashMap<AccountId, Balance>,
	min_validator_bond: Balance,
	min_nominator_bond: Balance,
) -> GenesisConfig {
	// Extends endowed accounts with council members, authorities and TC members
	for acc in council.iter().cloned() {
		*endowed_accounts.entry(acc).or_default() += InitialMemberBond::get();
	}
	for acc in authorities.iter().map(|auth| auth.controller.clone()) {
		*endowed_accounts.entry(acc).or_default() += min_validator_bond + AVL;
	}
	for acc in tech_committee_members.iter().cloned() {
		*endowed_accounts.entry(acc).or_default() += 10 * AVL;
	}

	GenesisConfig {
		// General
		system: config::make_system_config(),
		babe: config::make_babe_config(),
		indices: Default::default(),
		balances: config::make_balances_config(endowed_accounts),
		transaction_payment: Default::default(),
		elections: config::make_elections(council.into_iter()),
		staking: config::make_staking_config(
			authorities.iter(),
			min_validator_bond,
			min_nominator_bond,
		),
		session: config::make_session_config(authorities.iter()),
		democracy: Default::default(),
		// `council`'s members initialized by `elections`.
		council: Default::default(),
		technical_committee: config::make_technical_committee_config(tech_committee_members),
		// `grandpa`'s keys were initialized by `Session`.
		grandpa: Default::default(),
		treasury: Default::default(),
		sudo: config::make_sudo_config(sudo.clone()),
		// `im_online`'s keys were initialized by `Session::Historical`
		im_online: Default::default(),
		// `authority_discovery`'s keys were initialized by `Session`.
		authority_discovery: Default::default(),

		// Data Avail
		data_availability: config::make_data_avail_config(sudo),

		// Nomad
		nomad_home: config::nomad::make_home_config(NOMAD_LOCAL_DOMAIN, NOMAD_UPDATER),
		nomad_updater_manager: config::nomad::make_update_manager_config(NOMAD_UPDATER),
		nomad_da_bridge: Default::default(),

		nomination_pools: config::make_nomination_pools_config(),
		// `technical_membership`'s members were initialized on `technical_committee`
		technical_membership: Default::default(),
	}
}

#[cfg(test)]
pub(crate) mod tests {
	use sp_runtime::BuildStorage;

	use super::*;
	// use crate::service::{new_full_base, NewFullBase};

	/*
	   fn local_testnet_genesis_instant_single() -> GenesisConfig {
		   testnet_genesis(
			   vec![authority_keys_from_seed("Alice")],
			   vec![],
			   get_account_id_from_seed::<sr25519::Public>("Alice"),
			   None,
		   )
	   }

	   /// Local testnet config (single validator - Alice)
	   pub fn integration_test_config_with_single_authority() -> ChainSpec {
		   ChainSpec::from_genesis(
			   "Integration Test",
			   "test",
			   ChainType::Development,
			   local_testnet_genesis_instant_single,
			   vec![],
			   None,
			   None,
			   None,
			   Default::default(),
		   )
	   }

	   /// Local testnet config (multivalidator Alice + Bob)
	   pub fn integration_test_config_with_two_authorities() -> ChainSpec {
		   ChainSpec::from_genesis(
			   "Integration Test",
			   "test",
			   ChainType::Development,
			   local_testnet_genesis,
			   vec![],
			   None,
			   None,
			   None,
			   Default::default(),
		   )
	   }

	   // TODO `sc_service_test` is not a public crate.
	#[test]
	   #[ignore]
	   fn test_connectivity() {
		   sp_tracing::try_init_simple();

		   sc_service_test::connectivity(integration_test_config_with_two_authorities(), |config| {
			   let NewFullBase {
				   task_manager,
				   client,
				   network,
				   transaction_pool,
				   ..
			   } = new_full_base(config, |_, _| ())?;
			   Ok(sc_service_test::TestNetComponents::new(
				   task_manager,
				   client,
				   network,
				   transaction_pool,
			   ))
		   });
	   }
	   */

	#[test]
	fn test_create_development_chain_spec() { locals::solo::chain_spec().build_storage().unwrap(); }

	#[test]
	fn test_create_development_tri_chain_spec() {
		locals::tri::chain_spec().build_storage().unwrap();
	}

	#[test]
	fn test_create_local_testnet_chain_spec() {
		testnets::ada::chain_spec().build_storage().unwrap();
	}
}

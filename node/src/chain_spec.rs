use sp_core::{Pair, Public, sr25519};
use node_template_runtime::{
	AuthorityDiscoveryConfig, AccountId, BabeConfig, BalancesConfig, IndicesConfig, GenesisConfig, GrandpaConfig, DemocracyConfig,
	SystemConfig, WASM_BINARY, Signature, StakerStatus,
	SessionConfig, StakingConfig, ElectionsConfig, SessionKeys, Balance, CouncilConfig, TechnicalCommitteeConfig, ImOnlineConfig,
};
use node_template_runtime::Block;
use node_template_runtime::currency::*;
use sp_consensus_babe::AuthorityId as BabeId;
use sp_finality_grandpa::AuthorityId as GrandpaId;
use sp_runtime::traits::{Verify, IdentifyAccount};
use pallet_im_online::sr25519::{AuthorityId as ImOnlineId};
use sp_authority_discovery::AuthorityId as AuthorityDiscoveryId;
use sp_runtime::Perbill;
use sc_service::ChainType;
use frame_system::limits::BlockLength;
use frame_benchmarking::frame_support::traits::Len;
use sc_chain_spec::ChainSpecExtension;
use serde::{Serialize, Deserialize};

// The URL for the telemetry server.
// const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";
#[derive(Default, Clone, Serialize, Deserialize, ChainSpecExtension)]
#[serde(rename_all = "camelCase")]
pub struct Extensions {
	/// Block numbers with known hashes.
	pub fork_blocks: sc_client_api::ForkBlocks<Block>,
	/// Known bad block hashes.
	pub bad_blocks: sc_client_api::BadBlocks<Block>,
}

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig, Extensions, >;

fn session_keys(
	babe: BabeId,
	grandpa: GrandpaId,
	im_online: ImOnlineId,
	authority_discovery: AuthorityDiscoveryId,
) -> SessionKeys {
	SessionKeys {
		babe,
		grandpa,
		im_online,
		authority_discovery
	}
}

/// Generate a crypto pair from seed.
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

type AccountPublic = <Signature as Verify>::Signer;

/// Generate an account ID from seed.
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

// pub fn authority_keys_from_seed(s: &str) -> (BabeId, GrandpaId) {
// 	(
// 		get_from_seed::<BabeId>(s),
// 		get_from_seed::<GrandpaId>(s),
// 	)
// }

/// Generate an babe authority key.
pub fn authority_keys_from_seed(seed: &str) -> (
	AccountId,
	AccountId,
	GrandpaId,
	BabeId,
	ImOnlineId,
	AuthorityDiscoveryId,
) {
	(
		get_account_id_from_seed::<sr25519::Public>(&format!("{}//stash", seed)),
		get_account_id_from_seed::<sr25519::Public>(seed),
		get_from_seed::<GrandpaId>(seed),
		get_from_seed::<BabeId>(seed),
		get_from_seed::<ImOnlineId>(seed),
		get_from_seed::<AuthorityDiscoveryId>(seed),

	)
}

fn development_config_genesis(wasm_binary: &[u8]) -> GenesisConfig {
	testnet_genesis(
		wasm_binary,
		vec![
			authority_keys_from_seed("Alice"),
		],
		//vec![],
		get_account_id_from_seed::<sr25519::Public>("Alice"),
		None,
		true,
	)
}
// //single validator ALICE
pub fn development_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or("Development wasm not available")?;
	Ok(ChainSpec::from_genesis(
		"Avail-Dev",
		"Dev",
		ChainType::Development,
		move || development_config_genesis(wasm_binary),
		vec![],
		None,
		None,
		None,
		Default::default(),
	))
}

fn local_testnet_genesis(wasm_binary: &[u8]) -> GenesisConfig {
	testnet_genesis(
		wasm_binary,
		vec![
			authority_keys_from_seed("Alice"),
			authority_keys_from_seed("Bob"),
		],
		//vec![],
		get_account_id_from_seed::<sr25519::Public>("Alice"),
		None,
		false,
	)
}
//multivalidator ALICE+BOB
pub fn local_testnet_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or("Development wasm not available")?;
	Ok(ChainSpec::from_genesis(
		"Avail-Testnet",
		"local_testnet",
		ChainType::Local,
		move || local_testnet_genesis(wasm_binary),
		vec![],
		None,
		None,
		None,
		Default::default(),
	))
}

/// Configure initial storage state for FRAME modules.
pub fn testnet_genesis(
	wasm_binary: &[u8],
	initial_authorities: Vec<(
		AccountId,
		AccountId,
		GrandpaId,
		BabeId,
		ImOnlineId,
		AuthorityDiscoveryId,
	)>,
	//initial_nominators: Vec<AccountId>,
	_root_key: AccountId,
	endowed_accounts: Option<Vec<AccountId>>,
	_enable_println: bool,
) -> GenesisConfig {
	let num_endowed_accounts = endowed_accounts.len();
	let endowment: Balance = 10_000_000 * DOLLARS;
	let stash: Balance = endowment/1_000;
	let mut endowed_accounts: Vec<AccountId> = endowed_accounts.unwrap_or_else(|| {
		vec![
			get_account_id_from_seed::<sr25519::Public>("Alice"),
			get_account_id_from_seed::<sr25519::Public>("Bob"),
			get_account_id_from_seed::<sr25519::Public>("Charlie"),
			get_account_id_from_seed::<sr25519::Public>("Dave"),
			get_account_id_from_seed::<sr25519::Public>("Eve"),
			get_account_id_from_seed::<sr25519::Public>("Ferdie"),
			get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
			get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
			get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
			get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
			get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
			get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
		]
	});
	// endow all authorities and nominators.
	initial_authorities.iter().for_each(|x|
		if !endowed_accounts.contains(&x.0) {
			endowed_accounts.push(x.0.clone())
		}
	);
	GenesisConfig {
		frame_system: Some(SystemConfig {
			// Add Wasm runtime to storage.
			code: wasm_binary.to_vec(),
			changes_trie_config: Default::default(),
			kc_public_params: kate::testnet::KC_PUB_PARAMS.to_vec(),
			block_length: BlockLength::with_normal_ratio(128, 256, 64, 	Perbill::from_percent(90)),
		}),
		pallet_balances: Some(BalancesConfig {
			//Configure endowed accounts with initial balance of 20 << 60.
			balances: endowed_accounts.iter().cloned().map(|k| (k, 1 << 60)).collect(),
			// balances: endowed_accounts.iter().cloned()
			// 	.map(|k| (k, 100))
			// 	.collect(),
		}),
		pallet_indices: Some(IndicesConfig {
			indices: vec![],
		}),
		pallet_babe: Some(BabeConfig {
			authorities: vec![],
		}),
		pallet_grandpa: Some(GrandpaConfig {
			authorities: vec![],
		}),
		// pallet_sudo: Some(SudoConfig {
		// 	// Assign network admin rights.
		// 	key: root_key,
		// }),
		pallet_staking: Some(StakingConfig {
			validator_count: initial_authorities.len() as u32 * 2,
			minimum_validator_count: initial_authorities.len() as u32,
			stakers: initial_authorities.iter().map(|x| {
				(x.0.clone(), x.1.clone(), stash, StakerStatus::Validator)
			}).collect(),
			invulnerables: initial_authorities.iter().map(|x| x.0.clone()).collect(),
			slash_reward_fraction: Perbill::from_percent(10),
			.. Default::default()
		}),
		pallet_session: Some(SessionConfig {
			keys: initial_authorities.iter().map(|x| {
				(x.0.clone(), x.0.clone(), session_keys(
					x.3.clone(),
					x.2.clone(),
					x.4.clone(),
					x.5.clone(),
				))
			}).collect::<Vec<_>>(),
		}),
		pallet_im_online: Some(ImOnlineConfig {
			keys: vec![],
		}),
		pallet_collective_Instance1: Some(CouncilConfig::default()),
		pallet_collective_Instance2: Some(TechnicalCommitteeConfig {
			members: endowed_accounts.iter()
						.take((num_endowed_accounts + 1) / 2)
						.cloned()
						.collect(),
			phantom: Default::default(),
		}),
		pallet_membership_Instance1: Some(Default::default()),
		pallet_treasury: Some(Default::default()),
		pallet_authority_discovery: Some(AuthorityDiscoveryConfig {
			keys: vec![],
		}),
		pallet_democracy: Some(DemocracyConfig::default()),
		pallet_elections_phragmen: Some(ElectionsConfig {
			members: endowed_accounts.iter()
						.take((num_endowed_accounts + 1) / 2)
						.cloned()
						.map(|member| (member, stash))
						.collect(),
		}),
		
	}
}

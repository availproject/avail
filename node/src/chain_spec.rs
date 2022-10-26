use da_control::AppKeyInfo;
use da_primitives::currency::AVL;
use da_runtime::{
	wasm_binary_unwrap, AccountId, AuthorityDiscoveryConfig, BabeConfig, Balance, BalancesConfig,
	Block, CouncilConfig, DataAvailabilityConfig, DemocracyConfig, DesiredMembers, ElectionsConfig,
	GenesisConfig, GrandpaConfig, ImOnlineConfig, IndicesConfig, NomadHomeConfig, SessionConfig,
	SessionKeys, Signature, StakerStatus, StakingConfig, SudoConfig, SystemConfig,
	TechnicalCommitteeConfig, UpdaterManagerConfig, MAX_NOMINATIONS,
};
use frame_system::limits::BlockLength;
use kate::config::{MAX_BLOCK_COLUMNS, MAX_BLOCK_ROWS};
use pallet_im_online::sr25519::AuthorityId as ImOnlineId;
use sc_chain_spec::ChainSpecExtension;
use sc_service::{ChainType, Properties};
use serde::{Deserialize, Serialize};
use sp_authority_discovery::AuthorityId as AuthorityDiscoveryId;
use sp_consensus_babe::AuthorityId as BabeId;
use sp_core::{crypto::UncheckedInto, sr25519, Pair, Public};
use sp_finality_grandpa::AuthorityId as GrandpaId;
use sp_runtime::{
	traits::{IdentifyAccount, Verify},
	Perbill, SaturatedConversion,
};

mod testnet {
	use hex_literal::hex;

	use super::{AccountId, Balance, InitialAuthority, UncheckedInto, AVL};

	pub fn authorities() -> Vec<InitialAuthority> {
		vec![
			// Validator 1.
			InitialAuthority::new(
				hex!("b2dad89ebcbf90f48021d8f18ad72339da8eb3de16fcefac84402c7615dce932").into(),
				hex!("b2dad89ebcbf90f48021d8f18ad72339da8eb3de16fcefac84402c7615dce932").into(),
				hex!("9fa3481874ee8d02456438fff97273f320a9e11ad45f52cbc8391b9165d7aa70")
					.unchecked_into(),
			),
			// Validator 2.
			InitialAuthority::new(
				hex!("101020d67afe1df8b2811d30e8a6d57dc74bc2c1b76d4f8c4b20da21b581f710").into(),
				hex!("101020d67afe1df8b2811d30e8a6d57dc74bc2c1b76d4f8c4b20da21b581f710").into(),
				hex!("3f190c60714a0f233bcc6a8941e3802b3a1eeca6dbef157727c859a13c6bca1e")
					.unchecked_into(),
			),
			// Validator 3.
			InitialAuthority::new(
				hex!("e099cabb7e96b3e5033fcc9eac120f15877b03881aafe811c98fe48e67047e06").into(),
				hex!("e099cabb7e96b3e5033fcc9eac120f15877b03881aafe811c98fe48e67047e06").into(),
				hex!("2d12c83a0bd991a8888620a215c1e214d954790cb4c42a7adaf7e49d877f099f")
					.unchecked_into(),
			),
		]
	}

	pub fn technical_committee() -> Vec<AccountId> {
		vec![
			hex!("d2fba4e644431142a62d320c4ca1590a1b493af416d165a0e502b08376babc4a").into(), // TC 1
			hex!("a0777497f1d3b4c163c8cc265a201eb7c9fc0eaea2313641260ab622deb8882a").into(), // TC 2
			hex!("cac01e6ea3ecd574c8f91800669ee2fcd164f88b0a51b83bfb1fa5ed986d5023").into(), // TC 3
		]
	}

	pub const SUDO: AccountId = AccountId::new(hex!(
		"f8ce86d05d54a80ca05e8879bdaeecfc56169ec41f3a9bebf58c07dcaa5b0423"
	));

	pub fn user_accounts() -> Vec<(AccountId, Balance)> {
		vec![
			// Faucet bridge.
			(
				hex!("166efe5750d52f473dee8ef21a3b31779e09b2140753db34f9e9aa6cab6c9000").into(),
				1_000_000 * AVL,
			),
		]
	}
}

// The URL for the telemetry server.
// const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";
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

#[derive(Clone)]
pub struct InitialAuthority {
	pub stash: AccountId,
	pub stash_balance: Balance,
	pub controller: AccountId,
	pub controller_balance: Balance,
	pub session_keys: SessionKeys,
}

impl InitialAuthority {
	fn new(stash: AccountId, controller: AccountId, grandpa: GrandpaId) -> Self {
		Self {
			stash,
			stash_balance: 1_000_000 * AVL,
			session_keys: session_keys(controller.clone(), grandpa),
			controller,
			controller_balance: 10 * AVL,
		}
	}
}

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

type AccountPublic = <Signature as Verify>::Signer;

/// Generate an account ID from seed.
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Helper function to generate stash, controller and session key from seed
pub fn authority_keys_from_seed(
	seed: &str,
) -> (
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

/// Helper function to create GenesisConfig for testing
pub fn testnet_genesis(
	initial_authorities: Vec<(
		AccountId,
		AccountId,
		GrandpaId,
		BabeId,
		ImOnlineId,
		AuthorityDiscoveryId,
	)>,
	initial_nominators: Vec<AccountId>,
	root_key: AccountId,
	endowed_accounts: Option<Vec<AccountId>>,
) -> GenesisConfig {
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
	initial_authorities
		.iter()
		.map(|x| &x.0)
		.chain(initial_nominators.iter())
		.for_each(|x| {
			if !endowed_accounts.contains(x) {
				endowed_accounts.push(x.clone())
			}
		});

	// stakers: all validators and nominators.
	let mut rng = rand::thread_rng();
	let stakers = initial_authorities
		.iter()
		.map(|x| (x.0.clone(), x.1.clone(), STASH, StakerStatus::Validator))
		.chain(initial_nominators.iter().map(|x| {
			use rand::{seq::SliceRandom, Rng};
			let limit = (MAX_NOMINATIONS as usize).min(initial_authorities.len());
			let count = rng.gen::<usize>() % limit;
			let nominations = initial_authorities
				.as_slice()
				.choose_multiple(&mut rng, count)
				.into_iter()
				.map(|choice| choice.0.clone())
				.collect::<Vec<_>>();
			(
				x.clone(),
				x.clone(),
				STASH,
				StakerStatus::Nominator(nominations),
			)
		}))
		.collect::<Vec<_>>();

	let num_endowed_accounts = endowed_accounts.len();

	const ENDOWMENT: Balance = 10_000_000 * AVL;
	const STASH: Balance = ENDOWMENT / 1000;

	GenesisConfig {
		system: SystemConfig {
			// Add Wasm runtime to storage.
			code: wasm_binary_unwrap().to_vec(),
			kc_public_params: kate::testnet::public_params(MAX_BLOCK_COLUMNS as usize)
				.to_raw_var_bytes(),
			block_length: BlockLength::with_normal_ratio(
				MAX_BLOCK_ROWS,
				MAX_BLOCK_COLUMNS,
				32,
				Perbill::from_percent(90),
			),
		},
		balances: BalancesConfig {
			balances: endowed_accounts
				.iter()
				.cloned()
				.map(|x| (x, ENDOWMENT))
				.collect(),
		},
		indices: IndicesConfig { indices: vec![] },
		session: SessionConfig {
			keys: initial_authorities
				.iter()
				.map(|x| {
					(x.0.clone(), x.0.clone(), SessionKeys {
						grandpa: x.2.clone(),
						babe: x.3.clone(),
						im_online: x.4.clone(),
						authority_discovery: x.5.clone(),
					})
				})
				.collect::<Vec<_>>(),
		},
		staking: StakingConfig {
			validator_count: initial_authorities.len() as u32,
			minimum_validator_count: initial_authorities.len() as u32,
			invulnerables: initial_authorities.iter().map(|x| x.0.clone()).collect(),
			slash_reward_fraction: Perbill::from_percent(10),
			stakers,
			..Default::default()
		},
		democracy: DemocracyConfig::default(),
		elections: ElectionsConfig {
			members: endowed_accounts
				.iter()
				.take(DesiredMembers::get().saturated_into())
				.cloned()
				.map(|member| (member, STASH))
				.collect(),
		},
		council: CouncilConfig::default(),
		technical_committee: TechnicalCommitteeConfig {
			members: endowed_accounts
				.iter()
				.take((num_endowed_accounts + 1) / 2)
				.cloned()
				.collect(),
			phantom: Default::default(),
		},
		sudo: SudoConfig {
			key: root_key.clone(),
		},
		babe: BabeConfig {
			authorities: vec![],
			epoch_config: Some(da_runtime::BABE_GENESIS_EPOCH_CONFIG),
		},
		im_online: ImOnlineConfig { keys: vec![] },
		authority_discovery: AuthorityDiscoveryConfig { keys: vec![] },
		grandpa: GrandpaConfig {
			authorities: vec![],
		},
		technical_membership: Default::default(),
		treasury: Default::default(),
		scheduler: Default::default(),
		transaction_payment: Default::default(),
		data_availability: DataAvailabilityConfig {
			app_keys: vec![
				(b"Data Avail".to_vec(), AppKeyInfo {
					owner: root_key.clone(),
					id: 0.into(),
				}),
				(b"Ethereum".to_vec(), AppKeyInfo {
					owner: root_key.clone(),
					id: 1.into(),
				}),
				(b"Polygon".to_vec(), AppKeyInfo {
					owner: root_key,
					id: 2.into(),
				}),
			],
		},
		updater_manager: UpdaterManagerConfig {
			updater: "0x1563915e194d8cfba1943570603f7606a3115508"
				.parse()
				.unwrap(),
			_phantom: Default::default(),
		},
		nomad_home: NomadHomeConfig {
			local_domain: 2000,
			committed_root: Default::default(),
			updater: "0x1563915e194d8cfba1943570603f7606a3115508"
				.parse()
				.unwrap(),
			_phantom: Default::default(),
		},
		da_bridge: Default::default(),
	}
}

fn development_config_genesis() -> GenesisConfig {
	testnet_genesis(
		vec![authority_keys_from_seed("Alice")],
		vec![],
		get_account_id_from_seed::<sr25519::Public>("Alice"),
		None,
	)
}
// //single validator ALICE
pub fn development_config() -> ChainSpec {
	ChainSpec::from_genesis(
		"Avail-Dev",
		"Dev",
		ChainType::Development,
		development_config_genesis,
		vec![],
		None,
		None,
		chain_properties(),
		Default::default(),
	)
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

fn genesis_builder(
	sudo_init: (AccountId, Balance),
	initial_authorities: Vec<InitialAuthority>,
	technical_committee: Vec<AccountId>,
	user_accounts: Vec<(AccountId, Balance)>,
	stake_amount: Balance,
) -> GenesisConfig {
	const MIN_NOMINATOR_BOND: Balance = 10 * AVL;
	const ELECTION_STASH: Balance = AVL;

	// Stash accounts + Controller accs + User accounts + Sudo
	let balances = initial_authorities
		.iter()
		.cloned()
		.map(|auth| (auth.stash, auth.stash_balance))
		.chain(
			initial_authorities
				.iter()
				.filter(|auth| {
					!initial_authorities
						.iter()
						.any(|inner_auth| inner_auth.stash == auth.controller)
				})
				.cloned()
				.map(|auth| (auth.controller, auth.controller_balance)),
		)
		.chain(user_accounts.iter().cloned())
		.chain(Some(sudo_init.clone()).into_iter())
		.collect();
	let sudo_key = sudo_init.0;

	GenesisConfig {
		system: SystemConfig {
			// Add Wasm runtime to storage.
			code: wasm_binary_unwrap().to_vec(),
			kc_public_params: kate::testnet::public_params(MAX_BLOCK_COLUMNS as usize)
				.to_raw_var_bytes(),
			block_length: BlockLength::with_normal_ratio(
				MAX_BLOCK_ROWS,
				MAX_BLOCK_COLUMNS,
				32,
				Perbill::from_percent(90),
			),
		},
		balances: BalancesConfig { balances },
		indices: IndicesConfig { indices: vec![] },
		session: SessionConfig {
			keys: initial_authorities
				.iter()
				.cloned()
				.map(|acc| (acc.stash, acc.controller, acc.session_keys))
				.collect(),
		},
		staking: StakingConfig {
			validator_count: 3,
			minimum_validator_count: 3,
			invulnerables: vec![],
			slash_reward_fraction: Perbill::from_percent(10),
			stakers: initial_authorities
				.into_iter()
				.map(|auth| {
					(
						auth.stash,
						auth.controller,
						stake_amount + AVL,
						StakerStatus::Validator,
					)
				})
				.collect(),
			min_nominator_bond: MIN_NOMINATOR_BOND,
			min_validator_bond: stake_amount,
			..Default::default()
		},
		democracy: DemocracyConfig::default(),
		elections: ElectionsConfig {
			members: user_accounts
				.into_iter()
				.take(DesiredMembers::get().saturated_into())
				.map(|member| (member.0, ELECTION_STASH))
				.collect(),
		},
		council: CouncilConfig::default(),
		technical_committee: TechnicalCommitteeConfig {
			members: technical_committee,
			phantom: Default::default(),
		},
		sudo: SudoConfig {
			key: sudo_key.clone(),
		},
		babe: BabeConfig {
			authorities: vec![],
			epoch_config: Some(da_runtime::BABE_GENESIS_EPOCH_CONFIG),
		},
		im_online: ImOnlineConfig { keys: vec![] },
		authority_discovery: AuthorityDiscoveryConfig { keys: vec![] },
		grandpa: GrandpaConfig {
			authorities: vec![],
		},
		technical_membership: Default::default(),
		treasury: Default::default(),
		scheduler: Default::default(),
		transaction_payment: Default::default(),
		data_availability: DataAvailabilityConfig {
			app_keys: vec![
				(b"Data Avail".to_vec(), AppKeyInfo {
					owner: sudo_key.clone(),
					id: 0.into(),
				}),
				(b"Ethereum".to_vec(), AppKeyInfo {
					owner: sudo_key.clone(),
					id: 1.into(),
				}),
				(b"Polygon".to_vec(), AppKeyInfo {
					owner: sudo_key,
					id: 2.into(),
				}),
			],
		},
		updater_manager: Default::default(),
		nomad_home: Default::default(),
		da_bridge: Default::default(),
	}
}

/// Local testnet config (multivalidator Alice + Bob)
pub fn testnet_config() -> ChainSpec {
	ChainSpec::from_genesis(
		"Avail-Testnet",
		"da_testnet",
		ChainType::Live,
		|| {
			genesis_builder(
				(testnet::SUDO, 1_000 * AVL),
				testnet::authorities(),
				testnet::technical_committee(),
				testnet::user_accounts(),
				1_000 * AVL,
			)
		},
		vec![],
		None,
		Some("da1"),
		chain_properties(),
		Default::default(),
	)
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
	#[ignore]
	fn test_create_development_chain_spec() { development_config().build_storage().unwrap(); }

	#[test]
	#[ignore]
	fn test_create_local_testnet_chain_spec() { testnet_config().build_storage().unwrap(); }
}

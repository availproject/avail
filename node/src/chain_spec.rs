use da_control::AppKeyInfo;
use da_primitives::currency::AVL;
use da_runtime::{
	wasm_binary_unwrap, AccountId, AuthorityDiscoveryConfig, BabeConfig, Balance, BalancesConfig,
	Block, CouncilConfig, DataAvailabilityConfig, DemocracyConfig, DesiredMembers, ElectionsConfig,
	GenesisConfig, GrandpaConfig, ImOnlineConfig, IndicesConfig, SessionConfig, SessionKeys,
	Signature, StakerStatus, StakingConfig, SudoConfig, SystemConfig, TechnicalCommitteeConfig,
	MAX_NOMINATIONS,
};
use frame_system::limits::BlockLength;
use kate::config::{MAX_BLOCK_COLUMNS, MAX_BLOCK_ROWS};
use pallet_im_online::sr25519::AuthorityId as ImOnlineId;
use sc_chain_spec::ChainSpecExtension;
use sc_service::{ChainType, Properties};
use serde::{Deserialize, Serialize};
use sp_authority_discovery::AuthorityId as AuthorityDiscoveryId;
use sp_consensus_babe::AuthorityId as BabeId;
use sp_core::{crypto::Ss58Codec as _, sr25519, Pair, Public};
use sp_finality_grandpa::AuthorityId as GrandpaId;
use sp_runtime::{
	traits::{IdentifyAccount, Verify},
	Perbill, SaturatedConversion,
};

mod testnet {
	pub const VALIDATORS: &[&str] = &[
		"5G7DPfMuai9jUokYhxhiPMZ1RTtSAbY8bpHns4LUxB2Jw92p", // Validator 1
		"5CRmQzLgjknmzRHvgBFZw9mt4q1YgiS5nAcnqM7kRohTi4EG", // Validator 2
		"5H9CGZS3WJdew9dSLCxHk8ffqgkjdJVecozH6GHwQ7edtWZX", // Validator 3
	];
	pub const TECHNICAL_COMMITTEE: &[&str] = &[
		"5GqLfdcNekxmyDnM1jg7fs2HLXDkDXdvy6UP9pJNSFvaWefm", // TC 1
		"5Fh71D2b95sro2ZwXZULJMYy7FMst6PAgbYQd8MBifKkrbLx", // TC 2
		"5GeYbpM2uU35z8kR6bH3fXpRg6Q182WAuBBh843LZVv86ciN", // TC 3
	];
	pub const SUDO: &str = "5Hgw5p7Z2MtDiPPsBwJMghngdmmKeqLjWsNiSHqn8YHrehG7";
	pub const ACCOUNTS: &[&str] = &[
		"5HdWFbYTMdWQNeQGhHiKNJQiYrfjxdaJ24fg7K4KYbzzPHXD", // Controller validator 1.
		"5G6EzSquYhJe6pLw7CLtXFciovVvzF6cHDiHL1A74knHdACt", // Controller validator 2.
		"5GRT695LWMxf9ra37s7L4akM7i4imvFhnNFn9G4fTehUvHXT", // Controller validator 3.
		"5Ca7t9GrLun6PihvjzeMo2awnaNm7J9udfEBxy7QDTX8q54W",
	];
	pub const STAKERS: &[(&str, &str)] = &[
		(VALIDATORS[0], ACCOUNTS[0]),
		(VALIDATORS[1], ACCOUNTS[1]),
		(VALIDATORS[2], ACCOUNTS[2]),
	];
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

fn session_keys(
	grandpa: GrandpaId,
	babe: BabeId,
	im_online: ImOnlineId,
	authority_discovery: AuthorityDiscoveryId,
) -> SessionKeys {
	SessionKeys {
		grandpa,
		babe,
		im_online,
		authority_discovery,
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
					(
						x.0.clone(),
						x.0.clone(),
						session_keys(x.2.clone(), x.3.clone(), x.4.clone(), x.5.clone()),
					)
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
					id: 0,
				}),
				(b"Ethereum".to_vec(), AppKeyInfo {
					owner: root_key.clone(),
					id: 1,
				}),
				(b"Polygon".to_vec(), AppKeyInfo {
					owner: root_key,
					id: 2,
				}),
			],
		},
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

fn to_session_keys(acc: &AccountId) -> SessionKeys {
	let ss58 = acc.to_ss58check();
	// let raw :&[u8;32]= acc.as_ref();

	SessionKeys {
		babe: BabeId::from_ss58check(&ss58).expect("Invalid Babe SS58 .qed"),
		grandpa: GrandpaId::from_ss58check(&ss58).expect("Invalid Grandpa SS58 .qed"),
		im_online: ImOnlineId::from_ss58check(&ss58).expect("Invalid ImOnline SS58 .qed"),
		authority_discovery: AuthorityDiscoveryId::from_ss58check(&ss58)
			.expect("Invalid Auth Discovery SS58 .qed"),
	}
}

fn genesis_builder(
	sudo: &str,
	validators: &[&str],
	stakers: &[(&str, &str)],
	technical_committee: &[&str],
	accounts: &[&str],
	initial_amount: Balance,
) -> GenesisConfig {
	let sudo = AccountId::from_ss58check(sudo).expect("Invalid sudo key .qed");
	let validators = validators
		.to_vec()
		.into_iter()
		.map(AccountId::from_ss58check)
		.collect::<Result<Vec<_>, _>>()
		.expect("Validator SS58 addresses are invalid .qed");
	let stakers = stakers
		.to_vec()
		.into_iter()
		.map(|(stash, controller)| {
			(
				AccountId::from_ss58check(stash).expect("Invalid stash address"),
				AccountId::from_ss58check(controller).expect("Invalid controller address"),
				10_000 * AVL,
				StakerStatus::Validator,
			)
		})
		.collect::<Vec<_>>();
	let technical_committee = technical_committee
		.to_vec()
		.into_iter()
		.map(AccountId::from_ss58check)
		.collect::<Result<Vec<_>, _>>()
		.expect("Technical Committee addresses are invalid .qed");
	let accounts = accounts
		.to_vec()
		.into_iter()
		.map(AccountId::from_ss58check)
		.collect::<Result<Vec<_>, _>>()
		.expect("Account addresses are invalid .qed");
	let endowed_accs = validators
		.clone()
		.into_iter()
		.chain(accounts.into_iter())
		.chain(Some(sudo.clone()).into_iter())
		.map(|acc| (acc, initial_amount))
		.collect::<Vec<_>>();
	let session_keys = validators
		.clone()
		.into_iter()
		.map(|acc| {
			let keys = to_session_keys(&acc);
			(acc.clone(), acc, keys)
		})
		.collect::<Vec<_>>();

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
			balances: endowed_accs,
		},
		indices: IndicesConfig { indices: vec![] },
		session: SessionConfig { keys: session_keys },
		staking: StakingConfig {
			validator_count: validators
				.len()
				.try_into()
				.expect("Validator set is huge .qed"),
			minimum_validator_count: 2,
			invulnerables: validators.clone(),
			slash_reward_fraction: Perbill::from_percent(10),
			stakers,
			min_nominator_bond: 1_000 * AVL,
			min_validator_bond: 10_000 * AVL,
			..Default::default()
		},
		democracy: DemocracyConfig::default(),
		elections: ElectionsConfig { members: vec![] },
		council: CouncilConfig::default(),
		technical_committee: TechnicalCommitteeConfig {
			members: technical_committee,
			phantom: Default::default(),
		},
		sudo: SudoConfig { key: sudo.clone() },
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
					owner: sudo.clone(),
					id: 0,
				}),
				(b"Ethereum".to_vec(), AppKeyInfo {
					owner: sudo.clone(),
					id: 1,
				}),
				(b"Polygon".to_vec(), AppKeyInfo { owner: sudo, id: 2 }),
			],
		},
	}
}

/// Local testnet config (multivalidator Alice + Bob)
pub fn local_testnet_config() -> ChainSpec {
	ChainSpec::from_genesis(
		"Avail-Testnet",
		"local_testnet",
		ChainType::Local,
		|| {
			genesis_builder(
				testnet::SUDO,
				testnet::VALIDATORS,
				testnet::STAKERS,
				testnet::TECHNICAL_COMMITTEE,
				testnet::ACCOUNTS,
				1_000_000 * AVL,
			)
		},
		vec![],
		None,
		None,
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
	fn test_create_local_testnet_chain_spec() { local_testnet_config().build_storage().unwrap(); }
}

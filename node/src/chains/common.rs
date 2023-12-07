use super::{get_account_id_from_seed, AuthorityKeys};
use avail_core::BLOCK_CHUNK_SIZE;
use da_runtime::{
	constants, wasm_binary_unwrap, AccountId, BabeConfig, Balance, BalancesConfig,
	DataAvailabilityConfig, NomadHomeConfig, NomadUpdaterManagerConfig, NominationPoolsConfig,
	RuntimeGenesisConfig, SessionConfig, StakerStatus, StakingConfig, SudoConfig, SystemConfig,
	TechnicalCommitteeConfig, AVL,
};
use frame_system::limits::BlockLength;
use hex_literal::hex;
use kate::config::{MAX_BLOCK_COLUMNS, MAX_BLOCK_ROWS};
use primitive_types::H160;
use sc_telemetry::TelemetryEndpoints;
use sp_core::sr25519::Public;
use sp_runtime::{AccountId32, Perbill};

pub const PROTOCOL_ID: Option<&str> = Some("Avail");
pub const TELEMETRY_URL: &str = "ws://telemetry.avail.tools:8001/submit";
const NOMAD_LOCAL_DOMAIN: u32 = 2000;
const NOMAD_UPDATER: H160 = H160(hex!("695dFcFc604F9b2992642BDC5b173d1a1ed60b03"));
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

fn standard_system_configuration() -> (Vec<u8>, BlockLength) {
	let code = wasm_binary_unwrap().to_vec();

	let block_length = BlockLength::with_normal_ratio(
		MAX_BLOCK_ROWS,
		MAX_BLOCK_COLUMNS,
		BLOCK_CHUNK_SIZE,
		Perbill::from_percent(90),
	)
	.expect("Valid `BlockLength` genesis definition .qed");

	(code, block_length)
}

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
) -> RuntimeGenesisConfig {
	let balances = dev_endowed_accounts();
	let stakers = session_keys
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
	let session_keys = session_keys.into_iter().map(|k| k.into()).collect();

	let (code, block_length) = standard_system_configuration();
	RuntimeGenesisConfig {
		// General
		system: SystemConfig {
			code,
			block_length,
			..Default::default()
		},
		babe: BabeConfig {
			epoch_config: Some(da_runtime::constants::babe::GENESIS_EPOCH_CONFIG),
			..Default::default()
		},
		balances: BalancesConfig { balances },
		staking: StakingConfig {
			stakers,
			validator_count,
			minimum_validator_count: 1,
			..Default::default()
		},
		session: SessionConfig { keys: session_keys },
		technical_committee: TechnicalCommitteeConfig {
			members: technical_committee,
			..Default::default()
		},
		sudo: SudoConfig {
			key: Some(sudo.clone()),
		},
		nomad_home: NomadHomeConfig {
			local_domain: NOMAD_LOCAL_DOMAIN,
			updater: NOMAD_UPDATER,
			..Default::default()
		},
		nomad_updater_manager: NomadUpdaterManagerConfig {
			updater: NOMAD_UPDATER,
			..Default::default()
		},
		nomination_pools: NominationPoolsConfig {
			min_create_bond: constants::nomination_pools::MIN_CREATE_BOND,
			min_join_bond: constants::nomination_pools::MIN_JOIN_BOND,
			max_pools: Some(constants::nomination_pools::MAX_POOLS),
			max_members_per_pool: Some(constants::nomination_pools::MAX_MEMBERS_PER_POOL),
			max_members: Some(constants::nomination_pools::MAX_MEMBERS),
			..Default::default()
		},
		grandpa: Default::default(),
		treasury: Default::default(),
		im_online: Default::default(),
		authority_discovery: Default::default(),
		transaction_payment: Default::default(),
		indices: Default::default(),
		data_availability: make_data_avail_config(sudo),
		technical_membership: Default::default(),
	}
}

use super::{
	chain_properties, dev_endowed_accounts, make_data_avail_config, AuthorityKeys, ChainSpec,
	ENDOWMENT, NOMAD_LOCAL_DOMAIN, NOMAD_UPDATER, PROTOCOL_ID, STASH_BOND, TELEMETRY_URL,
};
use da_runtime::{
	constants, BabeConfig, BalancesConfig, ElectionsConfig, NomadHomeConfig,
	NomadUpdaterManagerConfig, NominationPoolsConfig, RuntimeGenesisConfig, SessionConfig,
	StakerStatus, StakingConfig, SudoConfig, SystemConfig, TechnicalCommitteeConfig,
};
use sc_chain_spec::ChainType;

pub fn chain_spec() -> ChainSpec {
	ChainSpec::from_genesis(
		"Avail Development Network",
		"avail_development_network",
		ChainType::Development,
		runtime_genesis_config,
		vec![],
		Some(super::to_telemetry_endpoint(TELEMETRY_URL.into())),
		PROTOCOL_ID,
		None,
		chain_properties(),
		Default::default(),
	)
}

pub fn runtime_genesis_config() -> RuntimeGenesisConfig {
	let alice = AuthorityKeys::from_seed("Alice");
	let bob = AuthorityKeys::from_seed("Bob");
	let charlie = AuthorityKeys::from_seed("Charlie");
	let endowed_accounts = dev_endowed_accounts();

	let stakers = [
		(alice.controller.clone(), alice.stash.clone()),
		(bob.controller.clone(), bob.stash.clone()),
		(charlie.controller.clone(), charlie.stash.clone()),
	]
	.into_iter()
	.map(|(c, s)| (c, s, STASH_BOND, StakerStatus::Validator))
	.collect();

	let (code, kc_public_params, block_length) = super::standard_system_configuration();
	RuntimeGenesisConfig {
		// General
		system: SystemConfig {
			code,
			kc_public_params,
			block_length,
			..Default::default()
		},
		babe: BabeConfig {
			epoch_config: Some(da_runtime::constants::babe::GENESIS_EPOCH_CONFIG),
			..Default::default()
		},
		balances: BalancesConfig {
			balances: endowed_accounts.clone().into_iter().collect(),
		},
		elections: ElectionsConfig {
			members: vec![
				alice.controller.clone(),
				bob.controller.clone(),
				charlie.controller.clone(),
			]
			.iter()
			.cloned()
			.map(|member| (member, ENDOWMENT))
			.collect(),
		},
		staking: StakingConfig {
			stakers,
			..Default::default()
		},
		session: SessionConfig {
			keys: vec![
				alice.clone().into(),
				bob.clone().into(),
				charlie.clone().into(),
			],
		},
		technical_committee: TechnicalCommitteeConfig {
			members: vec![alice.controller.clone()],
			..Default::default()
		},
		sudo: SudoConfig {
			key: Some(alice.controller.clone()),
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
		democracy: Default::default(),
		council: Default::default(),
		grandpa: Default::default(),
		treasury: Default::default(),
		im_online: Default::default(),
		authority_discovery: Default::default(),
		transaction_payment: Default::default(),
		indices: Default::default(),
		data_availability: make_data_avail_config(alice.controller.clone()),
		technical_membership: Default::default(),
	}
}

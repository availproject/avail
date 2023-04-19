use da_control::AppKeyInfo;
use da_primitives::asdr::AppId;
use da_runtime::{
	constants, wasm_binary_unwrap, AccountId, BabeConfig, Balance, BalancesConfig,
	DataAvailabilityConfig, DesiredMembers, ElectionsConfig, NominationPoolsConfig, SessionConfig,
	StakerStatus, StakingConfig, SudoConfig, SystemConfig, TechnicalCommitteeConfig,
};
use frame_system::limits::BlockLength;
use kate::{
	config::{MAX_BLOCK_COLUMNS, MAX_BLOCK_ROWS},
	testnet::public_params,
};
use sp_runtime::{Perbill, SaturatedConversion as _};

use crate::chain_spec::AuthorityKeys;

/// Creates the common System configuration.
pub(crate) fn make_system_config() -> SystemConfig {
	let code = wasm_binary_unwrap().to_vec();
	let kc_public_params = public_params(MAX_BLOCK_COLUMNS).to_raw_var_bytes();
	let block_length = BlockLength::with_normal_ratio(
		MAX_BLOCK_ROWS,
		MAX_BLOCK_COLUMNS,
		32,
		Perbill::from_percent(90),
	)
	.expect("Valid `BlockLength` genesis definition .qed");

	SystemConfig {
		code,
		kc_public_params,
		block_length,
	}
}

/// Creates the Balances configuration and endows each account in `accounts` with `amount`
pub(crate) fn make_balances_config<I: Iterator<Item = AccountId>>(
	accounts: I,
	amount: Balance,
) -> BalancesConfig {
	let balances = accounts.map(|acc| (acc, amount)).collect();
	BalancesConfig { balances }
}

/// Creates the Session configuration using `authorities` as initial authorities.
pub(crate) fn make_session_config<'a, I: Iterator<Item = &'a AuthorityKeys>>(
	authorities: I,
) -> SessionConfig {
	let keys = authorities
		.map(|a| (a.stash.clone(), a.stash.clone(), a.session_keys.clone()))
		.collect();

	SessionConfig { keys }
}

/// Creates the Staking configuration using `authorities` as initial authorities, and bounding
/// `min_validator_bond` amount on each of them.
pub(crate) fn make_staking_config<'a, I: Iterator<Item = &'a AuthorityKeys>>(
	authorities: I,
	min_validator_bond: Balance,
	min_nominator_bond: Balance,
) -> StakingConfig {
	let (stakers, invulnerables): (Vec<_>, Vec<_>) = authorities
		.map(|auth| {
			let invulnerable = auth.stash.clone();
			let staker = (
				auth.stash.clone(),
				auth.controller.clone(),
				min_validator_bond,
				StakerStatus::Validator,
			);
			(staker, invulnerable)
		})
		.unzip();

	let len: u32 = invulnerables
		.len()
		.try_into()
		.expect("Too much authorities .qed");

	StakingConfig {
		stakers,
		min_validator_bond,
		min_nominator_bond,
		invulnerables,
		slash_reward_fraction: Perbill::from_percent(10),
		validator_count: len,
		minimum_validator_count: len,
		..Default::default()
	}
}

/// Creates the Technical Committee configuration and adds `members` to it.
pub(crate) fn make_technical_committee_config(members: Vec<AccountId>) -> TechnicalCommitteeConfig {
	TechnicalCommitteeConfig {
		members,
		..Default::default()
	}
}

/// Uses `key` as `sudo` key.
pub(crate) fn make_sudo_config(key: AccountId) -> SudoConfig { SudoConfig { key: Some(key) } }

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

	DataAvailabilityConfig { app_keys }
}

pub(crate) fn make_nomination_pools_config() -> NominationPoolsConfig {
	NominationPoolsConfig {
		min_create_bond: constants::nomination_pools::MIN_CREATE_BOND,
		min_join_bond: constants::nomination_pools::MIN_JOIN_BOND,
		max_pools: Some(constants::nomination_pools::MAX_POOLS),
		max_members_per_pool: Some(constants::nomination_pools::MAX_MEMBERS_PER_POOL),
		max_members: Some(constants::nomination_pools::MAX_MEMBERS),
	}
}

pub(crate) fn make_babe_config() -> BabeConfig {
	let epoch_config = Some(da_runtime::constants::BABE_GENESIS_EPOCH_CONFIG);

	BabeConfig {
		// NOTE: `authorities` were initialized by `Session`.
		authorities: vec![],
		epoch_config,
	}
}

/// Creates the Phragmen Election configuration, using up to `T::DesiredMembers` from
/// `authorities` as members.
pub(crate) fn make_elections<'a, I: Iterator<Item = &'a AuthorityKeys>>(
	authorities: I,
	validator_bond: Balance,
) -> ElectionsConfig {
	let members = authorities
		.take(DesiredMembers::get().saturated_into())
		.map(|auth| (auth.stash.clone(), validator_bond))
		.collect();

	ElectionsConfig { members }
}

pub(crate) mod nomad {
	use da_runtime::{NomadHomeConfig, NomadUpdaterManagerConfig};
	use primitive_types::H160;

	pub(crate) fn make_home_config(local_domain: u32, updater: H160) -> NomadHomeConfig {
		NomadHomeConfig {
			local_domain,
			updater,
			..Default::default()
		}
	}

	pub(crate) fn make_update_manager_config(updater: H160) -> NomadUpdaterManagerConfig {
		NomadUpdaterManagerConfig {
			updater,
			..Default::default()
		}
	}
}

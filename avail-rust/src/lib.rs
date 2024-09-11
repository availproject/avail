mod api_dev;
mod config;
mod from_substrate;
mod rpcs;
mod sdk;
mod utils;

// Export types for internal and external consumption
pub mod primitives;
pub mod transactions;

pub type RewardDestination =
	api_dev::api::runtime_types::pallet_staking::RewardDestination<AccountId>;

pub use api_dev::api::data_availability::calls::types::create_application_key::Key;
pub use api_dev::api::data_availability::calls::types::submit_data::Data;
pub use api_dev::api::runtime_types::frame_support::dispatch::DispatchFeeModifier;
pub use api_dev::api::runtime_types::pallet_staking::ValidatorPrefs;
pub use subxt::config::polkadot::H256;
pub use subxt_signer::{sr25519::Keypair, SecretUri};

pub use api_dev::api as avail;
pub use config::*;
pub use sdk::{WaitFor, SDK};

pub use avail_core;
pub use kate_recovery;
pub use primitives::block::{
	AppUncheckedExtrinsic, AvailHeader, DefaultExtrinsicParams, DefaultExtrinsicParamsBuilder,
};
pub use primitives::kate::{Cell, GDataProof, GRow};
pub use sp_core;
pub use subxt;
pub use subxt::config::polkadot::U256;
pub use subxt_signer;
pub use utils::utils_raw;
pub use utils::FetchTransactionError;

mod api_dev;
mod config;
mod primitives;
mod rpcs;
mod sdk;
mod transaction_data;
mod transactions;
mod utils;

// Export types for internal and external consumption
pub type RewardDestination =
	api_dev::api::runtime_types::pallet_staking::RewardDestination<AccountId>;

pub use api_dev::api::data_availability::calls::types::create_application_key::Key;
pub use api_dev::api::data_availability::calls::types::submit_data::Data;
pub use api_dev::api::runtime_types::frame_support::dispatch::DispatchFeeModifier;
pub use api_dev::api::runtime_types::pallet_staking::ValidatorPrefs;
pub use primitives::{AvailExtrinsicParams, AvailExtrinsicParamsBuilder};
pub use subxt::config::polkadot::H256;
pub use subxt_signer::{sr25519::Keypair, SecretUri};

pub use crate::config::*;
pub use crate::primitives::*;
pub use crate::sdk::{WaitFor, SDK};
pub use api_dev::api as avail;

pub use avail_core;
pub use subxt;
pub use subxt::config::polkadot::U256;
pub use utils::utils_raw;
pub use utils::FetchTransactionError;

use crate::api_dev::api::data_availability::calls::types::submit_data::Data;
use crate::{avail, AccountId, Api, AvailBlocksClient, AvailConfig, H256};

use subxt::blocks::{Extrinsics, FoundExtrinsic, StaticExtrinsic};
use subxt::utils::MultiAddress;

use avail::data_availability::calls::types as DataAvailabilityCalls;
use avail::staking::calls::types as StakingCalls;

pub mod data_availability {
	use super::*;

	#[derive(Debug, Clone, Eq, PartialEq)]
	pub struct SubmitData {
		pub data: Data,
	}

	impl SubmitData {
		pub async fn new(
			block_hash: H256,
			tx_hash: H256,
			blocks: &AvailBlocksClient,
		) -> Result<Self, String> {
			let block = blocks.at(block_hash).await;
			let block = match block {
				Ok(b) => b,
				Err(error) => return Err(error.to_string()),
			};

			let extrinsics = block.extrinsics().await;
			let extrinsics = match extrinsics {
				Ok(e) => e,
				Err(error) => return Err(error.to_string()),
			};

			let maybe_transaction =
				find_transaction::<DataAvailabilityCalls::SubmitData>(tx_hash, &extrinsics).await;

			let Some(transaction) = maybe_transaction else {
				return Err(String::from("Transaction data not found"));
			};

			Ok(Self {
				data: transaction.value.data,
			})
		}
	}
}
pub mod staking {
	use super::*;

	pub struct Nominate {
		pub targets: Vec<String>,
	}

	impl Nominate {
		pub async fn new(
			block_hash: H256,
			tx_hash: H256,
			blocks: &AvailBlocksClient,
		) -> Result<Self, String> {
			let block = blocks.at(block_hash).await;
			let block = match block {
				Ok(b) => b,
				Err(error) => return Err(error.to_string()),
			};

			let extrinsics = block.extrinsics().await;
			let extrinsics = match extrinsics {
				Ok(e) => e,
				Err(error) => return Err(error.to_string()),
			};

			let maybe_transaction =
				find_transaction::<StakingCalls::Nominate>(tx_hash, &extrinsics).await;

			let Some(transaction) = maybe_transaction else {
				return Err(String::from("Transaction data not found"));
			};

			let targets = transaction.value.targets;
			let targets: Vec<AccountId> = targets
				.into_iter()
				.map(|a| match a {
					MultiAddress::Id(account) => account,
					_ => panic!("Should never happen"),
				})
				.collect();
			let targets = targets.into_iter().map(|a| std::format!("{}", a)).collect();

			Ok(Self { targets })
		}
	}
}

async fn find_transaction<E: StaticExtrinsic>(
	tx_hash: H256,
	extrinsics: &Extrinsics<AvailConfig, Api>,
) -> Option<FoundExtrinsic<AvailConfig, Api, E>> {
	let found_extrinsics = extrinsics.find::<E>();
	for ext in found_extrinsics {
		let ext = match ext {
			Ok(e) => e,
			Err(_) => continue,
		};
		let events = ext.details.events().await;
		let events = match events {
			Ok(e) => e,
			Err(_) => continue,
		};
		if events.extrinsic_hash() == tx_hash {
			return Some(ext);
		}
	}

	None
}

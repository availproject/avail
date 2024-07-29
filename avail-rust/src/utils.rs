use subxt::{
	blocks::{Extrinsics, FoundExtrinsic, StaticExtrinsic},
	tx::{TxProgress, TxStatus},
	utils::H256,
};

use crate::{Api, AvailBlocksClient, AvailConfig, TransactionInBlock, WaitFor};
use utils_raw::*;

#[derive(Debug, Clone, Copy)]
pub enum FetchTransactionError {
	FailedToFetchBlock,
	FailedToGetTransactionsFromBlock,
	TransactionNotFoundInTheBlock,
}

impl FetchTransactionError {
	pub fn to_string(&self) -> String {
		match self {
			FetchTransactionError::FailedToFetchBlock => String::from("Failed to fetch block"),
			FetchTransactionError::FailedToGetTransactionsFromBlock => {
				String::from("Failed to get transaction from block")
			},
			FetchTransactionError::TransactionNotFoundInTheBlock => {
				String::from("Transaction not found in the block")
			},
		}
	}
}

pub struct Util {
	blocks_api: AvailBlocksClient,
}

impl Util {
	pub fn new(api: Api) -> Self {
		let blocks_api = api.blocks();
		Self { blocks_api }
	}

	pub async fn fetch_transactions(
		&self,
		block_hash: H256,
	) -> Result<Extrinsics<AvailConfig, Api>, FetchTransactionError> {
		fetch_transactions(block_hash, &self.blocks_api).await
	}

	pub async fn fetch_transaction<E: StaticExtrinsic>(
		&self,
		block_hash: H256,
		tx_hash: H256,
	) -> Result<FoundExtrinsic<AvailConfig, Api, E>, FetchTransactionError> {
		fetch_transaction(block_hash, tx_hash, &self.blocks_api).await
	}

	pub async fn progress_transaction(
		&self,
		maybe_tx_progress: Result<TxProgress<AvailConfig, Api>, subxt::Error>,
		wait_for: WaitFor,
	) -> Result<TransactionInBlock, String> {
		progress_transaction(maybe_tx_progress, wait_for).await
	}
}

pub mod utils_raw {
	pub use super::*;

	pub async fn fetch_transactions(
		block_hash: H256,
		blocks_api: &AvailBlocksClient,
	) -> Result<Extrinsics<AvailConfig, Api>, FetchTransactionError> {
		let block = blocks_api.at(block_hash).await;
		let block = match block {
			Ok(b) => b,
			Err(_) => return Err(FetchTransactionError::FailedToFetchBlock),
		};

		let extrinsics = block.extrinsics().await;
		let extrinsics = match extrinsics {
			Ok(e) => e,
			Err(_) => return Err(FetchTransactionError::FailedToGetTransactionsFromBlock),
		};

		Ok(extrinsics)
	}

	pub async fn fetch_transaction<E: StaticExtrinsic>(
		block_hash: H256,
		tx_hash: H256,
		blocks_api: &AvailBlocksClient,
	) -> Result<FoundExtrinsic<AvailConfig, Api, E>, FetchTransactionError> {
		let extrinsics = fetch_transactions(block_hash, blocks_api).await?;

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
				return Ok(ext);
			}
		}

		Err(FetchTransactionError::TransactionNotFoundInTheBlock)
	}

	pub async fn progress_transaction(
		maybe_tx_progress: Result<TxProgress<AvailConfig, Api>, subxt::Error>,
		wait_for: WaitFor,
	) -> Result<TransactionInBlock, String> {
		if let Err(error) = maybe_tx_progress {
			return Err(error.to_string());
		}
		let mut tx_progress = maybe_tx_progress.unwrap();

		while let Some(tx_status) = tx_progress.next().await {
			let tx_status = match tx_status {
				Ok(tx_status) => tx_status,
				Err(err) => return Err(err.to_string()),
			};

			match tx_status {
				TxStatus::InBestBlock(tx_in_block) => {
					if wait_for == WaitFor::BlockInclusion {
						return Ok(tx_in_block);
					}
				},
				TxStatus::InFinalizedBlock(tx_in_block) => {
					if wait_for == WaitFor::BlockFinalization {
						return Ok(tx_in_block);
					}
				},
				TxStatus::Error { message } => return Err(message),
				TxStatus::Invalid { message } => return Err(message),
				TxStatus::Dropped { message } => return Err(message),
				_ => {},
			};
		}

		Err(String::from("Something went wrong."))
	}
}

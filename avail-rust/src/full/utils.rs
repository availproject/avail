use subxt::{
	backend::legacy::rpc_methods::Bytes,
	blocks::{Extrinsics, FoundExtrinsic, StaticExtrinsic},
	tx::{TxProgress, TxStatus},
};

use crate::{
	avail::runtime_types::da_runtime::primitives::SessionKeys, Api, AppUncheckedExtrinsic,
	AvailBlocksClient, AvailConfig, BlockHash, TransactionInBlock, WaitFor,
};
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

#[derive(Clone)]
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
		block_hash: BlockHash,
	) -> Result<Extrinsics<AvailConfig, Api>, FetchTransactionError> {
		fetch_transactions(block_hash, &self.blocks_api).await
	}

	pub async fn fetch_transaction<E: StaticExtrinsic>(
		&self,
		block_hash: BlockHash,
		tx_hash: BlockHash,
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

	pub fn decode_raw_block_rpc_extrinsics(
		&self,
		extrinsics: Vec<Bytes>,
	) -> Result<Vec<AppUncheckedExtrinsic>, String> {
		decode_raw_block_rpc_extrinsics(extrinsics)
	}

	pub fn deconstruct_session_keys(&self, session_keys: Vec<u8>) -> Result<SessionKeys, String> {
		deconstruct_session_keys(session_keys)
	}

	pub fn deconstruct_session_keys_string(
		&self,
		session_keys: String,
	) -> Result<SessionKeys, String> {
		if session_keys.len() != 256 {
			return Err(String::from(
				"Session keys len cannot have length be more or less than 256",
			));
		}

		let err = || String::from("Internal Math Error");
		let len = session_keys.len();
		let mut session_keys_u8: Vec<u8> = Vec::with_capacity(128);
		let mut iter = session_keys.chars();
		for _ in (0..len).step_by(2) {
			let value_1: u8 = iter
				.next()
				.and_then(|v| v.to_digit(16))
				.and_then(|v| Some((v * 16) as u8))
				.ok_or_else(err)?;
			let value_2: u8 = iter
				.next()
				.and_then(|v| v.to_digit(16))
				.and_then(|v| Some(v as u8))
				.ok_or_else(err)?;
			session_keys_u8.push(value_1 + value_2);
		}

		if session_keys_u8.len() != 128 {
			return Err(String::from(
				"Something went wrong and the length of the calculated session keys is wrong",
			));
		}

		deconstruct_session_keys(session_keys_u8)
	}
}

pub mod utils_raw {
	pub use super::*;

	pub async fn fetch_transactions(
		block_hash: BlockHash,
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
		block_hash: BlockHash,
		tx_hash: BlockHash,
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

	pub fn decode_raw_block_rpc_extrinsics(
		extrinsics: Vec<Bytes>,
	) -> Result<Vec<AppUncheckedExtrinsic>, String> {
		let extrinsics: Result<Vec<AppUncheckedExtrinsic>, String> = extrinsics
			.into_iter()
			.map(|e| AppUncheckedExtrinsic::try_from(e))
			.collect();

		extrinsics
	}

	pub fn deconstruct_session_keys(session_keys: Vec<u8>) -> Result<SessionKeys, String> {
		use crate::avail::runtime_types::sp_core::ed25519::Public as EDPublic;
		use crate::avail::runtime_types::sp_core::sr25519::Public as SRPublic;
		use crate::avail::runtime_types::{
			pallet_im_online, sp_authority_discovery, sp_consensus_babe, sp_consensus_grandpa,
		};
		use core::array::TryFromSliceError;

		if session_keys.len() != 128 {
			return Err(String::from(
				"Session keys len cannot have length be more or less than 128",
			));
		}

		let err = |e: TryFromSliceError| e.to_string();

		let babe: [u8; 32] = session_keys[0..32].try_into().map_err(err)?;
		let grandpa: [u8; 32] = session_keys[32..64].try_into().map_err(err)?;
		let im_online: [u8; 32] = session_keys[64..96].try_into().map_err(err)?;
		let authority_discovery: [u8; 32] = session_keys[96..128].try_into().map_err(err)?;

		Ok(SessionKeys {
			babe: sp_consensus_babe::app::Public(SRPublic(babe)),
			grandpa: sp_consensus_grandpa::app::Public(EDPublic(grandpa)),
			im_online: pallet_im_online::sr25519::app_sr25519::Public(SRPublic(im_online)),
			authority_discovery: sp_authority_discovery::app::Public(SRPublic(authority_discovery)),
		})
	}
}

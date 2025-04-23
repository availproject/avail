mod block_worker;
mod cache;
mod database_map;
mod database_worker;

pub mod constants;

use super::common::read_pallet_call_index;
use avail_core::OpaqueExtrinsic;
pub use block_worker::BlockWorker;
use codec::Encode;
use da_runtime::UncheckedExtrinsic;
pub use database_worker::DatabaseWorker;
use jsonrpsee::tokio::sync::{mpsc, Notify};
use serde::{Deserialize, Serialize};
use sp_core::{Blake2Hasher, Hasher, H256};
use std::sync::Arc;
use transaction_rpc::transaction_overview;

pub type Channel = BlockDetails;
pub type Receiver = mpsc::Receiver<BlockDetails>;
pub type Sender = mpsc::Sender<BlockDetails>;

#[derive(Clone, Default)]
pub struct CliDeps {
	pub enabled: bool,
	pub logging_interval: u64,
	pub block_pruning: usize,
	pub result_length: usize,
	// in kB
	pub event_cache_size: u64,
}

pub struct Deps {
	pub block_receiver: Receiver,
	pub block_sender: Sender,
	pub transaction_receiver: transaction_overview::Receiver,
	pub notifier: Arc<Notify>,
	pub cli: CliDeps,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockDetails {
	pub block_hash: H256,
	pub block_height: u32,
	pub finalized: bool,
	pub transactions: Vec<TransactionDetails>,
}

impl BlockDetails {
	pub fn from_opaques(
		opaques: Vec<OpaqueExtrinsic>,
		block_hash: H256,
		block_height: u32,
		finalized: bool,
	) -> Self {
		let mut transactions: Vec<TransactionDetails> = Vec::with_capacity(opaques.len());
		for (index, ext) in opaques.iter().enumerate() {
			let unchecked_ext =
				match UncheckedExtrinsic::decode_no_vec_prefix(&mut ext.0.as_slice()) {
					Ok(x) => x,
					Err(_) => continue,
				};

			let Some(dispatch_index) = read_pallet_call_index(&unchecked_ext) else {
				continue;
			};

			let hash = Blake2Hasher::hash(&unchecked_ext.encode());
			let info = TransactionDetails::new(hash, index as u32, dispatch_index);
			transactions.push(info);
		}

		BlockDetails {
			block_hash,
			block_height,
			finalized,
			transactions,
		}
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionDetails {
	pub hash: H256,
	pub index: u32,
	// (Pallet id, Call id)
	pub dispatch_index: (u8, u8),
}

impl TransactionDetails {
	pub fn new(hash: H256, index: u32, dispatch_index: (u8, u8)) -> Self {
		Self {
			hash,
			index,
			dispatch_index,
		}
	}
}

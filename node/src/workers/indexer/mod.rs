mod block_worker;
mod cache;
mod database_map;
mod database_worker;

use super::common::read_pallet_call_index;
use avail_core::OpaqueExtrinsic;
use block_rpc::{
	common::{DispatchIndex, TransactionLocation},
	transaction_overview, BlockIdentifier,
};
use codec::Encode;
use da_runtime::UncheckedExtrinsic;
pub use database_worker::DatabaseWorker;
use jsonrpsee::tokio::sync::{mpsc, Notify};
use serde::{Deserialize, Serialize};
use sp_core::{Blake2Hasher, Hasher};
use std::sync::Arc;

pub use block_worker::BlockWorker;

pub type Channel = BlockDetails;
pub type Receiver = mpsc::Receiver<BlockDetails>;
pub type Sender = mpsc::Sender<BlockDetails>;

pub const BLOCK_CHANNEL_LIMIT: usize = 20_000;
pub const RPC_CHANNEL_LIMIT: usize = 20_000;
pub const DATABASE_SIZE_BUFFER: usize = 180; // in blocks. cca every one hour

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
	pub block_id: BlockIdentifier,
	pub finalized: bool,
	pub transactions: Vec<TransactionDetails>,
}

impl BlockDetails {
	pub fn from_opaques(
		opaques: Vec<OpaqueExtrinsic>,
		block_id: BlockIdentifier,
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
			let location = TransactionLocation::from((hash, index as u32));
			let info = TransactionDetails::new(location, dispatch_index);
			transactions.push(info);
		}

		BlockDetails {
			block_id,
			finalized,
			transactions,
		}
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionDetails {
	pub location: TransactionLocation,
	pub dispatch_index: DispatchIndex,
}

impl TransactionDetails {
	pub fn new(location: TransactionLocation, dispatch_index: DispatchIndex) -> Self {
		Self {
			location,
			dispatch_index,
		}
	}
}

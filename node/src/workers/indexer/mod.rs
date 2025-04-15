pub mod constants;
mod database;
mod database_logger;
mod database_map;
mod worker;

pub use database::Database;
use jsonrpsee::tokio::sync::{mpsc, Notify};
use serde::{Deserialize, Serialize};
use sp_core::H256;
use std::sync::Arc;
use transaction_rpc::transaction_overview;
// pub use database_vec::Database as VecDatabase;
pub use worker::Worker;

pub type Channel = BlockDetails;
pub type Receiver = mpsc::Receiver<BlockDetails>;
pub type Sender = mpsc::Sender<BlockDetails>;

#[derive(Clone, Default)]
pub struct CliDeps {
	pub max_search_results: usize,
	pub max_stored_block_count: usize,
	pub logging_interval: u64,
	pub enabled: bool,
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
	pub transactions: Vec<TransactionState>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionState {
	pub tx_hash: H256,
	pub tx_index: u32,
	pub tx_success: bool,
	pub pallet_index: u8,
	pub call_index: u8,
}

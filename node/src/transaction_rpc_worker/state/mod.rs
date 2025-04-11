pub mod constants;
mod database;
mod database_logger;
mod database_map;
mod database_vec;
mod worker;
mod worker_finalized;
mod worker_included;
mod worker_logger;

pub use database::Database;
pub use database_map::Database as MapDatabase;
// pub use database_vec::Database as VecDatabase;
pub use worker_finalized::FinalizedWorker;
pub use worker_included::IncludedWorker;
pub use worker_logger::Logger as WorkerLogger;

use std::time::Duration;
use std::{ops::Add, sync::Arc};

use jsonrpsee::tokio::sync::{mpsc, Notify};
use serde::{Deserialize, Serialize};
use sp_core::H256;
use transaction_rpc::state;

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
	pub search_receiver: state::Receiver,
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

fn generate_duration_stats(
	array: &mut Vec<Duration>,
) -> (usize, Duration, Duration, Duration, Duration) {
	array.sort_unstable();

	let min = array
		.first()
		.cloned()
		.unwrap_or_else(|| Duration::default());

	let max = array.last().cloned().unwrap_or_else(|| Duration::default());

	let count = array.len();
	let total_duration = array.iter().fold(Duration::default(), |acc, x| acc.add(*x));
	let median = if count % 2 != 0 {
		array
			.get(count / 2)
			.cloned()
			.unwrap_or_else(|| Duration::default())
	} else {
		if let (Some(left), Some(right)) = (array.get(count / 2), array.get(count / 2 - 1)) {
			(left.add(*right)).div_f64(2.0)
		} else {
			Duration::default()
		}
	};

	(count, total_duration, min, median, max)
}

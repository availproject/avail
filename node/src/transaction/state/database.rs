use std::time::{Duration, Instant};

use super::constants::DATABASE_RESIZE_INTERVAL;
use jsonrpsee::tokio;
use jsonrpsee::tokio::sync::mpsc::Receiver;
use sc_telemetry::log;
use sp_core::H256;
use transaction_rpc::state_types::TxStateReceiver as SearchReceiver;
use transaction_rpc::state_types::{self, TxStateChannel};

use crate::transaction::macros::profile;

use super::database_logger::DatabaseLogging;
use super::BlockDetails;
pub struct Config {
	pub max_search_results: usize,
	pub max_stored_block_count: usize,
}
pub struct Database<T: DatabaseLike> {
	block_receiver: Receiver<BlockDetails>,
	search_receiver: SearchReceiver,
	logger: DatabaseLogging,
	inner: T,
	timer: Instant,
	timer_interval: Duration,
}

impl<T: DatabaseLike> Database<T> {
	pub fn new(
		block_receiver: Receiver<BlockDetails>,
		search_receiver: SearchReceiver,
		max_search_results: usize,
		max_stored_block_count: usize,
		logging_interval: u64,
	) -> Self {
		let config = Config {
			max_search_results,
			max_stored_block_count,
		};

		Self {
			block_receiver,
			search_receiver,
			logger: DatabaseLogging::new(logging_interval),
			inner: T::new(config),
			timer: Instant::now(),
			timer_interval: Duration::from_secs(DATABASE_RESIZE_INTERVAL),
		}
	}

	pub async fn run(mut self) {
		let config = self.inner.config();
		let variant = self.inner.variant();
		log::info!("👾 Transaction State Running with following parameters: Max Search Result: {}, Max Stored Block Count: {}, Variant: {}, Resize Interval: {}s, Logging Interval: {}s", config.max_search_results, config.max_stored_block_count, variant, DATABASE_RESIZE_INTERVAL, self.logger.timer_interval.as_secs());

		loop {
			if !self.block_receiver.is_empty() {
				while let Ok(block) = self.block_receiver.try_recv() {
					let now = Instant::now();
					self.inner.add_block(block);
					self.logger.add_block(now.elapsed());
				}
			}

			if !self.search_receiver.is_empty() {
				while let Ok(details) = self.search_receiver.try_recv() {
					let now = Instant::now();
					self.send_transaction_state(details);
					self.logger.add_rpc_call(now.elapsed());
				}
			}

			if self.logger.log_stats() {
				self.inner.log();
			}

			if self.timer.elapsed() >= self.timer_interval {
				let (duration, _) = profile!(self.inner.resize());
				self.logger.log_resize(duration);

				self.timer = Instant::now();
			}

			tokio::time::sleep(Duration::from_millis(
				super::constants::DATABASE_POOL_INTERVAL,
			))
			.await;
		}
	}

	fn send_transaction_state(&self, details: TxStateChannel) {
		let (tx_hash, is_finalized, oneshot) = details;

		let mut result: Vec<state_types::RPCResult> =
			self.inner.find_transaction_state(&tx_hash, is_finalized);
		result.sort_by(|x, y| y.block_height.cmp(&x.block_height));

		_ = oneshot.send(result);
	}
}

pub trait DatabaseLike {
	fn new(config: Config) -> Self;
	fn add_block(&mut self, block: BlockDetails);
	fn find_transaction_state(
		&self,
		tx_hash: &H256,
		is_finalized: bool,
	) -> Vec<state_types::RPCResult>;
	fn resize(&mut self);
	fn config(&self) -> &Config;
	fn variant(&self) -> &str;
	fn log(&self);
}

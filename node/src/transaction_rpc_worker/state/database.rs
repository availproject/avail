use std::{
	sync::Arc,
	time::{Duration, Instant},
};

use jsonrpsee::{tokio, tokio::sync::mpsc::Receiver};
use sc_telemetry::log;
use sp_core::H256;
use tokio::sync::Notify;
use transaction_rpc::transaction_overview;

use super::{constants::DATABASE_RESIZE_INTERVAL, database_logger::DatabaseLogging, BlockDetails};
use crate::transaction_rpc_worker::macros::profile;
pub struct Config {
	pub max_search_results: usize,
	pub max_stored_block_count: usize,
}

pub struct Database<T: DatabaseLike> {
	block_receiver: Receiver<BlockDetails>,
	search_receiver: transaction_overview::Receiver,
	logger: DatabaseLogging,
	inner: T,
	timer: Instant,
	timer_interval: Duration,
	notifier: Arc<Notify>,
}
impl<T: DatabaseLike> Database<T> {
	pub fn new(
		block_receiver: Receiver<BlockDetails>,
		search_receiver: transaction_overview::Receiver,
		max_search_results: usize,
		max_stored_block_count: usize,
		logging_interval: u64,
		notifier: Arc<Notify>,
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
			notifier,
		}
	}

	pub async fn run(mut self) {
		let config = self.inner.config();
		let variant = self.inner.variant();
		log::info!("👾 Transaction State Running with following parameters: Max Search Result: {}, Max Stored Block Count: {}, Variant: {}, Resize Interval: {}s, Logging Interval: {}s", config.max_search_results, config.max_stored_block_count, variant, DATABASE_RESIZE_INTERVAL, self.logger.timer_interval.as_secs());

		loop {
			if !self.block_receiver.is_empty() {
				while let Ok(block) = self.block_receiver.try_recv() {
					let (duration, _) = profile!(self.inner.add_block(block));
					self.logger.add_block(duration);
				}
			}

			if !self.search_receiver.is_empty() {
				while let Ok(details) = self.search_receiver.try_recv() {
					let (duration, _) = profile!(self.send_transaction_state(details));
					self.logger.add_rpc_call(duration);
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

			self.notifier.notified().await;
		}
	}

	fn send_transaction_state(&self, details: transaction_overview::Channel) {
		let (tx_hash, is_finalized, oneshot) = details;

		let mut result: Vec<transaction_overview::RPCResult> =
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
	) -> Vec<transaction_overview::RPCResult>;
	fn resize(&mut self);
	fn config(&self) -> &Config;
	fn variant(&self) -> &str;
	fn log(&self);
}

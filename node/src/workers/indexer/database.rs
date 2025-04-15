use super::{
	constants::DATABASE_RESIZE_INTERVAL, database_logger::DatabaseLogging, database_map, Deps,
	Receiver,
};
use crate::workers::{macros::profile, Timer};
use jsonrpsee::tokio;
use sc_telemetry::log;
use std::sync::Arc;
use tokio::sync::Notify;
use transaction_rpc::transaction_overview;

pub struct Config {
	pub max_search_results: usize,
	pub max_stored_block_count: usize,
}

pub struct Database {
	block_receiver: Receiver,
	rpc_receiver: transaction_overview::Receiver,
	logger: DatabaseLogging,
	inner: database_map::Database,
	timer: Timer,
	notifier: Arc<Notify>,
}
impl Database {
	pub fn new(deps: Deps) -> Self {
		let config = Config {
			max_search_results: deps.cli.max_search_results,
			max_stored_block_count: deps.cli.max_stored_block_count,
		};

		Self {
			block_receiver: deps.block_receiver,
			rpc_receiver: deps.transaction_receiver,
			logger: DatabaseLogging::new(deps.cli.logging_interval),
			inner: database_map::Database::new(config),
			timer: Timer::new(DATABASE_RESIZE_INTERVAL),
			notifier: deps.notifier,
		}
	}

	pub async fn run(mut self) {
		let config = self.inner.config();
		let variant = self.inner.variant();
		log::info!("👾 Transaction State Running with following parameters: Max Search Result: {}, Max Stored Block Count: {}, Variant: {}, Resize Interval: {}s, Logging Interval: {}s", config.max_search_results, config.max_stored_block_count, variant, DATABASE_RESIZE_INTERVAL, self.logger.timer.duration());

		loop {
			let (duration, _) = profile!(self.handle_queues());
			self.logger.increment_total_time(duration);

			self.resize();
			self.logger.log();

			self.notifier.notified().await;
		}
	}

	fn resize(&mut self) {
		if !self.timer.expired() {
			return;
		}

		let (duration, _) = profile!(self.inner.resize());
		self.logger.increment_resize_time(duration);
		self.timer.restart();
	}

	fn handle_queues(&mut self) {
		if !self.block_receiver.is_empty() {
			while let Ok(block) = self.block_receiver.try_recv() {
				self.inner.add_block(block);
				self.logger.increment_block();
			}
		}

		if !self.rpc_receiver.is_empty() {
			while let Ok(details) = self.rpc_receiver.try_recv() {
				self.transaction_overview_response(details);
				self.logger.increment_rpc_call();
			}
		}
	}

	fn transaction_overview_response(&self, details: transaction_overview::Channel) {
		let (tx_hash, is_finalized, oneshot) = details;

		let mut result: Vec<transaction_overview::Response> =
			self.inner.find_overview(&tx_hash, is_finalized);
		result.sort_by(|x, y| y.block_height.cmp(&x.block_height));

		_ = oneshot.send(result);
	}
}

use super::{constants::DATABASE_RESIZE_INTERVAL, database_map, CliDeps, Deps, Receiver};
use crate::workers::{macros::profile, Timer};
use jsonrpsee::tokio;
use sc_telemetry::log;
use std::{sync::Arc, time::Duration};
use tokio::sync::Notify;
use transaction_rpc::transaction_overview;

pub struct Database {
	block_receiver: Receiver,
	rpc_receiver: transaction_overview::Receiver,
	logger: Logger,
	inner: database_map::Database,
	timer: Timer,
	notifier: Arc<Notify>,
	cli: CliDeps,
}
impl Database {
	pub fn new(deps: Deps) -> Self {
		Self {
			block_receiver: deps.block_receiver,
			rpc_receiver: deps.transaction_receiver,
			logger: Logger::new(deps.cli.logging_interval),
			inner: database_map::Database::new(deps.cli.clone()),
			timer: Timer::new(DATABASE_RESIZE_INTERVAL),
			notifier: deps.notifier,
			cli: deps.cli,
		}
	}

	pub async fn run(mut self) {
		let message = std::format!("Running with following parameters: Max Search Result: {}, Max Stored Block Count: {}, Resize Interval: {}s, Logging Interval: {}s", self.cli.max_search_results, self.cli.max_stored_block_count, DATABASE_RESIZE_INTERVAL, self.logger.timer.duration());
		self.logger.log(message);

		loop {
			let (duration, _) = profile!(self.handle_queues());
			self.logger.increment_total_time(duration);

			self.resize();
			self.logger.log_stats(&self.inner);

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

struct Logger {
	blocks_count: u32,
	rpc_calls_count: u32,
	resize_count: u32,
	total_time: Duration,
	resize_time: Duration,
	pub timer: Timer,
}

impl Logger {
	pub fn new(logging_interval: u64) -> Self {
		Self {
			blocks_count: 0,
			rpc_calls_count: 0,
			resize_count: 0,
			total_time: Duration::default(),
			resize_time: Duration::default(),
			timer: Timer::new(logging_interval),
		}
	}

	pub fn increment_block(&mut self) {
		self.blocks_count += 1;
	}

	pub fn increment_rpc_call(&mut self) {
		self.rpc_calls_count += 1;
	}

	pub fn increment_total_time(&mut self, value: Duration) {
		self.total_time += value;
	}

	pub fn increment_resize_time(&mut self, value: Duration) {
		self.resize_time += value;
		self.resize_count += 1;
	}

	pub fn log_stats(&mut self, db: &database_map::Database) {
		if !self.timer.expired() {
			return;
		}

		let message = std::format!(
			"Total Duration: {} ms, Blocks Received Count: {}, RPC Calls Count: {}, Resize Total Duration: {} ms, Resize Count: {}",
			self.total_time.as_millis(),
			self.blocks_count,
			self.rpc_calls_count,
			self.resize_time.as_millis(),
			self.resize_count,
		);

		self.log(message);
		self.log(db.current_state());

		self.blocks_count = 0;
		self.rpc_calls_count = 0;
		self.resize_count = 0;
		self.resize_time = Duration::default();
		self.total_time = Duration::default();

		self.timer.restart();
	}

	pub fn log(&self, message: String) {
		log::info!("👾 Database: {}", message);
	}
}

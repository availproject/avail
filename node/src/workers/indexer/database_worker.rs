use super::{
	cache::{Cache, Cacheable, SharedCache},
	database_map, CliDeps, Deps, DATABASE_SIZE_BUFFER,
};
use crate::workers::{macros::profile, NodeContext, Timer, TransactionEvents, TxIdentifier};
use block_rpc::transaction_overview;
use frame_system_rpc_runtime_api::SystemFetchEventsParams;
use jsonrpsee::tokio;
use sc_telemetry::log;
use std::{
	sync::{Arc, RwLock},
	time::Duration,
};
use tokio::sync::Notify;

pub struct DatabaseWorker {
	ctx: NodeContext,
	block_receiver: super::Receiver,
	rpc_receiver: transaction_overview::Receiver,
	logger: Logger,
	inner: database_map::Database,
	notifier: Arc<Notify>,
	cli: CliDeps,
	cache: SharedCache,
}

impl DatabaseWorker {
	pub fn new(ctx: NodeContext, deps: Deps) -> Self {
		let cache = Arc::new(RwLock::new(Cache::new(deps.cli.event_cache_size)));

		Self {
			ctx,
			block_receiver: deps.block_receiver,
			rpc_receiver: deps.transaction_receiver,
			logger: Logger::new(deps.cli.logging_interval),
			inner: database_map::Database::new(&deps.cli),
			notifier: deps.notifier,
			cli: deps.cli,
			cache,
		}
	}

	pub async fn run(mut self) {
		let message = std::format!("Running with following parameters: Max Search Result: {}, Max Stored Block Count: {}, Logging Interval: {}s", self.cli.result_length, self.cli.block_pruning, self.logger.timer.duration());
		self.logger.log(message);

		loop {
			let (duration, _) = profile!(self.handle_queues().await);
			self.logger.increment_handle_queues_time(duration);

			self.resize();
			self.logger.log_stats(&self.inner);

			self.notifier.notified().await;
		}
	}

	fn resize(&mut self) {
		if (self.cli.block_pruning + DATABASE_SIZE_BUFFER) > self.inner.block_count() {
			return;
		}

		let (duration, _) = profile!(self.inner.resize(self.cli.block_pruning));
		self.logger.increment_resize_time(duration);
	}

	async fn handle_queues(&mut self) {
		while let Ok(block) = self.block_receiver.try_recv() {
			self.inner.add_block(block);
			self.logger.increment_block();
		}

		while let Ok(details) = self.rpc_receiver.try_recv() {
			self.transaction_overview_response(details).await;
			self.logger.increment_rpc_call();
		}
	}

	async fn transaction_overview_response(&mut self, details: transaction_overview::Channel) {
		let (params, oneshot) = details;

		let mut response = self.inner.find_overview(params.tx_hash, params.finalized);
		response.truncate(self.cli.result_length);
		response.sort_by(|x, y| y.block_id.height.cmp(&x.block_id.height));

		if params.fetch_events {
			let enable_decoding = params.enable_event_decoding;
			for res in &mut response {
				let id = TxIdentifier::from((res.block_id.hash, res.tx_index));
				res.events = self.get_and_transform_events(id, enable_decoding).await
			}
		}

		_ = oneshot.send(response);
	}

	async fn get_and_transform_events(
		&mut self,
		id: TxIdentifier,
		enable_decoding: bool,
	) -> Option<block_rpc::common::events::Events> {
		let event_entry = self.tx_events(id).await?;

		let mut tx_events = Vec::with_capacity(event_entry.events.len());
		for event in event_entry.events {
			tx_events.push(event.to_tx_rpc_event(enable_decoding));
		}

		Some(tx_events)
	}

	async fn tx_events(&mut self, id: TxIdentifier) -> Option<TransactionEvents> {
		if let Some(cached) = self.cache.read_cached_events(&id) {
			return Some(cached);
		}

		let params = SystemFetchEventsParams {
			enable_decoding: Some(true),
			filter_tx_indices: Some(vec![id.tx_index]),
			..Default::default()
		};

		let all_tx_events = self.ctx.fetch_events(id.block_hash, params).await?;
		let events = all_tx_events.tx_events(id.tx_index).cloned()?;
		self.cache.write_cached_events(id, &events);

		Some(events)
	}
}

struct Logger {
	blocks_count: u32,
	rpc_calls_count: u32,
	resize_count: u32,
	handle_queues_time: Duration,
	resize_time: Duration,
	pub timer: Timer,
}

impl Logger {
	pub fn new(logging_interval: u64) -> Self {
		Self {
			blocks_count: 0,
			rpc_calls_count: 0,
			resize_count: 0,
			handle_queues_time: Duration::default(),
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

	pub fn increment_handle_queues_time(&mut self, value: Duration) {
		self.handle_queues_time += value;
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
			self.handle_queues_time.as_millis(),
			self.blocks_count,
			self.rpc_calls_count,
			self.resize_time.as_millis(),
			self.resize_count,
		);

		self.log(message);
		self.log(db.current_state());

		self.reset();
	}

	pub fn log(&self, message: String) {
		log::info!("👾 Database: {}", message);
	}

	fn reset(&mut self) {
		self.blocks_count = 0;
		self.rpc_calls_count = 0;
		self.resize_count = 0;
		self.resize_time = Duration::default();
		self.handle_queues_time = Duration::default();
		self.timer.restart();
	}
}

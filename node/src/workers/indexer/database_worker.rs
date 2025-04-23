use super::{
	cache::{Cache, Cacheable, SharedCache},
	constants::DATABASE_RESIZE_INTERVAL,
	database_map, CliDeps, Deps,
};
use crate::workers::{
	cache::CachedEntryEvents,
	common::{self, Timer, UniqueTxId},
	macros::profile,
};
use frame_system_rpc_runtime_api::SystemFetchEventsParams;
use jsonrpsee::tokio;
use sc_service::RpcHandlers;
use sc_telemetry::log;
use std::{
	sync::{Arc, RwLock},
	time::Duration,
};
use tokio::sync::Notify;
use transaction_rpc::transaction_overview;

pub struct DatabaseWorker {
	block_receiver: super::Receiver,
	rpc_receiver: transaction_overview::Receiver,
	handlers: RpcHandlers,
	logger: Logger,
	inner: database_map::Database,
	timer: Timer,
	notifier: Arc<Notify>,
	cli: CliDeps,
	cache: SharedCache,
}

impl DatabaseWorker {
	pub fn new(deps: Deps, handlers: RpcHandlers) -> Self {
		let cache = Arc::new(RwLock::new(Cache::new(deps.cli.event_cache_size)));

		Self {
			block_receiver: deps.block_receiver,
			rpc_receiver: deps.transaction_receiver,
			handlers,
			logger: Logger::new(deps.cli.logging_interval),
			inner: database_map::Database::new(deps.cli.clone()),
			timer: Timer::new(DATABASE_RESIZE_INTERVAL),
			notifier: deps.notifier,
			cli: deps.cli,
			cache,
		}
	}

	pub async fn run(mut self) {
		let message = std::format!("Running with following parameters: Max Search Result: {}, Max Stored Block Count: {}, Resize Interval: {}s, Logging Interval: {}s", self.cli.result_length, self.cli.block_pruning, DATABASE_RESIZE_INTERVAL, self.logger.timer.duration());
		self.logger.log(message);

		loop {
			let (duration, _) = profile!(self.handle_queues().await);
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

	async fn handle_queues(&mut self) {
		if !self.block_receiver.is_empty() {
			while let Ok(block) = self.block_receiver.try_recv() {
				self.inner.add_block(block);
				self.logger.increment_block();
			}
		}

		if !self.rpc_receiver.is_empty() {
			while let Ok(details) = self.rpc_receiver.try_recv() {
				self.transaction_overview_response(details).await;
				self.logger.increment_rpc_call();
			}
		}
	}

	async fn transaction_overview_response(&mut self, details: transaction_overview::Channel) {
		let (params, oneshot) = details;

		let mut response: Vec<transaction_overview::Response> =
			self.inner.find_overview(params.tx_hash, params.finalized);

		response.sort_by(|x, y| y.block_height.cmp(&x.block_height));

		if params.fetch_events {
			use transaction_rpc::common::events::Event;

			let enable_decoding = params.enable_event_decoding;
			for res in &mut response {
				let id = UniqueTxId::from((res.block_hash, res.tx_index));
				let Ok(events) = self.tx_events(id).await else {
					break;
				};

				let events = events
					.events
					.iter()
					.map(|ev| {
						let decoded = enable_decoding
							.then(|| ev.decoded.clone())
							.or(None)
							.flatten();
						Event::new(ev.index, ev.emitted_index, decoded)
					})
					.collect();

				res.events = Some(events);
			}
		}

		_ = oneshot.send(response);
	}

	async fn tx_events(&mut self, id: UniqueTxId) -> Result<CachedEntryEvents, ()> {
		if let Some(cached) = self.cache.read_cached_events(&id) {
			return Ok(cached);
		}

		let params = SystemFetchEventsParams {
			enable_decoding: Some(true),
			filter_tx_indices: Some(vec![id.tx_index]),
			..Default::default()
		};

		let Some(events) = common::fetch_events(&self.handlers, id.block_hash, params).await else {
			return Err(());
		};

		let Some(events) = events.tx_events(id.tx_index).cloned() else {
			return Err(());
		};

		self.cache.write_cached_events(id, events.clone());

		Ok(events)
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

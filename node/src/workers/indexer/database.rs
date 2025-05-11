use super::{
	cache::{Cache, Cacheable, SharedCache},
	BlockDetails, CliDeps, Deps,
};
use crate::workers::{NodeContext, TransactionEvents, TransactionId};
use block_rpc::transaction_overview;
use frame_system_rpc_runtime_api::SystemFetchEventsParams;
use jsonrpsee::tokio;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use sp_core::H256;
use std::{
	collections::VecDeque,
	sync::{Arc, RwLock},
};
use tokio::sync::Notify;

pub struct DatabaseWorker {
	ctx: NodeContext,
	block_receiver: super::Receiver,
	rpc_receiver: transaction_overview::Receiver,
	inner: Database,
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
			inner: Database::new(&deps.cli),
			notifier: deps.notifier,
			cli: deps.cli,
			cache,
		}
	}

	pub async fn run(mut self) {
		loop {
			self.handle_queues().await;
			self.notifier.notified().await;
		}
	}

	async fn handle_queues(&mut self) {
		while let Ok(block) = self.block_receiver.try_recv() {
			self.inner.add_block(block);
		}

		while let Ok(details) = self.rpc_receiver.try_recv() {
			self.transaction_overview_response(details).await;
		}
	}

	async fn transaction_overview_response(&mut self, details: transaction_overview::Channel) {
		let (params, oneshot) = details;

		let mut response = self
			.inner
			.find_transaction_block_height(params.tx_hash, params.use_best_block);
		response.truncate(self.cli.result_length);
		response.sort_by(|x, y| y.block_id.height.cmp(&x.block_id.height));

		if params.fetch_events {
			let enable_decoding = params.enable_event_decoding;
			for res in &mut response {
				let id = TransactionId::from((res.block_id.hash, res.tx_location.index));
				res.events = self.get_and_transform_events(id, enable_decoding).await
			}
		}

		_ = oneshot.send(response);
	}

	async fn get_and_transform_events(
		&mut self,
		id: TransactionId,
		enable_decoding: bool,
	) -> Option<block_rpc::common::events::Events> {
		let event_entry = self.tx_events(id).await?;

		let mut tx_events = Vec::with_capacity(event_entry.events.len());
		for event in event_entry.events {
			tx_events.push(event.to_tx_rpc_event(enable_decoding));
		}

		Some(tx_events)
	}

	async fn tx_events(&mut self, id: TransactionId) -> Option<TransactionEvents> {
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

pub struct Database {
	best_block_tx: StoredBlocks,
	finalized_tx: StoredBlocks,
	result_length: usize,
}

impl Database {
	pub fn new(cli: &CliDeps) -> Self {
		Self {
			best_block_tx: StoredBlocks::default(),
			finalized_tx: StoredBlocks::default(),
			result_length: cli.result_length,
		}
	}

	pub fn add_block(&mut self, new_block: BlockDetails) {
		let block_height = new_block.block_height;

		// Clean up Included Tx
		if new_block.finalized {
			self.best_block_tx.filter_up_to_and_including(block_height);
		}

		// Add new transaction
		let map = match new_block.finalized {
			true => &mut self.finalized_tx,
			false => &mut self.best_block_tx,
		};

		let tx_hashes = new_block
			.transactions
			.iter()
			.map(|x| x.tx_location.hash)
			.collect();
		map.add_transactions(block_height, tx_hashes);
		while self.finalized_tx.list.len() >= self.result_length {
			self.finalized_tx.list.pop_back();
		}
	}

	pub fn find_transaction_block_height(&self, tx_hash: H256, use_best_block: bool) -> Vec<u32> {
		let mut result: Vec<u32> = Vec::new();
		if use_best_block {
			result.extend(self.best_block_tx.find_entries(tx_hash));
		}
		result.extend(self.finalized_tx.find_entries(tx_hash));

		result
	}
}

#[derive(Debug, Clone, Default)]
struct StoredBlocks {
	list: VecDeque<(u32, Vec<H256>)>,
}

impl StoredBlocks {
	fn find_entries(&self, tx_hash: H256) -> Vec<u32> {
		let mut result: Vec<u32> = Vec::new();
		if self.list.len() > 100_000 {
			let fliter = self.list.par_iter().filter(|x| x.1.contains(&tx_hash));
			let block_heights: Vec<u32> = fliter.map(|x| x.0).collect();
			result.extend(block_heights);
		} else {
			let filter = self.list.iter().filter(|x| x.1.contains(&tx_hash));
			let block_heights: Vec<u32> = filter.map(|x| x.0).collect();
			result.extend(block_heights);
		}

		result
	}

	fn add_transactions(&mut self, block_height: u32, tx_hashes: Vec<H256>) {
		if let Some(first) = self.list.front_mut() {
			if block_height == first.0 {
				first.1 = tx_hashes;
				return;
			}

			if block_height > first.0 {
				self.list.push_front((block_height, tx_hashes));
				return;
			}
		}

		if let Some(last) = self.list.back() {
			if last.0 > block_height {
				self.list.push_back((block_height, tx_hashes));
				return;
			}
		}

		// This push back and sort duo is expensive. I am not expecting this two lines of code to be ever triggered.
		self.list.push_back((block_height, tx_hashes));
		self.list.make_contiguous().sort_by(|x, y| y.0.cmp(&x.0));
	}

	fn filter_up_to_and_including(&mut self, block_height: u32) {
		while let Some(value) = self.list.back() {
			if value.0 > block_height {
				return;
			}

			self.list.pop_back();
		}
	}
}

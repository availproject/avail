use super::{
	cache::{Cache, Cacheable, SharedCache},
	Deps,
};
use crate::workers::{
	self, macros::profile, AllTransactionEvents, NodeContext, Timer, TransactionId,
};
use avail_core::OpaqueExtrinsic;
use block_rpc::{
	block_data, block_overview,
	common::{BlockState, Event, HashIndex, TransactionLocation},
	BlockIdentifier,
};
use codec::Encode;
use da_runtime::UncheckedExtrinsic;
use frame_system_rpc_runtime_api::SystemFetchEventsParams;
use jsonrpsee::tokio::sync::Notify;
use rayon::prelude::*;
use sc_telemetry::log;
use sp_core::{Blake2Hasher, Hasher, H256};
use sp_runtime::MultiAddress;
use std::sync::{Arc, RwLock};
use std::time::Duration;

pub struct Worker {
	ctx: NodeContext,
	overview_receiver: block_overview::Receiver,
	data_receiver: block_data::Receiver,
	notifier: Arc<Notify>,
	cache: SharedCache,
}

impl Worker {
	pub fn new(ctx: NodeContext, deps: Deps) -> Self {
		let cache = Arc::new(RwLock::new(Cache::new(&deps)));

		Self {
			ctx,
			overview_receiver: deps.overview_receiver,
			data_receiver: deps.data_receiver,
			notifier: deps.notifier,
			cache,
		}
	}

	pub async fn run(mut self) {
		let mut logger = Logger::new(6);
		loop {
			while let Ok((params, oneshot)) = self.data_receiver.try_recv() {
				let (duration, response) = profile!(self.data_task(params).await);
				logger.increment_data_task_time(duration);
				_ = oneshot.send(response);
			}

			while let Ok((params, oneshot)) = self.overview_receiver.try_recv() {
				let (duration, response) = profile!(self.overview_task(params).await);
				logger.increment_data_overview_time(duration);
				_ = oneshot.send(response);
			}

			logger.log_stats();
			self.notifier.notified().await;
		}
	}

	async fn data_task(
		&mut self,
		params: block_data::Params,
	) -> Result<block_data::Response, String> {
		let block_id = self.block_identifier(params.block_id).await?;
		let block_state = self.block_state(block_id)?;

		let calls = self.data_task_calls(block_id, &params).await?;
		let events = self.data_task_events(block_id, &params).await?;

		let response = block_data::Response::new(block_id, block_state, calls, events);
		Ok(response)
	}

	async fn data_task_calls(
		&mut self,
		block_id: BlockIdentifier,
		params: &block_data::Params,
	) -> Result<Option<Vec<block_data::CallData>>, String> {
		if !params.fetch_calls {
			return Ok(None);
		}
		let Some(block_body) = self.ctx.block_body_hash(block_id.hash) else {
			let message = std::format!("Failed to fetch block body for hash: {:?}", block_id.hash);
			return Err(message);
		};

		let opaques = block_body.par_iter().enumerate();
		let calls: Vec<block_data::CallData> = opaques
			.filter_map(|(i, opaq)| {
				let tx_id = TransactionId::from((block_id.hash, i as u32));
				iter_data_opaque(tx_id, opaq, self.cache.clone(), params)
			})
			.collect();
		Ok(Some(calls))
	}

	async fn data_task_events(
		&mut self,
		block_id: BlockIdentifier,
		params: &block_data::Params,
	) -> Result<Option<Vec<block_data::EventData>>, String> {
		use block_data::EventData;
		if !params.fetch_events {
			return Ok(None);
		}
		let block_events = self.block_events(block_id.hash).await?;

		let mut events = Vec::new();
		for cached_event in &block_events.0 {
			let phase = match cached_event.phase {
				frame_system::Phase::ApplyExtrinsic(x) => block_data::Phase::ApplyExtrinsic(x),
				frame_system::Phase::Finalization => block_data::Phase::Finalization,
				frame_system::Phase::Initialization => block_data::Phase::Initialization,
			};
			for ev in &cached_event.events {
				events.push(EventData::new(ev.emitted_index, phase, ev.encoded.clone()));
			}
		}
		Ok(Some(events))
	}

	async fn overview_task(
		&mut self,
		params: block_overview::Params,
	) -> Result<block_overview::Response, String> {
		let block_id = self.block_identifier(params.block_id).await?;
		let block_state = self.block_state(block_id)?;

		let events = if params.extension.fetch_events {
			Some(self.block_events(block_id.hash).await?)
		} else {
			None
		};

		let transactions = self
			.overview_task_transactions(block_id, &events, &params)
			.await?;
		let consensus_events = self.overview_task_consensus_events(&events, &params);

		let result =
			block_overview::Response::new(block_id, block_state, transactions, consensus_events);
		Ok(result)
	}

	async fn overview_task_transactions(
		&mut self,
		block_id: BlockIdentifier,
		events: &Option<Arc<AllTransactionEvents>>,
		params: &block_overview::Params,
	) -> Result<Vec<block_overview::TransactionData>, String> {
		let Some(block_body) = self.ctx.block_body_hash(block_id.hash) else {
			let message = std::format!("Failed to fetch block body for hash: {:?}", block_id.hash);
			return Err(message);
		};

		let transactions: Vec<block_overview::TransactionData> = block_body
			.par_iter()
			.enumerate()
			.filter_map(|(i, opaq)| {
				let tx_id = TransactionId::from((block_id.hash, i as u32));
				iter_overview_opaque(tx_id, opaq, self.cache.clone(), params, events)
			})
			.collect();

		Ok(transactions)
	}

	fn overview_task_consensus_events(
		&mut self,
		events: &Option<Arc<AllTransactionEvents>>,
		params: &block_overview::Params,
	) -> Option<block_overview::ConsensusEvents> {
		if params.extension.enable_consensus_event {
			return None;
		}

		if let Some(events) = &events {
			let enable_decoding = params.extension.enable_event_decoding;
			Some(read_consensus_events(enable_decoding, events))
		} else {
			None
		}
	}

	async fn block_identifier(&self, block_id: HashIndex) -> Result<BlockIdentifier, String> {
		match block_id {
			HashIndex::Hash(hash) => {
				let height = self.ctx.to_number(hash);
				let Some(height) = height.ok().flatten() else {
					return Err(std::format!(
						"No block height found for block hash: {:?}",
						hash
					));
				};
				Ok(BlockIdentifier::from((hash, height)))
			},
			HashIndex::Index(height) => {
				let hash = self.ctx.to_hash(height);
				let Some(hash) = hash.ok().flatten() else {
					return Err(std::format!(
						"No block hash found for block height: {}",
						height
					));
				};
				Ok(BlockIdentifier::from((hash, height)))
			},
		}
	}

	async fn block_events(
		&mut self,
		block_hash: H256,
	) -> Result<Arc<AllTransactionEvents>, String> {
		if let Some(cached) = self.cache.read_cached_events(&block_hash) {
			return Ok(cached);
		}

		let params = SystemFetchEventsParams {
			enable_decoding: Some(true),
			enable_encoding: Some(true),
			..Default::default()
		};

		let Some(events) = self.ctx.fetch_events(block_hash, params).await else {
			return Err("Failed to fetch events.".into());
		};

		let events = Arc::new(events);
		self.cache.write_cached_events(block_hash, &events);

		Ok(events)
	}

	fn block_state(&self, block_id: BlockIdentifier) -> Result<BlockState, String> {
		let chain_info = self.ctx.client.chain_info();
		if block_id.height > chain_info.finalized_number {
			return Ok(BlockState::Included);
		}

		let finalized_hash = self.ctx.to_hash(block_id.height);
		let finalized_hash = finalized_hash
			.map_err(|e| e.to_string())?
			.ok_or(String::from("Failed to convert block height to block hash"))?;

		if finalized_hash == block_id.hash {
			return Ok(BlockState::Finalized);
		}

		Ok(BlockState::Discarded)
	}
}

fn read_consensus_events(
	enable_decoding: bool,
	events: &Arc<AllTransactionEvents>,
) -> block_overview::ConsensusEvents {
	use block_overview::{ConsensusEvent, ConsensusEventPhase};

	let cached = events.consensus_events();

	let mut consensus_events = Vec::new();
	for cache in cached {
		let phase = match cache.phase {
			frame_system::Phase::Finalization => ConsensusEventPhase::Finalization,
			frame_system::Phase::Initialization => ConsensusEventPhase::Initialization,
			_ => continue,
		};

		for event in cache.events {
			let mut ev = ConsensusEvent {
				phase,
				emitted_index: event.emitted_index,
				decoded: None,
			};

			if enable_decoding {
				ev.decoded = event.decoded.clone()
			}

			consensus_events.push(ev);
		}
	}

	consensus_events
}

fn read_signature(ext: &UncheckedExtrinsic) -> Option<block_overview::TransactionSignature> {
	let Some(sig) = &ext.signature else {
		return None;
	};

	let ss58_address = if let MultiAddress::Id(id) = &sig.0 {
		Some(std::format!("{}", id))
	} else {
		None
	};
	let nonce = sig.2 .5 .0;
	let app_id = sig.2 .8 .0 .0;
	let mortality = match sig.2 .4 .0 {
		sp_runtime::generic::Era::Immortal => None,
		sp_runtime::generic::Era::Mortal(x, y) => Some((x, y)),
	};

	let value = block_overview::TransactionSignature {
		ss58_address,
		nonce,
		app_id,
		mortality,
	};
	Some(value)
}

fn iter_overview_opaque(
	tx_id: TransactionId,
	opaq: &OpaqueExtrinsic,
	cache: SharedCache,
	params: &block_overview::Params,
	events: &Option<Arc<AllTransactionEvents>>,
) -> Option<block_overview::TransactionData> {
	let filter = &params.filter;
	filter.transaction.filter_in_tx_index(tx_id.tx_index)?;

	let tx_hash = cache.read_cached_tx_hash(&tx_id);
	if let Some(tx_hash) = tx_hash {
		filter.transaction.filter_in_tx_hash(tx_hash)?;
	}

	let ext = UncheckedExtrinsic::decode_no_vec_prefix(&mut opaq.0.as_slice()).ok()?;
	let dispatch_index = workers::common::read_pallet_call_index(&ext)?;

	filter.transaction.filter_in_pallet(dispatch_index.0)?;
	filter.transaction.filter_in_pallet_call(dispatch_index)?;

	let tx_hash = tx_hash.unwrap_or_else(|| {
		let tx_hash = Blake2Hasher::hash(&ext.encode());
		cache.write_cached_tx_hash(tx_id, &tx_hash);
		tx_hash
	});
	filter.transaction.filter_in_tx_hash(tx_hash)?;

	let signature = read_signature(&ext);
	filter
		.signature
		.filter_in_app_id(signature.as_ref().map(|x| x.app_id))?;
	filter
		.signature
		.filter_in_nonce(signature.as_ref().map(|x| x.nonce))?;
	filter
		.signature
		.filter_in_ss58_address(signature.as_ref().and_then(|x| x.ss58_address.clone()))?;

	let events = iter_overview_opaque_events(tx_id, params, events)?;

	let value = block_overview::TransactionData {
		location: TransactionLocation::from((tx_hash, tx_id.tx_index)),
		dispatch_index,
		signature,
		decoded: None,
		events,
	};

	Some(value)
}

fn iter_overview_opaque_events(
	tx_id: TransactionId,
	params: &block_overview::Params,
	events: &Option<Arc<AllTransactionEvents>>,
) -> Option<Option<block_rpc::common::Events>> {
	use block_overview::TransactionFilterOptions;
	let enable_decoding = params.extension.enable_event_decoding;
	let filter = &params.filter;

	let Some(cached_events) = events else {
		return (!filter.transaction.is_has_events()).then_some(None);
	};

	let Some(events_entry) = cached_events.tx_events(tx_id.tx_index) else {
		return (!filter.transaction.is_has_events()).then_some(None);
	};

	let tx_events: Vec<Event> = events_entry
		.events
		.iter()
		.map(|x| {
			let decoded = enable_decoding.then(|| x.decoded.clone()).flatten();
			Event::new(x.index, x.emitted_index, decoded)
		})
		.collect();

	if let TransactionFilterOptions::HasEvent(expected_events) = &filter.transaction {
		for exp_ev in expected_events {
			if !tx_events.iter().any(|x| x.emitted_index == *exp_ev) {
				return None;
			}
		}
	}
	Some(Some(tx_events))
}

fn iter_data_opaque(
	tx_id: TransactionId,
	opaq: &OpaqueExtrinsic,
	cache: SharedCache,
	params: &block_data::Params,
) -> Option<block_data::CallData> {
	let filter = &params.call_filter;

	filter.transaction.filter_in_tx_index(tx_id.tx_index)?;

	let tx_hash = cache.read_cached_tx_hash(&tx_id);
	if let Some(tx_hash) = tx_hash {
		filter.transaction.filter_in_tx_hash(tx_hash)?
	}

	let ext = UncheckedExtrinsic::decode_no_vec_prefix(&mut opaq.0.as_slice()).ok()?;
	let dispatch_index = workers::common::read_pallet_call_index(&ext)?;

	filter.transaction.filter_in_pallet(dispatch_index.0)?;
	filter.transaction.filter_in_pallet_call(dispatch_index)?;

	let tx_hash = tx_hash.unwrap_or_else(|| {
		let tx_hash = Blake2Hasher::hash(&ext.encode());
		cache.write_cached_tx_hash(tx_id, &tx_hash);
		tx_hash
	});
	filter.transaction.filter_in_tx_hash(tx_hash)?;

	let signature = read_signature(&ext);
	filter
		.signature
		.filter_in_app_id(signature.as_ref().map(|x| x.app_id))?;
	filter
		.signature
		.filter_in_nonce(signature.as_ref().map(|x| x.nonce))?;
	filter
		.signature
		.filter_in_ss58_address(signature.as_ref().and_then(|x| x.ss58_address.clone()))?;

	let call = if let Some(call) = cache.read_cached_calls(&tx_id) {
		call
	} else {
		let call = opaque_to_json(opaq)?;
		cache.write_cached_calls(tx_id, &call);
		call
	};

	let location = TransactionLocation::from((tx_hash, tx_id.tx_index));
	let call_data = block_data::CallData::new(location, dispatch_index, call);
	Some(call_data)
}

fn opaque_to_json(value: &OpaqueExtrinsic) -> Option<String> {
	let mut call = serde_json::to_string(value).ok()?;
	if call.len() >= 2 {
		call.pop();
		call.remove(0);
	}

	Some(call)
}

struct Logger {
	data_task_time: Duration,
	data_task_count: u32,
	data_overview_time: Duration,
	data_overview_count: u32,
	pub timer: Timer,
}

impl Logger {
	pub fn new(logging_interval: u64) -> Self {
		Self {
			data_task_time: Duration::default(),
			data_task_count: 0,
			data_overview_time: Duration::default(),
			data_overview_count: 0,
			timer: Timer::new(logging_interval),
		}
	}

	pub fn increment_data_task_time(&mut self, duration: Duration) {
		self.data_task_time += duration;
		self.data_task_count += 1;
	}

	pub fn increment_data_overview_time(&mut self, duration: Duration) {
		self.data_overview_time += duration;
		self.data_overview_count += 1;
	}

	pub fn log(&self, message: String) {
		log::info!("👾 Database: {}", message);
	}

	pub fn log_stats(&mut self) {
		if !self.timer.expired() {
			return;
		}

		let message = std::format!(
			"Data Task Time: {} ms, Data Task Count: {}, Data Overview Time: {} ms, Data Overview Count: {}",
			self.data_task_time.as_millis(),
			self.data_task_count,
			self.data_overview_time.as_millis(),
			self.data_overview_count,
		);

		self.log(message);

		self.reset();
	}

	fn reset(&mut self) {
		self.data_overview_count = 0;
		self.data_task_count = 0;
		self.data_task_time = Duration::default();
		self.data_overview_time = Duration::default();
		self.timer.restart();
	}
}

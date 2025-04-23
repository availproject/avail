use std::sync::{Arc, RwLock};

use super::{
	cache::{Cache, Cacheable, SharedCache},
	logger::Logger,
	Deps,
};
use crate::{
	service::FullClient,
	workers::{
		cache::CachedEvents,
		common::{self, TxIdentifier},
		macros::profile,
	},
};
use avail_core::OpaqueExtrinsic;
use codec::Encode;
use da_runtime::UncheckedExtrinsic;
use frame_system_rpc_runtime_api::SystemFetchEventsParams;
use jsonrpsee::tokio::sync::Notify;
use rayon::prelude::*;
use sc_service::RpcHandlers;
use sc_telemetry::log;
use sp_core::{Blake2Hasher, Hasher, H256};
use sp_runtime::{generic::BlockId, traits::BlockIdTo, MultiAddress};
use transaction_rpc::{
	block_data::{self},
	block_overview,
	common::{BlockState, Event, HashIndex},
	BlockIdentifier,
};

pub struct Worker {
	client: Arc<FullClient>,
	rpc_handlers: RpcHandlers,
	overview_receiver: block_overview::Receiver,
	data_receiver: block_data::Receiver,
	notifier: Arc<Notify>,
	cache: SharedCache,
}

impl Worker {
	pub fn new(client: Arc<FullClient>, rpc_handlers: RpcHandlers, deps: Deps) -> Self {
		let cache = Arc::new(RwLock::new(Cache::new()));

		Self {
			client,
			rpc_handlers,
			overview_receiver: deps.overview_receiver,
			data_receiver: deps.data_receiver,
			notifier: deps.notifier,
			cache,
		}
	}

	pub async fn run(mut self) {
		log::info!("🐖 Transaction Data Running");

		loop {
			if !self.data_receiver.is_empty() {
				let (duration, _) = profile!({
					while let Ok((params, oneshot)) = self.data_receiver.try_recv() {
						let result = self.data_task(params).await;
						_ = oneshot.send(result);
					}
				});
				log::info!("🐖 Data Duration: {:.02?}", duration,);
			}

			if !self.overview_receiver.is_empty() {
				let (duration, _) = profile!({
					while let Ok((params, oneshot)) = self.overview_receiver.try_recv() {
						let result = self.overview_task(params).await;
						_ = oneshot.send(result);
					}
				});
				log::info!("🐖 Overview Duration: {:.02?}", duration,);
			}

			self.notifier.notified().await;
		}
	}

	async fn data_task(
		&mut self,
		params: block_data::RPCParams,
	) -> Result<block_data::Response, String> {
		use block_data::{CallData, EventData};

		let block_id = self.block_identifier(params.block_id).await?;
		let block_body = self.block_body(block_id.hash)?;
		let block_state = self.block_state(block_id)?;

		let mut maybe_calls: Option<Vec<CallData>> = None;
		if params.fetch_calls {
			let calls: Vec<block_data::CallData> = block_body
				.par_iter()
				.enumerate()
				.filter_map(|(i, opaq)| {
					let unique_id = TxIdentifier::from((block_id.hash, i as u32));
					iter_data_opaque(unique_id, opaq, self.cache.clone(), &params.call_filter)
				})
				.collect();
			maybe_calls = Some(calls);
		}

		let mut maybe_events: Option<Vec<block_data::EventData>> = None;
		if params.fetch_events {
			let block_events = self.block_events(block_id.hash).await?;

			let mut events = Vec::new();
			for cached_event in &block_events.0 {
				let phase = match cached_event.phase {
					frame_system::Phase::ApplyExtrinsic(x) => block_data::Phase::ApplyExtrinsic(x),
					frame_system::Phase::Finalization => block_data::Phase::Finalization,
					frame_system::Phase::Initialization => block_data::Phase::Initialization,
				};
				for ev in &cached_event.events {
					let data = EventData {
						emitted_index: ev.emitted_index,
						phase,
						data: ev.encoded.clone(),
					};
					events.push(data);
				}
			}
			maybe_events = Some(events)
		}

		let result = block_data::Response {
			block_id,
			block_state,
			calls: maybe_calls,
			events: maybe_events,
		};

		Ok(result)
	}

	async fn overview_task(
		&mut self,
		params: block_overview::Params,
	) -> Result<block_overview::Response, String> {
		let block_id = self.block_identifier(params.block_id).await?;
		let block_body = self.block_body(block_id.hash)?;
		let block_state = self.block_state(block_id)?;
		let enable_event_decoding = params.extension.enable_event_decoding;

		let events = if params.extension.fetch_events {
			Some(self.block_events(block_id.hash).await?)
		} else {
			None
		};

		let transactions: Vec<block_overview::TransactionData> = block_body
			.par_iter()
			.enumerate()
			.filter_map(|(i, opaq)| {
				iter_overview_opaque(
					TxIdentifier::from((block_id.hash, i as u32)),
					opaq,
					self.cache.clone(),
					&params.filter,
					enable_event_decoding,
					events.clone(),
				)
			})
			.collect();

		let mut consensus_events = None;
		if params.extension.enable_consensus_event {
			if let Some(events) = &events {
				consensus_events = Some(read_consensus_events(enable_event_decoding, events));
			}
		}

		let result = block_overview::Response {
			block_id,
			block_state,
			transactions,
			consensus_events,
		};

		Ok(result)
	}

	async fn block_identifier(&self, block_id: HashIndex) -> Result<BlockIdentifier, String> {
		match block_id {
			HashIndex::Hash(hash) => {
				let height = self.client.to_number(&BlockId::Hash(hash));
				let Some(height) = height.ok().flatten() else {
					return Err(std::format!(
						"No block height found for block hash: {:?}",
						hash
					));
				};
				Ok(BlockIdentifier::from((hash, height)))
			},
			HashIndex::Index(height) => {
				let hash = self.client.to_hash(&BlockId::Number(height));
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

	fn block_body(&self, block_hash: H256) -> Result<Arc<Vec<OpaqueExtrinsic>>, String> {
		let Some(block_body) = self.client.body(block_hash).ok().flatten() else {
			return Err(std::format!(
				"Failed to fetch block with block hash: {:?}",
				block_hash
			));
		};

		Ok(Arc::new(block_body))
	}

	async fn block_events(&mut self, block_hash: H256) -> Result<Arc<CachedEvents>, String> {
		if let Some(cached) = self.cache.read_cached_events(&block_hash) {
			return Ok(cached);
		}

		let params = SystemFetchEventsParams {
			enable_decoding: Some(true),
			enable_encoding: Some(true),
			..Default::default()
		};

		let events = common::fetch_events(&self.rpc_handlers, block_hash, params).await;
		let Some(events) = events else {
			return Ok(Arc::new(CachedEvents(Vec::new())));
		};

		let events = Arc::new(events);
		self.cache.write_cached_events(block_hash, events.clone());

		Ok(events)
	}

	fn block_state(&self, block_id: BlockIdentifier) -> Result<BlockState, String> {
		let chain_info = self.client.chain_info();
		let is_finalized = chain_info.finalized_number >= block_id.height;
		if !is_finalized {
			return Ok(BlockState::Included);
		}

		let finalized_hash = self
			.client
			.to_hash(&BlockId::Number(block_id.height))
			.map_err(|e| e.to_string())?;

		let Some(finalized_hash) = finalized_hash else {
			return Err("Failed to convert block height to block hash".into());
		};

		if finalized_hash == block_id.hash {
			return Ok(BlockState::Finalized);
		}

		Ok(BlockState::Discarded)
	}
}

fn read_consensus_events(
	enable_decoding: bool,
	events: &Arc<CachedEvents>,
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
	unique_id: TxIdentifier,
	opaq: &OpaqueExtrinsic,
	cache: SharedCache,
	filter: &block_overview::Filter,
	enable_event_decoding: bool,
	events: Option<Arc<CachedEvents>>,
) -> Option<block_overview::TransactionData> {
	use block_overview::TransactionFilterOptions;

	if let TransactionFilterOptions::TxIndex(tx_indexes) = &filter.transaction {
		if !tx_indexes.contains(&unique_id.tx_index) {
			return None;
		}
	}

	if let TransactionFilterOptions::HasEvent(..) = &filter.transaction {
		events.as_ref()?;
	}

	let tx_hash = cache.read_cached_tx_hash(&unique_id);
	if let TransactionFilterOptions::TxHash(tx_hashes) = &filter.transaction {
		if let Some(tx_hash) = &tx_hash {
			if !tx_hashes.contains(tx_hash) {
				return None;
			}
		}
	}

	let ext = UncheckedExtrinsic::decode_no_vec_prefix(&mut opaq.0.as_slice()).ok()?;
	let dispatch_index = common::read_pallet_call_index(&ext)?;

	if let TransactionFilterOptions::Pallet(pallets) = &filter.transaction {
		if !pallets.contains(&dispatch_index.0) {
			return None;
		}
	}

	if let TransactionFilterOptions::PalletCall(calls) = &filter.transaction {
		if !calls.contains(&dispatch_index) {
			return None;
		}
	}

	let tx_hash = if let Some(tx_hash) = tx_hash {
		tx_hash
	} else {
		let tx_hash = Blake2Hasher::hash(&ext.encode());
		cache.write_cached_tx_hash(unique_id, tx_hash)?;
		tx_hash
	};

	if let TransactionFilterOptions::TxHash(tx_hashes) = &filter.transaction {
		if !tx_hashes.contains(&tx_hash) {
			return None;
		}
	}

	let signature = read_signature(&ext);
	if let Some(expected_app_id) = &filter.signature.app_id {
		if let Some(signature) = &signature {
			if *expected_app_id != signature.app_id {
				return None;
			}
		} else {
			return None;
		}
	}

	if let Some(expected_none) = &filter.signature.nonce {
		if let Some(signature) = &signature {
			if *expected_none != signature.nonce {
				return None;
			}
		} else {
			return None;
		}
	}

	if filter.signature.ss58_address.is_some() {
		if let Some(signature) = &signature {
			if filter.signature.ss58_address != signature.ss58_address {
				return None;
			}
		} else {
			return None;
		}
	}

	let mut maybe_events = None;
	if let Some(all_events) = events {
		let tx_events = all_events.tx_events(unique_id.tx_index);
		if tx_events.is_none() {
			if let TransactionFilterOptions::HasEvent(..) = &filter.transaction {
				return None;
			}
		}

		if let Some(tx_events) = tx_events {
			let events: Vec<Event> = tx_events
				.events
				.iter()
				.map(|x| {
					let mut ev = Event {
						index: x.index,
						emitted_index: x.emitted_index,
						decoded: None,
					};
					if enable_event_decoding {
						ev.decoded = x.decoded.clone();
					}
					ev
				})
				.collect();

			if let TransactionFilterOptions::HasEvent(expected_events) = &filter.transaction {
				for exp_ev in expected_events {
					if !events.iter().any(|x| x.emitted_index == *exp_ev) {
						return None;
					}
				}
			}
			maybe_events = Some(events);
		}
	}

	let value = block_overview::TransactionData {
		hash: tx_hash,
		index: unique_id.tx_index,
		dispatch_index,
		signed: signature,
		decoded: None,
		events: maybe_events,
	};

	Some(value)
}

fn iter_data_opaque(
	unique_id: TxIdentifier,
	opaq: &OpaqueExtrinsic,
	cache: SharedCache,
	filter: &block_data::CallFilter,
) -> Option<block_data::CallData> {
	use block_data::TransactionFilterOptions;

	if let TransactionFilterOptions::TxIndex(tx_indexes) = &filter.transaction {
		if !tx_indexes.contains(&unique_id.tx_index) {
			return None;
		}
	}

	let tx_hash = cache.read_cached_tx_hash(&unique_id);
	if let TransactionFilterOptions::TxHash(tx_hashes) = &filter.transaction {
		if let Some(tx_hash) = &tx_hash {
			if !tx_hashes.contains(tx_hash) {
				return None;
			}
		}
	}

	let ext = UncheckedExtrinsic::decode_no_vec_prefix(&mut opaq.0.as_slice()).ok()?;
	let dispatch_index = common::read_pallet_call_index(&ext)?;

	if let TransactionFilterOptions::Pallet(pallets) = &filter.transaction {
		if !pallets.contains(&dispatch_index.0) {
			return None;
		}
	}

	if let TransactionFilterOptions::PalletCall(calls) = &filter.transaction {
		if !calls.contains(&dispatch_index) {
			return None;
		}
	}

	let tx_hash = if let Some(tx_hash) = tx_hash {
		tx_hash
	} else {
		let tx_hash = Blake2Hasher::hash(&ext.encode());
		cache.write_cached_tx_hash(unique_id, tx_hash)?;
		tx_hash
	};

	if let TransactionFilterOptions::TxHash(tx_hashes) = &filter.transaction {
		if !tx_hashes.contains(&tx_hash) {
			return None;
		}
	}

	let signature = read_signature(&ext);
	if let Some(expected_app_id) = &filter.signature.app_id {
		if let Some(signature) = &signature {
			if *expected_app_id != signature.app_id {
				return None;
			}
		} else {
			return None;
		}
	}

	if let Some(expected_none) = &filter.signature.nonce {
		if let Some(signature) = &signature {
			if *expected_none != signature.nonce {
				return None;
			}
		} else {
			return None;
		}
	}

	if filter.signature.ss58_address.is_some() {
		if let Some(signature) = &signature {
			if filter.signature.ss58_address != signature.ss58_address {
				return None;
			}
		} else {
			return None;
		}
	}

	let data = if let Some(data) = cache.read_cached_calls(&unique_id) {
		data
	} else {
		let Ok(mut data) = serde_json::to_string(opaq) else {
			return None;
		};

		if data.len() >= 2 {
			data.pop();
			data.remove(0);
		}

		cache.write_cached_calls(unique_id, data.clone());
		data
	};

	Some(block_data::CallData {
		dispatch_index,
		tx_index: unique_id.tx_index,
		tx_hash,
		data,
	})
}

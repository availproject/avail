use std::sync::{Arc, RwLock};

use avail_core::OpaqueExtrinsic;
use codec::{decode_from_bytes, DecodeAll, Encode};
use da_runtime::UncheckedExtrinsic;
use frame_system_rpc_runtime_api::{events::SemiDecodedEvent, SystemFetchEventsParams};
use jsonrpsee::tokio::sync::Notify;
use rayon::prelude::*;
use sc_service::RpcHandlers;
use sc_telemetry::log;
use sp_core::{Blake2Hasher, Hasher, H256};
use sp_runtime::{generic::BlockId, traits::BlockIdTo, AccountId32, MultiAddress};
use transaction_rpc::{
	block_data::{self},
	block_overview, BlockState, HashIndex,
};

use super::{
	super::chain_api,
	cache::{Cache, Cacheable, CachedEvent, CachedEventData, CachedEvents, SharedCache},
	logger::Logger,
	Deps,
};
use crate::{
	service::FullClient,
	workers::{macros::profile, read_pallet_call_index},
};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct UniqueTxId {
	pub block_hash: H256,
	pub tx_index: u32,
}

impl From<(H256, u32)> for UniqueTxId {
	fn from(value: (H256, u32)) -> Self {
		Self {
			block_hash: value.0,
			tx_index: value.1,
		}
	}
}

pub struct Worker {
	client: Arc<FullClient>,
	rpc_handlers: RpcHandlers,
	overview_receiver: block_overview::Receiver,
	data_receiver: block_data::Receiver,
	notifier: Arc<Notify>,
	logger: Logger,
	//
	// cache
	cache: SharedCache,
}

impl Worker {
	pub fn new(client: Arc<FullClient>, rpc_handlers: RpcHandlers, deps: Deps) -> Self {
		let logger = Logger::default();
		let cache = Arc::new(RwLock::new(Cache::new()));

		Self {
			client,
			rpc_handlers,
			overview_receiver: deps.overview_receiver,
			data_receiver: deps.data_receiver,
			notifier: deps.notifier,
			logger,
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

		let (block_hash, block_height) = self.block_metadata(params.block_id).await?;
		let block_body = self.block_body(block_hash)?;
		let block_state = self.block_state(block_hash, block_height)?;

		let mut maybe_calls: Option<Vec<CallData>> = None;
		if params.fetch_calls {
			let calls: Vec<block_data::CallData> = block_body
				.par_iter()
				.enumerate()
				.filter_map(|(i, opaq)| {
					let unique_id = UniqueTxId::from((block_hash, i as u32));
					iter_data_opaque(unique_id, opaq, self.cache.clone(), &params.call_filter)
				})
				.collect();
			maybe_calls = Some(calls);
		}

		let mut maybe_events: Option<Vec<block_data::EventData>> = None;
		if params.fetch_events {
			let block_events = self.block_events(block_hash).await?;

			let mut events = Vec::new();
			for cached_event in &block_events.0 {
				let phase = match cached_event.phase {
					frame_system::Phase::ApplyExtrinsic(x) => block_data::Phase::ApplyExtrinsic(x),
					frame_system::Phase::Finalization => block_data::Phase::Finalization,
					frame_system::Phase::Initialization => block_data::Phase::Initialization,
				};
				for ev in &cached_event.events {
					let data = EventData {
						id: (ev.pallet_id, ev.event_id),
						phase,
						data: ev.encoded.clone(),
					};
					events.push(data);
				}
			}
			maybe_events = Some(events)
		}

		let result = block_data::Response {
			block_hash,
			block_height,
			block_state,
			calls: maybe_calls,
			events: maybe_events,
		};

		Ok(result)
	}

	async fn overview_task(
		&mut self,
		params: block_overview::RPCParams,
	) -> Result<block_overview::Response, String> {
		let (block_hash, block_height) = self.block_metadata(params.block_id).await?;
		let block_body = self.block_body(block_hash)?;
		let block_state = self.block_state(block_hash, block_height)?;
		let enable_event_decoding = params.extension.enable_event_decoding;

		let events = if params.extension.fetch_events {
			Some(self.block_events(block_hash).await?)
		} else {
			None
		};

		let transactions: Vec<block_overview::TransactionData> = block_body
			.par_iter()
			.enumerate()
			.filter_map(|(i, opaq)| {
				iter_overview_opaque(
					UniqueTxId::from((block_hash, i as u32)),
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
				consensus_events = Some(read_consensus_events(enable_event_decoding, &events));
			}
		}

		let result = block_overview::Response {
			block_hash,
			block_height,
			block_state,
			transactions,
			consensus_events,
		};

		Ok(result)
	}

	async fn block_metadata(&self, block_id: HashIndex) -> Result<(H256, u32), String> {
		match block_id {
			HashIndex::Hash(hash) => {
				let height = self.client.to_number(&BlockId::Hash(hash.clone()));
				let Some(height) = height.ok().flatten() else {
					return Err(std::format!(
						"No block height found for block hash: {:?}",
						hash
					));
				};
				Ok((hash, height))
			},
			HashIndex::Index(height) => {
				let hash = self.client.to_hash(&BlockId::Number(height));
				let Some(hash) = hash.ok().flatten() else {
					return Err(std::format!(
						"No block hash found for block height: {}",
						height
					));
				};
				Ok((hash, height))
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

		let events = fetch_events(&self.rpc_handlers, block_hash).await;
		let Some(events) = events else {
			return Ok(Arc::new(CachedEvents(Vec::new())));
		};

		let events = Arc::new(events);
		self.cache.write_cached_events(block_hash, events.clone());

		Ok(events)
	}

	fn block_state(&self, hash: H256, height: u32) -> Result<BlockState, String> {
		let chain_info = self.client.chain_info();
		let is_finalized = chain_info.finalized_number >= height;
		if !is_finalized {
			return Ok(BlockState::Included);
		}

		let finalized_hash = self
			.client
			.to_hash(&BlockId::Number(height))
			.map_err(|e| e.to_string())?;

		let Some(finalized_hash) = finalized_hash else {
			return Err("Failed to convert block height to block hash".into());
		};

		if finalized_hash == hash {
			return Ok(BlockState::Finalized);
		}

		Ok(BlockState::Discarded)
	}
}

#[derive(codec::Decode)]
struct DataSubmittedEvent {
	pub who: AccountId32,
	pub data_hash: H256,
}

fn parse_decoded_event(semi: &SemiDecodedEvent) -> Option<block_overview::DecodedEventData> {
	use block_overview::DecodedEventData;
	use frame_system_rpc_runtime_api::events::event_id::*;

	match semi.pallet_id {
		system::PALLET_ID => {
			if semi.event_id == system::EXTRINSIC_SUCCESS {
				return Some(DecodedEventData::SystemExtrinsicSuccess);
			} else if semi.event_id == system::EXTRINSIC_FAILED {
				return Some(DecodedEventData::SystemExtrinsicFailed);
			}
		},
		sudo::PALLET_ID => {
			if semi.event_id == sudo::SUDID {
				let data = decode_from_bytes::<bool>(semi.data.clone().into()).ok()?;
				return Some(DecodedEventData::SudoSudid(data));
			} else if semi.event_id == sudo::SUDO_AS_DONE {
				let data = decode_from_bytes::<bool>(semi.data.clone().into()).ok()?;
				return Some(DecodedEventData::SudoSudoAsDone(data));
			}
		},
		data_availability::PALLET_ID => {
			if semi.event_id == data_availability::DATA_SUBMITTED {
				let value = DataSubmittedEvent::decode_all(&mut semi.data.as_slice()).ok()?;
				let data = block_overview::DataSubmittedEvent {
					who: std::format!("{}", value.who),
					data_hash: std::format!("{:?}", value.data_hash),
				};

				return Some(DecodedEventData::DataAvailabilityDataSubmitted(data));
			}
		},
		multisig::PALLET_ID => {
			if semi.event_id == multisig::MULTISIG_EXECUTED {
				let data = decode_from_bytes::<bool>(semi.data.clone().into()).ok()?;
				return Some(DecodedEventData::SudoSudoAsDone(data));
			}
		},
		proxy::PALLET_ID => {
			if semi.event_id == proxy::PROXY_EXECUTED {
				let data = decode_from_bytes::<bool>(semi.data.clone().into()).ok()?;
				return Some(DecodedEventData::SudoSudoAsDone(data));
			}
		},
		_ => (),
	}

	None
}

async fn fetch_events(handlers: &RpcHandlers, block_hash: H256) -> Option<CachedEvents> {
	let params = SystemFetchEventsParams {
		filter_tx_indices: None,
		enable_decoding: Some(true),
		enable_encoding: Some(true),
		..Default::default()
	};

	let rpc_events = chain_api::system_fetch_events(handlers, params, &block_hash).await;

	let Some(rpc_events) = rpc_events else {
		return None;
	};
	if rpc_events.error != 0 {
		return None;
	}

	let encoded_events = rpc_events.encoded;
	let decoded_events = rpc_events.decoded;

	let mut cached_events = Vec::<CachedEvent>::new();
	for enc in &encoded_events {
		let mut cached_event = CachedEvent {
			phase: enc.phase.clone(),
			events: Vec::new(),
		};

		let decoded = decoded_events
			.iter()
			.find(|x| x.phase == enc.phase)
			.map(|x| &x.events);

		for ev in &enc.events {
			let index = ev.index;

			let mut data = CachedEventData {
				index,
				pallet_id: ev.pallet_id,
				event_id: ev.pallet_id,
				encoded: std::format!("0x{}", hex::encode(&ev.data)),
				decoded: None,
			};

			if let Some(decoded) = &decoded {
				if let Some(ev) = decoded.iter().find(|x| x.index == index) {
					data.decoded = parse_decoded_event(ev);
				}
			}

			cached_event.events.push(data);
		}

		cached_events.push(cached_event);
	}

	Some(CachedEvents(cached_events))
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
				pallet_id: event.pallet_id,
				event_id: event.pallet_id,
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
	unique_id: UniqueTxId,
	opaq: &OpaqueExtrinsic,
	cache: SharedCache,
	filter: &block_overview::Filter,
	enable_event_decoding: bool,
	events: Option<Arc<CachedEvents>>,
) -> Option<block_overview::TransactionData> {
	use block_overview::{Event, TransactionFilterOptions};

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

	let Ok(ext) = UncheckedExtrinsic::decode_no_vec_prefix(&mut opaq.0.as_slice()) else {
		return None;
	};

	let Some((pallet_id, call_id)) = read_pallet_call_index(&ext) else {
		return None;
	};

	if let TransactionFilterOptions::Pallet(pallets) = &filter.transaction {
		if !pallets.contains(&pallet_id) {
			return None;
		}
	}

	if let TransactionFilterOptions::PalletCall(calls) = &filter.transaction {
		if !calls.contains(&(pallet_id, call_id)) {
			return None;
		}
	}

	let tx_hash = if let Some(tx_hash) = tx_hash {
		tx_hash
	} else {
		let tx_hash = Blake2Hasher::hash(&ext.encode());
		if cache.write_cached_tx_hash(unique_id, tx_hash).is_none() {
			return None;
		}
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
						pallet_id: x.pallet_id,
						event_id: x.event_id,
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
					if events
						.iter()
						.find(|x| x.pallet_id == exp_ev.0 && x.event_id == exp_ev.1)
						.is_none()
					{
						return None;
					}
				}
			}
			maybe_events = Some(events);
		}
	} else {
		if let TransactionFilterOptions::HasEvent(..) = &filter.transaction {
			return None;
		}
	}

	let value = block_overview::TransactionData {
		tx_hash,
		tx_index: unique_id.tx_index,
		pallet_id,
		call_id,
		signed: signature,
		decoded: None,
		events: maybe_events,
	};

	return Some(value);
}

fn iter_data_opaque(
	unique_id: UniqueTxId,
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

	let Ok(ext) = UncheckedExtrinsic::decode_no_vec_prefix(&mut opaq.0.as_slice()) else {
		return None;
	};

	let Some((pallet_id, call_id)) = read_pallet_call_index(&ext) else {
		return None;
	};

	if let TransactionFilterOptions::Pallet(pallets) = &filter.transaction {
		if !pallets.contains(&pallet_id) {
			return None;
		}
	}

	if let TransactionFilterOptions::PalletCall(calls) = &filter.transaction {
		if !calls.contains(&(pallet_id, call_id)) {
			return None;
		}
	}

	let tx_hash = if let Some(tx_hash) = tx_hash {
		tx_hash
	} else {
		let tx_hash = Blake2Hasher::hash(&ext.encode());
		if cache.write_cached_tx_hash(unique_id, tx_hash).is_none() {
			return None;
		}
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

		dbg!(&data);

		cache.write_cached_calls(unique_id, data.clone());
		data
	};

	let value = block_data::CallData {
		id: (pallet_id, call_id),
		tx_index: unique_id.tx_index,
		tx_hash,
		data,
	};

	return Some(value);
}

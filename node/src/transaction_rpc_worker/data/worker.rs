use super::cache::{Cacheable, CachedEvent, CachedEventData, CachedEvents, SharedCache};
use super::{super::runtime_api, cache::Cache, filter::*, logger::Logger};
use crate::transaction_rpc_worker::read_pallet_call_index;
use crate::{service::FullClient, transaction_rpc_worker::macros::profile};
use avail_core::OpaqueExtrinsic;
use codec::{decode_from_bytes, DecodeAll, Encode};
use da_runtime::UncheckedExtrinsic;
use frame_system_rpc_runtime_api::{events::SemiDecodedEvent, SystemFetchEventsParams};
use jsonrpsee::tokio::sync::Notify;
use rayon::prelude::*;
use sc_service::RpcHandlers;
use sc_telemetry::log;
use sp_core::{Blake2Hasher, Hasher, H256};
use sp_runtime::generic::BlockId;
use sp_runtime::traits::BlockIdTo;
use sp_runtime::AccountId32;
use std::sync::{Arc, RwLock};
use transaction_rpc::{block_data, block_overview, BlockState, HashIndex};

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
	pub fn new(
		client: Arc<FullClient>,
		rpc_handlers: RpcHandlers,
		overview_receiver: block_overview::Receiver,
		data_receiver: block_data::Receiver,
		notifier: Arc<Notify>,
	) -> Self {
		let logger = Logger::default();
		let cache = Arc::new(RwLock::new(Cache::new()));

		Self {
			client,
			rpc_handlers,
			overview_receiver,
			data_receiver,
			notifier,
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

	/* 	{
		let Ok(lock) = cache.read() else {
			return None;
		};


	} */

	async fn data_task(
		&mut self,
		params: block_data::RPCParams,
	) -> Result<block_data::RPCResult, String> {
		use block_data::{CallData, EventData};

		let (block_hash, block_height) = self.block_metadata(params.block_id).await?;
		let block_body = self.block_body(block_hash)?;
		let events = self.block_events(block_hash).await?;
		let block_state = self.block_state(block_hash, block_height)?;

		let mut calls: Vec<CallData> = Vec::new();
		for (i, opaq) in block_body.iter().enumerate() {
			let unique_id = (block_hash, i as u32);
			let ext = UncheckedExtrinsic::decode_no_vec_prefix(&mut opaq.0.as_slice());
			let ext = ext.unwrap();
			let (pallet_id, call_id) = read_pallet_call_index(&ext).unwrap();

			let tx_hash = if let Some(tx_hash) = self.cache.read_cached_tx_hash(&unique_id) {
				tx_hash
			} else {
				let tx_hash = Blake2Hasher::hash(&ext.encode());
				self.cache.write_cached_tx_hash(unique_id, tx_hash).unwrap();
				tx_hash
			};

			let data = if let Some(call) = self.cache.read_cached_calls(&unique_id) {
				call
			} else {
				let call = std::format!("0x{}", hex::encode(ext.function.encode()));
				self.cache.write_cached_calls(unique_id, call.clone());
				call
			};

			let call = CallData {
				id: (pallet_id, call_id),
				tx_id: unique_id.1,
				tx_hash,
				data,
			};
			calls.push(call);
		}

		let mut block_events = Vec::new();
		for cached_event in &events.0 {
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
				block_events.push(data);
			}
		}

		let result = block_data::RPCResult {
			block_hash,
			block_height,
			block_state,
			calls: Some(calls),
			events: Some(block_events),
		};

		Ok(result)
	}

	async fn overview_task(
		&mut self,
		params: block_overview::RPCParams,
	) -> Result<block_overview::RPCResult, String> {
		let extension = params.extension;
		let filter = params.filter.clone().unwrap_or_default();

		let (block_hash, block_height) = self.block_metadata(params.block_id).await?;
		let block_body = self.block_body(block_hash)?;
		let events = self.block_events(block_hash).await?;
		let block_state = self.block_state(block_hash, block_height)?;
		let consensus_events = consensus_events(&params, &events);

		let transactions: Vec<block_overview::TransactionData> = block_body
			.par_iter()
			.enumerate()
			.filter_map(|(i, opaq)| {
				filter_extrinsic(
					(block_hash, i as u32),
					opaq,
					&filter,
					&extension,
					self.cache.clone(),
					events.clone(),
				)
			})
			.collect();

		let result = block_overview::RPCResult {
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

	let rpc_events = runtime_api::system_fetch_events(handlers, params, &block_hash).await;

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

fn consensus_events(
	params: &block_overview::RPCParams,
	events: &Arc<CachedEvents>,
) -> Option<block_overview::Events> {
	if !params.extension.fetch_events {
		return None;
	}

	let Some(cached_event) = events.consensus_events() else {
		return None;
	};

	let mut consensus_events = Vec::with_capacity(cached_event.events.len());
	for data in &cached_event.events {
		let mut event = block_overview::Event {
			index: data.index,
			pallet_id: data.pallet_id,
			event_id: data.event_id,
			decoded: None,
		};

		if params.extension.enable_event_decoding {
			event.decoded = data.decoded.clone()
		}

		consensus_events.push(event);
	}

	Some(consensus_events)
}

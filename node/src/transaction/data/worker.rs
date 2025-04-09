use super::{super::runtime_api, cache::Cache, filter::*, logger::Logger};
use crate::{service::FullClient, transaction::macros::profile};
use avail_core::OpaqueExtrinsic;
use codec::{decode_from_bytes, DecodeAll, Encode};
use frame_system_rpc_runtime_api::{events::SemiDecodedEvent, SystemFetchEventsParams};
use jsonrpsee::tokio::sync::Notify;
use rayon::prelude::*;
use sc_service::RpcHandlers;
use sc_telemetry::log;
use sp_core::H256;
use sp_runtime::generic::BlockId;
use sp_runtime::traits::BlockIdTo;
use sp_runtime::AccountId32;
use std::sync::Arc;
use std::sync::RwLock;
use transaction_rpc::data_types::{
	self, DecodedEvents, EncodedEvents, HashIndex, RPCParams, RPCParamsExtension, RPCResult,
	TransactionData, TxDataReceiver,
};

pub(crate) type SharedCache = Arc<RwLock<Cache>>;

pub struct Worker {
	client: Arc<FullClient>,
	rpc_handlers: RpcHandlers,
	receiver: TxDataReceiver,
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
		receiver: TxDataReceiver,
		notifier: Arc<Notify>,
	) -> Self {
		let logger = Logger::default();
		let cache = Arc::new(RwLock::new(Cache::new()));

		Self {
			client,
			rpc_handlers,
			receiver,
			notifier,
			logger,
			cache,
		}
	}

	pub async fn run(mut self) {
		log::info!("🐖 Transaction Data Running");

		loop {
			if !self.receiver.is_empty() {
				let (duration, _) = profile!({
					while let Ok((params, oneshot)) = self.receiver.try_recv() {
						let result = self.task(params).await;
						_ = oneshot.send(result);
					}
				});
				self.logger.new_total(duration);
			}

			self.logger.log();

			self.notifier.notified().await;
		}
	}

	async fn task(&mut self, params: RPCParams) -> Result<RPCResult, String> {
		let extension = params.extension.unwrap_or_default();
		let filter = params.filter.clone().unwrap_or_default();

		let (block_hash, block_height) = self.block_metadata(&params).await?;
		let block_body = self.block_body(block_hash)?;

		let mut transactions: Vec<TransactionData> = block_body
			.par_iter()
			.enumerate()
			.filter_map(|(i, opaq)| {
				filter_extrinsic(
					block_hash,
					i as u32,
					opaq,
					&filter,
					&extension,
					self.cache.clone(),
				)
			})
			.collect();

		if extension.fetch_events.unwrap_or(false) && !transactions.is_empty() {
			let (duration, _) = profile!(
				cached_fetch_events(
					&self.rpc_handlers,
					block_hash,
					&extension,
					&self.cache,
					&mut transactions
				)
				.await?
			);
			self.logger.new_events(duration);
		}

		let result = RPCResult {
			block_hash,
			block_height,
			transactions,
		};

		Ok(result)
	}

	async fn block_metadata(&self, params: &RPCParams) -> Result<(H256, u32), String> {
		match params.block_id {
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
}

#[derive(codec::Decode)]
struct DataSubmittedEvent {
	pub who: AccountId32,
	pub data_hash: H256,
}

pub struct RPCEvent {
	pub tx_index: u32,
	pub encoded: EncodedEvents,
	pub decoded: DecodedEvents,
}

impl RPCEvent {
	pub fn weight(&self) -> u64 {
		use transaction_rpc::data_types::{DecodedEvent, EncodedEvent};

		let mut weight: usize = size_of::<RPCEvent>();
		weight += self.encoded.len() * size_of::<EncodedEvent>();
		weight += self.decoded.len() * size_of::<DecodedEvent>();

		for e in &self.encoded {
			weight += e.data.len();
		}

		weight as u64
	}
}

pub type RPCEvents = Vec<RPCEvent>;

fn parse_decoded_event(semi: SemiDecodedEvent) -> Option<data_types::DecodedEvent> {
	use data_types::{DecodedEvent, DecodedEventData};
	use frame_system_rpc_runtime_api::events::event_id::*;

	let mut ev = DecodedEvent::new(
		semi.index,
		semi.pallet_id,
		semi.event_id,
		DecodedEventData::Unknown,
	);

	match semi.pallet_id {
		system::PALLET_ID => {
			if semi.event_id == system::EXTRINSIC_SUCCESS {
				ev.data = DecodedEventData::SystemExtrinsicSuccess;
			} else if semi.event_id == system::EXTRINSIC_FAILED {
				ev.data = DecodedEventData::SystemExtrinsicFailed;
			}
		},
		sudo::PALLET_ID => {
			if semi.event_id == sudo::SUDID {
				let data = decode_from_bytes::<bool>(semi.data.into()).ok()?;
				ev.data = DecodedEventData::SudoSudid(data);
			} else if semi.event_id == sudo::SUDO_AS_DONE {
				let data = decode_from_bytes::<bool>(semi.data.into()).ok()?;
				ev.data = DecodedEventData::SudoSudoAsDone(data);
			}
		},
		data_availability::PALLET_ID => {
			if semi.event_id == data_availability::DATA_SUBMITTED {
				let encoded = semi.data;
				let value = DataSubmittedEvent::decode_all(&mut encoded.as_slice()).ok()?;
				let data = data_types::DataSubmittedEvent {
					who: std::format!("{}", value.who),
					data_hash: std::format!("{:?}", value.data_hash),
				};

				ev.data = DecodedEventData::DataAvailabilityDataSubmitted(data);
			}
		},
		multisig::PALLET_ID => {
			if semi.event_id == multisig::MULTISIG_EXECUTED {
				let data = decode_from_bytes::<bool>(semi.data.into()).ok()?;
				ev.data = DecodedEventData::SudoSudoAsDone(data);
			}
		},
		proxy::PALLET_ID => {
			if semi.event_id == proxy::PROXY_EXECUTED {
				let data = decode_from_bytes::<bool>(semi.data.into()).ok()?;
				ev.data = DecodedEventData::SudoSudoAsDone(data);
			}
		},
		_ => (),
	}

	Some(ev)
}

async fn cached_fetch_events(
	handlers: &RpcHandlers,
	block_hash: H256,
	extension: &RPCParamsExtension,
	cache: &SharedCache,
	txs: &mut Vec<TransactionData>,
) -> Result<(), String> {
	let enable_encoding = extension.enable_event_encoding.unwrap_or(false);
	let enable_decoding = extension.enable_event_decoding.unwrap_or(false);
	{
		let Ok(lock) = cache.read() else {
			return Err("Failed to lock cache. Internal Error".into());
		};
		let events = lock.events.get(&block_hash);
		if let Some(events) = events {
			for tx in txs {
				let (encoded, decoded) =
					filter_events(tx.tx_index, enable_encoding, enable_decoding, &events);
				tx.extension.encoded_events = encoded;
				tx.extension.decoded_events = decoded;
			}
			return Ok(());
		}
	}
	let events = fetch_events(handlers, block_hash).await;
	let Some(events) = events else {
		return Ok(());
	};

	for tx in txs {
		let (encoded, decoded) =
			filter_events(tx.tx_index, enable_encoding, enable_decoding, &events);
		tx.extension.encoded_events = encoded;
		tx.extension.decoded_events = decoded;
	}

	let Ok(mut lock) = cache.write() else {
		return Err("Failed to lock cache. Internal Error".into());
	};

	lock.events.insert(block_hash, events);

	Ok(())
}

async fn fetch_events(handlers: &RpcHandlers, block_hash: H256) -> Option<RPCEvents> {
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

	let mut rpc_events = Vec::<RPCEvent>::new();
	for enc in &encoded_events {
		let mut events = Vec::new();
		for x in &enc.events {
			let encoded = transaction_rpc::data_types::EncodedEvent {
				index: x.index,
				pallet_id: x.pallet_id,
				event_id: x.event_id,
				data: std::format!("0x{}", hex::encode(x.data.encode())),
			};
			events.push(encoded);
		}

		if let Some(pos) = rpc_events.iter().position(|x| x.tx_index == enc.tx_index) {
			rpc_events[pos].encoded = events;
		} else {
			rpc_events.push(RPCEvent {
				tx_index: enc.tx_index,
				encoded: events,
				decoded: Vec::new(),
			});
		};
	}

	for dec in decoded_events {
		let mut events = Vec::new();
		for x in dec.events {
			let Some(decoded) = parse_decoded_event(x) else {
				continue;
			};
			events.push(decoded);
		}

		if let Some(pos) = rpc_events.iter().position(|x| x.tx_index == dec.tx_index) {
			rpc_events[pos].decoded = events;
		} else {
			rpc_events.push(RPCEvent {
				tx_index: dec.tx_index,
				encoded: Vec::new(),
				decoded: events,
			});
		};
	}

	Some(rpc_events)
}

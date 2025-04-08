pub mod constants;
pub mod logger;

use super::runtime_api;
use crate::service::FullClient;
use crate::transaction::macros::profile;
use avail_core::OpaqueExtrinsic;
use codec::{decode_from_bytes, DecodeAll, Encode};
use da_runtime::UncheckedExtrinsic;
use frame_system_rpc_runtime_api::events::SemiDecodedEvent;
use frame_system_rpc_runtime_api::SystemFetchEventsParams;
use jsonrpsee::tokio::sync::Notify;
use sc_service::RpcHandlers;
use sc_telemetry::log;
use sp_core::H256;
use sp_core::{Blake2Hasher, Hasher};
use sp_runtime::generic::BlockId;
use sp_runtime::traits::BlockIdTo;
use sp_runtime::{AccountId32, MultiAddress};
use std::collections::HashMap;
use std::sync::Arc;
use transaction_rpc::data_types::{
	self, DecodedEvents, EncodedEvents, Filter, HashIndex, RPCParams, TransactionData,
	TransactionDataExtension, TransactionDataSigned, TxDataReceiver,
};

use super::read_pallet_call_index;

#[derive(Clone, Default)]
pub struct CliDeps {
	pub enabled: bool,
}

pub struct Deps {
	pub receiver: TxDataReceiver,
	pub notifier: Arc<Notify>,
}

type CachedTxHashKey = (H256, u32);
type CachedTxHashValue = H256;
type CachedEncodedCallKey = (H256, u32);
type CachedEncodedCallValue = Vec<u8>;
type CachedEventKey = H256;
type CachedEventValue = Vec<RPCEvent>;

#[derive(Default)]
pub struct Cache {
	tx_hash_cache: HashMap<CachedTxHashKey, CachedTxHashValue>,
	encoded_call_cache: HashMap<CachedEncodedCallKey, CachedEncodedCallValue>,
	events_cache: HashMap<CachedEventKey, CachedEventValue>,
}
impl Cache {
	pub fn get_encoded_call(&self, key: &CachedEncodedCallKey) -> Option<&CachedEncodedCallValue> {
		self.encoded_call_cache.get(key)
	}

	pub fn insert_encoded_call(
		&mut self,
		key: CachedEncodedCallKey,
		value: CachedEncodedCallValue,
	) {
		self.encoded_call_cache.insert(key, value);
	}

	pub fn get_tx_hash(&self, key: &CachedTxHashKey) -> Option<&CachedTxHashValue> {
		self.tx_hash_cache.get(key)
	}

	pub fn insert_tx_hash(&mut self, key: CachedTxHashKey, value: CachedTxHashValue) {
		self.tx_hash_cache.insert(key, value);
	}

	pub fn get_events(&self, key: &CachedEventKey) -> Option<&CachedEventValue> {
		self.events_cache.get(key)
	}

	pub fn insert_events(&mut self, key: CachedEventKey, value: CachedEventValue) {
		self.events_cache.insert(key, value);
	}

	pub fn resize(&mut self) {
		if self.tx_hash_cache.len() > 50_000 {
			self.tx_hash_cache.clear();
		}

		if self.encoded_call_cache.len() > 10_000 {
			self.encoded_call_cache.clear();
		}

		if self.events_cache.len() > 100 {
			self.events_cache.clear();
		}
	}
}

pub struct Worker {
	pub client: Arc<FullClient>,
	pub rpc_handlers: RpcHandlers,
	pub receiver: TxDataReceiver,
	pub cache: Cache,
	pub notifier: Arc<Notify>,
	pub logger: logger::Logger,
}

impl Worker {
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
			self.cache.resize();

			self.notifier.notified().await;
		}
	}

	async fn task(&mut self, params: RPCParams) -> Result<data_types::RPCResult, String> {
		let (block_hash, block_height) = match params.block_id {
			HashIndex::Hash(block_hash) => {
				let block_height = self.client.to_number(&BlockId::Hash(block_hash.clone()));
				let Some(block_height) = block_height.ok().flatten() else {
					return Err(std::format!(
						"No block height found for block hash: {:?}",
						block_hash
					));
				};
				(block_hash, block_height)
			},
			HashIndex::Index(block_height) => {
				let block_hash = self.client.to_hash(&BlockId::Number(block_height));
				let Some(block_hash) = block_hash.ok().flatten() else {
					return Err(std::format!(
						"No block hash found for block height: {}",
						block_height
					));
				};
				(block_hash, block_height)
			},
		};

		let extension = params.extension.unwrap_or_default();
		let transactions = self.extrinsics(block_hash, &params, extension).await?;

		let result = data_types::RPCResult {
			block_hash,
			block_height,
			transactions,
		};

		Ok(result)
	}

	fn parse_decoded_event(&self, semi: SemiDecodedEvent) -> Option<data_types::DecodedEvent> {
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

	async fn extrinsics(
		&mut self,
		block_hash: H256,
		params: &RPCParams,
		extension: data_types::RPCParamsExtension,
	) -> Result<Vec<TransactionData>, String> {
		let filter = params.filter.clone().unwrap_or_default();

		let Some(block_body) = self.client.body(block_hash).ok().flatten() else {
			return Err(std::format!(
				"Failed to fetch block with block hash: {:?}",
				block_hash
			));
		};

		let mut extrinsics = Vec::new();
		for (i, opaq) in block_body.iter().enumerate() {
			let (ext, stop) = self
				.filter_extrinsic(block_hash, i as u32, opaq, &filter, &extension)
				.await?;
			if let Some(ext) = ext {
				extrinsics.push(ext);
			}
			if stop {
				break;
			}
		}

		Ok(extrinsics)
	}

	async fn filter_extrinsic(
		&mut self,
		block_hash: H256,
		tx_index: u32,
		opaq: &OpaqueExtrinsic,
		filter: &Filter,
		rpc_extension: &data_types::RPCParamsExtension,
	) -> Result<(Option<TransactionData>, bool), String> {
		if let Some(HashIndex::Index(target_index)) = &filter.tx_id {
			if *target_index != tx_index as u32 {
				return Ok((None, false));
			}
		};

		if let Some(HashIndex::Hash(target_hash)) = &filter.tx_id {
			if let Some(cached) = self.cache.get_tx_hash(&(block_hash, tx_index)) {
				if target_hash != cached {
					return Ok((None, false));
				}
			}
		};

		let ext = UncheckedExtrinsic::decode_no_vec_prefix(&mut opaq.0.as_slice());
		let Ok(ext) = ext else {
			return Err(std::format!(
				"Failed to fetch transaction. tx index: {}, block hash: {:?}",
				tx_index,
				block_hash
			));
		};

		let (id, filtered_out) = self.filter_pallet_call_id(&ext, &filter);
		if filtered_out {
			return Ok((None, false));
		}

		let Some((pallet_id, call_id)) = id else {
			let err = std::format!(
				"Failed to read pallet and call id. Tx index: {}, block hash: {:?}",
				tx_index,
				block_hash
			);
			return Err(err);
		};

		let (signed, filtered_out) = self.filter_signature(&ext, &filter);
		if filtered_out {
			return Ok((None, false));
		}

		let (duration, tx_hash) = profile!(self.cached_tx_hash(block_hash, tx_index, &ext));
		self.logger.new_tx_hash(duration);

		if let Some(HashIndex::Hash(target_hash)) = &filter.tx_id {
			if tx_hash != *target_hash {
				return Ok((None, false));
			}
		};

		let mut tx_extension = TransactionDataExtension::default();
		if rpc_extension.fetch_call.unwrap_or(false) {
			let (duration, value) = profile!(self.cached_encoded_call(block_hash, tx_index, &ext));
			self.logger.new_encoded_call(duration);
			tx_extension.encoded_call = Some(value);
		}

		if rpc_extension.fetch_events.unwrap_or(false) {
			let enable_encoding = rpc_extension.enable_event_encoding.unwrap_or(true);
			let enable_decoding = rpc_extension.enable_event_decoding.unwrap_or(false);
			let (duration, (enc, dec)) = profile!(
				self.cached_events(block_hash, tx_index, enable_encoding, enable_decoding)
					.await
			);
			self.logger.new_events(duration);
			tx_extension.encoded_events = enc;
			tx_extension.decoded_events = dec;
		}

		let tx = TransactionData {
			tx_hash,
			tx_index,
			pallet_id,
			call_id,
			signed,
			extension: tx_extension,
		};

		if filter.tx_id.is_some() {
			return Ok((Some(tx), true));
		}

		Ok((Some(tx), false))
	}

	fn filter_pallet_call_id(
		&self,
		ext: &UncheckedExtrinsic,
		filter: &Filter,
	) -> (Option<(u8, u8)>, bool) {
		let Some((pallet_id, call_id)) = read_pallet_call_index(&ext) else {
			let should_filter = filter.pallet_id.is_some() || filter.call_id.is_some();
			return (None, should_filter);
		};

		if filter.pallet_id.is_some_and(|x| x != pallet_id) {
			return (None, true);
		};

		if filter.call_id.is_some_and(|x| x != call_id) {
			return (None, true);
		};

		(Some((pallet_id, call_id)), false)
	}

	fn filter_signature(
		&self,
		ext: &UncheckedExtrinsic,
		filter: &Filter,
	) -> (Option<TransactionDataSigned>, bool) {
		let requires_signed =
			filter.app_id.is_some() || filter.nonce.is_some() || filter.ss58_address.is_some();

		let Some(sig) = &ext.signature else {
			return (None, requires_signed);
		};

		let mut signed = TransactionDataSigned::default();

		if let MultiAddress::Id(id) = &sig.0 {
			signed.ss58_address = Some(std::format!("{}", id))
		};

		signed.nonce = sig.2 .5 .0;
		signed.app_id = sig.2 .8 .0 .0;
		match sig.2 .4 .0 {
			sp_runtime::generic::Era::Immortal => signed.mortality = None,
			sp_runtime::generic::Era::Mortal(x, y) => signed.mortality = Some((x, y)),
		};

		if filter.app_id.is_some_and(|x| x != signed.app_id) {
			return (None, true);
		}

		if filter.nonce.is_some_and(|x| x != signed.nonce) {
			return (None, true);
		}

		if filter.ss58_address.is_some() && filter.ss58_address != signed.ss58_address {
			return (None, true);
		}

		(Some(signed), false)
	}

	fn cached_tx_hash(
		&mut self,
		block_hash: H256,
		tx_index: u32,
		ext: &UncheckedExtrinsic,
	) -> H256 {
		if let Some(cached) = self.cache.get_tx_hash(&(block_hash, tx_index)) {
			return *cached;
		}

		let tx_hash = Blake2Hasher::hash(&ext.encode());
		self.cache.insert_tx_hash((block_hash, tx_index), tx_hash);
		tx_hash
	}

	fn cached_encoded_call(
		&mut self,
		block_hash: H256,
		tx_index: u32,
		ext: &UncheckedExtrinsic,
	) -> String {
		if let Some(cached) = self.cache.get_encoded_call(&(block_hash, tx_index)) {
			println!("Cache Hit");
			return std::format!("0x{}", hex::encode(cached));
		}
		println!("Cache Miss");
		let encoded = ext.function.encode();

		self.cache
			.insert_encoded_call((block_hash, tx_index), encoded.clone());

		std::format!("0x{}", hex::encode(encoded))
	}

	async fn cached_events(
		&mut self,
		block_hash: H256,
		tx_index: u32,
		enable_encoding: bool,
		enable_decoding: bool,
	) -> (Option<EncodedEvents>, Option<DecodedEvents>) {
		if let Some(cached) = self.cache.get_events(&(block_hash)) {
			let (encoded, decoded) =
				filter_events(tx_index, enable_encoding, enable_decoding, cached);
			return (encoded, decoded);
		}

		let params = SystemFetchEventsParams {
			filter_tx_indices: None,
			enable_decoding: Some(true),
			enable_encoding: Some(true),
			..Default::default()
		};

		let rpc_events =
			runtime_api::system_fetch_events(&self.rpc_handlers, params, &block_hash).await;

		let Some(rpc_events) = rpc_events else {
			return (None, None);
		};

		if rpc_events.error != 0 {
			return (None, None);
		}

		let encoded_events = rpc_events.encoded;
		let decoded_events = rpc_events.decoded;

		let mut rpc_events = Vec::<RPCEvent>::new();
		for enc in encoded_events {
			let mut events = Vec::new();
			for x in enc.events {
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
				let Some(decoded) = self.parse_decoded_event(x) else {
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

		let (encoded_events, decoded_events) =
			filter_events(tx_index, enable_encoding, enable_decoding, &rpc_events);

		self.cache.insert_events(block_hash, rpc_events);

		return (encoded_events, decoded_events);
	}
}

fn filter_events(
	tx_index: u32,
	enable_encoding: bool,
	enable_decoding: bool,
	rpc_events: &Vec<RPCEvent>,
) -> (Option<EncodedEvents>, Option<DecodedEvents>) {
	let Some(rpc_event) = rpc_events.iter().find(|x| x.tx_index == tx_index) else {
		return (None, None);
	};

	let mut encoded_result = None;
	let mut decoded_result = None;

	if enable_encoding {
		encoded_result = Some(rpc_event.encoded.clone())
	}

	if enable_decoding {
		decoded_result = Some(rpc_event.decoded.clone())
	}

	if !enable_encoding && !enable_encoding {
		let mut encoded = rpc_event.encoded.clone();
		encoded.iter_mut().for_each(|x| x.data = "".into());
		encoded_result = Some(encoded);
	}

	(encoded_result, decoded_result)
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

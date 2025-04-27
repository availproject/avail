use super::chain_api;
use crate::service::FullClient;
use avail_core::OpaqueExtrinsic;
use block_rpc::common::events::DecodedEventData;
use block_rpc::BlockIdentifier;
use codec::Encode;
use da_runtime::UncheckedExtrinsic;
use frame_system_rpc_runtime_api::SystemFetchEventsParams;
use jsonrpsee::tokio::time::sleep;
use sc_service::RpcHandlers;
use sp_core::H256;
use sp_runtime::AccountId32;
use sp_runtime::{generic::BlockId, traits::BlockIdTo};
use std::{
	sync::Arc,
	time::{Duration, Instant},
};

pub use events::*;

pub const SLEEP_ON_FETCH: Duration = Duration::from_secs(1);
pub const SLEEP_ON_SYNC: Duration = Duration::from_secs(30);

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct TxIdentifier {
	pub block_hash: H256,
	pub tx_index: u32,
}

impl From<(H256, u32)> for TxIdentifier {
	fn from(value: (H256, u32)) -> Self {
		Self {
			block_hash: value.0,
			tx_index: value.1,
		}
	}
}

#[derive(Clone)]
pub struct NodeContext {
	pub client: Arc<FullClient>,
	pub handlers: RpcHandlers,
}

impl NodeContext {
	pub fn to_number(&self, value: H256) -> Result<Option<u32>, sp_blockchain::Error> {
		self.client.to_number(&BlockId::Hash(value))
	}

	pub fn to_hash(&self, value: u32) -> Result<Option<H256>, sp_blockchain::Error> {
		self.client.to_hash(&BlockId::Number(value))
	}

	pub fn block_body(&self, height: u32) -> Option<(Vec<OpaqueExtrinsic>, BlockIdentifier)> {
		let block_hash = self.to_hash(height).ok().flatten()?;
		let opaques = self.client.body(block_hash).ok().flatten()?;

		Some((opaques, BlockIdentifier::from((block_hash, height))))
	}

	pub fn block_body_hash(&self, hash: H256) -> Option<Vec<OpaqueExtrinsic>> {
		self.client.body(hash).ok().flatten()
	}

	pub async fn fetch_events(
		&self,
		block_hash: H256,
		params: SystemFetchEventsParams,
	) -> Option<AllTransactionEvents> {
		let rpc_events =
			chain_api::system_fetch_events(&self.handlers, params, &block_hash).await?;

		if rpc_events.error != 0 {
			return None;
		}

		let runtime_events = rpc_events.entries.into_iter();
		let cached_tx_events = runtime_events.map(TransactionEvents::from).collect();

		Some(AllTransactionEvents(cached_tx_events))
	}

	pub async fn fetch_sync_status(&self) -> Option<bool> {
		chain_api::system_fetch_sync_status(&self.handlers).await
	}

	pub async fn wait_for_sync(&self) -> Result<(), ()> {
		loop {
			let status = self.fetch_sync_status().await;
			match status {
				Some(true) => (),
				Some(false) => return Ok(()),
				None => return Err(()),
			}

			sleep(SLEEP_ON_SYNC).await;
		}
	}

	pub async fn wait_for_new_best_block(&self, current_block_hash: H256) -> BlockIdentifier {
		loop {
			let chain_info = self.client.chain_info();
			let (block_hash, block_height) = (chain_info.best_hash, chain_info.best_number);
			if current_block_hash.eq(&block_hash) {
				sleep(SLEEP_ON_FETCH).await;
				continue;
			}

			return BlockIdentifier::from((block_hash, block_height));
		}
	}

	pub async fn wait_for_new_finalized_block(&self, expected_height: u32) {
		loop {
			let chain_info = self.client.chain_info();
			if expected_height > chain_info.finalized_number {
				sleep(SLEEP_ON_FETCH).await;
				continue;
			}

			break;
		}
	}
}

pub fn read_pallet_call_index(ext: &UncheckedExtrinsic) -> Option<(u8, u8)> {
	let ext = ext.function.encode();
	if ext.len() < 2 {
		return None;
	}
	let pallet_index = ext[0];
	let call_index = ext[1];

	Some((pallet_index, call_index))
}

pub struct Timer {
	now: Instant,
	// In sec
	duration: u64,
}

impl Timer {
	pub fn new(duration: u64) -> Self {
		Self {
			now: Instant::now(),
			duration,
		}
	}

	pub fn restart(&mut self) -> Instant {
		self.now = Instant::now();
		self.now
	}

	pub fn elapsed(&self) -> Duration {
		self.now.elapsed()
	}

	pub fn expired(&self) -> bool {
		self.elapsed().as_secs() > self.duration
	}

	pub fn duration(&self) -> u64 {
		self.duration
	}
}

pub mod decoding {
	use frame_system_rpc_runtime_api::events::RuntimeEvent;

	use super::*;

	#[derive(codec::Decode)]
	struct DataSubmitted {
		pub who: AccountId32,
		pub data_hash: H256,
	}

	#[derive(codec::Decode)]
	struct MultisigExecuted {
		pub multisig: AccountId32,
		pub call_hash: H256,
		pub result: bool,
	}

	pub fn parse_decoded_event(ev: &RuntimeEvent) -> Option<DecodedEventData> {
		use codec::{decode_from_bytes, DecodeAll};
		use frame_system_rpc_runtime_api::events::event_id::*;

		let Some(decoded) = &ev.decoded else {
			return None;
		};

		let (pallet_id, event_id) = ev.emitted_index;

		match pallet_id {
			system::PALLET_ID => {
				if event_id == system::EXTRINSIC_SUCCESS {
					return Some(DecodedEventData::SystemExtrinsicSuccess);
				} else if event_id == system::EXTRINSIC_FAILED {
					return Some(DecodedEventData::SystemExtrinsicFailed);
				}
			},
			sudo::PALLET_ID => {
				if event_id == sudo::SUDID {
					let data = decode_from_bytes::<bool>(decoded.clone().into()).ok()?;
					return Some(DecodedEventData::SudoSudid(data));
				} else if event_id == sudo::SUDO_AS_DONE {
					let data = decode_from_bytes::<bool>(decoded.clone().into()).ok()?;
					return Some(DecodedEventData::SudoSudoAsDone(data));
				}
			},
			data_availability::PALLET_ID => {
				if event_id == data_availability::DATA_SUBMITTED {
					let value = DataSubmitted::decode_all(&mut decoded.as_slice()).ok()?;
					let data = block_rpc::common::events::DataSubmitted {
						who: std::format!("{}", value.who),
						data_hash: std::format!("{:?}", value.data_hash),
					};

					return Some(DecodedEventData::DataAvailabilityDataSubmitted(data));
				}
			},
			multisig::PALLET_ID => {
				if event_id == multisig::MULTISIG_EXECUTED {
					let data = MultisigExecuted::decode_all(&mut decoded.as_slice()).ok()?;
					let data = block_rpc::common::events::MultisigExecuted {
						multisig: std::format!("{}", data.multisig),
						call_hash: std::format!("{:?}", data.call_hash),
						result: data.result,
					};
					return Some(DecodedEventData::MultisigMultisigExecuted(data));
				}
			},
			proxy::PALLET_ID => {
				if event_id == proxy::PROXY_EXECUTED {
					let data = decode_from_bytes::<bool>(decoded.clone().into()).ok()?;
					return Some(DecodedEventData::ProxyProxyExecuted(data));
				}
			},
			_ => (),
		}

		None
	}
}

pub mod events {
	use frame_system_rpc_runtime_api::events::{RuntimeEntryEvents, RuntimeEvent};

	use super::*;

	#[derive(Debug, Clone)]
	pub struct AllTransactionEvents(pub Vec<TransactionEvents>);

	impl AllTransactionEvents {
		pub fn weight(&self) -> u64 {
			let mut weight = size_of::<Self>() as u64;
			for e in &self.0 {
				weight += e.weight();
			}

			weight
		}

		pub fn consensus_events(&self) -> Vec<TransactionEvents> {
			use frame_system::Phase;

			let events: Vec<TransactionEvents> = self
				.0
				.iter()
				.filter_map(|x| match &x.phase {
					Phase::Finalization | Phase::Initialization => Some(x.clone()),
					_ => None,
				})
				.collect();

			events
		}

		pub fn tx_events(&self, tx_index: u32) -> Option<&TransactionEvents> {
			use frame_system::Phase;

			self.0
				.iter()
				.find(|x| x.phase == Phase::ApplyExtrinsic(tx_index))
		}
	}

	#[derive(Debug, Clone)]
	pub struct TransactionEvents {
		pub phase: frame_system::Phase,
		pub events: Vec<Event>,
	}

	impl From<RuntimeEntryEvents> for TransactionEvents {
		fn from(value: RuntimeEntryEvents) -> Self {
			let events_iter = value.events.into_iter();
			let events = events_iter.map(Event::from).collect();
			TransactionEvents {
				phase: value.phase.clone(),
				events,
			}
		}
	}

	impl TransactionEvents {
		pub fn weight(&self) -> u64 {
			let mut weight: u64 = size_of::<Self>() as u64;
			for e in &self.events {
				weight += e.weight();
			}

			weight as u64
		}
	}

	#[derive(Debug, Clone)]
	pub struct Event {
		pub index: u32,
		// (Pallet Id, Event Id)
		pub emitted_index: (u8, u8),
		pub encoded: String,
		pub decoded: Option<DecodedEventData>,
	}

	impl Event {
		pub fn weight(&self) -> u64 {
			let mut weight: usize = size_of::<Self>();
			weight += self.encoded.len();

			weight as u64
		}

		pub fn to_tx_rpc_event(&self, enable_decoding: bool) -> block_rpc::common::events::Event {
			use block_rpc::common::events::Event;
			let decoded = enable_decoding.then(|| self.decoded.clone()).flatten();
			Event {
				index: self.index,
				emitted_index: self.emitted_index,
				decoded,
			}
		}
	}

	impl From<RuntimeEvent> for Event {
		fn from(value: RuntimeEvent) -> Self {
			let decoded = decoding::parse_decoded_event(&value);
			let encoded = if let Some(enc) = &value.encoded {
				std::format!("0x{}", hex::encode(enc))
			} else {
				String::new()
			};

			Event {
				index: value.index,
				emitted_index: value.emitted_index,
				encoded,
				decoded,
			}
		}
	}
}

use std::{collections::HashMap, hash::Hash};

pub struct CachedValue<K: Hash + Eq, V: Clone> {
	value: HashMap<K, V>,
	current_weight: u64,
	max_weight: u64,
	calculate_weight: Box<dyn Fn(&V) -> u64 + Send + Sync + 'static>,
}

impl<K: Hash + Eq, V: Clone> CachedValue<K, V> {
	pub fn new(
		max_weight: u64,
		calculate_weight: Box<dyn Fn(&V) -> u64 + Send + Sync + 'static>,
	) -> Self {
		Self {
			value: HashMap::new(),
			current_weight: 0,
			max_weight,
			calculate_weight,
		}
	}

	pub fn insert(&mut self, key: K, value: &V) {
		let weight = (self.calculate_weight)(value);

		if weight > self.max_weight {
			return;
		}

		if (weight + self.current_weight) > self.max_weight {
			self.value.clear();
			self.current_weight = 0;
		}

		self.current_weight += weight;
		self.value.insert(key, value.clone());
	}

	pub fn get(&self, key: &K) -> Option<&V> {
		self.value.get(key)
	}
}

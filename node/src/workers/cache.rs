use frame_system_rpc_runtime_api::events::RuntimeEvent;
use std::{collections::HashMap, hash::Hash};
use transaction_rpc::common::DecodedEventData;

use super::common::decoding;

pub(crate) struct CachedValue<K: Hash + Eq, V: Clone> {
	value: HashMap<K, V>,
	current_weight: u64,
	max_weight: u64,
	calculate_weight: Box<dyn Fn(&V) -> u64 + Send + Sync + 'static>,
	max_allowed_weight_per_item: u64,
}

impl<K: Hash + Eq, V: Clone> CachedValue<K, V> {
	pub fn new(
		max_weight: u64,
		calculate_weight: Box<dyn Fn(&V) -> u64 + Send + Sync + 'static>,
		max_allowed_weight_per_item: u64,
	) -> Self {
		Self {
			value: HashMap::new(),
			current_weight: 0,
			max_weight,
			calculate_weight,
			max_allowed_weight_per_item,
		}
	}

	pub fn insert(&mut self, key: K, value: &V) {
		let weight = (self.calculate_weight)(value);

		if weight > self.max_weight {
			return;
		}

		if weight > self.max_allowed_weight_per_item {
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

#[derive(Debug, Clone)]
pub struct CachedEvents(pub Vec<CachedEntryEvents>);

impl CachedEvents {
	pub fn weight(&self) -> u64 {
		let mut weight = size_of::<Self>() as u64;
		for e in &self.0 {
			weight += e.weight();
		}

		weight
	}

	pub fn consensus_events(&self) -> Vec<CachedEntryEvents> {
		use frame_system::Phase;

		let events: Vec<CachedEntryEvents> = self
			.0
			.iter()
			.filter_map(|x| match &x.phase {
				Phase::Finalization | Phase::Initialization => Some(x.clone()),
				_ => None,
			})
			.collect();

		events
	}

	pub fn tx_events(&self, tx_index: u32) -> Option<&CachedEntryEvents> {
		use frame_system::Phase;

		self.0
			.iter()
			.find(|x| x.phase == Phase::ApplyExtrinsic(tx_index))
	}
}

#[derive(Debug, Clone)]
pub struct CachedEntryEvents {
	pub phase: frame_system::Phase,
	pub events: Vec<CachedEvent>,
}

impl CachedEntryEvents {
	pub fn weight(&self) -> u64 {
		let mut weight: u64 = size_of::<Self>() as u64;
		for e in &self.events {
			weight += e.weight();
		}

		weight as u64
	}
}

#[derive(Debug, Clone)]
pub struct CachedEvent {
	pub index: u32,
	// (Pallet Id, Event Id)
	pub emitted_index: (u8, u8),
	pub encoded: String,
	pub decoded: Option<DecodedEventData>,
}

impl CachedEvent {
	pub fn weight(&self) -> u64 {
		let mut weight: usize = size_of::<Self>();
		weight += self.encoded.len();

		weight as u64
	}

	pub fn from_runtime_event(ev: &RuntimeEvent) -> Self {
		let decoded = decoding::parse_decoded_event(ev);
		let encoded = if let Some(enc) = &ev.encoded {
			std::format!("0x{}", hex::encode(enc))
		} else {
			String::new()
		};

		CachedEvent {
			index: ev.index,
			emitted_index: ev.emitted_index,
			encoded,
			decoded,
		}
	}

	pub fn to_tx_rpc_event(&self, enable_decoding: bool) -> transaction_rpc::common::events::Event {
		use transaction_rpc::common::events::Event;
		let decoded = enable_decoding.then(|| self.decoded.clone()).flatten();
		Event {
			index: self.index,
			emitted_index: self.emitted_index,
			decoded,
		}
	}
}

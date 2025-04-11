use sp_core::H256;
use std::{
	collections::HashMap,
	hash::Hash,
	sync::{Arc, RwLock},
};
use transaction_rpc::block_overview;

pub(crate) type SharedCache = Arc<RwLock<Cache>>;

pub(crate) struct CachedValue<K: Hash + Eq, V> {
	value: HashMap<K, V>,
	current_weight: u64,
	max_weight: u64,
	calculate_weight: Box<dyn Fn(&V) -> u64 + Send + Sync + 'static>,
	max_allowed_weight_per_item: u64,
}

impl<K: Hash + Eq, V> CachedValue<K, V> {
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

	pub fn insert(&mut self, key: K, value: V) {
		let weight = (self.calculate_weight)(&value);

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
		self.value.insert(key, value);
	}

	pub fn get(&self, key: &K) -> Option<&V> {
		self.value.get(key)
	}
}

pub(crate) struct Cache {
	pub events: CachedValue<H256, Arc<CachedEvents>>,
	pub tx_hash: CachedValue<(H256, u32), H256>,
	// hex and scale encoded call
	pub calls: CachedValue<(H256, u32), String>,
}

impl Cache {
	pub fn new() -> Self {
		const TEMP_WEIGHT: u64 = 100_000_000;

		let weight_calc = Box::new(|x: &Arc<CachedEvents>| x.weight());
		let events =
			CachedValue::<H256, Arc<CachedEvents>>::new(TEMP_WEIGHT, weight_calc, TEMP_WEIGHT);

		let weight_calc = Box::new(|_x: &H256| size_of::<H256>() as u64);
		let tx_hash = CachedValue::<(H256, u32), H256>::new(TEMP_WEIGHT, weight_calc, TEMP_WEIGHT);

		let weight_calc = Box::new(|x: &String| (x.len() + size_of::<String>()) as u64);
		let calls = CachedValue::<(H256, u32), String>::new(TEMP_WEIGHT, weight_calc, TEMP_WEIGHT);

		Self {
			events,
			tx_hash,
			calls,
		}
	}
}

#[derive(Clone)]
pub struct CachedEvents(pub Vec<CachedEvent>);

impl CachedEvents {
	pub fn weight(&self) -> u64 {
		let mut weight = size_of::<Self>() as u64;
		for e in &self.0 {
			weight += e.weight();
		}

		weight
	}

	pub fn consensus_events(&self) -> Option<&CachedEvent> {
		use frame_system::Phase;

		for cached_event in self.0.iter() {
			match &cached_event.phase {
				Phase::Finalization => (),
				Phase::Initialization => (),
				_ => continue,
			};

			return Some(cached_event);
		}

		None
	}
}

#[derive(Clone)]
pub struct CachedEvent {
	pub phase: frame_system::Phase,
	pub events: Vec<CachedEventData>,
}

impl CachedEvent {
	pub fn weight(&self) -> u64 {
		let mut weight: u64 = size_of::<Self>() as u64;
		for e in &self.events {
			weight += e.weight();
		}

		weight as u64
	}
}

#[derive(Clone)]
pub struct CachedEventData {
	pub index: u32,
	pub pallet_id: u8,
	pub event_id: u8,
	pub encoded: String,
	pub decoded: Option<block_overview::DecodedEventData>,
}

impl CachedEventData {
	pub fn weight(&self) -> u64 {
		let mut weight: usize = size_of::<Self>();
		weight += self.encoded.len();

		weight as u64
	}
}

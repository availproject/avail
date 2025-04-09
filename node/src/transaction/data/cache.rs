use super::RPCEvents;
use sp_core::H256;
use std::{collections::HashMap, hash::Hash};

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

		if self.max_allowed_weight_per_item > weight {
			return;
		}

		if (weight + self.current_weight) > self.max_weight {
			self.value.clear();
			self.current_weight = 0;
		}

		self.value.insert(key, value);
	}

	pub fn get(&self, key: &K) -> Option<&V> {
		self.value.get(key)
	}
}

pub(crate) struct Cache {
	pub events: CachedValue<H256, RPCEvents>,
	pub tx_hash: CachedValue<(H256, u32), H256>,
	// scale encoded call
	pub encoded_call: CachedValue<(H256, u32), Vec<u8>>,
}

impl Cache {
	pub fn new() -> Self {
		const TEMP_WEIGHT: u64 = 100_000_000;

		let weight_calc = Box::new(rpc_events_weight_calculation);
		let events = CachedValue::<H256, RPCEvents>::new(TEMP_WEIGHT, weight_calc, TEMP_WEIGHT);

		let weight_calc = Box::new(|_x: &H256| size_of::<H256>() as u64);
		let tx_hash = CachedValue::<(H256, u32), H256>::new(TEMP_WEIGHT, weight_calc, TEMP_WEIGHT);

		let weight_calc = Box::new(|x: &Vec<u8>| x.len() as u64);
		let encoded_call =
			CachedValue::<(H256, u32), Vec<u8>>::new(TEMP_WEIGHT, weight_calc, TEMP_WEIGHT);

		Self {
			events,
			tx_hash,
			encoded_call,
		}
	}
}

fn rpc_events_weight_calculation(v: &RPCEvents) -> u64 {
	let mut weight = size_of::<RPCEvents>();
	for e in v {
		weight += e.weight() as usize;
	}

	weight as u64
}

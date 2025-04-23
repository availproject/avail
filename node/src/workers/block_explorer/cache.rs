use crate::workers::{
	cache::{CachedEvents, CachedValue},
	common::UniqueTxId,
};
use sp_core::H256;
use std::sync::{Arc, RwLock};

pub(super) type SharedCache = Arc<RwLock<Cache>>;
pub(super) struct Cache {
	pub events: CachedValue<H256, Arc<CachedEvents>>,
	pub tx_hash: CachedValue<UniqueTxId, H256>,
	// hex and scale encoded call
	pub calls: CachedValue<UniqueTxId, String>,
}

impl Cache {
	pub fn new() -> Self {
		const TEMP_WEIGHT: u64 = 100_000_000;

		let weight_calc = Box::new(|x: &Arc<CachedEvents>| x.weight());
		let events =
			CachedValue::<H256, Arc<CachedEvents>>::new(TEMP_WEIGHT, weight_calc, TEMP_WEIGHT);

		let weight_calc = Box::new(|_x: &H256| size_of::<H256>() as u64);
		let tx_hash = CachedValue::<UniqueTxId, H256>::new(TEMP_WEIGHT, weight_calc, TEMP_WEIGHT);

		let weight_calc = Box::new(|x: &String| (x.len() + size_of::<String>()) as u64);
		let calls = CachedValue::<UniqueTxId, String>::new(TEMP_WEIGHT, weight_calc, TEMP_WEIGHT);

		Self {
			events,
			tx_hash,
			calls,
		}
	}
}

pub(super) trait Cacheable {
	fn read_cached_tx_hash(&self, key: &UniqueTxId) -> Option<H256>;
	fn write_cached_tx_hash(&self, key: UniqueTxId, value: H256) -> Option<()>;
	fn read_cached_calls(&self, key: &UniqueTxId) -> Option<String>;
	fn write_cached_calls(&self, key: UniqueTxId, value: String) -> Option<()>;
	fn read_cached_events(&self, key: &H256) -> Option<Arc<CachedEvents>>;
	fn write_cached_events(&self, key: H256, value: Arc<CachedEvents>) -> Option<()>;
}

impl Cacheable for SharedCache {
	fn read_cached_tx_hash(&self, key: &UniqueTxId) -> Option<H256> {
		let lock = self.read().ok()?;
		lock.tx_hash.get(key).cloned()
	}

	fn write_cached_tx_hash(&self, key: UniqueTxId, value: H256) -> Option<()> {
		let mut lock = self.write().ok()?;
		lock.tx_hash.insert(key, value);
		Some(())
	}

	fn read_cached_calls(&self, key: &UniqueTxId) -> Option<String> {
		let lock = self.read().ok()?;
		lock.calls.get(key).cloned()
	}

	fn write_cached_calls(&self, key: UniqueTxId, value: String) -> Option<()> {
		let mut lock = self.write().ok()?;
		lock.calls.insert(key, value);
		Some(())
	}

	fn read_cached_events(&self, key: &H256) -> Option<Arc<CachedEvents>> {
		let lock = self.read().ok()?;
		lock.events.get(key).cloned()
	}

	fn write_cached_events(&self, key: H256, value: Arc<CachedEvents>) -> Option<()> {
		let mut lock = self.write().ok()?;
		lock.events.insert(key, value);
		Some(())
	}
}

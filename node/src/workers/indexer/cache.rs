use crate::workers::{CachedValue, TransactionEvents, TxIdentifier};
use std::sync::{Arc, RwLock};

pub type SharedCache = Arc<RwLock<Cache>>;
pub struct Cache {
	pub events: CachedValue<TxIdentifier, TransactionEvents>,
}

impl Cache {
	pub fn new(event_cache_size: u64) -> Self {
		let events_weight = event_cache_size * 1000;

		let weight_calc = Box::new(|x: &TransactionEvents| x.weight());
		let events =
			CachedValue::<TxIdentifier, TransactionEvents>::new(events_weight, weight_calc);

		Self { events }
	}
}

pub trait Cacheable {
	fn read_cached_events(&self, key: &TxIdentifier) -> Option<TransactionEvents>;
	fn write_cached_events(&self, key: TxIdentifier, value: &TransactionEvents) -> Option<()>;
}

impl Cacheable for SharedCache {
	fn read_cached_events(&self, key: &TxIdentifier) -> Option<TransactionEvents> {
		let lock = self.read().ok()?;
		lock.events.get(key).cloned()
	}

	fn write_cached_events(&self, key: TxIdentifier, value: &TransactionEvents) -> Option<()> {
		let mut lock = self.write().ok()?;
		lock.events.insert(key, value);
		Some(())
	}
}

use crate::workers::{
	cache::{CachedEntryEvents, CachedValue},
	common::UniqueTxId,
};
use std::sync::{Arc, RwLock};

pub(super) type SharedCache = Arc<RwLock<Cache>>;
pub(super) struct Cache {
	pub events: CachedValue<UniqueTxId, CachedEntryEvents>,
}

impl Cache {
	pub fn new(event_cache_size: u64) -> Self {
		let event_cache_size = event_cache_size * 1000;

		let weight_calc = Box::new(|x: &CachedEntryEvents| x.weight());
		let events = CachedValue::<UniqueTxId, CachedEntryEvents>::new(
			event_cache_size,
			weight_calc,
			event_cache_size,
		);

		Self { events }
	}
}

pub(super) trait Cacheable {
	fn read_cached_events(&self, key: &UniqueTxId) -> Option<CachedEntryEvents>;
	fn write_cached_events(&self, key: UniqueTxId, value: CachedEntryEvents) -> Option<()>;
}

impl Cacheable for SharedCache {
	fn read_cached_events(&self, key: &UniqueTxId) -> Option<CachedEntryEvents> {
		let lock = self.read().ok()?;
		lock.events.get(key).cloned()
	}

	fn write_cached_events(&self, key: UniqueTxId, value: CachedEntryEvents) -> Option<()> {
		let mut lock = self.write().ok()?;
		lock.events.insert(key, value);
		Some(())
	}
}

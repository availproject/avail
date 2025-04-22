use crate::workers::{
	cache::{CachedEvents, CachedValue},
	common::UniqueTxId,
};
use std::sync::{Arc, RwLock};

pub(crate) type SharedCache = Arc<RwLock<Cache>>;
pub(crate) struct Cache {
	pub events: CachedValue<UniqueTxId, Arc<CachedEvents>>,
}

impl Cache {
	pub fn new() -> Self {
		// 10 MB
		const TEMP_WEIGHT: u64 = 10_000_000;

		let weight_calc = Box::new(|x: &Arc<CachedEvents>| x.weight());
		let events = CachedValue::<UniqueTxId, Arc<CachedEvents>>::new(
			TEMP_WEIGHT,
			weight_calc,
			TEMP_WEIGHT,
		);

		Self { events }
	}
}

pub trait Cacheable {
	fn read_cached_events(&self, key: &UniqueTxId) -> Option<Arc<CachedEvents>>;
	fn write_cached_events(&self, key: UniqueTxId, value: Arc<CachedEvents>) -> Option<()>;
}

impl Cacheable for SharedCache {
	fn read_cached_events(&self, key: &UniqueTxId) -> Option<Arc<CachedEvents>> {
		let Ok(lock) = self.read() else {
			return None;
		};

		lock.events.get(key).map(|x| x.clone())
	}

	fn write_cached_events(&self, key: UniqueTxId, value: Arc<CachedEvents>) -> Option<()> {
		let Ok(mut lock) = self.write() else {
			return None;
		};

		lock.events.insert(key, value);

		Some(())
	}
}

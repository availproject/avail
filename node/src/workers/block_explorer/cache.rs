use super::Deps;
use crate::workers::{AllTransactionEvents, CachedValue, TxIdentifier};
use sp_core::H256;
use std::sync::{Arc, RwLock};

pub type SharedCache = Arc<RwLock<Cache>>;
pub struct Cache {
	pub events: CachedValue<H256, Arc<AllTransactionEvents>>,
	pub tx_hash: CachedValue<TxIdentifier, H256>,
	// hex and scale encoded call
	pub calls: CachedValue<TxIdentifier, String>,
}

impl Cache {
	pub fn new(deps: &Deps) -> Self {
		let events_weight: u64 = deps.cli.event_cache_size * 1000;
		let tx_hash_weight: u64 = deps.cli.tx_hash_cache_size * 1000;
		let call_weight: u64 = deps.cli.call_cache_size * 1000;

		let weight_calc = Box::new(|x: &Arc<AllTransactionEvents>| x.weight());
		let events =
			CachedValue::<H256, Arc<AllTransactionEvents>>::new(events_weight, weight_calc);

		let weight_calc = Box::new(|_x: &H256| size_of::<H256>() as u64);
		let tx_hash = CachedValue::<TxIdentifier, H256>::new(tx_hash_weight, weight_calc);

		let weight_calc = Box::new(|x: &String| (x.len() + size_of::<String>()) as u64);
		let calls = CachedValue::<TxIdentifier, String>::new(call_weight, weight_calc);

		Self {
			events,
			tx_hash,
			calls,
		}
	}
}

pub trait Cacheable {
	fn read_cached_tx_hash(&self, key: &TxIdentifier) -> Option<H256>;
	fn write_cached_tx_hash(&self, key: TxIdentifier, value: &H256) -> Option<()>;
	fn read_cached_calls(&self, key: &TxIdentifier) -> Option<String>;
	fn write_cached_calls(&self, key: TxIdentifier, value: &String) -> Option<()>;
	fn read_cached_events(&self, key: &H256) -> Option<Arc<AllTransactionEvents>>;
	fn write_cached_events(&self, key: H256, value: &Arc<AllTransactionEvents>) -> Option<()>;
}

impl Cacheable for SharedCache {
	fn read_cached_tx_hash(&self, key: &TxIdentifier) -> Option<H256> {
		let lock = self.read().ok()?;
		lock.tx_hash.get(key).cloned()
	}

	fn write_cached_tx_hash(&self, key: TxIdentifier, value: &H256) -> Option<()> {
		let mut lock = self.write().ok()?;
		lock.tx_hash.insert(key, value);
		Some(())
	}

	fn read_cached_calls(&self, key: &TxIdentifier) -> Option<String> {
		let lock = self.read().ok()?;
		lock.calls.get(key).cloned()
	}

	fn write_cached_calls(&self, key: TxIdentifier, value: &String) -> Option<()> {
		let mut lock = self.write().ok()?;
		lock.calls.insert(key, value);
		Some(())
	}

	fn read_cached_events(&self, key: &H256) -> Option<Arc<AllTransactionEvents>> {
		let lock = self.read().ok()?;
		lock.events.get(key).cloned()
	}

	fn write_cached_events(&self, key: H256, value: &Arc<AllTransactionEvents>) -> Option<()> {
		let mut lock = self.write().ok()?;
		lock.events.insert(key, value);
		Some(())
	}
}

/*
use sp_externalities::decl_extension;
use sc_client_api::AuxStore;
use sp_std::sync::Arc;


decl_extension! {
	pub struct AuxStoreExt(Arc<dyn AuxStore + 'static + Sync + Send>);
}

impl AuxStoreExt {
	fn insert_aux(&self, key: &[u8], value: &[u8]) {
		let _ = self.0.insert_aux(key, [value]);
	}

	fn get_aux(&self, key: &[u8]) -> Option<Vec<u8>> {
		self.0.get_aux(key).ok().flatten()
	}
}
*/
use codec::{Decode, Encode};
use sp_runtime_interface::runtime_interface;
use sp_std::{collections::btree_map::BTreeMap, vec::Vec};

#[cfg(feature = "std")]
use sp_std::sync::{OnceLock, RwLock};

pub type StorageMap = BTreeMap<Vec<u8>, Vec<u8>>;

#[cfg(feature = "std")]
pub(crate) mod native {
	use super::*;

	static MEM_TMP_STORAGE: OnceLock<RwLock<StorageMap>> = OnceLock::new();

	pub(crate) fn memory_tmp_storage() -> &'static RwLock<StorageMap> {
		MEM_TMP_STORAGE.get_or_init(|| RwLock::new(StorageMap::new()))
	}
}

fn log_poisoned_sync<E>(_: E) {
	log::error!("Memory Temporal Storage with a poisoned sync");
}

pub fn mts_get<T: Decode>(key: &[u8]) -> Option<T> {
	hosted_mem_tmp_storage::get(key).and_then(|raw| T::decode(&mut raw.as_slice()).ok())
}

pub fn mts_insert<T: Encode + Decode>(key: Vec<u8>, value: T) -> Option<T> {
	let raw_value = value.encode();
	hosted_mem_tmp_storage::insert(key, raw_value)
		.and_then(|raw| T::decode(&mut raw.as_slice()).ok())
}

pub fn mts_update<T, F>(key: Vec<u8>, f: F) -> Option<T>
where
	T: Encode + Decode + Default,
	F: FnOnce(&mut T),
{
	let mut value = mts_get(&key).unwrap_or_default();
	f(&mut value);
	mts_insert(key, value)
}

pub fn mts_clear() {
	hosted_mem_tmp_storage::clear();
}

pub fn mts_storage() -> StorageMap {
	StorageMap::from_iter(hosted_mem_tmp_storage::storage().into_iter())
}

#[runtime_interface]
pub trait HostedMemTmpStorage {
	fn insert(key: Vec<u8>, value: Vec<u8>) -> Option<Vec<u8>> {
		native::memory_tmp_storage()
			.write()
			.map_err(log_poisoned_sync)
			.ok()
			.and_then(|mut guard| guard.insert(key, value))
	}

	fn get(key: &[u8]) -> Option<Vec<u8>> {
		native::memory_tmp_storage()
			.read()
			.map_err(log_poisoned_sync)
			.ok()
			.and_then(|guard| guard.get(key).cloned())
	}

	fn clear() {
		if let Ok(mut guard) = native::memory_tmp_storage()
			.write()
			.map_err(log_poisoned_sync)
		{
			guard.clear();
		}
	}

	fn storage() -> Vec<(Vec<u8>, Vec<u8>)> {
		native::memory_tmp_storage()
			.read()
			.map_err(log_poisoned_sync)
			.map(|guard| guard.iter().map(|(k, v)| (k.clone(), v.clone())).collect())
			.unwrap_or_default()
	}
}

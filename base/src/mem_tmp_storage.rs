use codec::{Decode, Encode};
use sp_runtime_interface::runtime_interface;
use sp_std::{collections::btree_map::BTreeMap, vec::Vec};

/// A simple key-value storage in memory.
pub type StorageMap = BTreeMap<Vec<u8>, Vec<u8>>;

#[cfg(feature = "std")]
pub(crate) mod native {
	use super::*;
	use sp_std::sync::{OnceLock, RwLock};

	static MEM_TMP_STORAGE: OnceLock<RwLock<StorageMap>> = OnceLock::new();

	pub(crate) fn memory_tmp_storage() -> &'static RwLock<StorageMap> {
		MEM_TMP_STORAGE.get_or_init(|| RwLock::new(StorageMap::new()))
	}
}

/// Returns the value under `key` from the memory temporal storage.
pub fn mts_get<T: Decode>(key: &[u8]) -> Option<T> {
	hosted_mem_tmp_storage::get(key).and_then(|raw| T::decode(&mut raw.as_slice()).ok())
}

/// Encodes and inserts `value` into the memory temporal storage under `key`.
pub fn mts_insert<T: Encode + Decode>(key: Vec<u8>, value: T) -> Option<T> {
	let raw_value = value.encode();
	hosted_mem_tmp_storage::insert(key, raw_value)
		.and_then(|raw| T::decode(&mut raw.as_slice()).ok())
}

/// Updates the value under `key` in the memory temporal storage.
pub fn mts_update<T, F>(key: Vec<u8>, f: F) -> Option<T>
where
	T: Encode + Decode + Default,
	F: FnOnce(&mut T),
{
	let mut value = mts_get(&key).unwrap_or_default();
	f(&mut value);
	mts_insert(key, value)
}

/// Clears the memory temporal storage.
pub fn mts_clear() {
	hosted_mem_tmp_storage::clear();
}

/// Returns the content of the memory temporal storage.
pub fn mts_storage() -> StorageMap {
	hosted_mem_tmp_storage::storage().into_iter().collect()
}

/// The memory temporal storage will be cleared at the begining of each block building.
///
/// This is a simple global database not aware of forks. Can be used for storing auxiliary information like/// the failed `Vector::SendMessage` transaction indexers.
///
/// # TODO
/// - [ ] Improve error handling of poisoned sync: Panic?
#[runtime_interface]
pub trait HostedMemTmpStorage {
	/// Insert auxiliary data into key-value storage.
	fn insert(key: Vec<u8>, value: Vec<u8>) -> Option<Vec<u8>> {
		native::memory_tmp_storage()
			.write()
			.map_err(log_poisoned_sync)
			.ok()
			.and_then(|mut guard| guard.insert(key, value))
	}

	/// Returns the value under `key` from the memory temporal storage.
	fn get(key: &[u8]) -> Option<Vec<u8>> {
		native::memory_tmp_storage()
			.read()
			.map_err(log_poisoned_sync)
			.ok()
			.and_then(|guard| guard.get(key).cloned())
	}

	/// Clears the memory temporal storage.
	fn clear() {
		if let Ok(mut guard) = native::memory_tmp_storage()
			.write()
			.map_err(log_poisoned_sync)
		{
			guard.clear();
		}
	}

	/// Returns the content of the memory temporal storage as a list of key-value pairs.
	/// NOTE: Conversion to plain list is needed due to `ByPass` constraints.
	fn storage() -> Vec<(Vec<u8>, Vec<u8>)> {
		native::memory_tmp_storage()
			.read()
			.map_err(log_poisoned_sync)
			.map(|guard| guard.iter().map(|(k, v)| (k.clone(), v.clone())).collect())
			.unwrap_or_default()
	}
}

fn log_poisoned_sync<E>(_: E) {
	log::error!("Memory Temporal Storage with a poisoned sync");
}

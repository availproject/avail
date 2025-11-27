use codec::{Decode, Encode};
use sp_runtime_interface::{
	pass_by::{AllocateAndReturnByCodec, PassFatPointerAndRead},
	runtime_interface,
};
use sp_std::{collections::btree_map::BTreeMap, vec::Vec};

/// A simple key-value storage in memory.
pub type StorageMap = BTreeMap<Vec<u8>, Vec<u8>>;

//////
////// Runtime Code
//////

/// TODO docs
pub struct MemoryTemporaryStorage;
impl MemoryTemporaryStorage {
	/// Returns the value under `key` from the memory temporal storage.
	pub fn get<T: Decode>(key: &[u8]) -> Option<T> {
		hosted_mem_tmp_storage::get(key).and_then(|raw| T::decode(&mut raw.as_slice()).ok())
	}

	/// Encodes and inserts `value` into the memory temporal storage under `key`.
	pub fn insert<T: Encode + Decode>(key: Vec<u8>, value: T) -> Option<T> {
		let raw_value = value.encode();
		hosted_mem_tmp_storage::insert(&key, raw_value)
			.and_then(|raw| T::decode(&mut raw.as_slice()).ok())
	}

	/// TODO docs
	pub fn remove(key: &[u8]) -> bool {
		hosted_mem_tmp_storage::take(key).is_some()
	}

	/// TODO docs
	pub fn take<T: Decode>(key: &[u8]) -> Option<T> {
		hosted_mem_tmp_storage::take(key).and_then(|raw| T::decode(&mut raw.as_slice()).ok())
	}

	/// Updates the value under `key` in the memory temporal storage.
	pub fn update<T, F>(key: Vec<u8>, f: F) -> Option<T>
	where
		T: Encode + Decode + Default,
		F: FnOnce(&mut T),
	{
		let mut value = Self::get(&key).unwrap_or_default();
		f(&mut value);
		Self::insert(key, value)
	}

	/// Clears the memory temporal storage.
	pub fn clear() {
		hosted_mem_tmp_storage::clear();
	}

	/// Returns the content of the memory temporal storage.
	pub fn storage() -> StorageMap {
		hosted_mem_tmp_storage::storage().into_iter().collect()
	}
}

//////
////// Native Code
//////

#[cfg(not(substrate_runtime))]
pub(crate) mod native {
	use super::*;
	use parking_lot::RwLock;

	pub static MEM_TMP_STORAGE: RwLock<StorageMap> = RwLock::new(StorageMap::new());
}

/// The memory temporal storage will be cleared at the begining of each block building.
///
/// This is a simple global database not aware of forks. Can be used for storing auxiliary information like/// the failed `Vector::SendMessage` transaction indexers.
///
/// # TODO
/// - [ ] Improve error handling of poisoned sync: Panic?
#[runtime_interface]
pub trait HostedMemTmpStorage {
	fn insert(
		key: PassFatPointerAndRead<&[u8]>,
		value: PassFatPointerAndRead<Vec<u8>>,
	) -> AllocateAndReturnByCodec<Option<Vec<u8>>> {
		let mut guard = native::MEM_TMP_STORAGE.write();
		guard.insert(key.to_vec(), value)
	}

	fn get(key: PassFatPointerAndRead<&[u8]>) -> AllocateAndReturnByCodec<Option<Vec<u8>>> {
		let guard = native::MEM_TMP_STORAGE.read();
		guard.get(&*key).cloned()
	}

	fn take(key: PassFatPointerAndRead<&[u8]>) -> AllocateAndReturnByCodec<Option<Vec<u8>>> {
		let mut guard = native::MEM_TMP_STORAGE.write();
		guard.remove(&*key)
	}

	/// Clears the memory temporal storage.
	fn clear() {
		let mut guard = native::MEM_TMP_STORAGE.write();
		guard.clear();
	}

	/// Returns the content of the memory temporal storage as a list of key-value pairs.
	/// NOTE: Conversion to plain list is needed due to `ByPass` constraints.
	fn storage() -> AllocateAndReturnByCodec<Vec<(Vec<u8>, Vec<u8>)>> {
		let guard = native::MEM_TMP_STORAGE.read();
		guard.iter().map(|(k, v)| (k.clone(), v.clone())).collect()
	}
}

// #[cfg(not(substrate_runtime))]
// fn log_poisoned_sync() {
// 	log::error!("Memory Temporal Storage with a poisoned sync");
// }

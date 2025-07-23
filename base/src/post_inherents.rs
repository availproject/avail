use crate::{MemoryTemporaryStorage, StorageMap};

use sp_api::decl_runtime_apis;
use sp_authority_discovery::AuthorityId;
use sp_runtime::traits::Block as BlockT;
use sp_std::{collections::btree_map::BTreeMap, vec::Vec};

/// A pallet that provides or verifies an inherent extrinsic will implement this trait.
///
/// The pallet may provide an inherent, verify an inherent, or both provide and verify.
///
/// Briefly, inherent extrinsics ("inherents") are extrinsics that are added to a block by the block
/// producer. See [`sp_inherents`] for more documentation on inherents.
pub trait ProvidePostInherent {
	/// The call type of the pallet.
	type Call;
	/// The error returned by `check_inherent`.
	type Error: codec::Encode;

	/// Create an inherent out of the given `InherentData`.
	///
	/// NOTE: All checks necessary to ensure that the inherent is correct and that can be done in
	/// the runtime should happen in the returned `Call`.
	/// E.g. if this provides the timestamp, the call will check that the given timestamp is
	/// increasing the old timestamp by more than a minimum and it will also check that the
	/// timestamp hasn't already been set in the current block.
	fn create_inherent(data: &StorageMap) -> Option<Self::Call>;

	/// Check whether the given inherent is valid. Checking the inherent is optional and can be
	/// omitted by using the default implementation.
	///
	/// When checking an inherent, the first parameter represents the inherent that is actually
	/// included in the block by its author. Whereas the second parameter represents the inherent
	/// data that the verifying node calculates.
	///
	/// This is intended to allow for checks that cannot be done within the runtime such as, e.g.,
	/// the timestamp.
	///
	/// # Warning
	///
	/// This check is not guaranteed to be run by all full nodes and cannot be relied upon for
	/// ensuring that the block is correct.
	fn check_inherent(_: &Self::Call) -> Result<(), Self::Error> {
		Ok(())
	}

	/// Return whether the call is an inherent call.
	///
	/// NOTE: Signed extrinsics are not inherents, but a signed extrinsic with the given call
	/// variant can be dispatched.
	///
	/// # Warning
	///
	/// In FRAME, inherents are enforced to be executed before other extrinsics. For this reason,
	/// pallets with unsigned transactions **must ensure** that no unsigned transaction call
	/// is an inherent call, when implementing `ValidateUnsigned::validate_unsigned`.
	/// Otherwise block producers can produce invalid blocks by including them after non inherents.
	fn is_inherent(call: &Self::Call) -> bool;
}

decl_runtime_apis! {
	#[core_trait]
	#[allow(unused_imports)]
	pub trait PostInherentsProvider {
		fn create_post_inherent_extrinsics(data: StorageMap, blob_txs_summary: Vec<(
			sp_core::H256,
			bool,
			Option<String>,
			BTreeMap<u16, Vec<(AuthorityId, String, Vec<u8>)>>,
		)>, total_blob_size: u64) -> Vec<<Block as BlockT>::Extrinsic>;
	}
}

pub trait PostInherentsBackend {
	fn init_post_inherent_data(&self);
	fn post_inherent_data(&self) -> StorageMap;
}

/// Client API for post-inherents.
impl<T> PostInherentsBackend for T {
	fn init_post_inherent_data(&self) {
		MemoryTemporaryStorage::clear();
	}
	fn post_inherent_data(&self) -> StorageMap {
		MemoryTemporaryStorage::storage()
	}
}

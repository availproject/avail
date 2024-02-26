#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode, MaxEncodedLen};
use hex_literal::hex;
use scale_info::TypeInfo;
use sp_api::decl_runtime_apis;
use sp_core::H256;
use sp_runtime::traits::Block as BlockT;
use sp_std::vec::Vec;

#[cfg(feature = "std")]
pub mod metrics;

mod provide_post_inherent;
pub use provide_post_inherent::ProvidePostInherent;

pub mod aux_store_ext;
pub use aux_store_ext::{mts_clear, mts_get, mts_insert, mts_storage, mts_update, StorageMap};

decl_runtime_apis! {
	#[core_trait]
	pub trait PostInherentsProvider {
		fn create_post_inherent_extrinsics(data: StorageMap) -> Vec<<Block as BlockT>::Extrinsic>;
	}
}

const POST_INH_AUX_ID: &[u8] = b"post_inherent_data";

pub trait PostInherentsBackend {
	fn init_post_inherent_data(&self);
	fn post_inherent_data(&self) -> StorageMap;
}

impl<T> PostInherentsBackend for T {
	fn init_post_inherent_data(&self) {
		mts_clear();
	}
	fn post_inherent_data(&self) -> StorageMap {
		mts_storage()
	}
}

#[derive(Clone, Encode, Decode, TypeInfo, MaxEncodedLen)]
pub struct MRoots {
	pub data_root: H256,
	pub bridge_root: H256,
}

/// Kecckak_256 of `H256::zero`
pub const KECCAK_OF_ZERO: H256 = H256(hex!(
	"290decd9548b62a8d60345a988386fc84ba6bc95484008f6362f93160ef3e563"
));
impl Default for MRoots {
	fn default() -> Self {
		Self {
			data_root: KECCAK_OF_ZERO,
			bridge_root: KECCAK_OF_ZERO,
		}
	}
}

/*
#[cfg(test)]
mod tests {
	use super::*;
	use sp_io::hashing::keccak_256;

	// Default roots are just keccak_256(H256::zero())
	#[test]
	fn check_default_mroots() {
		let mroots = MRoots::default();
		let kecckak_of_zero: H256 = keccak_256(H256::zero().as_bytes()).into();

		assert_eq!(mroots.data_root, kecckak_of_zero);
		assert_eq!(mroots.bridge_root, kecckak_of_zero)
	}
}*/

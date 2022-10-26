#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::unnecessary_mut_passed)]

use da_primitives::DataProof;
use frame_system::limits::BlockLength;
use sp_core::H256;
use sp_std::vec::Vec;

sp_api::decl_runtime_apis! {
	pub trait KateParamsGetter {
		fn get_public_params() -> Vec<u8>;
		fn get_block_length() -> BlockLength;
		fn get_babe_vrf() -> [u8;32];

		/// Returns the submitted data root.
		fn submitted_data_root() -> H256;

		/// Calculates the data proof of `data_index`.
		/// It will return `None` if `data_index` is out of scope.
		fn submitted_data_proof(data_index: u32) -> Option<DataProof>;
	}
}

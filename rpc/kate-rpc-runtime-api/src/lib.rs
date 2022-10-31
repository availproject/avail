#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::unnecessary_mut_passed)]

use frame_system::limits::BlockLength;
use sp_std::vec::Vec;

sp_api::decl_runtime_apis! {
	pub trait KateParamsGetter {
		fn get_public_params() -> Vec<u8>;
		fn get_block_length() -> BlockLength;
		fn get_babe_vrf() -> [u8;32];
	}
}

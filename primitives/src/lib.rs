#![cfg_attr(not(feature = "std"), no_std)]

use sp_runtime::Perbill;

pub mod header;
pub use header::*;

pub mod kate_extrinsics_root;
pub use kate_extrinsics_root::*;

pub mod traits;

pub mod well_known_keys {
	/// Public params used to generate Kate commitment
	pub const KATE_PUBLIC_PARAMS: &'static [u8] = b":kate_public_params:";

	/// Max block length
	pub const BLOCK_LENGTH: &'static [u8] = b":block_length:";
}

/// We allow `Normal` extrinsics to fill up the block up to 90%, the rest can be used
/// by  Operational  extrinsics.
pub const NORMAL_DISPATCH_RATIO: Perbill = Perbill::from_percent(90);

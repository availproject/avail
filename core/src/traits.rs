use codec::{Codec, Decode};
use sp_arithmetic::traits::{AtLeast32BitUnsigned, Saturating};
use sp_core::U256;
use sp_std::{convert::TryFrom, fmt::Debug, hash::Hash as StdHash};

pub mod get_app_id;
pub use get_app_id::*;

pub mod extended_header;
pub use extended_header::*;

#[cfg(feature = "runtime")]
pub mod extended_block;
#[cfg(feature = "runtime")]
pub use extended_block::*;

/// Header block number trait.
pub trait HeaderBlockNumber:
	AtLeast32BitUnsigned + Codec + StdHash + Copy + Into<U256> + TryFrom<U256> + Debug + Eq + Saturating
{
}

impl<
		T: AtLeast32BitUnsigned
			+ Codec
			+ StdHash
			+ Copy
			+ Into<U256>
			+ TryFrom<U256>
			+ Debug
			+ Eq
			+ Saturating,
	> HeaderBlockNumber for T
{
}

/// Header hash.
#[cfg(feature = "runtime")]
pub trait HeaderHash: sp_runtime::traits::Hash {}
#[cfg(feature = "runtime")]
impl<T: sp_runtime::traits::Hash> HeaderHash for T {}

pub trait HeaderHashOutput: Decode + Ord {}
impl<T: Decode + Ord> HeaderHashOutput for T {}

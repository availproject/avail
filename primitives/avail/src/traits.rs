use codec::{Codec, Decode};
use sp_core::U256;
use sp_runtime::{
	traits::{
		AtLeast32BitUnsigned, Hash as HashT, MaybeDisplay, MaybeFromStr, MaybeSerializeDeserialize,
		Member, SimpleBitOps,
	},
	Digest,
};
use sp_std::{convert::TryFrom, fmt::Debug, hash::Hash as StdHash};

use crate::header::HeaderExtension;

/// Header block number trait.
pub trait HeaderBlockNumber:
	Member
	+ AtLeast32BitUnsigned
	+ Codec
	+ MaybeSerializeDeserialize
	+ MaybeDisplay
	+ MaybeFromStr
	+ MaybeFromStr
	+ StdHash
	+ Copy
	+ Into<U256>
	+ TryFrom<U256>
	+ Debug
	+ Eq
{
}
impl<
		T: Member
			+ AtLeast32BitUnsigned
			+ Codec
			+ MaybeSerializeDeserialize
			+ MaybeDisplay
			+ MaybeFromStr
			+ StdHash
			+ Copy
			+ Into<U256>
			+ TryFrom<U256>
			+ Debug
			+ Eq,
	> HeaderBlockNumber for T
{
}

/// Header hash.
pub trait HeaderHash: HashT {}
impl<T: HashT> HeaderHash for T {}

pub trait HeaderHashOutput: MaybeDisplay + Decode + SimpleBitOps + Ord {}
impl<T: MaybeDisplay + Decode + SimpleBitOps + Ord> HeaderHashOutput for T {}

/// Extended header access
pub trait ExtendedHeader {
	/// Header number.
	type Number;

	/// Header hash type
	type Hash;

	/// Creates new header.
	fn new(
		number: Self::Number,
		extrinsics_root: Self::Hash,
		state_root: Self::Hash,
		parent_hash: Self::Hash,
		digest: Digest,
		extension: HeaderExtension,
	) -> Self;

	fn extension(&self) -> &HeaderExtension;

	fn set_extension(&mut self, extension: HeaderExtension);
}

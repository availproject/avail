use crate::traits::ExtendedHeader;
use sp_runtime::{
	traits::{Block, Header},
	Digest,
};

/// Extended Block trait that extends substrate primitive Block to include ExtendedHeader in the header
pub trait ExtendedBlock<Extension>:
	Block
	+ DaHeaderProvider<
		<<Self as Block>::Header as Header>::Number,
		<<Self as Block>::Header as Header>::Hash,
		Digest,
		Extension,
		DaHeader = <Self as ExtendedBlock<Extension>>::DaHeader,
	>
{
	type DaHeader: Header<Hash = Self::Hash>
		+ ExtendedHeader<
			<<Self as Block>::Header as Header>::Number,
			<<Self as Block>::Header as Header>::Hash,
			Digest,
			Extension,
		>;
}

// Note: This is a workaround for a compiler bug (https://github.com/rust-lang/rust/issues/96634)
// and should be removed when the compiler bug is fixed.
pub trait DaHeaderProvider<Number, Hash, Digest, Extension> {
	/// DaHeader type.
	type DaHeader: ExtendedHeader<Number, Hash, Digest, Extension>;
}

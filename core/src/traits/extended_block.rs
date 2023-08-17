#[cfg(feature = "runtime")]
use crate::traits::ExtendedHeader;
#[cfg(feature = "runtime")]
use sp_runtime::{
	traits::{Block, Header},
	Digest,
};

/// Extended block trait that extends Block to include ExtendedHeader in the header
#[cfg(feature = "runtime")]
pub trait ExtendedBlock<Extension>: Block {
	type Header: Header<Hash = Self::Hash>
		+ ExtendedHeader<
			<<Self as Block>::Header as Header>::Number,
			<<Self as Block>::Header as Header>::Hash,
			Digest,
			Extension,
		>;
}

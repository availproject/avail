use crate::traits::ExtendedHeader;
use sp_runtime::traits::{Block, Header};
use sp_runtime::Digest;

/// Extended block trait that extends Block to include ExtendedHeader in the header
pub trait ExtendedBlock<Extension>: Block {
	type Header: Header<Hash = Self::Hash> + ExtendedHeader<
		<<Self as Block>::Header as Header>::Number,
		<<Self as Block>::Header as Header>::Hash,
		Digest,
		Extension,
	>;
}

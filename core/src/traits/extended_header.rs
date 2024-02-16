use codec::Codec;
use scale_info::TypeInfo;
use sp_runtime::{
	generic::Digest,
	traits::{Header, MaybeSerialize},
};
use sp_std::fmt::Debug;

/// Extended header access
pub trait ExtendedHeader: Header {
	type Extension: Clone + Send + Sync + Codec + Eq + MaybeSerialize + Debug + TypeInfo + 'static;

	/// Creates new header.
	fn new(
		number: Self::Number,
		extrinsics_root: Self::Hash,
		state_root: Self::Hash,
		parent_hash: Self::Hash,
		digest: Digest,
		extension: Self::Extension,
	) -> Self;

	fn extension(&self) -> &Self::Extension;

	fn set_extension(&mut self, extension: Self::Extension);
}

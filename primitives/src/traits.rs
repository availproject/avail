use sp_runtime::Digest;
use sp_std::vec::Vec;

pub trait ExtrinsicsRoot
/*:Member + MaybeSerializeDeserialize + Default + Debug + Codec + MaybeMallocSizeOf*/
{
	type HashOutput;

	fn hash(&self) -> &Self::HashOutput;
	fn commitment(&self) -> &Vec<u8>;

	fn new(hash: Self::HashOutput) -> Self;

	fn new_with_commitment(
		hash: Self::HashOutput,
		commitment: Vec<u8>,
		rows: u16,
		cols: u16,
	) -> Self;
}

pub trait Rooted {
	/// Root Data.
	type Root: ExtrinsicsRoot<HashOutput = Self::Hash>;

	/// Header number.
	type Number;
	/// Header hash type
	type Hash;

	fn extrinsics_root(&self) -> &Self::Root;
	fn set_extrinsics_root(&mut self, root: Self::Root);

	/// Creates new header.
	fn new(
		number: Self::Number,
		extrinsics_root: Self::Root,
		state_root: Self::Hash,
		parent_hash: Self::Hash,
		digest: Digest,
	) -> Self;
}

use sp_runtime::Digest;
use sp_std::vec::Vec;

use crate::asdr::DataLookup;

pub trait ExtrinsicsWithCommitment {
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

/// Extended header with :
///     - Extrinsics with commitments.
///     - Application data lookup.
pub trait ExtendedHeader {
	/// Root Data.
	type Root: ExtrinsicsWithCommitment<HashOutput = Self::Hash>;

	/// Header number.
	type Number;
	/// Header hash type
	type Hash;

	fn extrinsics_root(&self) -> &Self::Root;
	fn set_extrinsics_root(&mut self, root: Self::Root);

	fn data_lookup(&self) -> &DataLookup;

	/// Creates new header.
	fn new(
		number: Self::Number,
		extrinsics_root: Self::Root,
		state_root: Self::Hash,
		parent_hash: Self::Hash,
		digest: Digest,
		app_data_lookup: DataLookup,
	) -> Self;
}

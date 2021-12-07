use codec::{Decode, Encode};
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

/// Customized extrinsics root to save the commitment.
#[derive(PartialEq, Eq, Clone, sp_core::RuntimeDebug, Default, Encode, Decode)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "std", serde(deny_unknown_fields))]
pub struct KateExtrinsicsRoot<HashOutput> {
	/// The merkle root of the extrinsics.
	pub hash: HashOutput,
	/// Plonk commitment.
	pub commitment: Vec<u8>,
	/// Rows
	pub rows: u16,
	/// Cols
	pub cols: u16,
}

impl<HashOutput: ExtrinsicRootHash> traits::ExtrinsicsRoot for KateExtrinsicsRoot<HashOutput> {
	type HashOutput = HashOutput;

	fn hash(&self) -> &Self::HashOutput { &self.hash }

	fn commitment(&self) -> &Vec<u8> { &self.commitment }

	fn new(hash: HashOutput) -> Self { hash.into() }

	fn new_with_commitment(hash: HashOutput, commitment: Vec<u8>, rows: u16, cols: u16) -> Self {
		Self {
			hash,
			commitment,
			rows,
			cols,
		}
	}
}

impl<Hash: ExtrinsicRootHash> From<Hash> for KateExtrinsicsRoot<Hash> {
	fn from(hash: Hash) -> Self {
		Self {
			hash,
			commitment: Default::default(),
			rows: 0,
			cols: 0,
		}
	}
}

#[cfg(feature = "std")]
impl<HashOutput> parity_util_mem::MallocSizeOf for KateExtrinsicsRoot<HashOutput>
where
	HashOutput: parity_util_mem::MallocSizeOf,
{
	fn size_of(&self, ops: &mut parity_util_mem::MallocSizeOfOps) -> usize {
		self.hash.size_of(ops) + self.commitment.size_of(ops)
	}
}

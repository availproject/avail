use codec::{Decode, Encode};
use scale_info::TypeInfo;
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_core::H256;
#[cfg(feature = "std")]
use sp_std::fmt;
use sp_std::vec::Vec;

pub mod v1 {
	use super::*;

	/// Customized extrinsics root to save the commitment.
	#[derive(PartialEq, Eq, Clone, Default, Encode, Decode, TypeInfo)]
	#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
	#[cfg_attr(feature = "std", serde(deny_unknown_fields))]
	pub struct KateCommitment {
		/// Rows
		#[codec(compact)]
		pub rows: u16,
		/// Cols
		#[codec(compact)]
		pub cols: u16,
		/// The merkle root of the data submitted
		pub data_root: H256,
		/// Plonk commitment.
		pub commitment: Vec<u8>,
	}

	#[cfg(feature = "std")]
	impl fmt::Debug for KateCommitment {
		fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
			let data_root = format!("0x{}", hex::encode(self.data_root.as_ref()));
			let commitment = format!("0x{}", hex::encode(self.commitment.as_slice()));

			f.debug_struct("KateCommitment(v1)")
				.field("rows", &self.rows)
				.field("cols", &self.cols)
				.field("data_root", &data_root)
				.field("commitment", &commitment)
				.finish()
		}
	}

	#[cfg(feature = "std")]
	impl parity_util_mem::MallocSizeOf for KateCommitment {
		fn size_of(&self, ops: &mut parity_util_mem::MallocSizeOfOps) -> usize {
			self.commitment.size_of(ops)
				+ self.rows.size_of(ops)
				+ self.cols.size_of(ops)
				+ self.data_root.size_of(ops)
		}
	}
}

pub mod v2 {
	use super::*;

	/// Customized extrinsics root to save the commitment.
	#[derive(PartialEq, Eq, Clone, Default, Encode, Decode, TypeInfo)]
	#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
	#[cfg_attr(feature = "std", serde(deny_unknown_fields))]
	pub struct KateCommitment {
		/// Rows
		#[codec(compact)]
		pub rows: u16,
		/// Cols
		#[codec(compact)]
		pub cols: u16,
		/// The merkle root of the data submitted
		pub data_root: Option<H256>,
		/// Plonk commitment.
		pub commitment: Vec<u8>,
	}

	impl KateCommitment {
		pub fn new(rows: u16, cols: u16, data_root: H256, commitment: Vec<u8>) -> Self {
			let data_root = (!data_root.is_zero()).then_some(data_root);
			Self {
				rows,
				cols,
				data_root,
				commitment,
			}
		}
	}

	#[cfg(feature = "std")]
	impl fmt::Debug for KateCommitment {
		fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
			let data_root = self
				.data_root
				.as_ref()
				.map(|data_root| format!("Some(0x{})", hex::encode(data_root.as_ref())))
				.unwrap_or_else(|| "None".to_string());
			let commitment = format!("0x{}", hex::encode(self.commitment.as_slice()));

			f.debug_struct("KateCommitment(v2)")
				.field("rows", &self.rows)
				.field("cols", &self.cols)
				.field("data_root", &data_root)
				.field("commitment", &commitment)
				.finish()
		}
	}

	#[cfg(feature = "std")]
	impl parity_util_mem::MallocSizeOf for KateCommitment {
		fn size_of(&self, ops: &mut parity_util_mem::MallocSizeOfOps) -> usize {
			self.commitment.size_of(ops)
				+ self.rows.size_of(ops)
				+ self.cols.size_of(ops)
				+ self.data_root.size_of(ops)
		}
	}

	#[cfg(test)]
	mod tests {
		use super::*;
		use test_case::test_case;

		/// Double check that zero data root is compressed to `None`.
		#[test_case( H256([0u8;32]) => None; "Zero data root")]
		#[test_case( H256([1u8;32]) => Some(H256([1u8;32])); "NonZero data root")]
		fn compression_on_new(data_root: H256) -> Option<H256> {
			let kate = KateCommitment::new(1, 1, data_root, vec![]);
			kate.data_root
		}
	}
}

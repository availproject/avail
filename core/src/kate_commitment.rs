use codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_core::H256;
use sp_std::vec::Vec;

#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "std")]
use sp_core::hexdisplay::HexDisplay;
#[cfg(feature = "std")]
use sp_std::fmt;

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
			f.debug_struct("KateCommitment(v1)")
				.field("rows", &self.rows)
				.field("cols", &self.cols)
				.field("data_root", &HexDisplay::from(&self.data_root.as_ref()))
				.field("commitment", &HexDisplay::from(&self.commitment.as_slice()))
				.finish()
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
			let commitment = self.commitment.as_slice();
			let data_root = self
				.data_root
				.as_ref()
				.map(AsRef::as_ref)
				.unwrap_or_default();

			f.debug_struct("KateCommitment(v2)")
				.field("rows", &self.rows)
				.field("cols", &self.cols)
				.field("commitment", &HexDisplay::from(&commitment))
				.field("data_root", &HexDisplay::from(&data_root))
				.finish()
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

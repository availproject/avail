use codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_core::H256;
use sp_std::vec::Vec;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "serde")]
use sp_core::hexdisplay::HexDisplay;
#[cfg(feature = "serde")]
use sp_std::fmt;

pub mod v1 {
	use super::*;

	/// Customized extrinsics root to save the commitment.
	#[derive(PartialEq, Eq, Clone, Default, Encode, Decode, TypeInfo)]
	#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
	#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
	pub struct KateCommitment {
		/// Rows
		#[codec(compact)]
		pub rows: u16,
		/// Cols
		#[codec(compact)]
		pub cols: u16,
		/// Plonk commitment.
		pub commitment: Vec<u8>,
		/// The merkle root of the data submitted
		pub data_root: H256,
	}

	#[cfg(feature = "serde")]
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
	#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
	#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
	pub struct KateCommitment {
		/// Rows
		#[codec(compact)]
		pub rows: u16,
		/// Cols
		#[codec(compact)]
		pub cols: u16,
		/// Plonk commitment.
		pub commitment: Vec<u8>,
		/// The merkle root of the data submitted
		pub data_root: H256,
	}

	impl KateCommitment {
		pub fn new(rows: u16, cols: u16, data_root: H256, commitment: Vec<u8>) -> Self {
			Self {
				rows,
				cols,
				commitment,
				data_root,
			}
		}
	}

	#[cfg(feature = "serde")]
	impl fmt::Debug for KateCommitment {
		fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
			let commitment = self.commitment.as_slice();
			let data_root = self.data_root.as_ref();

			f.debug_struct("KateCommitment(v2)")
				.field("rows", &self.rows)
				.field("cols", &self.cols)
				.field("commitment", &HexDisplay::from(&commitment))
				.field("data_root", &HexDisplay::from(&data_root))
				.finish()
		}
	}
}

use codec::{Decode, Encode};
use scale_info::TypeInfo;
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_core::{RuntimeDebug, H256};

use crate::{asdr::DataLookup, v1::KateCommitment};

#[derive(PartialEq, Eq, Clone, RuntimeDebug, TypeInfo, Encode, Decode, Default)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct HeaderExtension {
	pub commitment: KateCommitment,
	pub app_lookup: DataLookup,
}

impl HeaderExtension {
	pub fn data_root(&self) -> H256 {
		self.commitment.data_root
	}

	pub fn app_lookup(&self) -> &DataLookup {
		&self.app_lookup
	}

	pub fn rows(&self) -> u16 {
		self.commitment.rows
	}

	pub fn cols(&self) -> u16 {
		self.commitment.cols
	}
}

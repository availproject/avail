use codec::{Decode, Encode};
use scale_info::TypeInfo;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use sp_core::{RuntimeDebug, H256};
use sp_std::vec::Vec;

use crate::{asdr::DataLookup, header::extension::v1, KateCommitment};

#[derive(PartialEq, Eq, Clone, RuntimeDebug, TypeInfo, Encode, Decode, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct HeaderExtension {
	pub new_field: Vec<u8>,
	pub commitment: KateCommitment,
	pub app_lookup: DataLookup,
}

impl HeaderExtension {
	pub fn data_root(&self) -> H256 {
		self.commitment.data_root
	}
}

impl From<v1::HeaderExtension> for HeaderExtension {
	fn from(ext: v1::HeaderExtension) -> Self {
		Self {
			commitment: ext.commitment,
			app_lookup: ext.app_lookup,
			new_field: [1, 1, 2, 3, 5, 8, 13, 21, 34, 55].into(),
		}
	}
}

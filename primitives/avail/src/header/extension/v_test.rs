use codec::{Decode, Encode};
#[cfg(feature = "std")]
use parity_util_mem::{MallocSizeOf, MallocSizeOfOps};
use scale_info::TypeInfo;
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_core::{RuntimeDebug, H256};
use sp_std::vec::Vec;

use crate::{asdr::DataLookup, header::extension::v1, KateCommitment};

#[derive(PartialEq, Eq, Clone, RuntimeDebug, TypeInfo, Encode, Decode, Default)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct HeaderExtension {
	pub new_field: Vec<u8>,
	pub commitment: KateCommitment,
	pub app_lookup: DataLookup,
}

impl HeaderExtension {
	pub fn data_root(&self) -> H256 { self.commitment.data_root }
}

#[cfg(feature = "std")]
impl MallocSizeOf for HeaderExtension {
	fn size_of(&self, ops: &mut MallocSizeOfOps) -> usize {
		self.new_field.size_of(ops) + self.commitment.size_of(ops) + self.app_lookup.size_of(ops)
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

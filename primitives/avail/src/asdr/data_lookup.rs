use codec::{Decode, Encode};
use frame_support::ensure;
#[cfg(feature = "std")]
use parity_util_mem::{MallocSizeOf, MallocSizeOfOps};
use scale_info::TypeInfo;
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_core::RuntimeDebug;
use sp_std::{convert::TryFrom, vec::Vec};

use crate::asdr::AppId;

#[derive(PartialEq, Eq, Clone, RuntimeDebug, Encode, Decode, Default, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct DataLookup {
	/// size of the look up
	pub size: u32,
	/// sorted vector of tuples(key, start index)
	pub index: Vec<(AppId, u32)>,
}

#[derive(PartialEq, RuntimeDebug)]
/// Errors during the creation from `extrinsics`.
pub enum TryFromError {
	/// Size overflows
	SizeOverflow,
	/// Extrinsics are not sorted.
	UnsortedExtrinsics,
}

impl TryFrom<&[(AppId, u32)]> for DataLookup {
	type Error = TryFromError;

	fn try_from(extrinsics: &[(AppId, u32)]) -> Result<Self, Self::Error> {
		let mut index = Vec::new();
		// transactions are order by application id
		// skip transactions with 0 application id - it's not a data txs
		let mut size = 0u32;
		let mut prev_app_id = 0u32;

		for (app_id, data_len) in extrinsics {
			if *app_id != 0 && prev_app_id != *app_id {
				index.push((*app_id, size));
			}

			size = size
				.checked_add(*data_len)
				.ok_or(Self::Error::SizeOverflow)?;
			ensure!(prev_app_id <= *app_id, Self::Error::UnsortedExtrinsics);
			prev_app_id = *app_id;
		}

		Ok(DataLookup { size, index })
	}
}

#[cfg(feature = "std")]
impl MallocSizeOf for DataLookup {
	fn size_of(&self, ops: &mut MallocSizeOfOps) -> usize {
		self.size.size_of(ops) + self.index.size_of(ops)
	}
}

#[cfg(test)]
mod test {
	use super::*;

	fn from_extrinsics_data() -> Vec<(Vec<(AppId, u32)>, Result<DataLookup, TryFromError>)> {
		vec![
			(
				vec![(0, 5), (0, 10), (1, 5), (1, 10), (2, 100), (2, 50)],
				Ok(DataLookup {
					size: 180,
					index: vec![(1, 15), (2, 30)],
				}),
			),
			(
				vec![(0, 5), (0, 10), (1, u32::MAX)],
				Err(TryFromError::SizeOverflow),
			),
			(
				vec![(0, 5), (0, 10), (1, 5), (2, 100), (1, 10), (2, 50)],
				Err(TryFromError::UnsortedExtrinsics),
			),
		]
	}

	#[test]
	fn from_extrinsics() {
		for (extrinsic, expected) in from_extrinsics_data() {
			let data_lookup = DataLookup::try_from(extrinsic.as_slice());
			assert_eq!(data_lookup, expected);
		}
	}
}

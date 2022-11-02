use codec::{Decode, Encode};
use frame_support::ensure;
#[cfg(feature = "std")]
use parity_util_mem::{MallocSizeOf, MallocSizeOfOps};
use scale_info::TypeInfo;
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_core::RuntimeDebug;
use sp_runtime::traits::Zero;
use sp_std::{convert::TryFrom, vec::Vec};

use crate::asdr::AppId;

#[derive(PartialEq, Eq, Clone, RuntimeDebug, Encode, Decode, Default, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct DataLookup {
	/// size of the look up
	#[codec(compact)]
	pub size: u32,
	/// sorted vector of tuples(key, start index)
	pub index: Vec<DataLookupIndexItem>,
}

#[derive(PartialEq, Eq, Copy, Clone, RuntimeDebug, Encode, Decode, Default, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct DataLookupIndexItem {
	pub app_id: AppId,
	#[codec(compact)]
	pub start: u32,
}

impl<A, S> From<(A, S)> for DataLookupIndexItem
where
	A: Into<AppId>,
	S: Into<u32>,
{
	fn from(value: (A, S)) -> Self {
		Self {
			app_id: value.0.into(),
			start: value.1.into(),
		}
	}
}

#[cfg(feature = "std")]
impl MallocSizeOf for DataLookupIndexItem {
	fn size_of(&self, ops: &mut MallocSizeOfOps) -> usize {
		self.app_id.size_of(ops) + self.start.size_of(ops)
	}
}

#[derive(PartialEq, Eq, RuntimeDebug)]
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
		let mut prev_app_id = Zero::zero();

		for (app_id, data_len) in extrinsics {
			if !app_id.is_zero() && prev_app_id != *app_id {
				index.push(DataLookupIndexItem {
					app_id: *app_id,
					start: size,
				});
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

	fn into_app_ids<I, T>(vals: I) -> Vec<(AppId, u32)>
	where
		I: IntoIterator<Item = (T, u32)>,
		T: Into<AppId>,
	{
		vals.into_iter()
			.map(|(id, idx)| (id.into(), idx))
			.collect::<Vec<_>>()
	}
	fn into_lookup_items<I, T>(vals: I) -> Vec<DataLookupIndexItem>
	where
		I: IntoIterator<Item = (T, u32)>,
		T: Into<AppId>,
	{
		vals.into_iter().map(Into::into).collect::<Vec<_>>()
	}

	fn from_extrinsics_data() -> Vec<(Vec<(AppId, u32)>, Result<DataLookup, TryFromError>)> {
		vec![
			(
				into_app_ids([(0, 5), (0, 10), (1, 5), (1, 10), (2, 100), (2, 50)]),
				Ok(DataLookup {
					size: 180,
					index: into_lookup_items([(1, 15), (2, 30)]),
				}),
			),
			(
				into_app_ids([(0, 5), (0, 10), (1, u32::MAX)]),
				Err(TryFromError::SizeOverflow),
			),
			(
				into_app_ids([(0, 5), (0, 10), (1, 5), (2, 100), (1, 10), (2, 50)]),
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

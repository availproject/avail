use alloc::vec::Vec;
use core::convert::TryFrom;
use derive_more::Constructor;
use num_traits::{CheckedAdd, Zero};
use parity_scale_codec::{Decode, Encode};
use scale_info::TypeInfo;
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_core::RuntimeDebug;
use thiserror_no_std::Error;

use crate::{ensure, AppId};

#[derive(PartialEq, Eq, Clone, Encode, Decode, Default, TypeInfo, RuntimeDebug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[cfg_attr(test, derive(Constructor))]
pub struct DataLookup {
	/// size of the look up
	#[codec(compact)]
	size: u32,
	/// sorted vector of tuples(key, start index)
	index: Vec<DataLookupIndexItem>,
}

#[derive(Error, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
	#[error("Input data is not sorted by AppId")]
	DataNotSorted,
	#[error("Data is empty on AppId {0}")]
	DataEmptyOn(AppId),
	#[error("Offset overflows")]
	OffsetOverflows,
}

impl DataLookup {
	/// Creates the `DataLookup` from an iterator sorted by `AppId`
	pub fn new_from_id_lenght<I, A, L>(data: I) -> Result<Self, Error>
	where
		I: Iterator<Item = (A, L)>,
		AppId: From<A>,
		L: Zero + CheckedAdd,
		u32: TryFrom<L>,
	{
		let mut offset = 0;
		let mut maybe_prev_id = None;

		let index = data
			// .skip_while(|(id, _)| id.is_zero())
			.map(|(id, len)| {
				// Check sorted by AppId
				let id = AppId::from(id);
				if let Some(prev_id) = maybe_prev_id.replace(id) {
					ensure!(prev_id < id, Error::DataNotSorted);
				}

				// Check non-empty data per AppId
				let len = u32::try_from(len).map_err(|_| Error::OffsetOverflows)?;
				ensure!(len > 0, Error::DataEmptyOn(id));

				let item = DataLookupIndexItem::new(id, offset);
				offset = offset.checked_add(len).ok_or(Error::OffsetOverflows)?;

				Ok::<DataLookupIndexItem, Error>(item)
			})
			.filter(|res_item| {
				// Filter valid items where AppId == 0
				if let Ok(item) = res_item.as_ref() {
					!item.app_id.is_zero()
				} else {
					true
				}
			})
			.collect::<Result<_, _>>()?;

		Ok(Self {
			size: offset,
			index,
		})
	}

	pub fn len(&self) -> u32 {
		self.size
	}

	pub fn is_empty(&self) -> bool {
		self.size == 0
	}

	pub fn index(&self) -> &Vec<DataLookupIndexItem> {
		&self.index
	}

	pub fn range_of(&self, app_id: AppId) -> Option<(u32, u32)> {
		self.index
			.iter()
			.position(|item| item.app_id == app_id)
			.map(|pos| {
				let start_idx = unsafe { self.index.get_unchecked(pos).start };
				let end_idx = self
					.index
					.get(pos.saturating_add(1))
					.map(|item| item.start)
					.unwrap_or(self.size);
				debug_assert!(start_idx < end_idx);
				(start_idx, end_idx)
			})
	}
}

#[derive(
	PartialEq, Eq, Copy, Clone, Encode, Decode, Default, TypeInfo, RuntimeDebug, Constructor,
)]
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

#[cfg(test)]
mod test {
	use super::*;
	use test_case::test_case;

	fn into_lookup_items<I, T>(vals: I) -> Vec<DataLookupIndexItem>
	where
		I: IntoIterator<Item = (T, u32)>,
		T: Into<AppId>,
	{
		vals.into_iter().map(Into::into).collect::<Vec<_>>()
	}

	#[test_case( vec![(0, 15), (1, 20), (2, 150)] => Ok(DataLookup::new(185, into_lookup_items([(1, 15), (2, 35)]))); "Valid case")]
	#[test_case( vec![(0, usize::MAX)] => Err(Error::OffsetOverflows); "Offset overflows at zero")]
	#[test_case( vec![(0, (u32::MAX -1) as usize), (1, 2)] => Err(Error::OffsetOverflows); "Offset overflows at non zero")]
	#[test_case( vec![(1, 10), (0, 2)] => Err(Error::DataNotSorted); "Unsortend data")]
	#[test_case( vec![] => Ok(DataLookup::new(0, vec![])); "Empty data")]
	fn from_len(id_len_data: Vec<(u32, usize)>) -> Result<DataLookup, Error> {
		let iter = id_len_data
			.into_iter()
			.map(|(id, len)| (AppId::from(id), len));

		DataLookup::new_from_id_lenght(iter)
	}
}

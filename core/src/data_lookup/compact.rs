use crate::{AppId, DataLookup};

use codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_std::vec::Vec;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Encode, Decode, TypeInfo, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct DataLookupItem {
	pub app_id: AppId,
	#[codec(compact)]
	pub start: u32,
}

impl DataLookupItem {
	pub fn new(app_id: AppId, start: u32) -> Self {
		Self { app_id, start }
	}
}

impl<A, S> From<(A, S)> for DataLookupItem
where
	u32: From<A>,
	u32: From<S>,
{
	fn from(value: (A, S)) -> Self {
		Self {
			app_id: AppId(value.0.into()),
			start: value.1.into(),
		}
	}
}

// If .size is equal to u32::MAX then the no commitment was generated
// because of an error that occurred.
//
// This is just a temporary solution that will be replaced by a more
// sofisticated one once we do to do the next header change.
//
#[derive(Encode, Decode, TypeInfo, Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CompactDataLookup {
	/// size of the look up
	#[codec(compact)]
	pub(crate) size: u32,
	/// sorted vector of tuples(key, start index)
	pub(crate) index: Vec<DataLookupItem>,
}

impl CompactDataLookup {
	pub fn new(size: u32, index: Vec<DataLookupItem>) -> Self {
		Self { size, index }
	}

	pub fn from_data_lookup(lookup: &DataLookup) -> Self {
		if lookup.is_error {
			// Data lookup is not valid if size is 0 and lookup index is not empty
			return CompactDataLookup {
				size: 0,
				index: [DataLookupItem {
					app_id: AppId(0),
					start: 0,
				}]
				.to_vec(),
			};
		}

		let index = lookup
			.index
			.iter()
			.filter(|(id, _)| *id != AppId(0))
			.map(|(id, range)| DataLookupItem::new(*id, range.start))
			.collect();
		let size = lookup.index.last().map_or(0, |(_, range)| range.end);
		Self { size, index }
	}
}

// We added this just to please the compiler regarding the Serde macro.
// Do not change this implementation!
//
impl From<DataLookup> for CompactDataLookup {
	fn from(lookup: DataLookup) -> Self {
		Self::from_data_lookup(&lookup)
	}
}

use crate::{AppId, DataLookup};

use codec::{Decode, Encode};
use derive_more::Constructor;
use scale_info::TypeInfo;
use sp_std::vec::Vec;

#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Encode, Decode, TypeInfo, Constructor, Debug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct DataLookupItem {
	pub app_id: AppId,
	#[codec(compact)]
	pub start: u32,
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

#[derive(Encode, Decode, TypeInfo, Constructor, Debug, Clone)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct CompactDataLookup {
	/// size of the look up
	#[codec(compact)]
	pub(crate) size: u32,
	/// sorted vector of tuples(key, start index)
	pub(crate) index: Vec<DataLookupItem>,
}

impl CompactDataLookup {
	pub fn from_expanded(lookup: &DataLookup) -> Self {
		let index = lookup
			.index
			.iter()
			.filter(|(id, _)| *id != AppId(0))
			.map(|(id, range)| DataLookupItem::new(*id, range.start))
			.collect();
		let size = lookup.index.last().map(|(_, range)| range.end).unwrap_or(0);
		Self { size, index }
	}
}

impl From<DataLookup> for CompactDataLookup {
	fn from(lookup: DataLookup) -> Self {
		CompactDataLookup::from_expanded(&lookup)
	}
}

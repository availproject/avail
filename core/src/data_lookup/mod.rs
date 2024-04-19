use codec::{Decode, Encode, Input};
use core::convert::TryFrom;
use scale_info::{Type, TypeInfo};
use sp_core::RuntimeDebug;
use sp_std::{ops::Range, vec::Vec};
use thiserror_no_std::Error;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{ensure, AppId};

pub mod compact;
use compact::CompactDataLookup;

pub type DataLookupRange = Range<u32>;

#[derive(Error, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
	#[error("Input data is not sorted by AppId")]
	DataNotSorted,
	#[error("Data is empty on AppId {0}")]
	DataEmptyOn(AppId),
	#[error("Offset overflows")]
	OffsetOverflows,
}

#[derive(PartialEq, Eq, Clone, Default, RuntimeDebug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(
	feature = "serde",
	serde(try_from = "CompactDataLookup", into = "CompactDataLookup")
)]
pub struct DataLookup {
	pub(crate) index: Vec<(AppId, DataLookupRange)>,
}

impl DataLookup {
	pub fn len(&self) -> u32 {
		self.index.last().map_or(0, |(_id, range)| range.end)
	}

	pub fn is_empty(&self) -> bool {
		self.len() == 0
	}

	pub fn is_error(&self) -> bool {
		self.is_empty() && !self.index.is_empty()
	}

	pub fn range_of(&self, app_id: AppId) -> Option<DataLookupRange> {
		self.index
			.iter()
			.find(|(id, _)| *id == app_id)
			.map(|(_, range)| range)
			.cloned()
	}

	pub fn projected_range_of(&self, app_id: AppId, chunk_size: u32) -> Option<DataLookupRange> {
		self.range_of(app_id).and_then(|range| {
			let start = range.start.checked_mul(chunk_size)?;
			let end = range.end.checked_mul(chunk_size)?;
			Some(start..end)
		})
	}

	/// It projects `self.index` into _chunked_ indexes.
	/// # Errors
	/// It raises `Error::OffsetOverflows` up if any index multiplied by `chunk_size` overflows.
	pub fn projected_ranges(&self, chunk_size: u32) -> Result<Vec<(AppId, Range<u32>)>, Error> {
		self.index
			.iter()
			.map(|(id, range)| {
				let start = range
					.start
					.checked_mul(chunk_size)
					.ok_or(Error::OffsetOverflows)?;
				let end = range
					.end
					.checked_mul(chunk_size)
					.ok_or(Error::OffsetOverflows)?;
				Ok((*id, start..end))
			})
			.collect()
	}
}

impl DataLookup {
	pub fn from_id_and_len_iter<I, A, L>(iter: I) -> Result<Self, Error>
	where
		I: Iterator<Item = (A, L)>,
		u32: From<A>,
		u32: TryFrom<L>,
	{
		let mut offset: u32 = 0;
		let mut maybe_prev_id = None;

		let index = iter
			.map(|(id, len)| {
				// Check sorted by AppId
				let id = AppId(id.into());
				if let Some(prev_id) = maybe_prev_id.replace(id) {
					ensure!(prev_id < id, Error::DataNotSorted);
				}

				// Check non-empty data per AppId
				let len = u32::try_from(len).map_err(|_| Error::OffsetOverflows)?;
				ensure!(len > 0, Error::DataEmptyOn(id));

				// Create range and update `offset`.
				let end = offset.checked_add(len).ok_or(Error::OffsetOverflows)?;
				let range = offset..end;
				offset = end;

				Ok((id, range))
			})
			.collect::<Result<_, _>>()?;

		Ok(Self { index })
	}

	/// This function is used a block contains no data submissions.
	pub fn new_empty() -> Self {
		Self { index: Vec::new() }
	}

	/// This function is only used when something has gone wrong during header extension building
	pub fn new_error() -> Self {
		Self {
			index: vec![(AppId(0), 0..0)],
		}
	}
}

impl TryFrom<CompactDataLookup> for DataLookup {
	type Error = Error;

	fn try_from(compacted: CompactDataLookup) -> Result<Self, Self::Error> {
		if compacted.is_error() {
			return Ok(DataLookup::new_error());
		}

		let mut offset = 0;
		let mut prev_id = AppId(0);
		let mut index = Vec::with_capacity(
			compacted
				.index
				.len()
				.checked_add(1)
				.ok_or(Error::OffsetOverflows)?,
		);

		for c_item in compacted.index {
			index.push((prev_id, offset..c_item.start));
			prev_id = c_item.app_id;
			offset = c_item.start;
		}

		let last_range = offset..compacted.size;
		if !last_range.is_empty() {
			index.push((prev_id, offset..compacted.size));
		}

		let lookup = DataLookup { index };
		ensure!(lookup.len() == compacted.size, Error::DataNotSorted);

		Ok(lookup)
	}
}

// Encoding
// ==================================

impl Encode for DataLookup {
	/// Encodes as a `compact::DataLookup`.
	fn encode(&self) -> Vec<u8> {
		let compacted: CompactDataLookup = CompactDataLookup::from_data_lookup(&self);
		compacted.encode()
	}
}

impl Decode for DataLookup {
	/// Decodes from a `compact::DataLookup`.
	fn decode<I: Input>(input: &mut I) -> Result<Self, codec::Error> {
		let compacted = CompactDataLookup::decode(input)?;
		DataLookup::try_from(compacted).map_err(|_| codec::Error::from("Invalid `DataLookup`"))
	}
}

impl TypeInfo for DataLookup {
	type Identity = Self;

	fn type_info() -> Type {
		CompactDataLookup::type_info()
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use test_case::test_case;

	#[test_case( vec![(0, 15), (1, 20), (2, 150)] => Ok(vec![(0,0..15),(1, 15..35), (2, 35..185)]); "Valid case")]
	#[test_case( vec![(0, usize::MAX)] => Err(Error::OffsetOverflows); "Offset overflows at zero")]
	#[test_case( vec![(0, (u32::MAX -1) as usize), (1, 2)] => Err(Error::OffsetOverflows); "Offset overflows at non zero")]
	#[test_case( vec![(1, 10), (0, 2)] => Err(Error::DataNotSorted); "Unsortend data")]
	#[test_case( vec![] => Ok(vec![]); "Empty data")]
	fn from_id_and_len(
		id_len_data: Vec<(u32, usize)>,
	) -> Result<Vec<(u32, DataLookupRange)>, Error> {
		let iter = id_len_data.into_iter().map(|(id, len)| (AppId(id), len));

		DataLookup::from_id_and_len_iter(iter).map(|lookup| {
			lookup
				.index
				.iter()
				.map(|(id, range)| (id.0, range.clone()))
				.collect::<Vec<_>>()
		})
	}

	#[test_case( vec![(0, 15), (1, 20), (2, 150)] => CompactDataLookup::new(185, vec![(1u32, 15u32).into(),(2u32,35u32).into()]).encode(); "Valid case")]
	#[test_case( vec![(0, 100)] => CompactDataLookup::new(100, vec![]).encode(); "Only Zero AppId")]
	#[test_case( vec![] => CompactDataLookup::new(0, vec![]).encode(); "Empty")]

	fn check_compressed_encode(id_lens: Vec<(u32, usize)>) -> Vec<u8> {
		let lookup = DataLookup::from_id_and_len_iter(id_lens.into_iter()).unwrap();
		lookup.encode()
	}

	#[test_case( vec![(0, 15), (1, 20), (2, 150)] ; "Valid case")]
	#[test_case( vec![(0, 15)] ; "Only Zero AppId")]
	#[test_case( vec![] ; "Empty")]
	fn compressed_conversions(id_lens: Vec<(u32, usize)>) {
		let lookup = DataLookup::from_id_and_len_iter(id_lens.into_iter()).unwrap();

		let compact_lookup = CompactDataLookup::from_data_lookup(&lookup);
		let expanded_lookup = DataLookup::try_from(compact_lookup.clone()).unwrap();

		assert_eq!(
			lookup, expanded_lookup,
			"Lookup: {lookup:?} -> Compacted: {compact_lookup:?} -> Expanded: {expanded_lookup:?}"
		);
	}

	#[test_case( vec![(0, 15), (1, 20), (2, 150)] ; "Valid case")]
	#[test_case( vec![(0, 15)] ; "Only Zero AppId")]
	#[test_case( vec![] ; "Empty")]
	fn serialization_compatibility(id_lens: Vec<(u32, usize)>) {
		let lookup = DataLookup::from_id_and_len_iter(id_lens.into_iter()).unwrap();
		let lookup_json = serde_json::to_string(&lookup).unwrap();
		let compressed_from_json = serde_json::from_str::<CompactDataLookup>(&lookup_json).unwrap();
		let expanded_lookup = DataLookup::try_from(compressed_from_json.clone()).unwrap();

		assert_eq!(lookup, expanded_lookup);
	}
}

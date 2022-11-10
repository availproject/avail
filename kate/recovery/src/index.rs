use std::{convert::TryFrom, iter::once, ops::Range};

use serde::{Deserialize, Serialize};

use crate::config;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppDataIndex {
	pub size: u32,
	pub index: Vec<(u32, u32)>,
}

#[derive(PartialEq, Eq, Debug)]
pub enum AppDataIndexError {
	SizeOverflow,
	UnsortedLayout,
}

impl AppDataIndex {
	/// Calculates cell ranges per application from extrinsic offsets.
	/// Range is from start index to end index in matrix.
	pub fn cells_ranges(&self) -> Vec<(u32, Range<u32>)> {
		// Case if first app_id in index is zero is ignored
		// since it should be asserted elsewhere
		let prepend = self.index.get(0).map_or(vec![(0, 0)], |&(_, offset)| {
			if offset == 0 {
				vec![]
			} else {
				vec![(0, 0)]
			}
		});

		let starts = prepend.iter().chain(self.index.iter());

		let ends = self
			.index
			.iter()
			.skip_while(|&&(_, offset)| offset == 0)
			.map(|&(_, offset)| offset)
			.chain(once(self.size));

		starts
			.zip(ends)
			.map(|(&(app_id, start), end)| (app_id, (start..end)))
			.collect::<Vec<_>>()
	}

	pub fn app_cells_range(&self, app_id: u32) -> Option<Range<u32>> {
		self.cells_ranges()
			.into_iter()
			.find(|&(id, _)| app_id == id)
			.map(|(_, range)| range)
	}

	fn app_cells_ranges(&self, app_id: u32) -> Vec<Range<u32>> {
		self.cells_ranges()
			.into_iter()
			.filter(|&(id, _)| app_id == id)
			.map(|(_, range)| range)
			.collect::<Vec<_>>()
	}

	/// Calculates data range per application from extrinsics layout.
	/// Range is from start index to end index in matrix flattened as byte array.
	pub fn data_ranges(&self) -> Vec<(u32, Range<u32>)> {
		const CHUNK_SIZE_U32: u32 = config::CHUNK_SIZE as u32;
		self.cells_ranges()
			.into_iter()
			.map(|(app_id, Range { start, end })| {
				(app_id, (start * CHUNK_SIZE_U32..end * CHUNK_SIZE_U32))
			})
			.collect::<Vec<_>>()
	}

	pub fn app_data_ranges(&self, app_id: u32) -> Vec<(u32, Range<u32>)> {
		const CHUNK_SIZE_U32: u32 = config::CHUNK_SIZE as u32;
		self.app_cells_ranges(app_id)
			.iter()
			.map(|Range { start, end }| (app_id, (start * CHUNK_SIZE_U32..end * CHUNK_SIZE_U32)))
			.collect::<Vec<_>>()
	}
}

impl<T> TryFrom<&[(T, u32)]> for AppDataIndex
where
	T: Clone + Into<u32>,
{
	type Error = AppDataIndexError;

	fn try_from(layout: &[(T, u32)]) -> Result<Self, Self::Error> {
		let mut index = Vec::new();
		// transactions are ordered by application id
		// skip transactions with 0 application id - it's not a data txs
		let mut size = 0u32;
		let mut prev_app_id = 0u32;

		for (app_id, data_len) in layout {
			let app_id: u32 = app_id.clone().into();
			if app_id != 0 && prev_app_id != app_id {
				index.push((app_id, size));
			}

			size = size
				.checked_add(*data_len)
				.ok_or(Self::Error::SizeOverflow)?;
			if prev_app_id > app_id {
				return Err(Self::Error::UnsortedLayout);
			}
			prev_app_id = app_id;
		}

		Ok(AppDataIndex { size, index })
	}
}

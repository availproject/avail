use std::{
	collections::HashMap,
	convert::{TryFrom, TryInto},
	iter::once,
	ops::Range,
};

use codec::Decode;
use dusk_bytes::Serializable;
use dusk_plonk::{fft::EvaluationDomain, prelude::BlsScalar};
use serde::{Deserialize, Serialize};
use thiserror_no_std::Error;

// TODO: Constants are copy from kate crate, we should move them to common place
pub const CHUNK_SIZE: usize = 32;
pub const DATA_CHUNK_SIZE: usize = 31;
const PADDING_TAIL_VALUE: u8 = 0x80;

pub struct ExtendedMatrixDimensions {
	pub rows: usize,
	pub cols: usize,
}

impl ExtendedMatrixDimensions {
	fn contains(&self, position: &Position) -> bool {
		(position.row as usize) < self.rows && (position.col as usize) < self.cols
	}
}

#[derive(Debug, Error)]
pub enum ReconstructionError {
	#[error("Missing cell (col {}, row {})", .position.col, .position.row)]
	MissingCell { position: Position },
	#[error("Invalid cell (col {}, row {})", .position.col, .position.row)]
	InvalidCell { position: Position },
	#[error("Duplicate cell found")]
	DuplicateCellFound,
	#[error("Column {0} contains less than half rows")]
	InvalidColumn(u16),
	#[error("Cannot reconstruct column: {0}")]
	ColumnReconstructionError(String),
	#[error("Cannot decode data: {0}")]
	DataDecodingError(String),
}

/// Creates hash map of columns, each being hash map of cells, from vector of cells.
/// Intention is to be able to find duplicates and to group cells by column.
fn map_cells(
	dimensions: &ExtendedMatrixDimensions,
	cells: Vec<DataCell>,
) -> Result<HashMap<u16, HashMap<u16, DataCell>>, ReconstructionError> {
	let mut result: HashMap<u16, HashMap<u16, DataCell>> = HashMap::new();
	for cell in cells {
		let position = cell.position.clone();
		if !dimensions.contains(&position) {
			return Err(ReconstructionError::InvalidCell { position });
		}
		let cells = result.entry(position.col).or_insert_with(HashMap::new);
		if cells.insert(position.row, cell).is_some() {
			return Err(ReconstructionError::DuplicateCellFound);
		}
	}
	Ok(result)
}

/// Generates empty cell positions in extended data matrix,
/// for data related to specified application ID.
/// Function returns `None` if there are no cells for given application ID.
/// When fetched, cells can be used to decode application related data.
///
/// # Arguments
///
/// * `index` - Application data index
/// * `dimensions` - Extended matrix dimensions
/// * `app_id` - Application ID
pub fn app_specific_cells(
	index: &AppDataIndex,
	dimensions: &ExtendedMatrixDimensions,
	app_id: u32,
) -> Option<Vec<Position>> {
	let ranges = index.cell_ranges();

	let (_, range) = ranges.into_iter().find(|&(id, _)| app_id == id)?;

	let result = range
		.map(|cell_number| Position {
			col: (cell_number * 2 / dimensions.rows) as u16,
			row: (cell_number * 2 % dimensions.rows) as u16,
		})
		.collect::<Vec<_>>();

	Some(result)
}

/// Generates cell positions of columns related to specified application ID.
/// Function returns `None` if there are no cells for given application ID.
///
/// # Arguments
///
/// * `index` - Application data index
/// * `dimensions` - Extended matrix dimensions
/// * `app_id` - Application ID
pub fn app_specific_column_cells(
	index: &AppDataIndex,
	dimensions: &ExtendedMatrixDimensions,
	app_id: u32,
) -> Option<Vec<Position>> {
	let ranges = index.data_ranges();

	let (_, range) = ranges.iter().find(|&&(id, _)| app_id == id)?;

	let row_size = dimensions.rows * CHUNK_SIZE;

	let column_start = (range.start * 2 / row_size) as u16;
	let mut column_end = (range.end * 2 / row_size) as u16;
	if range.end * 2 % row_size > 0 {
		column_end += 1;
	}

	Some(
		(column_start..column_end)
			.flat_map(|col| (0..dimensions.rows as u16).map(move |row| Position { col, row }))
			.collect::<Vec<_>>(),
	)
}

/// Reconstructs app extrinsics from extrinsics layout and data.
/// Only related extrinsics are reconstructed.
/// Only related data cells needs to be in matrix (unrelated columns can be empty).
///
/// # Arguments
///
/// * `index` - Application data index
/// * `dimensions` - Extended matrix dimensions
/// * `cells` - Cells from required columns, at least 50% cells per column
/// * `app_id` - Application ID
pub fn reconstruct_app_extrinsics(
	index: &AppDataIndex,
	dimensions: &ExtendedMatrixDimensions,
	cells: Vec<DataCell>,
	app_id: u32,
) -> Result<Vec<Vec<u8>>, ReconstructionError> {
	let data = reconstruct_available(dimensions, cells)?;
	let ranges = index
		.data_ranges()
		.into_iter()
		.filter(|&(id, _)| app_id == id)
		.collect::<Vec<_>>();

	Ok(unflatten_padded_data(ranges, data, CHUNK_SIZE)
		.map_err(ReconstructionError::DataDecodingError)?
		.into_iter()
		.flat_map(|(_, xts)| xts)
		.collect::<Vec<_>>())
}

/// Reconstructs all extrinsics from extrinsics layout and data.
///
/// # Arguments
///
/// * `index` - Application data index
/// * `dimensions` - Extended matrix dimensions
/// * `cells` - Cells from required columns, at least 50% cells per column
pub fn reconstruct_extrinsics(
	index: &AppDataIndex,
	dimensions: &ExtendedMatrixDimensions,
	cells: Vec<DataCell>,
) -> Result<Vec<(u32, Vec<Vec<u8>>)>, ReconstructionError> {
	let data = reconstruct_available(dimensions, cells)?;
	let ranges = index.data_ranges();
	unflatten_padded_data(ranges, data, CHUNK_SIZE).map_err(ReconstructionError::DataDecodingError)
}

fn reconstruct_available(
	dimensions: &ExtendedMatrixDimensions,
	cells: Vec<DataCell>,
) -> Result<Vec<u8>, ReconstructionError> {
	let mut column_numbers: Vec<u16> = vec![];
	let mut data: Vec<u8> = vec![];
	let cells_map = map_cells(dimensions, cells)?;
	for column_number in 0..dimensions.cols as u16 {
		match cells_map.get(&column_number) {
			None => data.extend(vec![0; dimensions.rows / 2 * CHUNK_SIZE]),
			Some(column_cells) => {
				if column_cells.len() < dimensions.rows / 2 {
					return Err(ReconstructionError::InvalidColumn(column_number));
				}
				let cells = column_cells.values().cloned().collect::<Vec<_>>();
				let scalars = reconstruct_column(dimensions.rows, &cells)
					.map_err(ReconstructionError::ColumnReconstructionError)?;
				let column_data = scalars.iter().flat_map(|e| e.to_bytes());
				column_numbers.push(column_number);
				data.extend(column_data);
			},
		}
	}
	Ok(data)
}

/// Decode app extrinsics from extrinsics layout and data cells.
/// Only related data cells are needed, without erasure coded data.
///
/// # Arguments
///
/// * `index` - Application data index
/// * `dimensions` - Extended matrix dimensions
/// * `cells` - Application specific data cells in extended matrix, without erasure coded data.
/// * `app_id` - Application ID
pub fn decode_app_extrinsics(
	index: &AppDataIndex,
	dimensions: &ExtendedMatrixDimensions,
	cells: Vec<DataCell>,
	app_id: u32,
) -> Result<Vec<Vec<u8>>, ReconstructionError> {
	let positions = app_specific_cells(index, dimensions, app_id).unwrap_or_default();
	if positions.is_empty() {
		return Ok(vec![]);
	}
	let cells_map = map_cells(dimensions, cells)?;

	for position in positions {
		cells_map
			.get(&position.col)
			.and_then(|column| column.get(&position.row))
			.filter(|cell| !cell.data.is_empty())
			.ok_or(ReconstructionError::MissingCell { position })?;
	}

	let mut app_data: Vec<u8> = vec![];
	for col_number in 0..dimensions.cols as u16 {
		for row_number in 0..dimensions.rows as u16 {
			if row_number % 2 > 0 {
				continue;
			}
			match cells_map
				.get(&col_number)
				.and_then(|column| column.get(&row_number))
				.filter(|cell| !cell.data.is_empty())
			{
				None => app_data.extend(vec![0; CHUNK_SIZE]),
				Some(cell) => app_data.extend(&cell.data),
			}
		}
	}
	let ranges = index
		.data_ranges()
		.into_iter()
		.filter(|(id, _)| *id == app_id)
		.collect::<Vec<_>>();

	Ok(unflatten_padded_data(ranges, app_data, CHUNK_SIZE)
		.map_err(ReconstructionError::DataDecodingError)?
		.into_iter()
		.flat_map(|(_, data)| data)
		.collect::<Vec<_>>())
}

// Removes both extrinsics and block padding (iec_9797 and seeded random data)
pub fn unflatten_padded_data(
	ranges: Vec<(u32, Range<usize>)>,
	data: Vec<u8>,
	chunk_size: usize,
) -> Result<Vec<(u32, Vec<Vec<u8>>)>, String> {
	if data.len() % chunk_size > 0 {
		return Err("Invalid data size".to_string());
	}

	fn trim_to_data_chunks(range_data: &[u8]) -> Result<Vec<u8>, String> {
		range_data
			.chunks_exact(CHUNK_SIZE)
			.map(|chunk| chunk.get(0..DATA_CHUNK_SIZE))
			.collect::<Option<Vec<&[u8]>>>()
			.map(|data_chunks| data_chunks.concat())
			.ok_or_else(|| format!("Chunk data size less than {DATA_CHUNK_SIZE}"))
	}

	fn trim_padding(mut data: Vec<u8>) -> Result<Vec<u8>, String> {
		while data.last() == Some(&0) {
			data.pop();
		}

		match data.pop() {
			None => Err("Cannot trim padding on empty data".to_string()),
			Some(PADDING_TAIL_VALUE) => Ok(data),
			Some(_) => Err("Invalid padding tail value".to_string()),
		}
	}

	fn decode_extrinsics(data: Vec<u8>) -> Result<Vec<Vec<u8>>, String> {
		<Vec<Vec<u8>>>::decode(&mut data.as_slice())
			.map_err(|err| format!("Cannot decode data: {err}"))
	}

	ranges
		.into_iter()
		.map(|(app_id, range)| {
			trim_to_data_chunks(&data[range])
				.and_then(trim_padding)
				.and_then(decode_extrinsics)
				.map(|data| (app_id, data))
		})
		.collect::<Result<Vec<(u32, Vec<Vec<u8>>)>, String>>()
}

// This module is taken from https://gist.github.com/itzmeanjan/4acf9338d9233e79cfbee5d311e7a0b4
// which I wrote few months back when exploring polynomial based erasure coding technique !

fn reconstruct_poly(
	// domain I'm working with
	// all (i)ffts to be performed on it
	eval_domain: EvaluationDomain,
	// subset of available data
	subset: Vec<Option<BlsScalar>>,
) -> Result<Vec<BlsScalar>, String> {
	let missing_indices = subset
		.iter()
		.enumerate()
		.filter(|e| e.1.is_none())
		.map(|(i, _)| i as u64)
		.collect::<Vec<_>>();
	let (mut zero_poly, zero_eval) =
		zero_poly_fn(eval_domain, missing_indices.as_slice(), subset.len() as u64);
	for i in 0..subset.len() {
		if subset[i].is_none() && zero_eval[i] != BlsScalar::zero() {
			return Err("bad zero poly evaluation !".to_owned());
		}
	}
	let mut poly_evals_with_zero: Vec<BlsScalar> = Vec::new();
	for i in 0..subset.len() {
		if let Some(v) = subset[i] {
			poly_evals_with_zero.push(v * zero_eval[i]);
		} else {
			poly_evals_with_zero.push(BlsScalar::zero());
		}
	}
	let mut poly_with_zero = eval_domain.ifft(&poly_evals_with_zero[..]);
	shift_poly(&mut poly_with_zero[..]);
	shift_poly(&mut zero_poly[..]);
	let mut eval_shifted_poly_with_zero = eval_domain.fft(&poly_with_zero[..]);
	let eval_shifted_zero_poly = eval_domain.fft(&zero_poly[..]);
	for i in 0..eval_shifted_poly_with_zero.len() {
		eval_shifted_poly_with_zero[i] *= eval_shifted_zero_poly[i].invert().unwrap();
	}

	let mut shifted_reconstructed_poly = eval_domain.ifft(&eval_shifted_poly_with_zero[..]);
	unshift_poly(&mut shifted_reconstructed_poly[..]);

	let short_domain = EvaluationDomain::new(eval_domain.size() / 2).unwrap();

	let reconstructed_data = short_domain.fft(&shifted_reconstructed_poly[..]);
	Ok(reconstructed_data)
}

fn expand_root_of_unity(eval_domain: EvaluationDomain) -> Vec<BlsScalar> {
	let root_of_unity = eval_domain.group_gen;
	let mut roots: Vec<BlsScalar> = vec![BlsScalar::one(), root_of_unity];
	let mut i = 1;
	while roots[i] != BlsScalar::one() {
		roots.push(roots[i] * root_of_unity);
		i += 1;
	}
	roots
}

fn zero_poly_fn(
	eval_domain: EvaluationDomain,
	missing_indices: &[u64],
	length: u64,
) -> (Vec<BlsScalar>, Vec<BlsScalar>) {
	let expanded_r_o_u = expand_root_of_unity(eval_domain);
	let domain_stride = eval_domain.size() as u64 / length;
	let mut zero_poly: Vec<BlsScalar> = Vec::with_capacity(length as usize);
	let mut sub: BlsScalar;
	for i in 0..missing_indices.len() {
		let v = missing_indices[i as usize];
		sub = BlsScalar::zero() - expanded_r_o_u[(v * domain_stride) as usize];
		zero_poly.push(sub);
		if i > 0 {
			zero_poly[i] = zero_poly[i] + zero_poly[i - 1];
			for j in (1..i).rev() {
				zero_poly[j] *= sub;
				zero_poly[j] = zero_poly[j] + zero_poly[j - 1];
			}
			zero_poly[0] *= sub
		}
	}
	zero_poly.push(BlsScalar::one());
	for _ in zero_poly.len()..zero_poly.capacity() {
		zero_poly.push(BlsScalar::zero());
	}
	let zero_eval = eval_domain.fft(&zero_poly[..]);
	(zero_poly, zero_eval)
}

// in-place shifting
fn shift_poly(poly: &mut [BlsScalar]) {
	// primitive root of unity
	let shift_factor = BlsScalar::from(5);
	let mut factor_power = BlsScalar::one();
	// hoping it won't panic, though it should be handled properly
	//
	// this is actually 1/ shift_factor --- multiplicative inverse
	let inv_factor = shift_factor.invert().unwrap();

	for coef in poly {
		*coef *= factor_power;
		factor_power *= inv_factor;
	}
}

// in-place unshifting
fn unshift_poly(poly: &mut [BlsScalar]) {
	// primitive root of unity
	let shift_factor = BlsScalar::from(5);
	let mut factor_power = BlsScalar::one();

	for coef in poly {
		*coef *= factor_power;
		factor_power *= shift_factor;
	}
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppDataIndex {
	pub size: u32,
	pub index: Vec<(u32, u32)>,
}

impl AppDataIndex {
	/// Calculates cell range per application from extrinsic offsets.
	/// Range is from start index to end index in matrix.
	fn cell_ranges(&self) -> Vec<(u32, Range<usize>)> {
		// Case if first app_id in index is zero is ignored
		// since it should be asserted elsewhere
		let prepend = self.index.first().map_or(vec![(0, 0)], |&(_, offset)| {
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
			.map(|(&(app_id, start), end)| (app_id, start as usize, end as usize))
			.map(|(app_id, start, end)| (app_id, Range { start, end }))
			.collect::<Vec<_>>()
	}

	/// Calculates data range per application from extrinsics layout.
	/// Range is from start index to end index in matrix flattened as byte array.
	pub fn data_ranges(&self) -> Vec<(u32, Range<usize>)> {
		self.cell_ranges()
			.into_iter()
			.map(|(app_id, range)| {
				(app_id, Range {
					start: range.start * CHUNK_SIZE,
					end: range.end * CHUNK_SIZE,
				})
			})
			.collect::<Vec<_>>()
	}
}

#[derive(PartialEq, Eq, Debug)]
pub enum AppDataIndexError {
	SizeOverflow,
	UnsortedLayout,
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

/// Position in a data matrix
#[derive(Default, Debug, Clone, Hash, Eq, PartialEq)]
pub struct Position {
	pub row: u16,
	pub col: u16,
}

impl Position {
	/// Refrence in format `block_number:column_number:row_number`
	pub fn reference(&self, block_number: u64) -> String {
		format!("{}:{}:{}", block_number, self.col, self.row)
	}
}

/// Position and data of a cell in extended matrix
#[derive(Default, Debug, Clone)]
pub struct DataCell {
	/// Cell's position
	pub position: Position,
	/// Cell's data
	pub data: [u8; 32],
}

/// Position and content of a cell in extended matrix
#[derive(Debug, Clone)]
pub struct Cell {
	/// Cell's position
	pub position: Position,
	/// Cell's data
	pub content: [u8; 80],
}

impl Cell {
	pub fn reference(&self, block: u64) -> String { self.position.reference(block) }

	pub fn data(&self) -> [u8; 32] { self.content[48..].try_into().expect("content is 80 bytes") }

	pub fn proof(&self) -> [u8; 48] { self.content[..48].try_into().expect("content is 80 bytes") }
}

impl From<Cell> for DataCell {
	fn from(cell: Cell) -> Self {
		DataCell {
			position: cell.position.clone(),
			data: cell.data(),
		}
	}
}

// use this function for reconstructing back all cells of certain column
// when at least 50% of them are available
//
// if everything goes fine, returned vector in case of success should have
// `row_count`-many cells of some specific column, in coded form
//
// performing one round of ifft should reveal original data which were
// coded together
pub fn reconstruct_column(row_count: usize, cells: &[DataCell]) -> Result<Vec<BlsScalar>, String> {
	// just ensures all rows are from same column !
	// it's required as that's how it's erasure coded during
	// construction in validator node
	fn check_cells(cells: &[DataCell]) {
		assert!(!cells.is_empty());
		let first_col = cells[0].position.col;
		assert!(cells.iter().all(|c| c.position.col == first_col));
	}

	// given row index in column of interest, finds it if present
	// and returns back wrapped in `Some`, otherwise returns `None`
	fn find_row_by_index(idx: usize, cells: &[DataCell]) -> Option<BlsScalar> {
		for cell in cells {
			if cell.position.row == idx as u16 {
				return Some(
					BlsScalar::from_bytes(
						&cell.data[..]
							.try_into()
							.expect("didn't find u8 array of length 32"),
					)
					.unwrap(),
				);
			}
		}
		None
	}

	// row count of data matrix must be power of two !
	assert!(row_count & (row_count - 1) == 0);
	assert!(cells.len() >= row_count / 2 && cells.len() <= row_count);
	check_cells(cells);

	let eval_domain = EvaluationDomain::new(row_count).unwrap();
	let mut subset: Vec<Option<BlsScalar>> = Vec::with_capacity(row_count);

	// fill up vector in ordered fashion
	// @note the way it's done should be improved
	for i in 0..row_count {
		subset.push(find_row_by_index(i, cells));
	}

	reconstruct_poly(eval_domain, subset)
}

#[cfg(test)]
mod tests {
	use std::convert::TryInto;

	use dusk_bytes::Serializable;
	use rand::{Rng, SeedableRng};
	use rand_chacha::ChaChaRng;

	use super::*;

	#[test]
	fn app_data_index_cell_ranges() {
		let cases = vec![
			(
				AppDataIndex {
					size: 8,
					index: vec![],
				},
				vec![(0, Range { start: 0, end: 8 })],
			),
			(
				AppDataIndex {
					size: 4,
					index: vec![(1, 0), (2, 2)],
				},
				vec![
					(1, Range { start: 0, end: 2 }),
					(2, Range { start: 2, end: 4 }),
				],
			),
			(
				AppDataIndex {
					size: 15,
					index: vec![(1, 3), (12, 8)],
				},
				vec![
					(0, Range { start: 0, end: 3 }),
					(1, Range { start: 3, end: 8 }),
					(12, Range { start: 8, end: 15 }),
				],
			),
		];

		for (index, result) in cases {
			assert_eq!(index.cell_ranges(), result);
		}
	}

	#[test]
	fn app_data_index_data_ranges() {
		let cases = vec![
			(
				AppDataIndex {
					size: 8,
					index: vec![],
				},
				vec![(0, Range { start: 0, end: 256 })],
			),
			(
				AppDataIndex {
					size: 4,
					index: vec![(1, 0), (2, 2)],
				},
				vec![
					(1, Range { start: 0, end: 64 }),
					(2, Range {
						start: 64,
						end: 128,
					}),
				],
			),
			(
				AppDataIndex {
					size: 15,
					index: vec![(1, 3), (12, 8)],
				},
				vec![
					(0, Range { start: 0, end: 96 }),
					(1, Range {
						start: 96,
						end: 256,
					}),
					(12, Range {
						start: 256,
						end: 480,
					}),
				],
			),
		];

		for (index, result) in cases {
			assert_eq!(index.data_ranges(), result);
		}
	}

	#[test]
	fn test_app_specific_cells() {
		let index = AppDataIndex {
			size: 8,
			index: vec![(1, 5)],
		};
		let dimensions = ExtendedMatrixDimensions { rows: 4, cols: 4 };

		let expected_0 = vec![(0, 0), (0, 2), (1, 0), (1, 2), (2, 0)];
		let result_0 = app_specific_cells(&index, &dimensions, 0).unwrap();

		assert_eq!(expected_0.len(), result_0.len());
		result_0.iter().zip(expected_0).for_each(|(a, (col, row))| {
			assert_eq!(a.col, col);
			assert_eq!(a.row, row);
		});

		let expected_1 = vec![(2, 2), (3, 0), (3, 2)];
		let result_1 = app_specific_cells(&index, &dimensions, 1).unwrap();

		assert_eq!(expected_1.len(), result_1.len());
		result_1.iter().zip(expected_1).for_each(|(a, (col, row))| {
			assert_eq!(a.col, col);
			assert_eq!(a.row, row);
		});

		assert!(app_specific_cells(&index, &dimensions, 2).is_none());
	}

	#[test]
	fn test_app_specific_column_cells() {
		let index = AppDataIndex {
			size: 8,
			index: vec![(1, 5)],
		};
		let dimensions = ExtendedMatrixDimensions { rows: 4, cols: 4 };

		let expected_0 = (0..=2).flat_map(|c| (0..=3).map(move |r| (c, r)));
		let result_0 = app_specific_column_cells(&index, &dimensions, 0).unwrap();

		assert_eq!(expected_0.clone().count(), result_0.len());
		result_0.iter().zip(expected_0).for_each(|(a, (col, row))| {
			assert_eq!(a.col, col);
			assert_eq!(a.row, row);
		});

		let expected_1 = (2..=3).flat_map(|c| (0..=3).map(move |r| (c, r)));
		let result_1 = app_specific_column_cells(&index, &dimensions, 1).unwrap();

		assert_eq!(expected_1.clone().count(), result_1.len());
		result_1.iter().zip(expected_1).for_each(|(a, (col, row))| {
			assert_eq!(a.col, col);
			assert_eq!(a.row, row);
		});

		assert!(app_specific_column_cells(&index, &dimensions, 2).is_none());
	}

	#[test]
	fn test_app_specific_column_cells_gt_chunk_size() {
		let index = AppDataIndex {
			size: 90,
			index: vec![(1, 1)],
		};
		let dimensions = ExtendedMatrixDimensions { rows: 2, cols: 128 };
		let expected = (1..=89).flat_map(|col| (0..=1).map(move |row| (col, row)));

		let result = app_specific_column_cells(&index, &dimensions, 1).unwrap();

		assert_eq!(expected.clone().count(), result.len());
		result.iter().zip(expected).for_each(|(a, (col, row))| {
			assert_eq!(a.col, col);
			assert_eq!(a.row, row);
		});
	}

	#[test]
	fn data_reconstruction_success() {
		let domain_size = 1usize << 4;
		let half_eval_domain = EvaluationDomain::new(domain_size).unwrap();
		let eval_domain = EvaluationDomain::new(domain_size * 2).unwrap();

		// some dummy source data I care about
		let mut src: Vec<BlsScalar> = Vec::with_capacity(domain_size * 2);
		for i in 0..domain_size {
			src.push(BlsScalar::from(1 << (i + 1)));
		}
		// fill extended portion of vector with zeros
		for _ in domain_size..(2 * domain_size) {
			src.push(BlsScalar::zero());
		}

		// erasure code it
		let temp = half_eval_domain.ifft(&src[0..domain_size]);
		let coded_src = eval_domain.fft(&temp);

		// choose random subset of it ( >= 50% )
		let (coded_src_subset, _) = random_subset(&coded_src, [42u8; 32]);
		// reconstruct 100% values from random coded subset
		let coded_recovered = reconstruct_poly(eval_domain, coded_src_subset).unwrap();

		for i in 0..domain_size {
			assert_eq!(src[i], coded_recovered[i]);
		}
	}

	#[test]
	fn data_reconstruction_failure_0() {
		let domain_size = 1usize << 4;
		let half_eval_domain = EvaluationDomain::new(domain_size).unwrap();
		let eval_domain = EvaluationDomain::new(domain_size * 2).unwrap();

		let mut src: Vec<BlsScalar> = Vec::with_capacity(domain_size * 2);
		for i in 0..domain_size {
			src.push(BlsScalar::from(1 << (i + 1)));
		}
		for _ in domain_size..(2 * domain_size) {
			src.push(BlsScalar::zero());
		}

		let temp = half_eval_domain.ifft(&src[0..domain_size]);
		let coded_src = eval_domain.fft(&temp);

		let (mut coded_src_subset, available) = random_subset(&coded_src, [42u8; 32]);
		// intentionally drop a few coded elements such that
		// < 50% is available
		drop_few(&mut coded_src_subset, available);

		// attempt to reconstruct 100% data from <50 % coded data
		// I've available
		let recovered = reconstruct_poly(eval_domain, coded_src_subset).unwrap();

		let mut mismatch_count = 0;
		for i in 0..domain_size {
			if coded_src[i] != recovered[i] {
				mismatch_count += 1;
			}
		}

		assert!(mismatch_count > 0);
	}

	// Context behind following two test cases, where one failure condition
	// along with one possible solution, is demonstrated
	//
	// Need for writing these test cases originates in a conversation
	// with Prabal<https://github.com/prabal-banerjee> where we were discussing
	// how to ensure input byte chunks to dusk-plonk's `BlsScalar::from_bytes()`
	// is always lesser than prime field modulus ( 255 bits wide ), because
	// we'll get data bytes from arbitrary sources which will be concatenated into
	// single large byte array & finally (multiple) field elements to be produced by chunking contiguous bytes,
	// splitting bytearray into smaller chunks, each of size 32 bytes.
	//
	// Now imagine we got a 32-bytes wide chunk with content like [0xff; 32] --- all 256 -bits are set
	//
	// When that's attempted to be converted into field element it should be wrapped
	// around and original value will be lost
	//
	// We want to specify a way for `how a large byte string can be splitted into field elements
	// such that no values are required to be wrapped around due to modular division i.e. all
	// values must be lesser than 255-bit prime before they are attempted to be converted to BLS scalar ?`
	//
	// One natural way to think about solving this problem is grouping large byte array into 254-bits
	// chunks, then we should not encounter that problem as value is always lesser than
	// prime number which is 255 -bits. But that requires indexing within a byte i.e. not at byte boundary.
	//
	// **Solution** So we decided to chunk contiguous 31 bytes from large input byte array and
	// append zero byte(s) to each chunk for making 32 -bytes wide before inputting
	// 256 -bit integer to `BlsScalar::from_bytes( ... )` function
	//
	// Note, this means, for each field element of 256 -bits, we've 6 -bits free to use
	// and at this moment that's just set to zeros !
	// Other 2 -bits of MSB will always be 0 for avoiding modular division related issue.
	//
	// Is there a way to make use of 6 -bits of most significant byte, in every field element ?
	//
	// Test `data_reconstruction_failure_1` shows how chunking with 32 contiguous bytes
	// results into error where data is lost due to modular division
	//
	// While `data_reconstruction_failure_2` shows how chunking 31 bytes together
	// makes it work smoothly without any data loss encountered !
	#[test]
	fn data_reconstruction_failure_1() {
		// Test code is removed but test case remains,
		// along with test description, for historical purposes.

		let mut data = [0xffu8; 32];
		assert_eq!(
			BlsScalar::from_bytes(&data),
			Err(dusk_bytes::Error::InvalidData)
		);

		data[31] = 0x3f;
		assert!(BlsScalar::from_bytes(&data).is_ok());
	}

	#[test]
	fn data_reconstruction_failure_2() {
		const GROUP_TOGETHER: usize = 31; // bytes

		let input = [0xffu8; 32 << 4];

		let domain_size = ((input.len() as f64) / GROUP_TOGETHER as f64).ceil() as usize;
		let half_eval_domain = EvaluationDomain::new(domain_size).unwrap();
		let eval_domain = EvaluationDomain::new(domain_size * 2).unwrap();

		let mut input_wide: Vec<[u8; 32]> = Vec::with_capacity(domain_size);

		for chunk in input.chunks(GROUP_TOGETHER) {
			let widened: [u8; 32] = {
				let mut v = chunk.to_vec();
				// pad last chunk with required -many zeros
				v.resize(GROUP_TOGETHER, 0u8);
				v.push(0u8); // v is now 32 -bytes
				v.try_into().unwrap()
			};

			input_wide.push(widened);
		}

		let src = input_wide
			.iter()
			.map(|e| BlsScalar::from_bytes(e).unwrap())
			.chain(vec![BlsScalar::zero(); domain_size].into_iter())
			.collect::<Vec<_>>();

		// erasure code it
		let temp = half_eval_domain.ifft(&src[0..domain_size]);
		let coded_src = eval_domain.fft(&temp);

		// choose random subset of it ( >= 50% )
		let (coded_src_subset, _) = random_subset(&coded_src, [42u8; 32]);
		// reconstruct 100% values from random coded subset
		let recovered = reconstruct_poly(eval_domain, coded_src_subset).unwrap();

		for i in 0..(domain_size) {
			assert_eq!(src[i], recovered[i]);
		}

		for i in 0..domain_size {
			let chunk_0 = if (i + 1) * GROUP_TOGETHER >= input.len() {
				&input[i * GROUP_TOGETHER..]
			} else {
				&input[i * GROUP_TOGETHER..(i + 1) * GROUP_TOGETHER]
			};
			let chunk_1 = &recovered[i].to_bytes()[..chunk_0.len()];

			assert_eq!(chunk_0, chunk_1, "at i = {}", i);
		}
	}

	fn drop_few(data: &mut [Option<BlsScalar>], mut available: usize) {
		assert!(available <= data.len());

		let mut idx = 0;
		while available >= data.len() / 2 {
			if data[idx].is_some() {
				data[idx] = None;
				available -= 1;
			}
			idx += 1;
		}
	}

	// select a random subset of coded data to be used for
	// reconstruction purpose
	//
	// @note this is just a helper function for writing test case
	fn random_subset(data: &[BlsScalar], seed: [u8; 32]) -> (Vec<Option<BlsScalar>>, usize) {
		let mut rng = ChaChaRng::from_seed(seed);
		let mut subset: Vec<Option<BlsScalar>> = Vec::with_capacity(data.len());
		let mut available = 0;
		for item in data {
			if rng.gen::<u8>() % 2 == 0 {
				subset.push(Some(*item));
				available += 1;
			} else {
				subset.push(None);
			}
		}

		// already we've >=50% data available
		// so just return & attempt to reconstruct back
		if available >= data.len() / 2 {
			(subset, available)
		} else {
			for i in 0..data.len() {
				if subset[i].is_none() {
					// enough data added, >=50% needs
					// to be present
					if available >= data.len() / 2 {
						break;
					}

					subset[i] = Some(data[i]);
					available += 1;
				}
			}
			(subset, available)
		}
	}

	// Following test cases attempt to figure out any loop holes
	// I might be leaving, when reconstructing whole column of data
	// matrix when >= 50% of those cells along a certain column
	// are available

	#[test]
	fn reconstruct_column_success_0() {
		// This is fairly standard test
		//
		// In general it should be the way how this
		// function should be used

		let domain_size = 1usize << 2;
		let row_count = 2 * domain_size;
		let eval_domain = EvaluationDomain::new(domain_size).unwrap();

		let mut src: Vec<BlsScalar> = Vec::with_capacity(row_count);
		for i in 0..domain_size {
			src.push(BlsScalar::from(1 << (i + 1)));
		}
		eval_domain.ifft_slice(src.as_mut_slice());
		for _ in domain_size..row_count {
			src.push(BlsScalar::zero());
		}

		// erasure coded all data
		let eval_domain = EvaluationDomain::new(row_count).unwrap();
		let coded = eval_domain.fft(&src);
		assert!(coded.len() == row_count);

		let cells = vec![
			DataCell {
				position: Position { row: 0, col: 0 },
				data: coded[0].to_bytes(),
			},
			DataCell {
				position: Position { row: 4, col: 0 },
				data: coded[4].to_bytes(),
			},
			DataCell {
				position: Position { row: 6, col: 0 },
				data: coded[6].to_bytes(),
			},
			DataCell {
				position: Position { row: 2, col: 0 },
				data: coded[2].to_bytes(),
			},
		];

		let reconstructed = reconstruct_column(row_count, &cells[..]).unwrap();
		for i in 0..domain_size {
			assert_eq!(coded[i * 2], reconstructed[i], "{} elem doesn't match", i);
		}
	}

	#[test]
	#[should_panic]
	fn reconstruct_column_failure_0() {
		// Notice how I attempt to construct `cells`
		// vector, I'm intentionally keeping duplicate data
		// so it must fail to reconstruct back [ will panic !]

		let domain_size = 1usize << 2;
		let row_count = 2 * domain_size;
		let eval_domain = EvaluationDomain::new(domain_size).unwrap();

		let mut src: Vec<BlsScalar> = Vec::with_capacity(row_count);
		for i in 0..domain_size {
			src.push(BlsScalar::from(1 << (i + 1)));
		}
		eval_domain.ifft_slice(src.as_mut_slice());
		for _ in domain_size..row_count {
			src.push(BlsScalar::zero());
		}

		// erasure coded all data
		let eval_domain = EvaluationDomain::new(row_count).unwrap();
		let coded = eval_domain.fft(&src);
		assert!(coded.len() == row_count);

		let cells = vec![
			DataCell {
				position: Position { row: 0, col: 0 },
				data: coded[0].to_bytes(),
			},
			DataCell {
				position: Position { row: 0, col: 0 },
				data: coded[0].to_bytes(),
			},
			DataCell {
				position: Position { row: 6, col: 0 },
				data: coded[6].to_bytes(),
			},
			DataCell {
				position: Position { row: 2, col: 0 },
				data: coded[2].to_bytes(),
			},
		];

		let reconstructed = reconstruct_column(row_count, &cells[..]).unwrap();
		for i in 0..domain_size {
			assert_eq!(coded[i * 2], reconstructed[i]);
		}
	}

	#[test]
	#[should_panic]
	fn reconstruct_column_failure_1() {
		// Again notice how I'm constructing `cells`
		// vector, it must have at least 50% data available
		// to be able to reconstruct whole data back properly

		let domain_size = 1usize << 2;
		let row_count = 2 * domain_size;
		let eval_domain = EvaluationDomain::new(domain_size).unwrap();

		let mut src: Vec<BlsScalar> = Vec::with_capacity(row_count);
		for i in 0..domain_size {
			src.push(BlsScalar::from(1 << (i + 1)));
		}
		eval_domain.ifft_slice(src.as_mut_slice());
		for _ in domain_size..row_count {
			src.push(BlsScalar::zero());
		}

		// erasure coded all data
		let eval_domain = EvaluationDomain::new(row_count).unwrap();
		let coded = eval_domain.fft(&src);
		assert!(coded.len() == row_count);

		let cells = vec![
			DataCell {
				position: Position { row: 4, col: 0 },
				data: coded[4].to_bytes(),
			},
			DataCell {
				position: Position { row: 6, col: 0 },
				data: coded[6].to_bytes(),
			},
			DataCell {
				position: Position { row: 2, col: 0 },
				data: coded[2].to_bytes(),
			},
		];

		let reconstructed = reconstruct_column(row_count, &cells[..]).unwrap();
		for i in 0..domain_size {
			assert_eq!(coded[i * 2], reconstructed[i]);
		}
	}

	#[test]
	#[should_panic]
	fn reconstruct_column_failure_2() {
		// Again check how I construct `cells` vector
		// where I put wrong row's data in place of wrong
		// row index [ will panic !]

		let domain_size = 1usize << 2;
		let row_count = 2 * domain_size;
		let eval_domain = EvaluationDomain::new(domain_size).unwrap();

		let mut src: Vec<BlsScalar> = Vec::with_capacity(row_count);
		for i in 0..domain_size {
			src.push(BlsScalar::from(1 << (i + 1)));
		}
		eval_domain.ifft_slice(src.as_mut_slice());
		for _ in domain_size..row_count {
			src.push(BlsScalar::zero());
		}

		// erasure coded all data
		let eval_domain = EvaluationDomain::new(row_count).unwrap();
		let coded = eval_domain.fft(&src);
		assert!(coded.len() == row_count);

		let cells = vec![
			DataCell {
				position: Position { row: 0, col: 0 },
				data: coded[0].to_bytes(),
			},
			DataCell {
				position: Position { row: 5, col: 0 },
				data: coded[4].to_bytes(),
			},
			DataCell {
				position: Position { row: 6, col: 0 },
				data: coded[6].to_bytes(),
			},
			DataCell {
				position: Position { row: 2, col: 0 },
				data: coded[2].to_bytes(),
			},
		];

		let reconstructed = reconstruct_column(row_count, &cells[..]).unwrap();
		for i in 0..domain_size {
			assert_eq!(coded[i * 2], reconstructed[i]);
		}
	}
}

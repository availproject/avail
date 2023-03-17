use codec::Decode;
use dusk_bytes::Serializable;
use dusk_plonk::{fft::EvaluationDomain, prelude::BlsScalar};
use num::ToPrimitive;
use rand::seq::SliceRandom;
use std::{
	collections::{HashMap, HashSet},
	convert::TryFrom,
	iter::FromIterator,
};
use thiserror::Error;

use crate::{
	config::{self, CHUNK_SIZE},
	data, index, matrix,
};

#[derive(Debug, Error)]
pub enum ReconstructionError {
	#[error("Missing cell (col {}, row {})", .position.col, .position.row)]
	MissingCell { position: matrix::Position },
	#[error("Invalid cell (col {}, row {})", .position.col, .position.row)]
	InvalidCell { position: matrix::Position },
	#[error("Duplicate cell found")]
	DuplicateCellFound,
	#[error("Column {0} contains less than half rows")]
	InvalidColumn(u16),
	#[error("Cannot reconstruct column: {0}")]
	ColumnReconstructionError(String),
	#[error("Cannot decode data: {0}")]
	DataDecodingError(String),
	#[error("Column reconstruction supports up to {}", u16::MAX)]
	RowCountExceeded,
}

/// From given positions, constructs related columns positions, up to given factor.
/// E.g. if factor is 0.66, 66% of matched columns will be returned.
/// Positions in columns are random.
/// Function panics if factor is above 1.0.
pub fn columns_positions(
	dimensions: &matrix::Dimensions,
	positions: &[matrix::Position],
	factor: f64,
) -> Vec<matrix::Position> {
	assert!(factor <= 1.0);

	let cells = (factor * dimensions.extended_rows() as f64)
		.to_usize()
		.expect("result is lesser than usize maximum");

	let rng = &mut rand::thread_rng();

	let columns: HashSet<u16> = HashSet::from_iter(positions.iter().map(|position| position.col));

	columns
		.into_iter()
		.map(|col| dimensions.col_positions(col))
		.flat_map(|col| col.choose_multiple(rng, cells).cloned().collect::<Vec<_>>())
		.collect::<Vec<matrix::Position>>()
}

/// Creates hash map of columns, each being hash map of cells, from vector of cells.
/// Intention is to be able to find duplicates and to group cells by column.
fn map_cells(
	dimensions: &matrix::Dimensions,
	cells: Vec<data::DataCell>,
) -> Result<HashMap<u16, HashMap<u32, data::DataCell>>, ReconstructionError> {
	let mut result: HashMap<u16, HashMap<u32, data::DataCell>> = HashMap::new();
	for cell in cells {
		let position = cell.position.clone();
		if !dimensions.extended_contains(&position) {
			return Err(ReconstructionError::InvalidCell { position });
		}
		let cells = result.entry(position.col).or_insert_with(HashMap::new);
		if cells.insert(position.row, cell).is_some() {
			return Err(ReconstructionError::DuplicateCellFound);
		}
	}
	Ok(result)
}

/// Returns indexes of rows related to given application ID,
/// or empty vector if there are no rows.
///
/// # Arguments
///
/// * `index` - Application data index
/// * `dimensions` - Extended matrix dimensions
/// * `app_id` - Application ID
pub fn app_specific_rows(
	index: &index::AppDataIndex,
	dimensions: &matrix::Dimensions,
	app_id: u32,
) -> Vec<u32> {
	index
		.app_cells_range(app_id)
		.map(|range| dimensions.extended_data_rows(range))
		.unwrap_or_else(std::vec::Vec::new)
}

/// Generates empty cell positions in extended data matrix,
/// for data related to specified application ID.
/// When fetched, cells can be used to decode application related data.
///
/// # Arguments
///
/// * `index` - Application data index
/// * `dimensions` - Extended matrix dimensions
/// * `app_id` - Application ID
pub fn app_specific_cells(
	index: &index::AppDataIndex,
	dimensions: &matrix::Dimensions,
	app_id: u32,
) -> Option<Vec<matrix::Position>> {
	index
		.app_cells_range(app_id)
		.map(|range| dimensions.extended_data_positions(range))
}

/// Application data, represents list of extrinsics encoded in a block.
pub type AppData = Vec<Vec<u8>>;

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
	index: &index::AppDataIndex,
	dimensions: &matrix::Dimensions,
	cells: Vec<data::DataCell>,
	app_id: u32,
) -> Result<AppData, ReconstructionError> {
	let data = reconstruct_available(dimensions, cells)?;
	let ranges = index.app_data_ranges(app_id);

	Ok(unflatten_padded_data(ranges, data)
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
	index: &index::AppDataIndex,
	dimensions: &matrix::Dimensions,
	cells: Vec<data::DataCell>,
) -> Result<Vec<(u32, AppData)>, ReconstructionError> {
	let data = reconstruct_available(dimensions, cells)?;
	let ranges = index.data_ranges();
	unflatten_padded_data(ranges, data).map_err(ReconstructionError::DataDecodingError)
}

/// Reconstructs columns for given cells.
///
/// # Arguments
///
/// * `dimensions` - Extended matrix dimensions
/// * `cells` - Cells from required columns, at least 50% cells per column
pub fn reconstruct_columns(
	dimensions: &matrix::Dimensions,
	cells: &[data::Cell],
) -> Result<HashMap<u16, Vec<[u8; CHUNK_SIZE]>>, ReconstructionError> {
	let cells: Vec<data::DataCell> = cells.iter().cloned().map(Into::into).collect::<Vec<_>>();
	let columns = map_cells(dimensions, cells)?;

	columns
		.iter()
		.map(|(&col, cells)| {
			if cells.len() < dimensions.rows().into() {
				return Err(ReconstructionError::InvalidColumn(col));
			}

			let cells = cells.values().cloned().collect::<Vec<_>>();

			let column = reconstruct_column(dimensions.extended_rows(), &cells)
				.map_err(ReconstructionError::ColumnReconstructionError)?
				.iter()
				.map(BlsScalar::to_bytes)
				.collect::<Vec<[u8; CHUNK_SIZE]>>();

			Ok((col, column))
		})
		.collect::<Result<_, _>>()
}

fn reconstruct_available(
	dimensions: &matrix::Dimensions,
	cells: Vec<data::DataCell>,
) -> Result<Vec<u8>, ReconstructionError> {
	let columns = map_cells(dimensions, cells)?;

	let scalars = (0..dimensions.cols())
		.map(|col| match columns.get(&col) {
			None => Ok(vec![None; dimensions.rows() as usize]),
			Some(column_cells) => {
				if column_cells.len() < dimensions.rows() as usize {
					return Err(ReconstructionError::InvalidColumn(col));
				}
				let cells = column_cells.values().cloned().collect::<Vec<_>>();

				reconstruct_column(dimensions.extended_rows(), &cells)
					.map(|scalars| scalars.into_iter().map(Some).collect::<Vec<_>>())
					.map_err(ReconstructionError::ColumnReconstructionError)
			},
		})
		.collect::<Result<Vec<Vec<_>>, ReconstructionError>>()?;

	let mut result: Vec<u8> = Vec::with_capacity(scalars.len() * config::CHUNK_SIZE);

	for (row, col) in dimensions.iter_data() {
		let bytes = scalars
			.get(col)
			.and_then(|col| col.get(row))
			.map(Option::as_ref)
			.unwrap_or(None)
			.map(BlsScalar::to_bytes)
			.unwrap_or_else(|| [0; config::CHUNK_SIZE]);
		result.extend(bytes);
	}
	Ok(result)
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
	index: &index::AppDataIndex,
	dimensions: &matrix::Dimensions,
	cells: Vec<data::DataCell>,
	app_id: u32,
) -> Result<AppData, ReconstructionError> {
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
	for (row_number, col_number) in dimensions.iter_extended_data_positions() {
		match cells_map
			.get(&col_number)
			.and_then(|column| column.get(&row_number))
			.filter(|cell| !cell.data.is_empty())
		{
			None => app_data.extend(vec![0; config::CHUNK_SIZE]),
			Some(cell) => app_data.extend(cell.data),
		}
	}
	let ranges = index.app_data_ranges(app_id);

	Ok(unflatten_padded_data(ranges, app_data)
		.map_err(ReconstructionError::DataDecodingError)?
		.into_iter()
		.flat_map(|(_, data)| data)
		.collect::<Vec<_>>())
}

// Removes both extrinsics and block padding (iec_9797 and seeded random data)
pub fn unflatten_padded_data(
	ranges: Vec<(u32, AppDataRange)>,
	data: Vec<u8>,
) -> Result<Vec<(u32, AppData)>, String> {
	if data.len() % config::CHUNK_SIZE > 0 {
		return Err("Invalid data size".to_string());
	}

	fn trim_to_data_chunks(range_data: &[u8]) -> Result<Vec<u8>, String> {
		range_data
			.chunks_exact(config::CHUNK_SIZE)
			.map(|chunk| chunk.get(0..config::DATA_CHUNK_SIZE))
			.collect::<Option<Vec<&[u8]>>>()
			.map(|data_chunks| data_chunks.concat())
			.ok_or_else(|| format!("Chunk data size less than {}", config::DATA_CHUNK_SIZE))
	}

	fn trim_padding(mut data: Vec<u8>) -> Result<Vec<u8>, String> {
		while data.last() == Some(&0) {
			data.pop();
		}

		match data.pop() {
			None => Err("Cannot trim padding on empty data".to_string()),
			Some(config::PADDING_TAIL_VALUE) => Ok(data),
			Some(_) => Err("Invalid padding tail value".to_string()),
		}
	}

	fn decode_extrinsics(data: Vec<u8>) -> Result<AppData, String> {
		<AppData>::decode(&mut data.as_slice()).map_err(|err| format!("Cannot decode data: {err}"))
	}

	ranges
		.into_iter()
		.map(|(app_id, range)| {
			let range = range.start as usize..range.end as usize;
			trim_to_data_chunks(&data[range])
				.and_then(trim_padding)
				.and_then(decode_extrinsics)
				.map(|data| (app_id, data))
		})
		.collect::<Result<Vec<(u32, AppData)>, String>>()
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
		let v = missing_indices[i];
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

pub type AppDataRange = std::ops::Range<u32>;
// use this function for reconstructing back all cells of certain column
// when at least 50% of them are available
//
// if everything goes fine, returned vector in case of success should have
// `row_count`-many cells of some specific column, in coded form
//
// performing one round of ifft should reveal original data which were
// coded together
pub fn reconstruct_column(
	row_count: u32,
	cells: &[data::DataCell],
) -> Result<Vec<BlsScalar>, String> {
	// just ensures all rows are from same column !
	// it's required as that's how it's erasure coded during
	// construction in validator node
	fn check_cells(cells: &[data::DataCell]) {
		assert!(!cells.is_empty());
		let first_col = cells[0].position.col;
		assert!(cells.iter().all(|c| c.position.col == first_col));
	}

	// given row index in column of interest, finds it if present
	// and returns back wrapped in `Some`, otherwise returns `None`
	fn find_row_by_index(idx: u32, cells: &[data::DataCell]) -> Option<BlsScalar> {
		cells
			.iter()
			.find(|cell| cell.position.row == idx)
			.map(|cell| {
				<[u8; BlsScalar::SIZE]>::try_from(&cell.data[..])
					.expect("didn't find u8 array of length 32")
			})
			.and_then(|data| BlsScalar::from_bytes(&data).ok())
	}

	// row count of data matrix must be power of two !
	assert!(row_count % 2 == 0);
	assert!(cells.len() >= (row_count / 2) as usize && cells.len() <= row_count as usize);
	check_cells(cells);

	let eval_domain = EvaluationDomain::new(row_count as usize).unwrap();
	let mut subset: Vec<Option<BlsScalar>> = Vec::with_capacity(row_count as usize);

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
	use test_case::test_case;

	use super::*;
	use crate::{
		data::DataCell,
		index::AppDataIndex,
		matrix::{Dimensions, Position},
	};

	#[test]
	fn app_data_index_cell_ranges() {
		let cases = vec![
			(
				AppDataIndex {
					size: 8,
					index: vec![],
				},
				vec![(0, 0..8)],
			),
			(
				AppDataIndex {
					size: 4,
					index: vec![(1, 0), (2, 2)],
				},
				vec![(1, 0..2), (2, 2..4)],
			),
			(
				AppDataIndex {
					size: 15,
					index: vec![(1, 3), (12, 8)],
				},
				vec![(0, 0..3), (1, 3..8), (12, 8..15)],
			),
		];

		for (index, result) in cases {
			assert_eq!(index.cells_ranges(), result);
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
				vec![(0, 0..256)],
			),
			(
				AppDataIndex {
					size: 4,
					index: vec![(1, 0), (2, 2)],
				},
				vec![(1, 0..64), (2, 64..128)],
			),
			(
				AppDataIndex {
					size: 15,
					index: vec![(1, 3), (12, 8)],
				},
				vec![(0, 0..96), (1, 96..256), (12, 256..480)],
			),
		];

		for (index, result) in cases {
			assert_eq!(index.data_ranges(), result);
		}
	}

	#[test_case(0, &[0] ; "App 0 spans 2 rows form row 0")]
	#[test_case(1, &[0, 2] ; "App 1 spans 2 rows from row 0")]
	#[test_case(2, &[2] ; "App 2 spans 1 rows from row 2")]
	#[test_case(3, &[4, 6] ; "App 3 spans 2 rows from row 4")]
	#[test_case(4, &[] ; "There is no app 4")]
	fn test_app_specific_rows(app_id: u32, expected: &[u32]) {
		let index = AppDataIndex {
			size: 16,
			index: vec![(1, 2), (2, 5), (3, 8)],
		};
		let dimensions = Dimensions::new(8, 4).unwrap();
		let result = app_specific_rows(&index, &dimensions, app_id);
		assert_eq!(expected.len(), result.len());
	}

	#[test_case(0, &[(0, 0), (0, 1), (0, 2), (0, 3), (2, 0)] ; "App 0 has five cells")]
	#[test_case(1, &[(2, 1), (2, 2), (2, 3)] ; "App 1 has 3 cells")]
	#[test_case(2, &[] ; "App 2 has no cells")]
	fn test_app_specific_cells(app_id: u32, expected: &[(u32, u16)]) {
		let index = AppDataIndex {
			size: 8,
			index: vec![(1, 5)],
		};
		let dimensions = Dimensions::new(4, 4).unwrap();
		let result = app_specific_cells(&index, &dimensions, app_id).unwrap_or_default();
		assert_eq!(expected.len(), result.len());
		result.iter().zip(expected).for_each(|(a, &(row, col))| {
			assert_eq!(a.row, row);
			assert_eq!(a.col, col);
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

	fn build_coded_eval_domain() -> (Vec<BlsScalar>, u32, u16) {
		let domain_size = 1u16 << 2;
		let row_count = domain_size.checked_mul(2).unwrap();
		let eval_domain = EvaluationDomain::new(domain_size as usize).unwrap();

		let mut src: Vec<BlsScalar> = Vec::with_capacity(row_count as usize);
		for i in 0..domain_size {
			src.push(BlsScalar::from(1 << (i + 1)));
		}
		eval_domain.ifft_slice(src.as_mut_slice());
		for _ in domain_size..row_count {
			src.push(BlsScalar::zero());
		}

		// erasure coded all data
		let eval_domain = EvaluationDomain::new(row_count as usize).unwrap();
		let coded = eval_domain.fft(&src);
		assert!(coded.len() == row_count as usize);

		(coded, row_count.into(), domain_size)
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
		let (coded, row_count, domain_size) = build_coded_eval_domain();

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
			assert_eq!(
				coded[i as usize * 2],
				reconstructed[i as usize],
				"{} elem doesn't match",
				i
			);
		}
	}

	#[test]
	#[should_panic]
	fn reconstruct_column_failure_0() {
		// Notice how I attempt to construct `cells`
		// vector, I'm intentionally keeping duplicate data
		// so it must fail to reconstruct back [ will panic !]
		let (coded, row_count, domain_size) = build_coded_eval_domain();

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
			assert_eq!(coded[i as usize * 2], reconstructed[i as usize]);
		}
	}

	#[test]
	#[should_panic]
	fn reconstruct_column_failure_1() {
		// Again notice how I'm constructing `cells`
		// vector, it must have at least 50% data available
		// to be able to reconstruct whole data back properly
		let (coded, row_count, domain_size) = build_coded_eval_domain();

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
			assert_eq!(coded[i as usize * 2], reconstructed[i as usize]);
		}
	}

	#[test]
	#[should_panic]
	fn reconstruct_column_failure_2() {
		// Again check how I construct `cells` vector
		// where I put wrong row's data in place of wrong
		// row index [ will panic !]
		let (coded, row_count, domain_size) = build_coded_eval_domain();

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
			assert_eq!(coded[i as usize * 2], reconstructed[i as usize]);
		}
	}
}

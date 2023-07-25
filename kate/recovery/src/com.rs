use crate::matrix;
use core::{num::TryFromIntError, ops::Range};

use avail_core::{data_lookup::Error as DataLookupError, AppId, DataLookup};

use sp_std::prelude::*;
use thiserror_no_std::Error;

#[cfg(feature = "std")]
use crate::data;
#[cfg(feature = "std")]
use crate::{config, sparse_slice_read::SparseSliceRead};
#[cfg(feature = "std")]
use avail_core::ensure;
#[cfg(feature = "std")]
use codec::{Decode, IoReader};
#[cfg(feature = "std")]
use dusk_bytes::Serializable as _;
#[cfg(feature = "std")]
use dusk_plonk::{fft::EvaluationDomain, prelude::BlsScalar};
#[cfg(feature = "std")]
pub use sp_arithmetic::{traits::SaturatedConversion as _, Percent};
#[cfg(feature = "std")]
use static_assertions::{const_assert, const_assert_ne};
#[cfg(feature = "std")]
use std::{
	collections::{HashMap, HashSet},
	convert::{TryFrom, TryInto},
	iter::FromIterator,
};

#[derive(Debug, Error)]
pub enum ReconstructionError {
	#[error("Missing cell ({0})")]
	MissingCell(matrix::Position),
	#[error("Invalid cell ({0})")]
	InvalidCell(matrix::Position),
	#[error("Maximum cells allowed {0}")]
	MaxCells(usize),
	#[error("Minimum cells allowed {0}")]
	MinCells(usize),
	#[error("Duplicate cell found")]
	DuplicateCellFound,
	#[error("Column {0} contains less than half rows")]
	InvalidColumn(u16),
	#[error("Cannot decode data: {0}")]
	DataDecodingError(#[from] UnflattenError),
	#[error("Column reconstruction supports up to {}", u16::MAX)]
	RowCountExceeded,
	#[error("Rows must be power of two")]
	InvalidRowCount,
	#[error("Missing AppId {0}")]
	MissingId(AppId),
	#[error("DataLookup {0}")]
	DataLookup(#[from] DataLookupError),
	#[error("Some cells are from different columns")]
	CellsFromDifferentCols,
	#[error("Invalid evaluation domain")]
	InvalidEvaluationDomain,
	#[error("Bad zero poly evaluation")]
	BadZeroPoly,
}

#[cfg(feature = "std")]
impl std::error::Error for ReconstructionError {
	fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
		match &self {
			Self::DataDecodingError(unflatten) => Some(unflatten),
			_ => None,
		}
	}
}

/// From given positions, constructs related columns positions, up to given factor.
/// E.g. if factor is 0.66, 66% of matched columns will be returned.
/// Positions in columns are random.
/// Function panics if factor is above 1.0.
#[cfg(feature = "std")]
pub fn columns_positions<R: rand::RngCore>(
	dimensions: matrix::Dimensions,
	positions: &[matrix::Position],
	factor: Percent,
	rng: &mut R,
) -> Vec<matrix::Position> {
	use rand::seq::SliceRandom;

	let cells = factor
		.mul_ceil(dimensions.extended_rows())
		.saturated_into::<usize>();

	let columns: HashSet<u16> = HashSet::from_iter(positions.iter().map(|position| position.col));

	columns
		.into_iter()
		.map(|col| dimensions.col_positions(col))
		.flat_map(|col| col.choose_multiple(rng, cells).cloned().collect::<Vec<_>>())
		.collect::<Vec<matrix::Position>>()
}

/// Creates hash map of columns, each being hash map of cells, from vector of cells.
/// Intention is to be able to find duplicates and to group cells by column.
#[cfg(feature = "std")]
fn map_cells(
	dimensions: matrix::Dimensions,
	cells: Vec<data::DataCell>,
) -> Result<HashMap<u16, HashMap<u32, data::DataCell>>, ReconstructionError> {
	let mut result: HashMap<u16, HashMap<u32, data::DataCell>> = HashMap::new();
	for cell in cells {
		let position = cell.position;
		if !dimensions.extended_contains(&position) {
			return Err(ReconstructionError::InvalidCell(position));
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
	index: &DataLookup,
	dimensions: matrix::Dimensions,
	app_id: AppId,
) -> Vec<u32> {
	index
		.range_of(app_id)
		.and_then(|range| dimensions.extended_data_rows(range))
		.unwrap_or_default()
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
	index: &DataLookup,
	dimensions: matrix::Dimensions,
	id: AppId,
) -> Option<Vec<matrix::Position>> {
	index
		.range_of(id)
		.and_then(|range| dimensions.extended_data_positions(range))
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
#[cfg(feature = "std")]
pub fn reconstruct_app_extrinsics(
	index: &DataLookup,
	dimensions: matrix::Dimensions,
	cells: Vec<data::DataCell>,
	app_id: AppId,
) -> Result<AppData, ReconstructionError> {
	let data = reconstruct_available(dimensions, cells)?;
	const_assert!(config::CHUNK_SIZE as u64 <= u32::MAX as u64);
	let range = index
		.projected_range_of(app_id, config::CHUNK_SIZE as u32)
		.ok_or(ReconstructionError::MissingId(app_id))?;

	Ok(unflatten_padded_data(vec![(app_id, range)], data)?
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
#[cfg(feature = "std")]
pub fn reconstruct_extrinsics(
	lookup: &DataLookup,
	dimensions: matrix::Dimensions,
	cells: Vec<data::DataCell>,
) -> Result<Vec<(AppId, AppData)>, ReconstructionError> {
	let data = reconstruct_available(dimensions, cells)?;

	const_assert!(config::CHUNK_SIZE as u64 <= u32::MAX as u64);
	let ranges = lookup.projected_ranges(config::CHUNK_SIZE as u32)?;
	unflatten_padded_data(ranges, data).map_err(ReconstructionError::DataDecodingError)
}

/// Reconstructs columns for given cells.
///
/// # Arguments
///
/// * `dimensions` - Extended matrix dimensions
/// * `cells` - Cells from required columns, at least 50% cells per column
#[cfg(feature = "std")]
pub fn reconstruct_columns(
	dimensions: matrix::Dimensions,
	cells: &[data::Cell],
) -> Result<HashMap<u16, Vec<[u8; config::CHUNK_SIZE]>>, ReconstructionError> {
	let cells: Vec<data::DataCell> = cells.iter().cloned().map(Into::into).collect::<Vec<_>>();
	let columns = map_cells(dimensions, cells)?;

	columns
		.iter()
		.map(|(&col, cells)| {
			ensure!(
				cells.len() >= dimensions.height(),
				ReconstructionError::InvalidColumn(col)
			);

			let cells = cells.values().cloned().collect::<Vec<_>>();

			let column = reconstruct_column(dimensions.extended_rows(), &cells)?
				.iter()
				.map(BlsScalar::to_bytes)
				.collect::<Vec<[u8; config::CHUNK_SIZE]>>();

			Ok((col, column))
		})
		.collect::<Result<_, _>>()
}

#[cfg(feature = "std")]
fn reconstruct_available(
	dimensions: matrix::Dimensions,
	cells: Vec<data::DataCell>,
) -> Result<Vec<u8>, ReconstructionError> {
	let columns = map_cells(dimensions, cells)?;
	let rows: usize = dimensions.height();

	let scalars = (0..dimensions.cols().get())
		.map(|col| match columns.get(&col) {
			None => Ok(vec![None; rows]),
			Some(column_cells) => {
				ensure!(
					column_cells.len() >= rows,
					ReconstructionError::InvalidColumn(col)
				);
				let cells = column_cells.values().cloned().collect::<Vec<_>>();

				reconstruct_column(dimensions.extended_rows(), &cells)
					.map(|scalars| scalars.into_iter().map(Some).collect::<Vec<_>>())
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
#[cfg(feature = "std")]
pub fn decode_app_extrinsics(
	index: &DataLookup,
	dimensions: matrix::Dimensions,
	cells: Vec<data::DataCell>,
	app_id: AppId,
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
			.ok_or(ReconstructionError::MissingCell(position))?;
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

	const_assert!((config::CHUNK_SIZE as u64) <= (u32::MAX as u64));
	let ranges = index
		.projected_range_of(app_id, config::CHUNK_SIZE as u32)
		.map(|range| vec![(app_id, range)])
		.unwrap_or_default();

	Ok(unflatten_padded_data(ranges, app_data)
		.map_err(ReconstructionError::DataDecodingError)?
		.into_iter()
		.flat_map(|(_, data)| data)
		.collect::<Vec<_>>())
}

#[derive(Error, Clone, Debug)]
pub enum UnflattenError {
	#[error("`AppDataRange` cannot be converted into `Range<usize>`")]
	RangeConversion(#[from] TryFromIntError),
	#[error("`AppData` cannot be decoded due to {0}")]
	Codec(#[from] codec::Error),
	#[error("Invalid data size, it needs to be a multiple of CHUNK_SIZE")]
	InvalidLen,
}

#[cfg(feature = "std")]
impl std::error::Error for UnflattenError {
	fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
		match &self {
			Self::RangeConversion(try_int) => Some(try_int),
			Self::Codec(codec) => Some(codec),
			_ => None,
		}
	}
}

#[cfg(feature = "std")]
// Removes both extrinsics and block padding (iec_9797 and seeded random data)
pub fn unflatten_padded_data(
	ranges: Vec<(AppId, AppDataRange)>,
	data: Vec<u8>,
) -> Result<Vec<(AppId, AppData)>, UnflattenError> {
	ensure!(
		data.len() % config::CHUNK_SIZE == 0,
		UnflattenError::InvalidLen
	);

	fn extract_encoded_extrinsic(range_data: &[u8]) -> SparseSliceRead {
		const_assert_ne!(config::CHUNK_SIZE, 0);
		const_assert_ne!(config::DATA_CHUNK_SIZE, 0);

		// INTERNAL: Chunk into 32 bytes (CHUNK_SIZE), then remove padding (0..30 bytes).
		SparseSliceRead::from_iter(
			range_data
				.chunks_exact(config::CHUNK_SIZE)
				.map(|chunk| &chunk[0..config::DATA_CHUNK_SIZE]),
		)
	}

	ranges
		.into_iter()
		.map(|(app_id, range)| {
			//let range = range.start as usize..range.end as usize;
			let range: Range<usize> = range.start.try_into()?..range.end.try_into()?;
			let reader = extract_encoded_extrinsic(&data[range]);
			let extrinsic = <AppData>::decode(&mut IoReader(reader))?;

			Ok((app_id, extrinsic))
		})
		.collect::<Result<Vec<_>, _>>()
}

// This module is taken from https://gist.github.com/itzmeanjan/4acf9338d9233e79cfbee5d311e7a0b4
// which I wrote few months back when exploring polynomial based erasure coding technique !
#[cfg(feature = "std")]
fn reconstruct_poly(
	// domain I'm working with
	// all (i)ffts to be performed on it
	eval_domain: EvaluationDomain,
	// subset of available data
	subset: Vec<Option<BlsScalar>>,
) -> Result<Vec<BlsScalar>, ReconstructionError> {
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
			return Err(ReconstructionError::BadZeroPoly);
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

#[cfg(feature = "std")]
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

#[cfg(feature = "std")]
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
#[cfg(feature = "std")]
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
#[cfg(feature = "std")]
fn unshift_poly(poly: &mut [BlsScalar]) {
	// primitive root of unity
	let shift_factor = BlsScalar::from(5);
	let mut factor_power = BlsScalar::one();

	for coef in poly {
		*coef *= factor_power;
		factor_power *= shift_factor;
	}
}

pub type AppDataRange = Range<u32>;

// use this function for reconstructing back all cells of certain column
// when at least 50% of them are available
//
// if everything goes fine, returned vector in case of success should have
// `row_count`-many cells of some specific column, in coded form
//
// performing one round of ifft should reveal original data which were
// coded together
#[cfg(feature = "std")]
pub fn reconstruct_column(
	row_count: u32,
	cells: &[data::DataCell],
) -> Result<Vec<BlsScalar>, ReconstructionError> {
	// just ensures all rows are from same column !
	// it's required as that's how it's erasure coded during
	// construction in validator node
	fn check_cells(cells: &[data::DataCell]) -> bool {
		if cells.is_empty() {
			return false;
		}
		let first_col = cells[0].position.col;
		cells.iter().all(|c| c.position.col == first_col)
	}

	// given row index in column of interest, finds it if present
	// and returns back wrapped in `Some`, otherwise returns `None`
	fn find_row_by_index(idx: u32, cells: &[data::DataCell]) -> Option<BlsScalar> {
		cells
			.iter()
			.find(|cell| cell.position.row == idx)
			.and_then(|cell| {
				<[u8; BlsScalar::SIZE]>::try_from(&cell.data[..])
					.map(|data| BlsScalar::from_bytes(&data).ok())
					.ok()
					.flatten()
			})
	}

	// row count of data matrix must be power of two !
	let row_count_sz =
		usize::try_from(row_count).map_err(|_| ReconstructionError::RowCountExceeded)?;
	ensure!(row_count % 2 == 0, ReconstructionError::InvalidRowCount);
	ensure!(
		cells.len() >= row_count_sz / 2,
		ReconstructionError::MinCells(row_count_sz / 2)
	);
	ensure!(
		cells.len() <= row_count_sz,
		ReconstructionError::MaxCells(row_count_sz)
	);
	ensure!(
		check_cells(cells),
		ReconstructionError::CellsFromDifferentCols
	);

	let eval_domain = EvaluationDomain::new(row_count_sz)
		.map_err(|_| ReconstructionError::InvalidEvaluationDomain)?;
	let mut subset: Vec<Option<BlsScalar>> = Vec::with_capacity(row_count_sz);

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
		matrix::{Dimensions, Position},
	};

	#[test_case(0 => vec![0] ; "App 0 spans 2 rows form row 0")]
	#[test_case(1 => vec![0, 2] ; "App 1 spans 2 rows from row 0")]
	#[test_case(2 => vec![2] ; "App 2 spans 1 rows from row 2")]
	#[test_case(3 => vec![4, 6] ; "App 3 spans 2 rows from row 4")]
	#[test_case(4 => Vec::<u32>::new() ; "There is no app 4")]
	fn test_app_specific_rows(id: u32) -> Vec<u32> {
		let id_lens: Vec<(u32, u32)> = vec![(0, 2), (1, 3), (2, 3), (3, 8)];
		let index = DataLookup::from_id_and_len_iter(id_lens.into_iter()).unwrap();
		let dimensions = Dimensions::new(8, 4).unwrap();

		app_specific_rows(&index, dimensions, AppId(id))
	}

	fn to_matrix_pos(data: &[(u32, u16)]) -> Vec<Position> {
		data.iter().cloned().map(Position::from).collect()
	}

	#[test_case(0 => to_matrix_pos(&[(0, 0), (0, 1), (0, 2), (0, 3), (2, 0)]) ; "App 0 has five cells")]
	#[test_case(1 => to_matrix_pos(&[(2, 1), (2, 2), (2, 3)]) ; "App 1 has 3 cells")]
	#[test_case(2 => Vec::<Position>::new() ; "App 2 has no cells")]
	fn test_app_specific_cells(app_id: u32) -> Vec<Position> {
		let id_lens: Vec<(u32, usize)> = vec![(0, 5), (1, 3)];
		let index = DataLookup::from_id_and_len_iter(id_lens.into_iter()).unwrap();
		let dimensions = Dimensions::new(4, 4).unwrap();

		app_specific_cells(&index, dimensions, AppId(app_id)).unwrap_or_default()
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

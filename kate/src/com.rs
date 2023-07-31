use core::num::NonZeroU32;
use std::{
	convert::{TryFrom, TryInto},
	mem::size_of,
	num::TryFromIntError,
	time::Instant,
};

use avail_core::{
	data_lookup::Error as DataLookupError, ensure, AppExtrinsic, AppId, BlockLengthColumns,
	BlockLengthRows, DataLookup,
};
use codec::Encode;
use derive_more::Constructor;
use dusk_bytes::Serializable;
use dusk_plonk::{
	commitment_scheme::kzg10,
	error::Error as PlonkError,
	fft::{EvaluationDomain, Evaluations},
	prelude::{BlsScalar, CommitKey},
};
#[cfg(feature = "std")]
use kate_recovery::matrix::Dimensions;
use nalgebra::base::DMatrix;
use rand_chacha::{
	rand_core::{Error as ChaChaError, RngCore, SeedableRng},
	ChaChaRng,
};
use rayon::prelude::*;
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_arithmetic::traits::SaturatedConversion;
use static_assertions::const_assert_eq;
use thiserror_no_std::Error;

use crate::{
	com::kzg10::commitment::Commitment,
	config::{
		COL_EXTENSION, DATA_CHUNK_SIZE, EXTENSION_FACTOR, MAXIMUM_BLOCK_SIZE, MINIMUM_BLOCK_SIZE,
		PROOF_SIZE, ROW_EXTENSION, SCALAR_SIZE,
	},
	metrics::Metrics,
	padded_len_of_pad_iec_9797_1, BlockDimensions, Seed, TryFromBlockDimensionsError, LOG_TARGET,
	U32_USIZE_ERR,
};
#[cfg(feature = "std")]
use kate_recovery::testnet;

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Constructor, Clone, Copy, PartialEq, Eq, Debug)]
pub struct Cell {
	pub row: BlockLengthRows,
	pub col: BlockLengthColumns,
}

#[derive(Error, Debug)]
pub enum Error {
	PlonkError(#[from] PlonkError),
	DuskBytesError(#[from] dusk_bytes::Error),
	MultiproofError(#[from] poly_multiproof::Error),
	CellLengthExceeded,
	BadHeaderHash,
	BlockTooBig,
	InvalidChunkLength,
	DimensionsMismatch,
	ZeroDimension,
	InvalidDimensionExtension,
	DomainSizeInvalid,
	InvalidDataLookup(#[from] DataLookupError),
	Rng(#[from] ChaChaError),
	/// The base grid width, before extension, does not fit cleanly into a domain for FFTs
	BaseGridDomainSizeInvalid(usize),
	/// The extended grid width does not fit cleanly into a domain for FFTs
	ExtendedGridDomianSizeInvalid(usize),
}

impl From<TryFromIntError> for Error {
	fn from(_: TryFromIntError) -> Self {
		Self::ZeroDimension
	}
}

impl From<TryFromBlockDimensionsError> for Error {
	fn from(_: TryFromBlockDimensionsError) -> Self {
		Self::BlockTooBig
	}
}

/// We cannot derive `PartialEq` becasue `PlonkError` does not support it in the current version.
/// and we only need to double check its discriminat for testing.
/// Only needed on tests by now.
#[cfg(test)]
impl PartialEq for Error {
	fn eq(&self, other: &Self) -> bool {
		std::mem::discriminant(self) == std::mem::discriminant(other)
	}
}

pub type XtsLayout = Vec<(AppId, u32)>;
type FlatData = Vec<u8>;
type DataChunk = [u8; DATA_CHUNK_SIZE];
const PADDING_TAIL_VALUE: u8 = 0x80;
/// Helper which groups extrinsics data that share the same app_id.
/// We assume the input extrinsics are already sorted by app_id, i.e. extrinsics with the same app_id are consecutive.
/// This function does the same thing as group_by (unstable), just less general.
fn app_extrinsics_group_by_app_id(extrinsics: &[AppExtrinsic]) -> Vec<(AppId, Vec<Vec<u8>>)> {
	extrinsics.iter().fold(vec![], |mut acc, e| {
		match acc.last_mut() {
			Some((app_id, data)) if e.app_id == *app_id => data.push(e.data.clone()),
			None | Some(_) => acc.push((e.app_id, vec![e.data.clone()])),
		}
		acc
	})
}

pub fn flatten_and_pad_block(
	max_rows: BlockLengthRows,
	max_cols: BlockLengthColumns,
	chunk_size: NonZeroU32,
	extrinsics: &[AppExtrinsic],
	rng_seed: Seed,
) -> Result<(XtsLayout, FlatData, BlockDimensions), Error> {
	// First, sort the extrinsics by their app_id
	let mut extrinsics = extrinsics.to_vec();
	extrinsics.sort_by(|a, b| a.app_id.cmp(&b.app_id));

	// Pad data before determining exact block size
	// Padding occurs both inside a single chunk and with additional chunk (if needed)
	let (tx_layout, padded_chunks): (Vec<_>, Vec<_>) = app_extrinsics_group_by_app_id(&extrinsics)
		.iter()
		.map(|e| {
			let app_id = e.0;
			let data = e.1.encode();
			let chunks = pad_iec_9797_1(data);
			let chunks_len = u32::try_from(chunks.len()).map_err(|_| Error::BlockTooBig)?;
			Ok(((app_id, chunks_len), chunks))
		})
		.collect::<Result<Vec<_>, Error>>()?
		.into_iter()
		.unzip();

	let mut padded_block = padded_chunks
		.into_iter()
		.flat_map(|e| {
			e.into_iter()
				.flat_map(|e| pad_to_chunk(e, chunk_size))
				.collect::<Vec<_>>()
		})
		.collect::<Vec<_>>();
	let padded_block_len: u32 = padded_block
		.len()
		.try_into()
		.map_err(|_| Error::BlockTooBig)?;

	// Determine the block size after padding
	let block_dims = get_block_dimensions(padded_block_len, max_rows, max_cols, chunk_size)?;
	let chunk_size = usize::try_from(NonZeroU32::get(block_dims.chunk_size)).expect(U32_USIZE_ERR);

	let block_dims_size = block_dims.size();
	ensure!(padded_block.len() <= block_dims_size, Error::BlockTooBig);

	let mut rng = ChaChaRng::from_seed(rng_seed);

	// SAFETY: `padded_block.len() <= block_dims.size()` checked some lines above.
	if cfg!(debug_assertions) {
		let dims_sub_pad = block_dims_size
			.checked_sub(padded_block.len())
			.expect("`padded_block.len() <= block_dims.size() .qed");
		let rem = dims_sub_pad
			.checked_rem(chunk_size)
			.expect("`chunk_size != 0 .qed");
		assert_eq!(rem, 0);
	}

	#[allow(clippy::integer_arithmetic)]
	// SAFETY: `chunk_size` comes from `NonZeroU32::get(...)` so we can safetly use `/`.
	let last = block_dims_size.saturating_sub(padded_block.len()) / chunk_size;
	for _ in 0..last {
		let mut rnd_values = DataChunk::default();
		rng.try_fill_bytes(&mut rnd_values)?;
		padded_block.append(&mut pad_with_zeroes(rnd_values.to_vec(), chunk_size));
	}

	Ok((tx_layout, padded_block, block_dims))
}

pub fn get_block_dimensions(
	block_size: u32,
	max_rows: BlockLengthRows,
	max_cols: BlockLengthColumns,
	chunk_size: NonZeroU32,
) -> Result<BlockDimensions, Error> {
	let max_block_dimensions =
		BlockDimensions::new(max_rows, max_cols, chunk_size).ok_or(Error::BlockTooBig)?;
	let max_block_dimensions_size = max_block_dimensions.size();

	let block_size = usize::try_from(block_size)?;
	ensure!(block_size <= max_block_dimensions_size, Error::BlockTooBig);

	if block_size == max_block_dimensions_size || MAXIMUM_BLOCK_SIZE {
		return Ok(max_block_dimensions);
	}

	// Both row number and column number have to be a power of 2, because of the Plonk FFT constraints
	// Implicitly, if both of the assumptions above are correct, the total_cells number will also be a power of 2
	let mut nearest_power_2_size = 2_usize.pow((block_size as f32).log2().ceil() as u32);
	if nearest_power_2_size < MINIMUM_BLOCK_SIZE {
		nearest_power_2_size = MINIMUM_BLOCK_SIZE;
	}

	let total_cells = (nearest_power_2_size as f32 / chunk_size.get() as f32).ceil() as u32;

	// we must minimize number of rows, to minimize header size
	// (performance wise it doesn't matter)
	let nz_max_cols = NonZeroU32::new(max_cols.0).ok_or(Error::ZeroDimension)?;
	let (cols, rows) = if total_cells > max_cols.0 {
		(max_cols, BlockLengthRows(total_cells / nz_max_cols))
	} else {
		(BlockLengthColumns(total_cells), BlockLengthRows(1))
	};

	BlockDimensions::new(rows, cols, chunk_size).ok_or(Error::BlockTooBig)
}

#[inline]
fn pad_with_zeroes(mut chunk: Vec<u8>, len: usize) -> Vec<u8> {
	chunk.resize(len, 0);
	chunk
}

fn pad_to_chunk(chunk: DataChunk, chunk_size: NonZeroU32) -> Vec<u8> {
	const_assert_eq!(DATA_CHUNK_SIZE, size_of::<DataChunk>());
	let chunk_size = usize::try_from(chunk_size.get()).expect(U32_USIZE_ERR);
	debug_assert!(
		chunk_size >= DATA_CHUNK_SIZE,
		"`BlockLength.chunk_size` is valid by design .qed"
	);

	let mut padded = chunk.to_vec();
	padded.resize(chunk_size, 0);
	padded
}

fn pad_iec_9797_1(mut data: Vec<u8>) -> Vec<DataChunk> {
	let padded_size = padded_len_of_pad_iec_9797_1(data.len().saturated_into());
	// Add `PADDING_TAIL_VALUE` and fill with zeros.
	data.push(PADDING_TAIL_VALUE);
	data.resize(padded_size as usize, 0u8);

	// Transform into `DataChunk`.
	const_assert_eq!(DATA_CHUNK_SIZE, size_of::<DataChunk>());
	data.chunks(DATA_CHUNK_SIZE)
		.map(|e| e.try_into())
		.collect::<Result<Vec<DataChunk>, _>>()
		.expect("Const assertion ensures this transformation to `DataChunk`. qed")
}

fn extend_column_with_zeros(column: &[BlsScalar], height: usize) -> Vec<BlsScalar> {
	let mut extended = Vec::with_capacity(height);
	let copied = core::cmp::min(height, column.len());

	extended.extend_from_slice(&column[..copied]);
	extended.resize(height, BlsScalar::zero());

	extended
}

pub fn to_bls_scalar(chunk: &[u8]) -> Result<BlsScalar, Error> {
	// TODO: Better error type for BlsScalar case?
	let scalar_size_chunk =
		<[u8; SCALAR_SIZE]>::try_from(chunk).map_err(|_| Error::InvalidChunkLength)?;
	BlsScalar::from_bytes(&scalar_size_chunk).map_err(|_| Error::CellLengthExceeded)
}

fn make_dims(bd: BlockDimensions) -> Result<Dimensions, Error> {
	Dimensions::new_from(bd.rows.0, bd.cols.0).ok_or(Error::ZeroDimension)
}

/// Build extended data matrix, by columns.
/// We are using dusk plonk for erasure coding,
/// which is using roots of unity as evaluation domain for fft and ifft.
/// This means that extension factor has to be multiple of 2,
/// and that original data will be interleaved with erasure codes,
/// instead of being in first k chunks of a column.
///
/// `block` should be the raw data of a matrix, stored in row-major orientation.
#[cfg(feature = "parallel")]
pub fn par_extend_data_matrix<M: Metrics>(
	block_dims: BlockDimensions,
	block: &[u8],
	metrics: &M,
) -> Result<DMatrix<BlsScalar>, Error> {
	let start = Instant::now();
	let dims = make_dims(block_dims)?;
	let (ext_rows, _): (usize, usize) = dims
		.extend(ROW_EXTENSION, COL_EXTENSION)
		.ok_or(Error::InvalidDimensionExtension)?
		.into();
	let (rows, cols) = dims.into();

	// simple length with mod check would work...
	let chunk_size =
		usize::try_from(block_dims.chunk_size.get()).map_err(|_| Error::BlockTooBig)?;

	let chunks = block.par_chunks_exact(chunk_size);
	ensure!(chunks.remainder().is_empty(), Error::DimensionsMismatch);

	let scalars = chunks
		.into_par_iter()
		.map(to_bls_scalar)
		.collect::<Result<Vec<BlsScalar>, Error>>()?;

	let extended_column_eval_domain = EvaluationDomain::new(ext_rows)?;
	let column_eval_domain = EvaluationDomain::new(rows)?; // rows_num = column_length

	// The data is currently row-major, so we need to put it into column-major
	let col_wise_scalars = DMatrix::from_row_iterator(rows, cols, scalars.into_iter());

	let ext_columns_wise = (0..cols)
		.into_par_iter()
		.flat_map(|col| {
			let col_view = col_wise_scalars.column(col).data.into_slice();
			debug_assert_eq!(col_view.len(), rows);
			let mut ext_col = extend_column_with_zeros(col_view, ext_rows);
			// (i)fft functions input parameter slice size has to be a power of 2, otherwise it panics
			column_eval_domain.ifft_slice(&mut ext_col[0..rows]);
			extended_column_eval_domain.fft_slice(ext_col.as_mut_slice());
			debug_assert_eq!(ext_col.len(), ext_rows);
			ext_col
		})
		.collect::<Vec<_>>();
	debug_assert_eq!(Some(ext_columns_wise.len()), cols.checked_mul(ext_rows));

	let ext_matrix = DMatrix::from_iterator(ext_rows, cols, ext_columns_wise.into_iter());

	metrics.extended_block_time(start.elapsed());

	Ok(ext_matrix)
}

//TODO cache extended data matrix
//TODO explore faster Variable Base Multi Scalar Multiplication
pub fn build_proof<M: Metrics>(
	public_params: &kzg10::PublicParameters,
	block_dims: BlockDimensions,
	ext_data_matrix: &DMatrix<BlsScalar>,
	cells: &[Cell],
	metrics: &M,
) -> Result<Vec<u8>, Error> {
	let dims = make_dims(block_dims)?;
	let (ext_rows, ext_cols): (usize, usize) = dims
		.extend(ROW_EXTENSION, COL_EXTENSION)
		.ok_or(Error::InvalidDimensionExtension)?
		.into();
	let (_, cols): (usize, usize) = dims.into();

	const SPROOF_SIZE: usize = PROOF_SIZE + SCALAR_SIZE;

	let (prover_key, _) = public_params.trim(cols).map_err(Error::from)?;

	// Generate all the x-axis points of the domain on which all the row polynomials reside
	let row_eval_domain = EvaluationDomain::new(cols)?;
	let row_dom_x_pts = row_eval_domain.elements().collect::<Vec<_>>();

	let mut result_bytes: Vec<u8> = vec![0u8; SPROOF_SIZE.saturating_mul(cells.len())];

	let prover_key = &prover_key;
	let row_dom_x_pts = &row_dom_x_pts;

	// generate proof only for requested cells
	let total_start = Instant::now();

	// attempt to parallelly compute proof for all requested cells
	let cell_iter = cells.iter().zip(result_bytes.chunks_exact_mut(SPROOF_SIZE));

	for (cell, res) in cell_iter {
		let r_index = usize::try_from(cell.row.0)?;
		if r_index >= ext_rows || cell.col >= block_dims.cols {
			res.fill(0); // for bad cell identifier, fill whole proof with zero bytes !
		} else {
			let c_index = usize::try_from(cell.col.0)?;
			let get_ext_data_matrix =
				|j: usize| ext_data_matrix[r_index.saturating_add(j.saturating_mul(ext_rows))];

			// construct polynomial per extended matrix row
			#[cfg(feature = "parallel")]
			let row = {
				let mut row =
					Vec::with_capacity(ext_cols.checked_add(1).ok_or(Error::BlockTooBig)?);
				(0..ext_cols)
					.into_par_iter()
					.map(get_ext_data_matrix)
					.collect_into_vec(&mut row);
				row
			};
			#[cfg(not(feature = "parallel"))]
			let row = (0..ext_cols)
				.map(get_ext_data_matrix)
				.collect::<Vec<BlsScalar>>();

			// row has to be a power of 2, otherwise interpolate() function panics TODO: cache evaluations
			let poly = Evaluations::from_vec_and_domain(row, row_eval_domain).interpolate();
			let witness = prover_key.compute_single_witness(&poly, &row_dom_x_pts[c_index]);
			match prover_key.commit(&witness) {
				Ok(commitment_to_witness) => {
					let evaluated_point =
						ext_data_matrix[r_index.saturating_add(c_index.saturating_mul(ext_rows))];

					res[0..PROOF_SIZE].copy_from_slice(&commitment_to_witness.to_bytes());
					res[PROOF_SIZE..].copy_from_slice(&evaluated_point.to_bytes());
				},
				Err(_) => {
					res.fill(0); // for bad cell identifier, fill whole proof with zero bytes !
				},
			};
		}
	}

	metrics.proof_build_time(total_start.elapsed(), cells.len().saturated_into());

	Ok(result_bytes)
}

#[cfg(feature = "std")]
pub fn par_build_commitments<M: Metrics>(
	rows: BlockLengthRows,
	cols: BlockLengthColumns,
	chunk_size: NonZeroU32,
	extrinsics_by_key: &[AppExtrinsic],
	rng_seed: Seed,
	metrics: &M,
) -> Result<(XtsLayout, Vec<u8>, BlockDimensions, DMatrix<BlsScalar>), Error> {
	let start = Instant::now();

	// generate data matrix first
	let (tx_layout, block, block_dims) =
		flatten_and_pad_block(rows, cols, chunk_size, extrinsics_by_key, rng_seed)?;

	metrics.block_dims_and_size(block_dims, block.len().saturated_into());

	let ext_matrix = par_extend_data_matrix(block_dims, &block, metrics)?;

	let block_dims_cols = usize::try_from(block_dims.cols.0)?;
	let block_dims_rows = usize::try_from(block_dims.rows.0)?;
	let extended_rows = block_dims_rows
		.checked_mul(EXTENSION_FACTOR as usize)
		.ok_or(Error::BlockTooBig)?;

	metrics.preparation_block_time(start.elapsed());

	let public_params = testnet::public_params(block_dims_cols);

	if log::log_enabled!(target: LOG_TARGET, log::Level::Debug) {
		let raw_pp = public_params.to_raw_var_bytes();
		let hash_pp = hex::encode(sp_core::hashing::blake2_128(&raw_pp));
		let hex_pp = hex::encode(raw_pp);
		log::debug!(
			target: LOG_TARGET,
			"Public params (len={}): hash: {}",
			hex_pp.len(),
			hash_pp,
		);
	}

	let (prover_key, _) = public_params.trim(block_dims_cols)?;
	let row_eval_domain = EvaluationDomain::new(block_dims_cols)?;

	let start = Instant::now();
	let mut commitments =
		Vec::with_capacity(extended_rows.checked_add(1).ok_or(Error::BlockTooBig)?);
	(0..extended_rows)
		.into_par_iter()
		.map(|row_idx| {
			let ext_row = get_row(&ext_matrix, row_idx);
			commit(&prover_key, row_eval_domain, ext_row)
		})
		.collect_into_vec(&mut commitments);

	let commitments = commitments.into_iter().collect::<Result<Vec<_>, _>>()?;
	let commitments_bytes = commitments
		.into_par_iter()
		.flat_map(|c| c.to_bytes())
		.collect();

	metrics.commitment_build_time(start.elapsed());

	Ok((tx_layout, commitments_bytes, block_dims, ext_matrix))
}

#[cfg(feature = "std")]
fn get_row(m: &DMatrix<BlsScalar>, row_idx: usize) -> Vec<BlsScalar> {
	m.row(row_idx).iter().cloned().collect()
}

#[cfg(feature = "std")]
// Generate a commitment
fn commit(
	prover_key: &CommitKey,
	domain: EvaluationDomain,
	row: Vec<BlsScalar>,
) -> Result<Commitment, Error> {
	let poly = Evaluations::from_vec_and_domain(row, domain).interpolate();
	prover_key.commit(&poly).map_err(Error::from)
}

#[cfg(feature = "std")]
pub fn scalars_to_app_rows(
	id: AppId,
	lookup: &DataLookup,
	dimensions: Dimensions,
	matrix: &DMatrix<BlsScalar>,
) -> Vec<Option<Vec<u8>>> {
	let app_rows = kate_recovery::com::app_specific_rows(lookup, dimensions, id);
	dimensions
		.iter_extended_rows()
		.map(|i| {
			app_rows.iter().find(|&&row| row == i).map(|_| {
				let row = get_row(matrix, i as usize);
				row.iter()
					.flat_map(BlsScalar::to_bytes)
					.collect::<Vec<u8>>()
			})
		})
		.collect()
}

#[cfg(feature = "std")]
pub fn scalars_to_rows(rows: &[u32], data: &DMatrix<BlsScalar>) -> Vec<Vec<u8>> {
	rows.iter()
		.map(|i| {
			let row = get_row(data, *i as usize);
			row.iter()
				.flat_map(BlsScalar::to_bytes)
				.collect::<Vec<u8>>()
		})
		.collect::<Vec<Vec<u8>>>()
}

#[cfg(test)]
mod tests {
	use avail_core::DataLookup;
	use dusk_bytes::Serializable;
	use dusk_plonk::bls12_381::BlsScalar;
	use hex_literal::hex;
	use kate_recovery::{
		com::*,
		commitments, config,
		config::CHUNK_SIZE,
		data::{self, DataCell},
		matrix::{Dimensions, Position},
		proof,
	};
	use proptest::{
		collection::{self, size_range},
		prelude::*,
	};
	use rand::{prelude::IteratorRandom, Rng, SeedableRng};
	use sp_arithmetic::Percent;
	use static_assertions::const_assert;
	use std::{convert::TryInto, iter::repeat};
	use test_case::test_case;

	use super::*;
	use crate::{
		com::{get_block_dimensions, pad_iec_9797_1, par_extend_data_matrix, BlockDimensions},
		config::DATA_CHUNK_SIZE,
		metrics::IgnoreMetrics,
		padded_len,
	};

	const TCHUNK: NonZeroU32 = unsafe { NonZeroU32::new_unchecked(32) };
	#[test_case(0,   256, 256 => (1, 4, 32) ; "block size zero")]
	#[test_case(11,   256, 256 => (1, 4, 32) ; "below minimum block size")]
	#[test_case(300,  256, 256 => (1, 16, 32) ; "regular case")]
	#[test_case(513,  256, 256 => (1, 32, 32) ; "minimum overhead after 512")]
	#[test_case(8192, 256, 256 => (1, 256, 32) ; "maximum cols")]
	#[test_case(8224, 256, 256 => (2, 256, 32) ; "two rows")]
	#[test_case(2097152, 256, 256 => (256, 256, 32) ; "max block size")]
	#[test_case(2097155, 256, 256 => panics "BlockTooBig" ; "too much data")]
	// newapi done
	fn test_get_block_dimensions(size: u32, rows: u32, cols: u32) -> (u32, u32, u32) {
		let dims = get_block_dimensions(
			size,
			BlockLengthRows(rows),
			BlockLengthColumns(cols),
			TCHUNK,
		)
		.unwrap();

		(dims.rows.0, dims.cols.0, dims.chunk_size.get())
	}

	// newapi done
	#[test]
	fn test_extend_data_matrix() {
		let expected = [
			// Col 0
			hex!("000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e00"),
			hex!("bc1c6b8b4b02ca677b825ec9dace9aa706813f3ec47abdf9f03c680f4468555e"),
			hex!("7c7d7e7f808182838485868788898a8b8c8d8e8f909192939495969798999a00"),
			hex!("c16115f73784be22106830c9bc6bbb469bf5026ee80325e403efe5ccc3f55016"),
			// Col 1
			hex!("1f202122232425262728292a2b2c2d2e2f303132333435363738393a3b3c3d00"),
			hex!("db3b8aaa6a21e9869aa17de8f9edb9c625a05e5de399dc18105c872e6387745e"),
			hex!("9b9c9d9e9fa0a1a2a3a4a5a6a7a8a9aaabacadaeafb0b1b2b3b4b5b6b7b8b900"),
			hex!("e080341657a3dd412f874fe8db8ada65ba14228d07234403230e05ece2147016"),
			// Col 2
			hex!("3e3f404142434445464748494a4b4c4d4e4f505152535455565758595a5b5c00"),
			hex!("fa5aa9c9894008a6b9c09c07190dd9e544bf7d7c02b9fb372f7ba64d82a6935e"),
			hex!("babbbcbdbebfc0c1c2c3c4c5c6c7c8c9cacbcccdcecfd0d1d2d3d4d5d6d7d800"),
			hex!("ff9f533576c2fc604ea66e07fba9f984d93341ac26426322422d240b02348f16"),
			// Col 3
			hex!("5d5e5f606162636465666768696a6b6c6d6e6f707172737475767778797a7b00"),
			hex!("197ac8e8a85f27c5d8dfbb26382cf80464de9c9b21d81a574e9ac56ca1c5b25e"),
			hex!("d9dadbdcdddedfe0e1e2e3e4e5e6e7e8e9eaebecedeeeff0f1f2f3f4f5f6f700"),
			hex!("1ebf725495e11b806dc58d261ac918a4f85260cb45618241614c432a2153ae16"),
		]
		.iter()
		.map(BlsScalar::from_bytes)
		.collect::<Result<Vec<_>, _>>()
		.expect("Invalid Expected result");
		let expected = DMatrix::from_iterator(4, 4, expected.into_iter());

		let block_dims =
			BlockDimensions::new(BlockLengthRows(2), BlockLengthColumns(4), TCHUNK).unwrap();
		let chunk_size = usize::try_from(block_dims.chunk_size.get()).unwrap();
		let block = (0..=247)
			.collect::<Vec<u8>>()
			.chunks_exact(DATA_CHUNK_SIZE)
			.flat_map(|chunk| pad_with_zeroes(chunk.to_vec(), chunk_size))
			.collect::<Vec<u8>>();
		let ext_matrix = par_extend_data_matrix(block_dims, &block, &IgnoreMetrics {}).unwrap();
		assert_eq!(ext_matrix, expected);
	}

	#[test_case( 1..=29 => "0102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d8000" ; "chunk more than 3 values shorter")]
	#[test_case( 1..=30 => "0102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e80" ; "Chunk 2 values shorter")]
	#[test_case( 1..=31 => "0102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f80000000000000000000000000000000000000000000000000000000000000" ; "Chunk 1 value shorter")]
	#[test_case( 1..=32 => "0102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f20800000000000000000000000000000000000000000000000000000000000" ; "Chunk same size")]
	#[test_case( 1..=33 => "0102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f20218000000000000000000000000000000000000000000000000000000000" ; "Chunk 1 value longer")]
	#[test_case( 1..=34 => "0102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f20212280000000000000000000000000000000000000000000000000000000" ; "Chunk 2 value longer")]
	// newapi ignore
	fn test_padding<I: Iterator<Item = u8>>(block: I) -> String {
		let padded = pad_iec_9797_1(block.collect())
			.iter()
			.flat_map(|e| e.to_vec())
			.collect::<Vec<_>>();

		hex::encode(padded)
	}

	// newapi done
	#[test]
	fn test_flatten_block() {
		let extrinsics: Vec<AppExtrinsic> = vec![
			AppExtrinsic::new(AppId(0), (1..=29).collect()),
			AppExtrinsic::new(AppId(1), (1..=30).collect()),
			AppExtrinsic::new(AppId(2), (1..=31).collect()),
			AppExtrinsic::new(AppId(3), (1..=60).collect()),
		];

		let expected_dims =
			BlockDimensions::new(BlockLengthRows(1), BlockLengthColumns(16), TCHUNK).unwrap();
		let (layout, data, dims) = flatten_and_pad_block(
			BlockLengthRows(128),
			BlockLengthColumns(256),
			TCHUNK,
			extrinsics.as_slice(),
			Seed::default(),
		)
		.unwrap();

		let expected_layout = vec![(AppId(0), 2), (AppId(1), 2), (AppId(2), 2), (AppId(3), 3)];
		assert_eq!(layout, expected_layout, "The layouts don't match");

		let expected_data = hex!("04740102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d00800000000000000000000000000000000000000000000000000000000000000004780102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d001e80000000000000000000000000000000000000000000000000000000000000047c0102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d001e1f80000000000000000000000000000000000000000000000000000000000004f00102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d001e1f202122232425262728292a2b2c2d2e2f303132333435363738393a3b3c00800000000000000000000000000000000000000000000000000000000000000076b8e0ada0f13d90405d6ae55386bd28bdd219b8a08ded1aa836efcc8b770d00da41597c5157488d7724e03fb8d84a376a43b8f41518a11cc387b669b2ee65009f07e7be5551387a98ba977c732d080dcb0f29a048e3656912c6533e32ee7a0029b721769ce64e43d57133b074d839d531ed1f28510afb45ace10a1f4b794d002d09a0e663266ce1ae7ed1081968a0758e718e997bd362c6b0c34634a9a0b300012737681f7b5d0f281e3afde458bc1e73d2d313c9cf94c05ff3716240a248001320a058d7b3566bd520daaa3ed2bf0ac5b8b120fb852773c3639734b45c9100");

		assert_eq!(dims, expected_dims, "Dimensions don't match the expected");
		assert_eq!(data, expected_data, "Data doesn't match the expected data");
		let lookup = DataLookup::from_id_and_len_iter(layout.into_iter()).unwrap();

		const_assert!((CHUNK_SIZE as u64) <= (u32::MAX as u64));
		let data_lookup = lookup.projected_ranges(CHUNK_SIZE as u32).unwrap();
		let res = unflatten_padded_data(data_lookup, data).unwrap();
		assert_eq!(
			res.len(),
			extrinsics.len(),
			"Number of extrinsics is not as expected."
		);

		for ((id, data), exp) in res.iter().zip(extrinsics.iter()) {
			assert_eq!(id.0, *exp.app_id);
			assert_eq!(data[0], exp.data);
		}
	}

	fn sample_cells_from_matrix(
		matrix: &DMatrix<BlsScalar>,
		columns: Option<&[u16]>,
	) -> Vec<DataCell> {
		fn random_indexes(length: usize, seed: Seed) -> Vec<usize> {
			// choose random len/2 (unique) indexes
			let mut idx = (0..length).collect::<Vec<_>>();
			let mut chosen_idx = Vec::<usize>::new();
			let mut rng = ChaChaRng::from_seed(seed);

			for _ in 0..length / 2 {
				let i = rng.gen_range(0..idx.len());
				let v = idx.remove(i);
				chosen_idx.push(v);
			}
			chosen_idx
		}
		const RNG_SEED: Seed = [42u8; 32];

		let (rows, cols) = matrix.shape();
		let cols = u16::try_from(cols).unwrap();
		let indexes = random_indexes(rows, RNG_SEED);

		(0u16..cols)
			.filter(|col_idx| match &columns {
				None => true,
				Some(allowed) => allowed.contains(&col_idx),
			})
			.flat_map(|col_idx| {
				let col_view = matrix.column(col_idx.into()).data.into_slice();

				indexes
					.iter()
					.map(|row_idx| {
						let row_pos = u32::try_from(*row_idx).unwrap();
						let position = Position::new(row_pos, col_idx);
						debug_assert!(*row_idx < col_view.len());
						let data = col_view[*row_idx].to_bytes();
						DataCell::new(position, data)
					})
					.collect::<Vec<_>>()
			})
			.collect()
	}

	fn app_extrinsic_strategy() -> impl Strategy<Value = AppExtrinsic> {
		(
			any::<u32>(),
			any_with::<Vec<u8>>(size_range(1..2048).lift()),
		)
			.prop_map(|(app_id, data)| AppExtrinsic {
				app_id: AppId(app_id),
				data,
			})
	}

	fn app_extrinsics_strategy() -> impl Strategy<Value = Vec<AppExtrinsic>> {
		collection::vec(app_extrinsic_strategy(), size_range(1..16)).prop_map(|xts| {
			let mut new_xts = xts;
			new_xts.sort_by(|a1, a2| a1.app_id.cmp(&a2.app_id));
			new_xts
		})
	}

	fn random_cells(
		max_cols: BlockLengthColumns,
		max_rows: BlockLengthRows,
		percents: Percent,
	) -> Vec<Cell> {
		let max_cols = max_cols.into();
		let max_rows = max_rows.into();

		let rng = &mut ChaChaRng::from_seed([0u8; 32]);
		let amount: usize = percents
			.mul_ceil::<u32>(max_cols * max_rows)
			.saturated_into();

		(0..max_cols)
			.flat_map(move |col| {
				(0..max_rows)
					.map(move |row| Cell::new(BlockLengthRows(row), BlockLengthColumns(col)))
			})
			.choose_multiple(rng, amount)
	}

	proptest! {
	#![proptest_config(ProptestConfig::with_cases(10))]
	#[test]
	// newapi done
	fn test_build_and_reconstruct(ref xts in app_extrinsics_strategy())  {
		let metrics = IgnoreMetrics {};
		let (layout, commitments, dims, matrix) = par_build_commitments(
			BlockLengthRows(64), BlockLengthColumns(16), TCHUNK, xts, Seed::default(), &metrics).unwrap();

		let columns = sample_cells_from_matrix(&matrix, None);
		let extended_dims = dims.try_into().unwrap();
		let index = DataLookup::from_id_and_len_iter(layout.into_iter()).unwrap();
		let reconstructed = reconstruct_extrinsics(&index, extended_dims, columns).unwrap();
		for ((app_id, data), xt) in reconstructed.iter().zip(xts) {
			prop_assert_eq!(app_id.0, *xt.app_id);
			prop_assert_eq!(data[0].as_slice(), &xt.data);
		}

		let dims_cols = usize::try_from(dims.cols.0).unwrap();
		let public_params = testnet::public_params(dims_cols);
		for cell in random_cells(dims.cols, dims.rows, Percent::one() ) {
			let row = usize::try_from(cell.row.0).unwrap();

			let proof = build_proof(&public_params, dims, &matrix, &[cell], &metrics).unwrap();
			prop_assert!(proof.len() == 80);

			let col: u16 = cell.col.0.try_into().expect("`random_cells` function generates a valid `u16` for columns");
			let position = Position { row: cell.row.0, col};
			let cell = data::Cell { position,  content: proof.try_into().unwrap() };

			let extended_dims = dims.try_into().unwrap();
			let commitment = commitments::from_slice(&commitments).unwrap()[row];
			let verification =  proof::verify(&public_params, extended_dims, &commitment,  &cell);
			prop_assert!(verification.is_ok());
			prop_assert!(verification.unwrap());
		}
	}
	}

	proptest! {
	#![proptest_config(ProptestConfig::with_cases(20))]
	#[test]
	// newapi done
	fn test_commitments_verify(ref xts in app_extrinsics_strategy())  {
		let (layout, commitments, dims, matrix) = par_build_commitments(BlockLengthRows(64), BlockLengthColumns(16), TCHUNK, xts, Seed::default(), &IgnoreMetrics{}).unwrap();

		let index = DataLookup::from_id_and_len_iter(layout.into_iter()).unwrap();
		let dims_cols = usize::try_from(dims.cols.0).unwrap();
		let public_params = testnet::public_params(dims_cols);
		let extended_dims = dims.try_into().unwrap();
		let commitments = commitments::from_slice(&commitments).unwrap();
		for xt in xts {
			let rows = scalars_to_app_rows(xt.app_id, &index, extended_dims, &matrix);
			let (_, missing) = commitments::verify_equality(&public_params, &commitments, rows.as_slice(), &index, extended_dims, xt.app_id).unwrap();
			prop_assert!(missing.is_empty());
		}
	}
	}

	proptest! {
	#![proptest_config(ProptestConfig::with_cases(20))]
	#[test]
	// newapi done
	fn verify_commitmnets_missing_row(ref xts in app_extrinsics_strategy())  {
		let (layout, commitments, dims, matrix) = par_build_commitments(BlockLengthRows(64), BlockLengthColumns(16), TCHUNK, xts, Seed::default(), &IgnoreMetrics{}).unwrap();

		let index = DataLookup::from_id_and_len_iter(layout.into_iter()).unwrap();
		let dims_cols = usize::try_from(dims.cols.0).unwrap();
		let public_params = testnet::public_params(dims_cols);
		let extended_dims =  dims.try_into().unwrap();
		let commitments = commitments::from_slice(&commitments).unwrap();
		for xt in xts {
			let mut rows = scalars_to_app_rows(xt.app_id, &index, extended_dims, &matrix);
			let app_row_index = rows.iter().position(Option::is_some).unwrap();
			rows.remove(app_row_index);
			let (_, missing) = commitments::verify_equality(&public_params, &commitments, &rows,&index, extended_dims,xt.app_id).unwrap();
			prop_assert!(!missing.is_empty());
		}
	}
	}

	#[test]
	// Test build_commitments() function with a predefined input
	// newapi done
	fn test_build_commitments_simple_commitment_check() {
		let block_rows = BlockLengthRows(256);
		let block_cols = BlockLengthColumns(256);
		let original_data = br#"test"#;
		let hash: Seed = hex!("4c29ae91bb0c61204b6f95d1f3c3a50aa6ac2f29da18d4423e05bbbf81056903");

		let (_, commitments, dimensions, _) = par_build_commitments(
			block_rows,
			block_cols,
			TCHUNK,
			&[AppExtrinsic::from(original_data.to_vec())],
			hash,
			&IgnoreMetrics {},
		)
		.unwrap();

		assert_eq!(
			dimensions,
			BlockDimensions::new(BlockLengthRows(1), BlockLengthColumns(4), TCHUNK).unwrap(),
		);
		let expected_commitments = hex!("9046c691ce4c7ba93c9860746d6ff3dfb5560e119f1eac26aa9a10b6fe29d5c8e2b90f23e2ef3a7a950965b08035470d9046c691ce4c7ba93c9860746d6ff3dfb5560e119f1eac26aa9a10b6fe29d5c8e2b90f23e2ef3a7a950965b08035470d");
		assert_eq!(commitments, expected_commitments);
	}

	#[test]
	// newapi wip
	fn test_reconstruct_app_extrinsics_with_app_id() -> Result<(), Error> {
		let app_id_1_data = br#""This is mocked test data. It will be formatted as a matrix of BLS scalar cells and then individual columns 
get erasure coded to ensure redundancy."#;

		let app_id_2_data = br#""Let's see how this gets encoded and then reconstructed by sampling only some data."#;

		let hash = Seed::default();
		let xts = vec![
			AppExtrinsic::new(AppId(0), vec![0]),
			AppExtrinsic::new(AppId(1), app_id_1_data.to_vec()),
			AppExtrinsic::new(AppId(2), app_id_2_data.to_vec()),
		];

		let (layout, data, dims) = flatten_and_pad_block(
			BlockLengthRows(32),
			BlockLengthColumns(4),
			TCHUNK,
			&xts,
			hash,
		)?;
		let matrix = par_extend_data_matrix(dims, &data[..], &IgnoreMetrics {})?;

		let cols_1 = sample_cells_from_matrix(&matrix, Some(&[0, 1, 2, 3]));

		let extended_dims = dims.try_into()?;

		let index = DataLookup::from_id_and_len_iter(layout.into_iter()).unwrap();
		let res_1 = reconstruct_app_extrinsics(&index, extended_dims, cols_1, AppId(1)).unwrap();
		assert_eq!(res_1[0], app_id_1_data);

		let cols_2 = sample_cells_from_matrix(&matrix, Some(&[0, 2, 3]));

		let res_2 = reconstruct_app_extrinsics(&index, extended_dims, cols_2, AppId(2)).unwrap();
		assert_eq!(res_2[0], app_id_2_data);
		Ok(())
	}

	#[test]
	// newapi done
	fn test_decode_app_extrinsics() -> Result<(), Error> {
		let app_id_1_data = br#""This is mocked test data. It will be formatted as a matrix of BLS scalar cells and then individual columns 
get erasure coded to ensure redundancy."#;

		let app_id_2_data = br#""Let's see how this gets encoded and then reconstructed by sampling only some data."#;

		let data = [vec![0], app_id_1_data.to_vec(), app_id_2_data.to_vec()];

		let hash = Seed::default();
		let xts = (0..=2)
			.zip(data)
			.map(|(app_id, data)| AppExtrinsic::new(AppId(app_id), data))
			.collect::<Vec<_>>();

		let (layout, data, dims) = flatten_and_pad_block(
			BlockLengthRows(32),
			BlockLengthColumns(4),
			TCHUNK,
			&xts,
			hash,
		)?;
		let matrix = par_extend_data_matrix(dims, &data[..], &IgnoreMetrics {})?;
		let dimensions: Dimensions = dims.try_into()?;

		let index = DataLookup::from_id_and_len_iter(layout.into_iter()).unwrap();
		for xt in xts {
			let positions = app_specific_cells(&index, dimensions, xt.app_id).unwrap();
			let cells = positions
				.into_iter()
				.map(|position| {
					let col: usize = position.col.into();
					let row = usize::try_from(position.row).unwrap();
					let data = matrix.get((row, col)).map(BlsScalar::to_bytes).unwrap();
					DataCell::new(position, data)
				})
				.collect::<Vec<_>>();
			let data = &decode_app_extrinsics(&index, dimensions, cells, xt.app_id).unwrap()[0];
			assert_eq!(data, &xt.data);
		}

		assert!(matches!(
			decode_app_extrinsics(&index, dimensions, vec![], AppId(0)),
			Err(ReconstructionError::MissingCell { .. })
		));
		Ok(())
	}

	#[test]
	// newapi done
	fn test_extend_mock_data() -> Result<(), Error> {
		let orig_data = br#"This is mocked test data. It will be formatted as a matrix of BLS scalar cells and then individual columns 
get erasure coded to ensure redundancy.
Let's see how this gets encoded and then reconstructed by sampling only some data."#;

		// The hash is used for seed for padding the block to next power of two value
		let hash = Seed::default();
		let (layout, data, dims) = flatten_and_pad_block(
			BlockLengthRows(128),
			BlockLengthColumns(2),
			TCHUNK,
			&[AppExtrinsic::from(orig_data.to_vec())],
			hash,
		)?;

		let matrix = par_extend_data_matrix(dims, &data[..], &IgnoreMetrics {})?;

		let cols = sample_cells_from_matrix(&matrix, None);

		let extended_dims = dims.try_into()?;
		let index = DataLookup::from_id_and_len_iter(layout.into_iter()).unwrap();
		let res = reconstruct_extrinsics(&index, extended_dims, cols).unwrap();
		let s = String::from_utf8_lossy(res[0].1[0].as_slice());

		assert_eq!(res[0].1[0], orig_data);
		eprintln!("Decoded: {}", s);
		Ok(())
	}

	#[test]
	// newapi done
	fn test_multiple_extrinsics_for_same_app_id() -> Result<(), Error> {
		let xt1 = vec![5, 5];
		let xt2 = vec![6, 6];
		let xts = [
			AppExtrinsic::new(AppId(1), xt1.clone()),
			AppExtrinsic::new(AppId(1), xt2.clone()),
		];
		// The hash is used for seed for padding the block to next power of two value
		let hash = Seed::default();
		let (layout, data, dims) = flatten_and_pad_block(
			BlockLengthRows(128),
			BlockLengthColumns(2),
			TCHUNK,
			&xts,
			hash,
		)?;

		let matrix = par_extend_data_matrix(dims, &data[..], &IgnoreMetrics {})?;

		let cols = sample_cells_from_matrix(&matrix, None);
		let extended_dims = dims.try_into().unwrap();

		let index = DataLookup::from_id_and_len_iter(layout.into_iter()).unwrap();
		let res = reconstruct_extrinsics(&index, extended_dims, cols).unwrap();

		assert_eq!(res[0].1[0], xt1);
		assert_eq!(res[0].1[1], xt2);
		Ok(())
	}

	#[test]
	// newapi ignore
	fn test_extrinsics_grouping() {
		let xt1 = vec![5, 5];
		let xt2 = vec![6, 6];
		let xt3 = vec![7];
		let xt4 = vec![];
		let xts = [
			AppExtrinsic::new(AppId(1), xt1.clone()),
			AppExtrinsic::new(AppId(1), xt2.clone()),
			AppExtrinsic::new(AppId(2), xt3.clone()),
			AppExtrinsic::new(AppId(3), xt4.clone()),
		];

		let expected = vec![
			(AppId(1), vec![xt1, xt2]),
			(AppId(2), vec![xt3]),
			(AppId(3), vec![xt4]),
		];
		let rez = app_extrinsics_group_by_app_id(&xts);
		println!("{:?}", rez);

		assert_eq!(rez, expected);
	}

	fn build_extrinsics(lens: &[usize]) -> Vec<Vec<u8>> {
		lens.iter()
			.map(|len| repeat(b'a').take(*len).collect::<Vec<_>>())
			.collect()
	}

	fn padded_len_group(lens: &[u32], chunk_size: u32) -> u32 {
		let chunk_size = NonZeroU32::new(chunk_size).unwrap();
		lens.iter().map(|len| padded_len(*len, chunk_size)).sum()
	}

	#[test_case( build_extrinsics(&[5,30,31]), 32 => padded_len_group(&[5,30,31], 32) ; "Single chunk per ext")]
	#[test_case( build_extrinsics(&[5,30,32]), 32 => padded_len_group(&[5,30,32], 32) ; "Extra chunk per ext")]
	#[test_case( build_extrinsics(&[5,64,120]), 32 => padded_len_group(&[5,64,120], 32) ; "Extra chunk 2 per ext")]
	#[test_case( build_extrinsics(&[]), 32 => padded_len_group(&[], 32) ; "Empty chunk list")]
	#[test_case( build_extrinsics(&[4096]), 32 => padded_len_group(&[4096], 32) ; "4K chunk")]
	fn test_padding_len(extrinsics: Vec<Vec<u8>>, chunk_size: u32) -> u32 {
		let chunk_size = NonZeroU32::new(chunk_size).expect("Invalid chunk size .qed");
		extrinsics
			.into_iter()
			.flat_map(pad_iec_9797_1)
			.map(|chunk| pad_to_chunk(chunk, chunk_size).len())
			.sum::<usize>()
			.saturated_into()
	}

	#[test]
	// newapi ignore
	fn par_build_commitments_column_wise_constant_row() {
		// This test will fail once we switch to row-wise orientation.
		// We should move `should_panic` to next test, until constant line issue is fixed.
		// After the fix, should_panic should be removed.
		let hash = Seed::default();
		let data = (0..3).flat_map(|i| vec![i; 31]).collect::<Vec<_>>();
		let xts = (0..4)
			.map(|app_id| AppExtrinsic {
				app_id: AppId(app_id),
				data: data.clone(),
			})
			.collect::<Vec<_>>();
		par_build_commitments(
			BlockLengthRows(4),
			BlockLengthColumns(4),
			TCHUNK,
			&xts,
			hash,
			&IgnoreMetrics {},
		)
		.unwrap();
	}

	#[test]
	// newapi done
	fn par_build_commitments_row_wise_constant_row() {
		// Due to scale encoding, first line is not constant.
		// We will use second line to ensure constant row.
		let hash = Seed::default();
		let xts = vec![AppExtrinsic {
			app_id: AppId(0),
			data: vec![0; 31 * 8],
		}];
		par_build_commitments(
			BlockLengthRows(4),
			BlockLengthColumns(4),
			TCHUNK,
			&xts,
			hash,
			&IgnoreMetrics {},
		)
		.unwrap();
	}
	#[test_case( ([1,1,1,1]).to_vec(); "All values are non-zero but same")]
	#[test_case( ([0,0,0,0]).to_vec(); "All values are zero")]
	#[test_case( ([0,5,2,1]).to_vec(); "All values are different")]
	// newapi done
	fn test_zero_deg_poly_commit(row_values: Vec<u8>) {
		// There are two main cases that generate a zero degree polynomial. One is for data that is non-zero, but the same.
		// The other is for all-zero data. They differ, as the former yields a polynomial with one coefficient, and latter generates zero coefficients.
		let len = row_values.len();
		let public_params = testnet::public_params(len);
		let (prover_key, _) = public_params.trim(len).map_err(Error::from).unwrap();
		let row_eval_domain = EvaluationDomain::new(len).map_err(Error::from).unwrap();

		let row = row_values
			.iter()
			.map(|val| {
				let mut value = [0u8; 32];
				let v = value.last_mut().unwrap();
				*v = *val;
				BlsScalar::from_bytes(&value).unwrap()
			})
			.collect::<Vec<_>>();

		assert_eq!(row.len(), len);
		println!("Row: {:?}", row);
		let commitment = commit(&prover_key, row_eval_domain, row.clone())
			.map(|com| <[u8; config::COMMITMENT_SIZE]>::try_from(com.to_bytes()).unwrap())
			.unwrap();
		println!("Commitment: {commitment:?}");

		// We artificially extend the matrix by doubling values, this is not proper erasure coding.
		let ext_m =
			DMatrix::from_row_iterator(1, row.len() * 2, row.into_iter().flat_map(|e| vec![e, e]));

		let rows: u16 = len.try_into().expect("rows length should be valid `u16`");
		let metrics = IgnoreMetrics {};

		for col in 0..rows {
			// Randomly chosen cell to prove, probably should test all of them
			let cell = Cell {
				col: BlockLengthColumns(col.into()),
				row: BlockLengthRows(0),
			};
			let proof = build_proof(
				&public_params,
				BlockDimensions::new(BlockLengthRows(1), BlockLengthColumns(4), TCHUNK).unwrap(),
				&ext_m,
				&[cell],
				&metrics,
			)
			.unwrap();
			println!("Proof: {proof:?}");

			assert_eq!(proof.len(), 80);

			let dims = Dimensions::new(1, 4).unwrap();
			let cell = data::Cell {
				position: Position { row: 0, col },
				content: proof.try_into().unwrap(),
			};
			let verification = proof::verify(&public_params, dims, &commitment, &cell);
			assert!(verification.is_ok());
			assert!(verification.unwrap())
		}
	}

	#[test_case( r#"{ "row": 42, "col": 99 }"# => Cell::new(BlockLengthRows(42), BlockLengthColumns(99)) ; "Simple" )]
	#[test_case( r#"{ "row": 4294967295, "col": 99 }"# => Cell::new(BlockLengthRows(4_294_967_295),BlockLengthColumns(99)) ; "Max row" )]
	// newapi ignore
	fn serde_block_length_types_untagged(data: &str) -> Cell {
		serde_json::from_str(data).unwrap()
	}
}

use std::{
	convert::{TryFrom, TryInto},
	mem::size_of,
	time::Instant,
};

use codec::Encode;
use da_primitives::asdr::{AppExtrinsic, AppId};
use dusk_bytes::Serializable;
use dusk_plonk::{
	commitment_scheme::kzg10,
	error::Error as PlonkError,
	fft::{EvaluationDomain, Evaluations},
	prelude::{BlsScalar, CommitKey},
};
use frame_support::ensure;
#[cfg(feature = "std")]
use kate_recovery::{com::app_specific_rows, index, matrix};
use log::info;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaChaRng;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use static_assertions::const_assert_eq;

#[cfg(feature = "std")]
use crate::testnet;
use crate::{
	config::{
		DATA_CHUNK_SIZE, EXTENSION_FACTOR, MAXIMUM_BLOCK_SIZE, MINIMUM_BLOCK_SIZE, PROOF_SIZE,
		PROVER_KEY_SIZE, SCALAR_SIZE,
	},
	padded_len_of_pad_iec_9797_1, BlockDimensions, Seed, LOG_TARGET,
};

#[derive(Serialize, Deserialize)]
pub struct Cell {
	pub row: u32,
	pub col: u32,
}

#[derive(Debug)]
pub enum Error {
	PlonkError(PlonkError),
	CellLenghtExceeded,
	BadHeaderHash,
	BlockTooBig,
	InvalidChunkLength,
}

impl From<PlonkError> for Error {
	fn from(error: PlonkError) -> Self {
		Self::PlonkError(error)
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

#[cfg(feature = "std")]
pub fn scalars_to_rows(
	app_id: u32,
	index: &index::AppDataIndex,
	dimensions: &matrix::Dimensions,
	data: &[BlsScalar],
) -> Vec<Option<Vec<u8>>> {
	let extended_rows = dimensions.extended_rows() as usize;
	let cols = dimensions.cols as usize;
	let app_rows = app_specific_rows(index, dimensions, app_id);
	dimensions
		.iter_extended_rows()
		.map(|i| {
			app_rows.iter().find(|&&row| row == i).map(|_| {
				row(data, i as usize, cols, extended_rows)
					.iter()
					.flat_map(BlsScalar::to_bytes)
					.collect::<Vec<u8>>()
			})
		})
		.collect::<Vec<Option<Vec<u8>>>>()
}

pub fn flatten_and_pad_block(
	max_rows_num: usize,
	max_cols_num: usize,
	chunk_size: usize,
	extrinsics: &[AppExtrinsic],
	rng_seed: Seed,
) -> Result<(XtsLayout, FlatData, BlockDimensions), Error> {
	// First, sort the extrinsics by their app_id
	let mut extrinsics = extrinsics.to_vec();
	extrinsics.sort_by(|a, b| a.app_id.cmp(&b.app_id));

	let extrinsics = app_extrinsics_group_by_app_id(&extrinsics)
		.iter()
		.map(|e| (e.0, e.1.encode()))
		.collect::<Vec<_>>();

	// Pad data before determining exact block size
	// Padding occurs both inside a single chunk and with additional chunk (if needed)
	let (tx_layout, padded_chunks): (Vec<_>, Vec<_>) = extrinsics
		.iter()
		.map(|(app_id, data)| {
			let chunks = pad_iec_9797_1(data.clone());
			((*app_id, chunks.len() as u32), chunks)
		})
		.unzip();

	let mut padded_block = padded_chunks
		.into_iter()
		.flat_map(|e| {
			e.into_iter()
				.flat_map(|e| pad_to_chunk(e, chunk_size))
				.collect::<Vec<_>>()
		})
		.collect::<Vec<_>>();

	// Determine the block size after padding
	let block_dims =
		get_block_dimensions(padded_block.len(), max_rows_num, max_cols_num, chunk_size)?;

	ensure!(padded_block.len() <= block_dims.size(), Error::BlockTooBig);

	let mut rng = ChaChaRng::from_seed(rng_seed);

	assert!((block_dims.size() - padded_block.len()) % block_dims.chunk_size == 0);

	for _ in 0..((block_dims.size() - padded_block.len()) / block_dims.chunk_size) {
		let rnd_values: DataChunk = rng.gen();
		padded_block.append(&mut pad_with_zeroes(rnd_values.to_vec(), chunk_size));
	}

	Ok((tx_layout, padded_block, block_dims))
}

pub fn get_block_dimensions(
	block_size: usize,
	max_rows_num: usize,
	max_cols_num: usize,
	chunk_size: usize,
) -> Result<BlockDimensions, Error> {
	let max_block_dimensions = BlockDimensions {
		rows: max_rows_num,
		cols: max_cols_num,
		chunk_size,
	};

	ensure!(
		block_size <= max_block_dimensions.size(),
		Error::BlockTooBig
	);

	if block_size == max_block_dimensions.size() || MAXIMUM_BLOCK_SIZE {
		return Ok(max_block_dimensions);
	}

	// Both row number and column number have to be a power of 2, because of the Plonk FFT constraints
	// Implicitly, if both of the assumptions above are correct, the total_cells number will also be a power of 2
	let mut nearest_power_2_size = 2_usize.pow((block_size as f32).log2().ceil() as u32);
	if nearest_power_2_size < MINIMUM_BLOCK_SIZE {
		nearest_power_2_size = MINIMUM_BLOCK_SIZE;
	}

	let total_cells = (nearest_power_2_size as f32 / chunk_size as f32).ceil() as usize;

	// we must minimize number of rows, to minimize header size
	// (performance wise it doesn't matter)
	let (cols, rows) = if total_cells > max_cols_num {
		(max_cols_num, total_cells / max_cols_num)
	} else {
		(total_cells, 1)
	};

	Ok(BlockDimensions {
		cols,
		rows,
		chunk_size,
	})
}

#[inline]
fn pad_with_zeroes(mut chunk: Vec<u8>, length: usize) -> Vec<u8> {
	chunk.resize(length, 0);
	chunk
}

fn pad_to_chunk(chunk: DataChunk, chunk_size: usize) -> Vec<u8> {
	const_assert_eq!(DATA_CHUNK_SIZE, size_of::<DataChunk>());
	debug_assert!(
		chunk_size >= DATA_CHUNK_SIZE,
		"`BlockLength.chunk_size` is valid by design .qed"
	);

	let mut padded = chunk.to_vec();
	padded.resize(chunk_size, 0);
	padded
}

fn pad_iec_9797_1(mut data: Vec<u8>) -> Vec<DataChunk> {
	let padded_size = padded_len_of_pad_iec_9797_1(data.len() as u32);
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

fn extend_column_with_zeros(column: &[BlsScalar], extended_rows_num: usize) -> Vec<BlsScalar> {
	let mut result = column.to_vec();
	result.resize(extended_rows_num, BlsScalar::zero());
	result
}

pub fn to_bls_scalar(chunk: &[u8]) -> Result<BlsScalar, Error> {
	// TODO: Better error type for BlsScalar case?
	let scalar_size_chunk =
		<[u8; SCALAR_SIZE]>::try_from(chunk).map_err(|_| Error::InvalidChunkLength)?;
	BlsScalar::from_bytes(&scalar_size_chunk).map_err(|_| Error::CellLenghtExceeded)
}

/// Build extended data matrix, by columns.
/// We are using dusk plonk for erasure coding,
/// which is using roots of unity as evaluation domain for fft and ifft.
/// This means that extension factor has to be multiple of 2,
/// and that original data will be interleaved with erasure codes,
/// instead of being in first k chunks of a column.
#[cfg(feature = "alloc")]
pub fn par_extend_data_matrix(
	block_dims: BlockDimensions,
	block: &[u8],
) -> Result<Vec<BlsScalar>, Error> {
	use kate_recovery::matrix::Dimensions;

	let start = Instant::now();
	let dimensions = Dimensions::new(block_dims.rows as u16, block_dims.cols as u16);
	let rows_num: usize = dimensions.rows.into();
	let extended_rows_num: usize = dimensions.extended_rows() as usize;
	let chunks = block.par_chunks_exact(block_dims.chunk_size);
	assert!(chunks.remainder().is_empty());

	let scalars = chunks
		.into_par_iter()
		.map(to_bls_scalar)
		.collect::<Result<Vec<BlsScalar>, Error>>()?;

	let mut row_wise_scalars = Vec::with_capacity(dimensions.size() as usize);
	dimensions
		.iter_cells()
		.for_each(|cell_i| row_wise_scalars.push(scalars[cell_i as usize]));

	let mut chunk_elements = row_wise_scalars
		.par_chunks_exact(rows_num)
		.flat_map(|column| extend_column_with_zeros(column, extended_rows_num))
		.collect::<Vec<BlsScalar>>();

	// extend data matrix, column by column
	let extended_column_eval_domain = EvaluationDomain::new(extended_rows_num)?;
	let column_eval_domain = EvaluationDomain::new(rows_num)?; // rows_num = column_length

	chunk_elements
		.par_chunks_exact_mut(extended_rows_num)
		.for_each(|col| {
			// (i)fft functions input parameter slice size has to be a power of 2, otherwise it panics
			column_eval_domain.ifft_slice(&mut col[0..rows_num]);
			extended_column_eval_domain.fft_slice(col);
		});

	info!(
		target: LOG_TARGET,
		"Time to extend block {:?}",
		start.elapsed()
	);

	Ok(chunk_elements)
}

//TODO cache extended data matrix
//TODO explore faster Variable Base Multi Scalar Multiplication
pub fn build_proof(
	public_params: &kzg10::PublicParameters,
	block_dims: BlockDimensions,
	ext_data_matrix: &[BlsScalar],
	cells: &[Cell],
) -> Result<Vec<u8>, Error> {
	let rows_num = block_dims.rows;
	let cols_num = block_dims.cols;
	let extended_rows_num = rows_num * EXTENSION_FACTOR;

	const SPROOF_SIZE: usize = PROOF_SIZE + SCALAR_SIZE;

	let (prover_key, _) = public_params.trim(cols_num).map_err(Error::from)?;

	// Generate all the x-axis points of the domain on which all the row polynomials reside
	let row_eval_domain = EvaluationDomain::new(cols_num).map_err(Error::from)?;
	let mut row_dom_x_pts = Vec::with_capacity(row_eval_domain.size());
	row_dom_x_pts.extend(row_eval_domain.elements());

	let mut result_bytes: Vec<u8> = Vec::new();
	result_bytes.reserve_exact(SPROOF_SIZE * cells.len());
	unsafe {
		result_bytes.set_len(SPROOF_SIZE * cells.len());
	}

	let prover_key = &prover_key;
	let row_dom_x_pts = &row_dom_x_pts;

	info!(
		target: LOG_TARGET,
		"Number of CPU cores: {:#?}",
		num_cpus::get()
	);
	// generate proof only for requested cells
	let total_start = Instant::now();

	// attempt to parallelly compute proof for all requested cells
	cells
		.into_par_iter()
		.zip(result_bytes.par_chunks_exact_mut(SPROOF_SIZE))
		.for_each(|(cell, res)| {
			let r_index = cell.row as usize;
			let c_index = cell.col as usize;

			if (r_index >= extended_rows_num) || (c_index >= cols_num) {
				res.fill(0); // for bad cell identifier, fill whole proof with zero bytes !
			} else {
				// construct polynomial per extended matrix row
				let row = (0..cols_num)
					.into_par_iter()
					.map(|j| ext_data_matrix[r_index + j * extended_rows_num])
					.collect::<Vec<BlsScalar>>();

				// row has to be a power of 2, otherwise interpolate() function panics
				let poly = Evaluations::from_vec_and_domain(row, row_eval_domain).interpolate();
				let witness = prover_key.compute_single_witness(&poly, &row_dom_x_pts[c_index]);
				match prover_key.commit(&witness) {
					Ok(commitment_to_witness) => {
						let evaluated_point =
							ext_data_matrix[r_index + c_index * extended_rows_num];

						res[0..PROOF_SIZE].copy_from_slice(&commitment_to_witness.to_bytes());
						res[PROOF_SIZE..].copy_from_slice(&evaluated_point.to_bytes());
					},
					Err(_) => {
						res.fill(0); // for bad cell identifier, fill whole proof with zero bytes !
						return;
					},
				};
			}
		});

	info!(
		target: LOG_TARGET,
		"Time to build proofs of {} cells: {:?}",
		cells.len(),
		total_start.elapsed()
	);

	Ok(result_bytes)
}

#[cfg(feature = "std")]
pub fn par_build_commitments(
	rows_num: usize,
	cols_num: usize,
	chunk_size: usize,
	extrinsics_by_key: &[AppExtrinsic],
	rng_seed: Seed,
) -> Result<(XtsLayout, Vec<u8>, BlockDimensions, Vec<BlsScalar>), Error> {
	let start = Instant::now();

	// generate data matrix first
	let (tx_layout, block, block_dims) =
		flatten_and_pad_block(rows_num, cols_num, chunk_size, extrinsics_by_key, rng_seed)?;

	info!(
		target: LOG_TARGET,
		"Rows: {} Cols: {} Size: {}",
		block_dims.rows,
		block_dims.cols,
		block.len(),
	);

	let ext_data_matrix = par_extend_data_matrix(block_dims, &block)?;
	let extended_rows_num = block_dims.rows * EXTENSION_FACTOR;

	info!(target: LOG_TARGET, "Time to prepare {:?}", start.elapsed());

	let public_params = testnet::public_params(block_dims.cols);

	if log::log_enabled!(target: LOG_TARGET, log::Level::Debug) {
		let raw_pp = public_params.to_raw_var_bytes();
		let hash_pp = hex::encode(sp_core::blake2_128(&raw_pp));
		let hex_pp = hex::encode(raw_pp);
		log::debug!(
			target: LOG_TARGET,
			"Public params (len={}): hash: {}",
			hex_pp.len(),
			hash_pp,
		);
	}

	let (prover_key, _) = public_params.trim(block_dims.cols).map_err(Error::from)?;
	let row_eval_domain = EvaluationDomain::new(block_dims.cols).map_err(Error::from)?;

	let mut result_bytes: Vec<u8> = Vec::new();
	result_bytes.reserve_exact(PROVER_KEY_SIZE * extended_rows_num);
	unsafe {
		result_bytes.set_len(PROVER_KEY_SIZE * extended_rows_num);
	}

	info!(
		target: "system",
		"Number of CPU cores: {:#?}",
		num_cpus::get()
	);

	let start = Instant::now();

	(0..extended_rows_num)
		.into_par_iter()
		.map(|i| row(&ext_data_matrix, i, block_dims.cols, extended_rows_num))
		.zip(result_bytes.par_chunks_exact_mut(PROVER_KEY_SIZE))
		.map(|(row, res)| commit(&prover_key, row_eval_domain, row, res))
		.collect::<Result<_, _>>()?;

	info!(
		target: "system",
		"Time to build a commitment {:?}",
		start.elapsed()
	);

	Ok((tx_layout, result_bytes, block_dims, ext_data_matrix))
}

#[cfg(feature = "std")]
fn row(matrix: &[BlsScalar], i: usize, cols: usize, extended_rows: usize) -> Vec<BlsScalar> {
	let mut row = Vec::with_capacity(cols);
	for j in 0..cols as usize {
		row.push(matrix[i + j * extended_rows]);
	}
	row
}

#[cfg(feature = "std")]
// Generate a commitment and store it into result
fn commit(
	prover_key: &CommitKey,
	domain: EvaluationDomain,
	row: Vec<BlsScalar>,
	result: &mut [u8],
) -> Result<(), Error> {
	let poly = Evaluations::from_vec_and_domain(row, domain).interpolate();
	let commitment = prover_key.commit(&poly).map_err(Error::from)?;
	result.copy_from_slice(&commitment.to_bytes());
	Ok(())
}

#[cfg(test)]
mod tests {
	use std::{convert::TryInto, iter::repeat, str::from_utf8};

	use da_primitives::asdr::AppExtrinsic;
	use dusk_bytes::Serializable;
	use dusk_plonk::bls12_381::BlsScalar;
	use hex_literal::hex;
	use kate_recovery::{
		com::{
			app_specific_cells, decode_app_extrinsics, reconstruct_app_extrinsics,
			reconstruct_extrinsics, unflatten_padded_data, ReconstructionError,
		},
		data::DataCell,
		index::{AppDataIndex, AppDataIndexError},
		matrix::{Dimensions, Position},
	};
	use proptest::{
		collection::{self, size_range},
		prelude::*,
	};
	use rand::{prelude::IteratorRandom, Rng, SeedableRng};
	use test_case::test_case;

	use super::*;
	use crate::{
		com::{get_block_dimensions, pad_iec_9797_1, par_extend_data_matrix, BlockDimensions},
		config::DATA_CHUNK_SIZE,
		padded_len,
	};

	fn app_data_index_try_from_layout(
		layout: Vec<(AppId, u32)>,
	) -> Result<AppDataIndex, AppDataIndexError> {
		let mut index = Vec::new();
		// transactions are ordered by application id
		// skip transactions with 0 application id - it's not a data txs
		let mut size = 0u32;
		let mut prev_app_id = AppId(0u32);

		for (app_id, data_len) in layout {
			if app_id.0 != 0 && prev_app_id != app_id {
				index.push((app_id.0, size));
			}

			size = size
				.checked_add(data_len)
				.ok_or(AppDataIndexError::SizeOverflow)?;
			if prev_app_id > app_id {
				return Err(AppDataIndexError::UnsortedLayout);
			}
			prev_app_id = app_id;
		}

		Ok(AppDataIndex { size, index })
	}

	#[test_case(0,   256, 256 => BlockDimensions { rows: 1, cols: 4  , chunk_size: 32} ; "block size zero")]
	#[test_case(11,   256, 256 => BlockDimensions { rows: 1, cols: 4  , chunk_size: 32} ; "below minimum block size")]
	#[test_case(300,  256, 256 => BlockDimensions { rows: 1, cols: 16 , chunk_size: 32} ; "regular case")]
	#[test_case(513,  256, 256 => BlockDimensions { rows: 1, cols: 32 , chunk_size: 32} ; "minimum overhead after 512")]
	#[test_case(8192, 256, 256 => BlockDimensions { rows: 1, cols: 256, chunk_size: 32} ; "maximum cols")]
	#[test_case(8224, 256, 256 => BlockDimensions { rows: 2, cols: 256, chunk_size: 32} ; "two rows")]
	#[test_case(2097152, 256, 256 => BlockDimensions { rows: 256, cols: 256, chunk_size: 32} ; "max block size")]
	#[test_case(2097155, 256, 256 => panics "BlockTooBig" ; "too much data")]
	fn test_get_block_dimensions(size: usize, rows: usize, cols: usize) -> BlockDimensions {
		get_block_dimensions(size, rows, cols, 32).unwrap()
	}

	#[test]
	fn test_extend_data_matrix() {
		let expected_result = vec![
			b"000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e00",
			b"bc1c6b8b4b02ca677b825ec9dace9aa706813f3ec47abdf9f03c680f4468555e",
			b"7c7d7e7f808182838485868788898a8b8c8d8e8f909192939495969798999a00",
			b"c16115f73784be22106830c9bc6bbb469bf5026ee80325e403efe5ccc3f55016",
			b"1f202122232425262728292a2b2c2d2e2f303132333435363738393a3b3c3d00",
			b"db3b8aaa6a21e9869aa17de8f9edb9c625a05e5de399dc18105c872e6387745e",
			b"9b9c9d9e9fa0a1a2a3a4a5a6a7a8a9aaabacadaeafb0b1b2b3b4b5b6b7b8b900",
			b"e080341657a3dd412f874fe8db8ada65ba14228d07234403230e05ece2147016",
			b"3e3f404142434445464748494a4b4c4d4e4f505152535455565758595a5b5c00",
			b"fa5aa9c9894008a6b9c09c07190dd9e544bf7d7c02b9fb372f7ba64d82a6935e",
			b"babbbcbdbebfc0c1c2c3c4c5c6c7c8c9cacbcccdcecfd0d1d2d3d4d5d6d7d800",
			b"ff9f533576c2fc604ea66e07fba9f984d93341ac26426322422d240b02348f16",
			b"5d5e5f606162636465666768696a6b6c6d6e6f707172737475767778797a7b00",
			b"197ac8e8a85f27c5d8dfbb26382cf80464de9c9b21d81a574e9ac56ca1c5b25e",
			b"d9dadbdcdddedfe0e1e2e3e4e5e6e7e8e9eaebecedeeeff0f1f2f3f4f5f6f700",
			b"1ebf725495e11b806dc58d261ac918a4f85260cb45618241614c432a2153ae16",
		]
		.into_iter()
		.map(|e| {
			e.chunks_exact(2)
				.map(|h| u8::from_str_radix(from_utf8(h).unwrap(), 16).unwrap())
				.collect::<Vec<u8>>()
		})
		.map(|e| {
			BlsScalar::from_bytes(e.as_slice().try_into().expect("wrong number of elems")).unwrap()
		})
		.collect::<Vec<_>>();

		let block_dims = BlockDimensions {
			rows: 2,
			cols: 4,
			chunk_size: 32,
		};
		let block = (0..=247)
			.collect::<Vec<u8>>()
			.chunks_exact(DATA_CHUNK_SIZE)
			.map(|chunk| pad_with_zeroes(chunk.to_vec(), block_dims.chunk_size))
			.flatten()
			.collect::<Vec<u8>>();
		let res = par_extend_data_matrix(block_dims, &block);
		eprintln!("result={:?}", res);
		eprintln!("expect={:?}", expected_result);
		assert_eq!(res.unwrap(), expected_result);
	}

	#[test_case( 1..=29 => "0102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d8000" ; "chunk more than 3 values shorter")]
	#[test_case( 1..=30 => "0102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e80" ; "Chunk 2 values shorter")]
	#[test_case( 1..=31 => "0102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f80000000000000000000000000000000000000000000000000000000000000" ; "Chunk 1 value shorter")]
	#[test_case( 1..=32 => "0102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f20800000000000000000000000000000000000000000000000000000000000" ; "Chunk same size")]
	#[test_case( 1..=33 => "0102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f20218000000000000000000000000000000000000000000000000000000000" ; "Chunk 1 value longer")]
	#[test_case( 1..=34 => "0102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f20212280000000000000000000000000000000000000000000000000000000" ; "Chunk 2 value longer")]
	fn test_padding<I: Iterator<Item = u8>>(block: I) -> String {
		let padded = pad_iec_9797_1(block.collect())
			.iter()
			.flat_map(|e| e.to_vec())
			.collect::<Vec<_>>();

		hex::encode(padded)
	}

	#[test]
	fn test_flatten_block() {
		let chunk_size = 32;
		let extrinsics: Vec<AppExtrinsic> = vec![
			AppExtrinsic {
				app_id: 0.into(),
				data: (1..=29).collect(),
			},
			AppExtrinsic {
				app_id: 1.into(),
				data: (1..=30).collect(),
			},
			AppExtrinsic {
				app_id: 2.into(),
				data: (1..=31).collect(),
			},
			AppExtrinsic {
				app_id: 3.into(),
				data: (1..=60).collect(),
			},
		];

		let expected_dims = BlockDimensions {
			rows: 1,
			cols: 16,
			chunk_size,
		};
		let (layout, data, dims) =
			flatten_and_pad_block(128, 256, chunk_size, extrinsics.as_slice(), Seed::default())
				.unwrap();

		let expected_layout = vec![(0.into(), 2), (1.into(), 2), (2.into(), 2), (3.into(), 3)];
		assert_eq!(layout, expected_layout, "The layouts don't match");

		let expected_data = hex!("04740102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d00800000000000000000000000000000000000000000000000000000000000000004780102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d001e80000000000000000000000000000000000000000000000000000000000000047c0102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d001e1f80000000000000000000000000000000000000000000000000000000000004f00102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d001e1f202122232425262728292a2b2c2d2e2f303132333435363738393a3b3c00800000000000000000000000000000000000000000000000000000000000000076a04053bda0a88bda5177b86a15c3b29f559873cb481232299cd5743151ac004b2d63ae198e7bb0a9011f28e473c95f4013d7d53ec5fbc3b42df8ed101f6d00e831e52bfb76e51cca8b4e9016838657edfae09cb9a71eb219025c4c87a67c004aaa86f20ac0aa792bc121ee42e2c326127061eda15599cb5db3db870bea5a00ecf353161c3cb528b0c5d98050c4570bfc942d8b19ed7b0cbba5725e03e5f000b7e30db36b6df82ac151f668f5f80a5e2a9cac7c64991dd6a6ce21c060175800edb9260d2a86c836efc05f17e5c59525e404c6a93d051651fe2e4eefae281300");

		assert_eq!(dims, expected_dims, "Dimensions don't match the expected");
		assert_eq!(data, expected_data, "Data doesn't match the expected data");
		let index = app_data_index_try_from_layout(layout).unwrap();
		let res = unflatten_padded_data(index.data_ranges(), data).unwrap();
		assert_eq!(
			res.len(),
			extrinsics.len(),
			"Number of extrinsics is not as expected."
		);

		for (res, exp) in res.iter().zip(extrinsics.iter()) {
			assert_eq!(res.0, *exp.app_id);
			assert_eq!(res.1[0], exp.data);
		}
	}

	fn sample_cells_from_matrix(
		matrix: &[BlsScalar],
		dimensions: &BlockDimensions,
		columns: Option<&[u16]>,
	) -> Vec<DataCell> {
		fn random_indexes(length: usize, seed: Seed) -> Vec<u16> {
			// choose random len/2 (unique) indexes
			let mut idx = (0..length).collect::<Vec<_>>();
			let mut chosen_idx = Vec::<u16>::new();
			let mut rng = ChaChaRng::from_seed(seed);

			for _ in 0..length / 2 {
				let i = rng.gen_range(0..idx.len());
				let v = idx.remove(i);
				chosen_idx.push(v as u16);
			}
			chosen_idx
		}

		const RNG_SEED: Seed = [42u8; 32];
		matrix
			.chunks_exact(dimensions.rows * 2)
			.enumerate()
			.map(|(col, e)| (col as u16, e))
			.flat_map(|(col, e)| {
				random_indexes(e.len(), RNG_SEED)
					.into_iter()
					.map(|row| DataCell {
						position: Position {
							row: row as u32,
							col,
						},
						data: e[row as usize].to_bytes(),
					})
					.filter(|cell| {
						columns.is_none() || columns.unwrap_or(&[]).contains(&cell.position.col)
					})
					.collect::<Vec<_>>()
			})
			.collect::<Vec<_>>()
	}

	fn app_extrinsic_strategy() -> impl Strategy<Value = AppExtrinsic> {
		(
			any::<u32>(),
			any_with::<Vec<u8>>(size_range(1..2048).lift()),
		)
			.prop_map(|(app_id, data)| AppExtrinsic {
				app_id: app_id.into(),
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

	fn random_cells(cols: usize, rows: usize, percents: usize) -> Vec<Cell> {
		assert!(percents > 0 && percents <= 100);

		let rng = &mut ChaChaRng::from_seed([0u8; 32]);
		let amount = (cols as f32 * rows as f32 * (percents as f32 / 100.0)).ceil() as usize;

		(0..cols)
			.flat_map(move |col| (0..rows).map(move |row| (row, col)))
			.map(|(row, col)| Cell {
				col: col as u32,
				row: row as u32,
			})
			.choose_multiple(rng, amount)
	}

	proptest! {
	#![proptest_config(ProptestConfig::with_cases(20))]
	#[test]
	fn test_build_and_reconstruct(ref xts in app_extrinsics_strategy())  {
		let (layout, commitments, dims, matrix) = par_build_commitments(64, 16, 32, xts, Seed::default()).unwrap();

		let columns = sample_cells_from_matrix(&matrix, &dims, None);
		let extended_dims = Dimensions::new(dims.rows as u16, dims.cols as u16);
		let index = app_data_index_try_from_layout(layout).unwrap();
		let reconstructed = reconstruct_extrinsics(&index, &extended_dims, columns).unwrap();
		for (result, xt) in reconstructed.iter().zip(xts) {
		prop_assert_eq!(result.0, *xt.app_id);
		prop_assert_eq!(result.1[0].as_slice(), &xt.data);
		}

		let public_params = testnet::public_params(dims.cols);
		for cell in random_cells(dims.cols, dims.rows, 1) {
			let col = cell.col;
			let row = cell.row as usize;

			let proof = build_proof(&public_params, dims, &matrix, &[cell]).unwrap();
			prop_assert!(proof.len() == 80);

			let commitment = &commitments[row * 48..(row + 1) * 48];
			let verification =  kate_proof::kc_verify_proof(col, &proof, commitment, dims.rows as usize, dims.cols as usize, &public_params);
			prop_assert!(verification.is_ok());
		}
	}
	}

	proptest! {
	#![proptest_config(ProptestConfig::with_cases(20))]
	#[test]
	fn test_commitments_verify(ref xts in app_extrinsics_strategy())  {
		let (layout, commitments, dims, matrix) = par_build_commitments(64, 16, 32, xts, Seed::default()).unwrap();

		let index = app_data_index_try_from_layout(layout).unwrap();
		let public_params = testnet::public_params(dims.cols);
		let extended_dims = Dimensions::new(dims.rows as u16, dims.cols as u16);
		for xt in xts {
			let rows = &scalars_to_rows(xt.app_id.0, &index, &extended_dims, &matrix);
			prop_assert!(kate_recovery::commitments::verify_equality(&public_params, &commitments, rows,&index,&extended_dims,xt.app_id.0).unwrap());
		}
	}
	}

	proptest! {
	#![proptest_config(ProptestConfig::with_cases(20))]
	#[test]
	fn verify_commitmnets_missing_row(ref xts in app_extrinsics_strategy())  {
		let (layout, commitments, dims, matrix) = par_build_commitments(64, 16, 32, xts, Seed::default()).unwrap();

		let index = app_data_index_try_from_layout(layout).unwrap();
		let public_params = testnet::public_params(dims.cols);
		let extended_dims = Dimensions::new(dims.rows as u16, dims.cols as u16);
		for xt in xts {
			let mut rows = scalars_to_rows(xt.app_id.0, &index, &extended_dims, &matrix);
			let app_row_index = rows.iter().position(Option::is_some).unwrap();
			rows.remove(app_row_index);
			prop_assert!(kate_recovery::commitments::verify_equality(&public_params, &commitments, &rows,&index,&extended_dims,xt.app_id.0).is_err());
		}
	}
	}

	#[test]
	// Test build_commitments() function with a predefined input
	fn test_build_commitments_simple_commitment_check() {
		let block_rows = 256;
		let block_cols = 256;
		let chunk_size = 32;
		let original_data = br#"test"#;
		let hash: Seed = [
			76, 41, 174, 145, 187, 12, 97, 32, 75, 111, 149, 209, 243, 195, 165, 10, 166, 172, 47,
			41, 218, 24, 212, 66, 62, 5, 187, 191, 129, 5, 105, 3,
		];

		let (_, commitments, dimensions, _) = par_build_commitments(
			block_rows,
			block_cols,
			chunk_size,
			&[AppExtrinsic::from(original_data.to_vec())],
			hash,
		)
		.unwrap();

		assert_eq!(
			dimensions,
			BlockDimensions {
				rows: 1,
				cols: 4,
				chunk_size: 32
			}
		);
		let expected_commitments = hex!("960F08F97D3A8BD21C3F5682366130132E18E375A587A1E5900937D7AA5F33C4E20A1C0ACAE664DCE1FD99EDC2693B8D960F08F97D3A8BD21C3F5682366130132E18E375A587A1E5900937D7AA5F33C4E20A1C0ACAE664DCE1FD99EDC2693B8D");
		assert_eq!(commitments, expected_commitments);
	}

	#[test]
	fn test_reconstruct_app_extrinsics_with_app_id() {
		let app_id_1_data = br#""This is mocked test data. It will be formatted as a matrix of BLS scalar cells and then individual columns 
get erasure coded to ensure redundancy."#;

		let app_id_2_data = br#""Let's see how this gets encoded and then reconstructed by sampling only some data."#;

		let hash = Seed::default();
		let xts = vec![
			AppExtrinsic {
				app_id: 0.into(),
				data: vec![0],
			},
			AppExtrinsic {
				app_id: 1.into(),
				data: app_id_1_data.to_vec(),
			},
			AppExtrinsic {
				app_id: 2.into(),
				data: app_id_2_data.to_vec(),
			},
		];

		let chunk_size = 32;

		let (layout, data, dims) = flatten_and_pad_block(32, 4, chunk_size, &xts, hash).unwrap();
		let coded: Vec<BlsScalar> = par_extend_data_matrix(dims, &data[..]).unwrap();

		let cols_1 = sample_cells_from_matrix(&coded, &dims, Some(&[0, 1, 2, 3]));

		let extended_dims = Dimensions::new(dims.rows as u16, dims.cols as u16);

		let index = app_data_index_try_from_layout(layout).unwrap();
		let res_1 = reconstruct_app_extrinsics(&index, &extended_dims, cols_1, 1).unwrap();
		assert_eq!(res_1[0], app_id_1_data);

		let cols_2 = sample_cells_from_matrix(&coded, &dims, Some(&[0, 2, 3]));

		let res_2 = reconstruct_app_extrinsics(&index, &extended_dims, cols_2, 2).unwrap();
		assert_eq!(res_2[0], app_id_2_data);
	}

	#[test]
	fn test_decode_app_extrinsics() {
		let app_id_1_data = br#""This is mocked test data. It will be formatted as a matrix of BLS scalar cells and then individual columns 
get erasure coded to ensure redundancy."#;

		let app_id_2_data = br#""Let's see how this gets encoded and then reconstructed by sampling only some data."#;

		let data = [vec![0], app_id_1_data.to_vec(), app_id_2_data.to_vec()];

		let hash = Seed::default();
		let xts = (0..=2)
			.zip(data)
			.map(|(app_id, data)| AppExtrinsic {
				app_id: app_id.into(),
				data,
			})
			.collect::<Vec<_>>();

		let chunk_size = 32;

		let (layout, data, dims) = flatten_and_pad_block(32, 4, chunk_size, &xts, hash).unwrap();
		let coded = par_extend_data_matrix(dims, &data[..]).unwrap();

		let dimensions = Dimensions::new(dims.rows as u16, dims.cols as u16);
		let extended_matrix = coded
			.chunks(dimensions.extended_rows() as usize)
			.collect::<Vec<_>>();

		let index = app_data_index_try_from_layout(layout).unwrap();
		for xt in xts {
			let positions = app_specific_cells(&index, &dimensions, xt.app_id.0).unwrap();
			let cells = positions
				.iter()
				.map(|position| DataCell {
					position: position.clone(),
					data: extended_matrix[position.col as usize][position.row as usize].to_bytes(),
				})
				.collect::<Vec<_>>();
			let data = &decode_app_extrinsics(&index, &dimensions, cells, xt.app_id.0).unwrap()[0];
			assert_eq!(data, &xt.data);
		}

		assert!(matches!(
			decode_app_extrinsics(&index, &dimensions, vec![], 0),
			Err(ReconstructionError::MissingCell { .. })
		));
	}

	#[test]
	fn test_extend_mock_data() {
		let orig_data = br#"This is mocked test data. It will be formatted as a matrix of BLS scalar cells and then individual columns 
get erasure coded to ensure redundancy.
Let's see how this gets encoded and then reconstructed by sampling only some data."#;

		// The hash is used for seed for padding the block to next power of two value
		let hash = Seed::default();
		let chunk_size = 32;
		let (layout, data, dims) = flatten_and_pad_block(
			128,
			2,
			chunk_size,
			&[AppExtrinsic::from(orig_data.to_vec())],
			hash,
		)
		.unwrap();

		let coded: Vec<BlsScalar> = par_extend_data_matrix(dims, &data[..]).unwrap();

		let cols = sample_cells_from_matrix(&coded, &dims, None);

		let extended_dims = Dimensions::new(dims.rows as u16, dims.cols as u16);
		let index = app_data_index_try_from_layout(layout).unwrap();
		let res = reconstruct_extrinsics(&index, &extended_dims, cols).unwrap();
		let s = String::from_utf8_lossy(res[0].1[0].as_slice());

		assert_eq!(res[0].1[0], orig_data);

		eprintln!("Decoded: {}", s);
	}

	#[test]
	fn test_multiple_extrinsics_for_same_app_id() {
		let xt1 = vec![5, 5];
		let xt2 = vec![6, 6];
		let xts = [
			AppExtrinsic {
				app_id: 1.into(),
				data: xt1.clone(),
			},
			AppExtrinsic {
				app_id: 1.into(),
				data: xt2.clone(),
			},
		];
		// The hash is used for seed for padding the block to next power of two value
		let hash = Seed::default();
		let chunk_size = 32;
		let (layout, data, dims) = flatten_and_pad_block(128, 2, chunk_size, &xts, hash).unwrap();

		let coded: Vec<BlsScalar> = par_extend_data_matrix(dims, &data[..]).unwrap();

		let cols = sample_cells_from_matrix(&coded, &dims, None);
		let extended_dims = Dimensions::new(dims.rows as u16, dims.cols as u16);

		let index = app_data_index_try_from_layout(layout).unwrap();
		let res = reconstruct_extrinsics(&index, &extended_dims, cols).unwrap();

		assert_eq!(res[0].1[0], xt1);
		assert_eq!(res[0].1[1], xt2);
	}

	#[test]
	fn test_extrinsics_grouping() {
		let xt1 = vec![5, 5];
		let xt2 = vec![6, 6];
		let xt3 = vec![7];
		let xt4 = vec![];
		let xts = [
			AppExtrinsic {
				app_id: 1.into(),
				data: xt1.clone(),
			},
			AppExtrinsic {
				app_id: 1.into(),
				data: xt2.clone(),
			},
			AppExtrinsic {
				app_id: 2.into(),
				data: xt3.clone(),
			},
			AppExtrinsic {
				app_id: 3.into(),
				data: xt4.clone(),
			},
		];

		let expected = vec![
			(1.into(), vec![xt1, xt2]),
			(2.into(), vec![xt3]),
			(3.into(), vec![xt4]),
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
		lens.iter().map(|len| padded_len(*len, chunk_size)).sum()
	}

	#[test_case( build_extrinsics(&[5,30,31]), 32 => padded_len_group(&[5,30,31], 32) ; "Single chunk per ext")]
	#[test_case( build_extrinsics(&[5,30,32]), 32 => padded_len_group(&[5,30,32], 32) ; "Extra chunk per ext")]
	#[test_case( build_extrinsics(&[5,64,120]), 32 => padded_len_group(&[5,64,120], 32) ; "Extra chunk 2 per ext")]
	#[test_case( build_extrinsics(&[]), 32 => padded_len_group(&[], 32) ; "Empty chunk list")]
	#[test_case( build_extrinsics(&[4096]), 32 => padded_len_group(&[4096], 32) ; "4K chunk")]
	fn test_padding_len(extrinsics: Vec<Vec<u8>>, chunk_size: usize) -> u32 {
		extrinsics
			.into_iter()
			.flat_map(pad_iec_9797_1)
			.map(|chunk| pad_to_chunk(chunk, chunk_size).len() as u32)
			.sum()
	}

	#[test]
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
		par_build_commitments(4, 4, 32, &xts, hash).unwrap();
	}

	#[test]
	#[should_panic]
	fn par_build_commitments_row_wise_constant_row() {
		// Due to scale encoding, first line is not constant.
		// We will use second line to ensure constant row.
		let hash = Seed::default();
		let xts = vec![AppExtrinsic {
			app_id: AppId(0),
			data: vec![0; 31 * 8],
		}];
		par_build_commitments(4, 4, 32, &xts, hash).unwrap();
	}
}

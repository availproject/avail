use std::{
	convert::{TryFrom, TryInto},
	time::Instant,
};

use da_primitives::asdr::AppExtrinsic;
use dusk_bytes::Serializable;
use dusk_plonk::{
	commitment_scheme::kzg10,
	error::Error as PlonkError,
	fft::{EvaluationDomain, Evaluations},
	prelude::BlsScalar,
};
use frame_support::ensure;
use log::info;
use rand::{rngs::StdRng, Rng};
use serde::{Deserialize, Serialize};

use super::*;

#[derive(Serialize, Deserialize)]
pub struct Cell {
	pub row: u32,
	pub col: u32,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct BlockDimensions {
	pub rows: usize,
	pub cols: usize,
	pub size: usize,
	pub chunk_size: usize,
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
	fn from(error: PlonkError) -> Self { Self::PlonkError(error) }
}

pub type XtsLayout = Vec<(u32, u32)>;
type FlatData = Vec<u8>;
type DataChunk = [u8; config::DATA_CHUNK_SIZE];
const PADDING_TAIL_VALUE: u8 = 0x80;

pub fn flatten_and_pad_block(
	max_rows_num: usize,
	max_cols_num: usize,
	chunk_size: usize,
	extrinsics: &[AppExtrinsic],
	header_hash: &[u8],
) -> Result<(XtsLayout, FlatData, BlockDimensions), Error> {
	// Pad data before determining exact block size
	// Padding occurs both inside a single chunk and with additional chunk (if needed)
	let (tx_layout, padded_chunks): (Vec<_>, Vec<_>) = extrinsics
		.iter()
		.map(|e| {
			let chunks = pad_iec_9797_1(e.data.as_slice(), config::DATA_CHUNK_SIZE);
			((e.app_id, chunks.len() as u32), chunks)
		})
		.unzip();

	let mut padded_block = padded_chunks
		.into_iter()
		.flat_map(|e| {
			e.into_iter()
				.map(|e| pad_to_chunk(e, chunk_size))
				.flatten()
				.collect::<Vec<_>>()
		})
		.collect::<Vec<_>>();

	// Determine the block size after padding
	let block_dims =
		get_block_dimensions(padded_block.len(), max_rows_num, max_cols_num, chunk_size)?;

	ensure!(padded_block.len() <= block_dims.size, Error::BlockTooBig);

	let seed = <[u8; 32]>::try_from(header_hash).map_err(|_| Error::BadHeaderHash)?;
	let mut rng: StdRng = rand::SeedableRng::from_seed(seed);

	assert!((block_dims.size - padded_block.len()) % block_dims.chunk_size == 0);

	for _ in 0..((block_dims.size - padded_block.len()) / block_dims.chunk_size) {
		let rnd_values: DataChunk = rng.gen();
		padded_block.append(&mut pad_with_zeroes(&rnd_values, chunk_size));
	}

	Ok((tx_layout, padded_block, block_dims))
}

pub fn get_block_dimensions(
	block_size: usize,
	max_rows_num: usize,
	max_cols_num: usize,
	chunk_size: usize,
) -> Result<BlockDimensions, Error> {
	let max_block_size = max_rows_num * max_cols_num * chunk_size;

	ensure!(block_size <= max_block_size, Error::BlockTooBig);

	if block_size == max_block_size {
		return Ok(BlockDimensions {
			cols: max_cols_num,
			rows: max_rows_num,
			size: max_block_size,
			chunk_size,
		});
	}

	// Both row number and column number have to be a power of 2, because of the Plonk FFT constraints
	// Implicitly, if both of the assumptions above are correct, the total_cells number will also be a power of 2
	let mut nearest_power_2_size = 2_usize.pow((block_size as f32).log2().ceil() as u32);
	if nearest_power_2_size < config::MINIMUM_BLOCK_SIZE {
		nearest_power_2_size = config::MINIMUM_BLOCK_SIZE;
	}

	let total_cells = (nearest_power_2_size as f32 / chunk_size as f32).ceil() as usize;

	// we must minimize number of rows, to minimize header size
	// (performance wise it doesn't matter)
	let (cols, rows) = if total_cells > max_cols_num {
		(max_cols_num, total_cells / max_cols_num)
	} else {
		(total_cells, 1)
	};

	let size = rows * cols * chunk_size;

	Ok(BlockDimensions {
		cols,
		rows,
		size,
		chunk_size,
	})
}

fn pad_with_zeroes(chunk: &[u8], length: usize) -> Vec<u8> {
	let mut bytes = chunk.to_vec();
	bytes.resize(length, 0);
	bytes
}

fn pad_to_chunk(chunk: DataChunk, chunk_size: usize) -> Vec<u8> {
	let len = chunk.len();
	let mut padded = chunk.to_vec();
	padded.extend(vec![0].repeat(chunk_size - len));
	padded
}

fn pad_iec_9797_1(data: &[u8], chunk_size: usize) -> Vec<DataChunk> {
	let mut padded = data.to_vec();
	padded.push(PADDING_TAIL_VALUE);
	padded.extend(vec![0].repeat(chunk_size - (data.len() % chunk_size) - 1));

	padded
		.chunks(chunk_size)
		.map(|e| e.try_into().unwrap())
		.collect::<Vec<DataChunk>>()
}

fn extend_column_with_zeros(column: &[BlsScalar], extended_rows_num: usize) -> Vec<BlsScalar> {
	let mut result = column.to_vec();
	result.resize(extended_rows_num, BlsScalar::zero());
	result
}

fn to_bls_scalar(chunk: &[u8]) -> Result<BlsScalar, Error> {
	// TODO: Better error type for BlsScalar case?
	let scalar_size_chunk =
		<[u8; config::SCALAR_SIZE]>::try_from(chunk).map_err(|_| Error::InvalidChunkLength)?;
	BlsScalar::from_bytes(&scalar_size_chunk).map_err(|_| Error::CellLenghtExceeded)
}

#[cfg(feature = "alloc")]
/// build extended data matrix, by columns
pub fn extend_data_matrix(
	block_dims: BlockDimensions,
	block: &[u8],
) -> Result<Vec<BlsScalar>, Error> {
	let start = Instant::now();
	let rows_num = block_dims.rows;
	let extended_rows_num = rows_num * config::EXTENSION_FACTOR;

	let chunks = block.chunks_exact(block_dims.chunk_size);
	assert!(chunks.remainder().is_empty());

	let mut chunk_elements = chunks
		.map(to_bls_scalar)
		.collect::<Result<Vec<BlsScalar>, Error>>()?
		.chunks_exact(rows_num)
		.flat_map(|column| extend_column_with_zeros(column, extended_rows_num))
		.collect::<Vec<BlsScalar>>();

	// extend data matrix, column by column
	let extended_column_eval_domain = EvaluationDomain::new(extended_rows_num)?;
	let column_eval_domain = EvaluationDomain::new(rows_num)?; // rows_num = column_length

	chunk_elements
		.chunks_exact_mut(extended_rows_num)
		.for_each(|col| {
			let half_len = col.len() / 2;
			// (i)fft functions input parameter slice size has to be a power of 2, otherwise it panics
			column_eval_domain.ifft_slice(&mut col[0..half_len]);
			extended_column_eval_domain.fft_slice(col);
		});

	info!(
		target: "system",
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
	let extended_rows_num = rows_num * config::EXTENSION_FACTOR;

	ensure!(
		cells.len() <= config::MAX_PROOFS_REQUEST,
		Error::CellLenghtExceeded
	);

	let (prover_key, _) = public_params.trim(cols_num).map_err(Error::from)?;

	// Generate all the x-axis points of the domain on which all the row polynomials reside
	let row_eval_domain = EvaluationDomain::new(cols_num).map_err(Error::from)?;
	let mut row_dom_x_pts = Vec::with_capacity(row_eval_domain.size());
	row_dom_x_pts.extend(row_eval_domain.elements());

	let mut result_bytes: Vec<u8> = Vec::new();
	let serialized_proof_size = config::SCALAR_SIZE + config::PROOF_SIZE;
	result_bytes.reserve_exact(serialized_proof_size * cells.len());
	unsafe {
		result_bytes.set_len(serialized_proof_size * cells.len());
	}

	let prover_key = &prover_key;
	let row_dom_x_pts = &row_dom_x_pts;
	let mut cell_index = 0;

	info!(
		target: "system",
		"Number of CPU cores: {:#?}",
		num_cpus::get()
	);
	// generate proof only for requested cells
	let total_start = Instant::now();
	cells
		.iter()
		.try_for_each(|cell| -> Result<(), PlonkError> {
			let row_index = cell.row as usize;
			let col_index = cell.col as usize;

			if row_index < extended_rows_num && col_index < cols_num {
				// construct polynomial per extended matrix row
				let mut row = Vec::with_capacity(cols_num);

				for j in 0..cols_num {
					row.push(ext_data_matrix[row_index + j * extended_rows_num]);
				}
				// row has to be a power of 2, otherwise interpolate() function panics
				let polynomial =
					Evaluations::from_vec_and_domain(row, row_eval_domain).interpolate();
				let witness =
					prover_key.compute_single_witness(&polynomial, &row_dom_x_pts[col_index]);
				let commitment_to_witness = prover_key.commit(&witness)?;
				let evaluated_point = ext_data_matrix[row_index + col_index * extended_rows_num];

				unsafe {
					std::ptr::copy(
						commitment_to_witness.to_bytes().as_ptr(),
						result_bytes
							.as_mut_ptr()
							.add(cell_index * serialized_proof_size),
						config::PROOF_SIZE,
					);

					std::ptr::copy(
						evaluated_point.to_bytes().as_ptr(),
						result_bytes
							.as_mut_ptr()
							.add(cell_index * serialized_proof_size + config::PROOF_SIZE),
						config::SCALAR_SIZE,
					);
				}

				cell_index += 1;
			}
			Ok(())
		})
		.map_err(Error::from)?;

	unsafe {
		result_bytes.set_len(serialized_proof_size * cell_index);
	}

	info!(
		target: "system",
		"Time to build 1 row of proofs {:?}",
		total_start.elapsed()
	);

	Ok(result_bytes)
}

// TODO @miguel Remove that param?
#[cfg(feature = "alloc")]
pub fn build_commitments(
	rows_num: usize,
	cols_num: usize,
	chunk_size: usize,
	extrinsics_by_key: &[AppExtrinsic],
	header_hash: &[u8],
) -> Result<(XtsLayout, Vec<u8>, BlockDimensions, Vec<BlsScalar>), Error> {
	let start = Instant::now();

	// generate data matrix first
	let (tx_layout, block, block_dims) = flatten_and_pad_block(
		rows_num,
		cols_num,
		chunk_size,
		extrinsics_by_key,
		header_hash,
	)?;

	info!(
		target: "system",
		"Rows: {:?} Cols: {:?} Size: {:?}",
		block_dims.rows,
		block_dims.cols,
		block.len(),
	);

	let ext_data_matrix = extend_data_matrix(block_dims, &block)?;
	let extended_rows_num = block_dims.rows * config::EXTENSION_FACTOR;

	info!(
		target: "system",
		"Time to prepare {:?}",
		start.elapsed()
	);

	// construct commitments in parallel
	let public_params = testnet::public_params(config::MAX_BLOCK_COLUMNS as usize);
	if log::log_enabled!(target: "system", log::Level::Debug) {
		let raw_pp = public_params.to_raw_var_bytes();
		let hash_pp = hex::encode(sp_core::blake2_128(&raw_pp));
		let hex_pp = hex::encode(raw_pp);
		log::debug!(
			target: "system",
			"Public params (len={}): hash: {}", hex_pp.len(), hash_pp,
		);
	}

	let (prover_key, _) = public_params.trim(block_dims.cols).map_err(Error::from)?;
	let row_eval_domain = EvaluationDomain::new(block_dims.cols).map_err(Error::from)?;

	let mut result_bytes: Vec<u8> = Vec::new();
	result_bytes.reserve_exact(config::PROVER_KEY_SIZE * extended_rows_num);
	unsafe {
		result_bytes.set_len(config::PROVER_KEY_SIZE * extended_rows_num);
	}

	info!(
		target: "system",
		"Number of CPU cores: {:#?}",
		num_cpus::get()
	);

	let start = Instant::now();
	for i in 0..extended_rows_num {
		let mut row = Vec::with_capacity(block_dims.cols);

		for j in 0..block_dims.cols {
			row.push(ext_data_matrix[i + j * extended_rows_num]);
		}

		let polynomial = Evaluations::from_vec_and_domain(row, row_eval_domain).interpolate();
		let key_bytes = &prover_key
			.commit(&polynomial)
			.map_err(Error::from)?
			.to_bytes();

		unsafe {
			std::ptr::copy(
				key_bytes.as_ptr(),
				result_bytes.as_mut_ptr().add(i * config::PROVER_KEY_SIZE),
				config::PROVER_KEY_SIZE,
			);
		}
	}

	info!(
		target: "system",
		"Time to build a commitment {:?}",
		start.elapsed()
	);

	Ok((tx_layout, result_bytes, block_dims, ext_data_matrix))
}

#[cfg(test)]
mod tests {
	use std::{collections::HashSet, convert::TryInto, str::from_utf8};

	use da_primitives::asdr::AppExtrinsic;
	use dusk_bytes::Serializable;
	use dusk_plonk::bls12_381::BlsScalar;
	use kate_recovery::com::{reconstruct_app_extrinsics, unflatten_padded_data};
	use proptest::{
		collection::{self, size_range},
		prelude::*,
	};
	use rand::Rng;
	use test_case::test_case;

	use super::{build_commitments, flatten_and_pad_block, pad_with_zeroes};
	use crate::{
		com::{extend_data_matrix, get_block_dimensions, pad_iec_9797_1, BlockDimensions},
		config,
	};

	#[test_case(11,   256, 256 => BlockDimensions { size: 128  , rows: 1, cols: 4  , chunk_size: 32} ; "below minimum block size")]
	#[test_case(300,  256, 256 => BlockDimensions { size: 512  , rows: 1, cols: 16 , chunk_size: 32} ; "regular case")]
	#[test_case(513,  256, 256 => BlockDimensions { size: 1024 , rows: 1, cols: 32 , chunk_size: 32} ; "minimum overhead after 512")]
	#[test_case(8192, 256, 256 => BlockDimensions { size: 8192 , rows: 1, cols: 256, chunk_size: 32} ; "maximum cols")]
	#[test_case(8224, 256, 256 => BlockDimensions { size: 16384, rows: 2, cols: 256, chunk_size: 32} ; "two rows")]

	fn test_get_block_dimensions(size: usize, rows: usize, cols: usize) -> BlockDimensions {
		get_block_dimensions(size, rows, cols, 32).unwrap()
	}

	#[test]
	fn test_extend_data_matrix() {
		let expected_result = vec![
			b"000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e00",
			b"ef471ce5550437df64279fba7f3d31b50d6ddd1d80eebf4f0ea22d18e6efab17",
			b"1f202122232425262728292a2b2c2d2e2f303132333435363738393a3b3c3d00",
			b"31d90640d024f44dc965927aba9fc7db36ac0731cf32c530892cc366c4109d5c",
			b"3e3f404142434445464748494a4b4c4d4e4f505152535455565758595a5b5c00",
			b"2d865a239442751da365ddf8bd7b6ff34bab1b5cbe2cfe8d4ce06b56242eea17",
			b"5d5e5f606162636465666768696a6b6c6d6e6f707172737475767778797a7b00",
			b"6f17457e0e63328c07a4d0b8f8dd051a75ea456f0d71036fc76a01a5024fdb5c",
			b"7c7d7e7f808182838485868788898a8b8c8d8e8f909192939495969798999a00",
			b"6bc49861d280b35be1a31b37fcb9ad318ae9599afc6a3ccc8a1eaa94626c2818",
			b"9b9c9d9e9fa0a1a2a3a4a5a6a7a8a9aaabacadaeafb0b1b2b3b4b5b6b7b8b900",
			b"ad5583bc4ca170ca45e20ef7361c4458b32884ad4baf41ad05a93fe3408d195d",
			b"babbbcbdbebfc0c1c2c3c4c5c6c7c8c9cacbcccdcecfd0d1d2d3d4d5d6d7d800",
			b"a902d79f10bff1991fe259753af8eb6fc82798d83aa97a0ac95ce8d2a0aa6618",
			b"d9dadbdcdddedfe0e1e2e3e4e5e6e7e8e9eaebecedeeeff0f1f2f3f4f5f6f700",
			b"eb93c1fa8adfae0884204d35755a8296f166c2eb89ed7feb43e77d217fcb575d",
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
			size: 256,
			chunk_size: 32,
		};
		let block = (0..=247)
			.collect::<Vec<u8>>()
			.chunks_exact(config::DATA_CHUNK_SIZE)
			.map(|chunk| pad_with_zeroes(chunk, block_dims.chunk_size))
			.flatten()
			.collect::<Vec<u8>>();
		let res = extend_data_matrix(block_dims, &block);
		eprintln!("result={:?}", res);
		eprintln!("expect={:?}", expected_result);
		assert_eq!(res.unwrap(), expected_result);
	}

	#[test]
	fn test_padding() {
		let block: Vec<u8> = (1..=29).collect();
		let expected: Vec<u8> = vec![
			1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
			25, 26, 27, 28, 29, 128, 0,
		];

		let res: Vec<u8> = pad_iec_9797_1(block.as_slice(), config::DATA_CHUNK_SIZE)
			.iter()
			.flat_map(|e| e.to_vec())
			.collect();
		assert_eq!(
			res, expected,
			"Padding the chunk more than 3 values shorter failed."
		);

		let block: Vec<u8> = (1..=30).collect();
		let expected: Vec<u8> = vec![
			1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
			25, 26, 27, 28, 29, 30, 128,
		];
		let res: Vec<u8> = pad_iec_9797_1(block.as_slice(), config::DATA_CHUNK_SIZE)
			.iter()
			.flat_map(|e| e.to_vec())
			.collect();
		assert_eq!(res, expected, "Padding the chunk 2 values shorter failed.");

		let block: Vec<u8> = (1..=31).collect();
		let expected: Vec<u8> = vec![
			1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
			25, 26, 27, 28, 29, 30, 31, 128, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
			0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
		];
		let res: Vec<u8> = pad_iec_9797_1(block.as_slice(), config::DATA_CHUNK_SIZE)
			.iter()
			.flat_map(|e| e.to_vec())
			.collect();
		assert_eq!(res, expected, "Padding the chunk 1 value shorter failed.");
	}

	#[test]
	fn test_flatten_block() {
		let chunk_size = 32;
		let extrinsics: Vec<AppExtrinsic> = vec![
			AppExtrinsic {
				app_id: 0,
				data: (1..=29).collect(),
			},
			AppExtrinsic {
				app_id: 1,
				data: (1..=30).collect(),
			},
			AppExtrinsic {
				app_id: 2,
				data: (1..=31).collect(),
			},
			AppExtrinsic {
				app_id: 3,
				data: (1..=60).collect(),
			},
		];

		// The hash is used for seed for padding the block to next power of two value
		let hash: Vec<u8> = vec![0].repeat(32);
		let expected_dims = BlockDimensions {
			rows: 1,
			cols: 8,
			size: 256,
			chunk_size,
		};
		let (layout, data, dims) =
			flatten_and_pad_block(128, 256, chunk_size, extrinsics.as_slice(), &hash).unwrap();

		let expected_layout = vec![(0, 1), (1, 1), (2, 2), (3, 2)];
		assert_eq!(layout, expected_layout, "The layouts don't match");

		let expected_data: Vec<u8> = vec![
			// First extrinsic
			1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
			25, 26, 27, 28, 29, 128, 0, 0, // Second extrinsic
			1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
			25, 26, 27, 28, 29, 30, 128, 0, // Third extrinsic
			1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
			25, 26, 27, 28, 29, 30, 31, 0, 128, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
			0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // Fourth extrinsic
			1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
			25, 26, 27, 28, 29, 30, 31, 0, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45,
			46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 128, 0, 0,
			// Random seeded data
			155, 7, 129, 95, 4, 73, 126, 46, 5, 210, 44, 172, 58, 160, 97, 65, 11, 32, 134, 140,
			198, 25, 21, 76, 66, 161, 198, 27, 233, 144, 39, 0, 23, 178, 75, 142, 193, 104, 254,
			72, 150, 245, 183, 12, 203, 16, 241, 167, 5, 224, 97, 208, 253, 250, 24, 150, 24, 210,
			139, 13, 68, 239, 239, 0,
		];

		assert_eq!(dims, expected_dims, "Dimensions don't match the expected");
		assert_eq!(data, expected_data, "Data doesn't match the expected data");

		let res = unflatten_padded_data(layout, data, chunk_size);
		assert_eq!(
			res.len(),
			extrinsics.len(),
			"Number of extrinsics is not as expected."
		);

		for (res, exp) in res.iter().zip(extrinsics.iter()) {
			assert_eq!(res.0, exp.app_id);
			assert_eq!(res.1, exp.data);
		}
	}

	fn sample_cells_from_matrix(
		matrix: Vec<BlsScalar>,
		dimensions: &BlockDimensions,
	) -> Vec<Vec<kate_recovery::com::Cell>> {
		fn cell_from_scalar(row_idx: u16, scalar: &BlsScalar) -> kate_recovery::com::Cell {
			kate_recovery::com::Cell {
				row: row_idx,
				data: scalar.clone().to_bytes().to_vec(),
				..Default::default()
			}
		}

		fn random_indexes(length: usize) -> Vec<usize> {
			// choose random len/2 (unique) indexes
			let mut idx = (0..length).collect::<Vec<_>>();
			let mut chosen_idx = Vec::<usize>::new();
			let mut rng = rand::thread_rng();
			for _ in 0..length / 2 {
				let i = rng.gen_range(0..idx.len());
				let v = idx.remove(i);
				chosen_idx.push(v);
			}
			chosen_idx
		}

		matrix
			.chunks_exact(dimensions.rows * 2)
			.map(|e| {
				random_indexes(e.len())
					.into_iter()
					.map(|i| cell_from_scalar(i as u16, &e[i]))
					.collect::<Vec<_>>()
			})
			.collect::<Vec<_>>()
	}

	fn app_extrinsic_strategy() -> impl Strategy<Value = AppExtrinsic> {
		(
			any::<u32>(),
			any_with::<Vec<u8>>(size_range(1..2048).lift()),
		)
			.prop_map(|(app_id, data)| AppExtrinsic { app_id, data })
	}

	fn app_extrinsics_strategy() -> impl Strategy<Value = Vec<AppExtrinsic>> {
		collection::vec(app_extrinsic_strategy(), size_range(1..16)).prop_filter(
			"no app_id duplicates",
			|xts| {
				let mut uniq = HashSet::new();
				xts.iter().all(|xt| uniq.insert(xt.app_id))
			},
		)
	}

	proptest! {
		#![proptest_config(ProptestConfig::with_cases(50))]
		#[test]
		fn test_build_and_reconstruct(ref xts in app_extrinsics_strategy()) {
			let hash: Vec<u8> = (0..=31).collect::<Vec<u8>>();

			let (layout, _, dims, matrix) = build_commitments(64, 16, 32, xts, hash.as_slice()).unwrap();

			let columns = sample_cells_from_matrix(matrix, &dims);
			let reconstructed = reconstruct_app_extrinsics(layout, columns, dims.rows, dims.chunk_size);

			for (result, xt) in reconstructed.iter().zip(xts) {
				prop_assert_eq!(result.0, xt.app_id);
				prop_assert_eq!(&result.1, &xt.data);
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
		let hash: Vec<u8> = vec![
			76, 41, 174, 145, 187, 12, 97, 32, 75, 111, 149, 209, 243, 195, 165, 10, 166, 172, 47,
			41, 218, 24, 212, 66, 62, 5, 187, 191, 129, 5, 105, 3,
		];

		let (_, commitments, dimensions, _) = build_commitments(
			block_rows,
			block_cols,
			chunk_size,
			&[AppExtrinsic::from(original_data.to_vec())],
			&hash,
		)
		.unwrap();

		assert!(
			dimensions
				== BlockDimensions {
					rows: 1,
					cols: 4,
					size: 128,
					chunk_size: 32
				}
		);
		assert!(
			commitments
				== vec![
					179, 146, 111, 178, 52, 189, 223, 114, 103, 194, 180, 22, 184, 118, 15, 103,
					140, 42, 181, 96, 218, 184, 195, 8, 1, 191, 222, 69, 86, 117, 238, 244, 176,
					238, 88, 46, 194, 158, 112, 16, 111, 28, 161, 155, 214, 174, 118, 212, 179,
					146, 111, 178, 52, 189, 223, 114, 103, 194, 180, 22, 184, 118, 15, 103, 140,
					42, 181, 96, 218, 184, 195, 8, 1, 191, 222, 69, 86, 117, 238, 244, 176, 238,
					88, 46, 194, 158, 112, 16, 111, 28, 161, 155, 214, 174, 118, 212
				]
		);
	}

	#[test]
	fn test_extend_mock_data() {
		let orig_data = br#"This is mocked test data. It will be formatted as a matrix of BLS scalar cells and then individual columns 
get erasure coded to ensure redundancy.
Let's see how this gets encoded and then reconstructed by sampling only some data."#;

		// The hash is used for seed for padding the block to next power of two value
		let hash: Vec<u8> = vec![0].repeat(32);
		let chunk_size = 32;
		let (layout, data, dims) = flatten_and_pad_block(
			128,
			2,
			chunk_size,
			&[AppExtrinsic::from(orig_data.to_vec())],
			&hash,
		)
		.unwrap();

		let coded: Vec<BlsScalar> = extend_data_matrix(dims, &data[..]).unwrap();

		let cols = sample_cells_from_matrix(coded, &dims);

		let res = reconstruct_app_extrinsics(layout, cols, dims.rows, dims.chunk_size);

		// let decoded = decode_scalars(&res.as_slice());
		let s = String::from_utf8_lossy(res[0].1.as_slice());

		assert_eq!(res[0].1.as_slice(), orig_data);

		eprintln!("Decoded: {}", s);
	}
}

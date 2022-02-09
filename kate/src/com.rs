use std::{convert::TryFrom, time::Instant};

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

#[derive(Clone, Copy, Debug)]
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

pub fn flatten_and_pad_block(
	rows_num: usize,
	cols_num: usize,
	chunk_size: usize,
	extrinsics: &[AppExtrinsic],
	header_hash: &[u8],
) -> Result<(XtsLayout, FlatData, BlockDimensions), Error> {
	let mut tx_layout = Vec::with_capacity(extrinsics.len());
	let mut block: Vec<u8> =
		Vec::with_capacity((config::SCALAR_SIZE_WIDE + 100) * extrinsics.len());

	for xt in extrinsics {
		// get aligned xt length
		let aligned_len =
			xt.data.len() + (config::SCALAR_SIZE_WIDE - (xt.data.len() % config::SCALAR_SIZE_WIDE));
		// insert into flat buffer
		block.extend(&xt.data);

		// add extra 0's if required
		block.resize(aligned_len, 0);

		// save tx by app id and size in chunks
		tx_layout.push((xt.app_id, (aligned_len / config::SCALAR_SIZE_WIDE) as u32));
	}

	let block_dims = get_block_dimensions(block.len(), rows_num, cols_num, chunk_size);

	if block.len() > block_dims.size {
		log::info!(
			target: "system",
			"BlockTooBig: block.len()={} block_dims:{:?}",
			block.len(),
			block_dims);
	}
	ensure!(block.len() <= block_dims.size, Error::BlockTooBig);

	let seed = <[u8; 32]>::try_from(header_hash).map_err(|_| Error::BadHeaderHash)?;
	let mut rng: StdRng = rand::SeedableRng::from_seed(seed);
	block.resize_with(block_dims.size, || rng.gen::<u8>());

	Ok((tx_layout, block, block_dims))
}

pub fn get_block_dimensions(
	block_size: usize,
	max_rows_num: usize,
	max_cols_num: usize,
	chunk_size: usize,
) -> BlockDimensions {
	let max_block_size = max_rows_num * max_cols_num * chunk_size;
	let mut rows = max_rows_num;
	let mut cols = max_cols_num;
	let mut size = block_size + (block_size as f32 / config::CHUNK_SIZE as f32).ceil() as usize;

	if size < max_block_size {
		let mut nearest_power_2_size = (2 as usize).pow((size as f32).log2().ceil() as u32);
		if nearest_power_2_size < config::MINIMUM_BLOCK_SIZE {
			nearest_power_2_size = config::MINIMUM_BLOCK_SIZE;
		}

		let total_cells = (nearest_power_2_size as f32 / chunk_size as f32).ceil() as usize;
		// we must minimize number of rows, to minimize header size
		// (performance wise it doesn't matter)
		if total_cells > max_cols_num {
			rows = total_cells / max_cols_num;
		} else {
			rows = 1;
			cols = total_cells;
		}
		size = rows * cols * chunk_size;
	} else if size > max_block_size {
		panic!("block is too big, must not happen!");
	}

	BlockDimensions {
		cols,
		rows,
		size,
		chunk_size,
	}
}

fn pad_to_cell_size(size: usize) -> impl Fn(&[u8]) -> Vec<u8> {
	move |chunk: &[u8]| -> Vec<u8> {
		let mut bytes: Vec<u8> = vec![];
		bytes.extend(chunk);
		bytes.extend(vec![0].repeat(size - config::CHUNK_SIZE));
		bytes
	}
}

fn extend_row_with_zeros(extension_size: usize) -> impl Fn(&[BlsScalar]) -> Vec<BlsScalar> {
	move |row: &[BlsScalar]| -> Vec<BlsScalar> {
		let mut result = row.to_vec();
		result.resize(extension_size, BlsScalar::zero());
		result
	}
}

#[cfg(feature = "alloc")]
/// build extended data matrix, by columns
pub fn extend_data_matrix(
	block_dims: BlockDimensions,
	block: &[u8],
) -> Result<Vec<BlsScalar>, Error> {
	let start = Instant::now();
	let rows_num = block_dims.rows;
	let cols_num = block_dims.cols;
	let extended_rows_num = rows_num * config::EXTENSION_FACTOR;

	let chunks = block.chunks_exact(config::CHUNK_SIZE);
	assert!(chunks.remainder().len() == 0);

	// TODO: Better error type for BlsScalar case?
	let mut chunk_elements = chunks
		.map(pad_to_cell_size(block_dims.chunk_size))
		.map(|chunk| <[u8; config::SCALAR_SIZE]>::try_from(&chunk[..]))
		.map(|result| result.map_err(|_| Error::InvalidChunkLength))
		.map(|chunk| BlsScalar::from_bytes(&chunk?).map_err(|_| Error::InvalidChunkLength))
		.collect::<Result<Vec<BlsScalar>, Error>>()?
		.chunks_exact(rows_num)
		.map(extend_row_with_zeros(extended_rows_num))
		.flatten()
		.collect::<Vec<_>>();

	// extend data matrix, column by column
	let extended_column_eval_domain = EvaluationDomain::new(extended_rows_num)?;
	let column_eval_domain = EvaluationDomain::new(rows_num)?;

	for i in 0..cols_num {
		let original_column =
			&mut chunk_elements[i * extended_rows_num..(i + 1) * extended_rows_num - rows_num];
		column_eval_domain.ifft_slice(original_column);

		let extended_column =
			&mut chunk_elements[i * extended_rows_num..(i + 1) * extended_rows_num];
		extended_column_eval_domain.fft_slice(extended_column);
	}

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
		block.len()
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

mod tests {
	use std::{convert::TryInto, str::from_utf8};

	use bls12_381::Scalar;
	use da_primitives::asdr::AppExtrinsic;
	use dusk_bytes::Serializable;
	use dusk_plonk::bls12_381::BlsScalar;

	use super::flatten_and_pad_block;
	use crate::com::{extend_data_matrix, get_block_dimensions, BlockDimensions};

	#[test]
	fn test_get_block_dimensions() {
		let res = get_block_dimensions(11, 256, 256, 32);
		assert_eq!(res.size, 256);
		assert_eq!(res.cols, 8);
		assert_eq!(res.rows, 1);

		let res = get_block_dimensions(300, 256, 256, 32);
		assert_eq!(res.size, 512);
		assert_eq!(res.cols, 16);
		assert_eq!(res.rows, 1);

		let res = get_block_dimensions(513, 256, 256, 32);
		assert_eq!(res.size, 1024);
		assert_eq!(res.cols, 32);
		assert_eq!(res.rows, 1);
	}

	#[test]
	fn test_extend_data_matrix() {
		let block = (0..=255).collect::<Vec<u8>>();

		let expected_result = vec![
			b"1f21232527292b2d2f31333537393b3d3f41434547494b4d4f51535557595b00",
			b"c29bb585daec9259aa83760d5fefdf1cd88867f486efad8e486e9d7695f03711",
			b"1f202122232425262728292a2b2c2d2e2f303132333435363738393a3b3c3d00",
			b"01250a39e2cf28610e913fa9596991e9deffe9c32e95374fa9b7a759635b5962",
			b"9b9d9fa1a3a5a7a9abadafb1b3b5b7b9bbbdbfc1c3c5c7c9cbcdcfd1d3d5d700",
			b"e50d81c135ee30a9efa24aae7a325724e2a872e8b6149450a4d913a654f20956",
			b"5d5e5f606162636465666768696a6b6c6d6e6f707172737475767778797a7b00",
			b"a07400bf48904cd38a332dcaffe7dba396a1a091c031134f0f0ef3eb651b491d",
			b"171a1c1e20222426282a2c2e30323436383a3c3e40424446484a4c4e50525401",
			b"07804cfd91efcef83566204f93d110d8e6f0dbd2de6140dfb7c7ecabc04cee26",
			b"9b9c9d9e9fa0a1a2a3a4a5a6a7a8a9aaabacadaeafb0b1b2b3b4b5b6b7b8b900",
			b"40c4f644ae507045063219eba80ae4b1531bf9685aa62882bde1dba7bb82264c",
			b"9396989a9c9ea0a2a4a6a8aaacaeb0b2b4b6b8babcbec0c2c4c6c8caccced001",
			b"2af21739edf06c487b85f4efae1488dff010e7c60e8726a1133363db7f4ec06b",
			b"d9dadbdcdddedfe0e1e2e3e4e5e6e7e8e9eaebecedeeeff0f1f2f3f4f5f6f700",
			b"df13edca141194b782d4060c4f892e6c0bbdaf36ec4204822338273abe421607",
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
		let res = extend_data_matrix(block_dims, &block);
		eprintln!("result={:?}", res);
		eprintln!("expect={:?}", expected_result);
		assert_eq!(res.unwrap(), expected_result);
	}

	#[test]
	fn test_flatten_block() {
		// Values acquired from extrinsics in the first test block
		let block: Vec<u8> = vec![40, 4, 3, 0, 11, 230, 228, 0, 196, 126, 1];
		let block_data: [u8; 11] = [40, 4, 3, 0, 11, 230, 228, 0, 196, 126, 1];
		let block_len = block.len().clone();
		// The hash is used for seed for padding the block to next power of two value
		let hash: Vec<u8> = vec![0].repeat(32);
		let expected_dims = BlockDimensions {
			rows: 1,
			cols: 8,
			size: 256,
			chunk_size: 32,
		};
		let (_, data, dims) =
			flatten_and_pad_block(128, 256, 32, &[AppExtrinsic::from(block)], &hash).unwrap();

		assert_eq!(block_data, data[0..block_len]);
		assert_eq!(dims, expected_dims);
	}
}

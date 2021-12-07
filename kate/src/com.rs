use dusk_plonk::commitment_scheme::kzg10;
use dusk_plonk::fft::{EvaluationDomain,Evaluations};
use dusk_bytes::Serializable;
use std::time::{Instant};
use log::{info};
use std::convert::{TryInto, TryFrom};
use serde::{Serialize, Deserialize};
use rand::{rngs::StdRng, Rng};
use super::*;
use dusk_plonk::prelude::BlsScalar;

#[derive(Serialize, Deserialize)]
pub struct Cell {
	pub row: u32,
	pub col: u32,
}

#[derive(Clone, Copy)]
pub struct BlockDimensions {
	pub rows: usize,
	pub cols: usize,
	pub size: usize,
	pub chunk_size: usize,
}

pub fn flatten_and_pad_block(
	rows_num: usize,
	cols_num: usize,
	chunk_size: usize,
	extrinsics: &Vec<Vec<u8>>,
	header_hash: &[u8]
) -> (Vec<u8>, BlockDimensions) {
	let block_len = extrinsics.iter().map(|ext| ext.len()).sum();
	let mut block:Vec<u8> = Vec::with_capacity(block_len);
	extrinsics.iter().for_each(|ext| block.extend_from_slice(ext));

	let block_dims = get_block_dimensions(block.len(), rows_num, cols_num, chunk_size);

	if block.len() < block_dims.size {
		let more_elems = block_dims.size - block.len();
		block.reserve_exact(more_elems);
		let mut rng:StdRng = rand::SeedableRng::from_seed(<[u8; 32]>::try_from(header_hash).unwrap());
		for _ in 0..more_elems {
			// pseudo random values
			block.push(rng.gen::<u8>());
		}
	} else if block.len() > block_dims.size {
		panic!("block is too big, must not happen!");
	}

	(block, block_dims)
}

pub fn get_block_dimensions(
	block_size: usize,
	rows_num: usize,
	cols_num: usize,
	chunk_size: usize
) -> BlockDimensions {
	let max_block_size = rows_num * cols_num * chunk_size;
	let mut size = block_size;
	let mut rows = rows_num;
	let mut cols = cols_num;

	if block_size < max_block_size {
		let mut nearest_power_2_size = (2 as usize).pow((block_size as f32).log2().ceil() as u32);
		if nearest_power_2_size < config::MINIMUM_BLOCK_SIZE {
			nearest_power_2_size = config::MINIMUM_BLOCK_SIZE;
		}

		size = nearest_power_2_size;

		// we must minimize number of rows, to minimize header size (performance wise it doesn't matter)
		let total_cells = (nearest_power_2_size as f32 / chunk_size as f32).ceil() as usize;
		if total_cells > cols {
			rows = total_cells / cols;
			size = rows * cols * chunk_size;
		} else {
			rows = 1;
			cols = total_cells;
		}
	} else if block_size > max_block_size {
		panic!("block is too big, must not happen!");
	}

	return BlockDimensions{
		cols,
		rows,
		size,
		chunk_size
	}
}

#[cfg(feature = "alloc")]
/// build extended data matrix, by columns
pub fn extend_data_matrix(
	block_dims: BlockDimensions,
	block: &Vec<u8>
) -> Vec<BlsScalar> {
	let start = Instant::now();
	let rows_num = block_dims.rows;
	let cols_num = block_dims.cols;
	let extended_rows_num = rows_num * config::EXTENSION_FACTOR;

	let mut chunk_elements = Vec::new();
	// prepare extended size
	chunk_elements.resize(extended_rows_num * cols_num, BlsScalar::zero());

	// generate column by column and pack into extended array of scalars
	let chunk_bytes_offset = rows_num * block_dims.chunk_size;
	let mut offset = 0;
	for i in 0..cols_num {
		let mut chunk = block[i * chunk_bytes_offset..(i+1) * chunk_bytes_offset].chunks_exact(config::SCALAR_SIZE_WIDE);
		for _ in 0..rows_num {
			// from_bytes_wide expects [u8;64]
			chunk_elements[offset] = BlsScalar::from_bytes_wide(chunk.next().unwrap().try_into().expect("slice with incorrect length"));
			offset += 1;
		}

		// offset extra rows
		offset += rows_num;
	}

	// extend data matrix, column by column
	let extended_column_eval_domain = EvaluationDomain::new(extended_rows_num).unwrap();
	let column_eval_domain = EvaluationDomain::new(rows_num).unwrap();

	for i in 0..cols_num {
		let original_column = &mut chunk_elements[i * extended_rows_num..(i+1) * extended_rows_num - extended_rows_num / config::EXTENSION_FACTOR];
		column_eval_domain.ifft_slice(original_column);

		let extended_column = &mut chunk_elements[i * extended_rows_num..(i+1) * extended_rows_num];
		extended_column_eval_domain.fft_slice(extended_column);
	}

	info!(
		target: "system",
		"Time to extend block {:?}",
		start.elapsed()
	);

	chunk_elements
}

//TODO cache extended data matrix
//TODO explore faster Variable Base Multi Scalar Multiplication
pub fn build_proof(
	public_params_data: &Vec<u8>,
	block_dims: BlockDimensions,
	ext_data_matrix: &Vec<BlsScalar>,
	cells: Vec<Cell>
) -> Option<Vec<u8>> {
	let rows_num = block_dims.rows;
	let cols_num = block_dims.cols;
	let extended_rows_num = rows_num * config::EXTENSION_FACTOR;

	if cells.len() > config::MAX_PROOFS_REQUEST {
		()
	}

	let public_params = kzg10::PublicParameters::from_slice(public_params_data.as_slice()).unwrap();
	let (prover_key, _) = public_params.trim(cols_num).unwrap();

	// Generate all the x-axis points of the domain on which all the row polynomials reside
	let row_eval_domain = EvaluationDomain::new(cols_num).unwrap();
	let mut row_dom_x_pts = Vec::with_capacity(row_eval_domain.size());
	row_dom_x_pts.extend(row_eval_domain.elements());

	let mut result_bytes: Vec<u8> = Vec::new();
	let serialized_proof_size = config::SCALAR_SIZE + config::PROOF_SIZE;
	result_bytes.reserve_exact(serialized_proof_size * cells.len());
	unsafe {
		result_bytes.set_len(serialized_proof_size * cells.len());
	}

	let prover_key = &prover_key;
	let ext_data_matrix = &ext_data_matrix;
	let row_dom_x_pts = &row_dom_x_pts;
	let mut cell_index = 0;

	info!(
		target: "system",
		"Number of CPU cores: {:#?}",
		num_cpus::get()
	);
	// generate proof only for requested cells
	let total_start= Instant::now();
	Iterator::enumerate(cells.iter()).for_each(|(_, cell)| {
		let row_index = cell.row as usize;
		let col_index = cell.col as usize;

		if row_index < extended_rows_num && col_index < cols_num {
			// construct polynomial per extended matrix row
			let mut row = Vec::with_capacity(cols_num);

			for j in 0..cols_num {
				row.push(ext_data_matrix[row_index + j * extended_rows_num]);
			}
			let polynomial = Evaluations::from_vec_and_domain(row, row_eval_domain).interpolate();
			let witness = prover_key.compute_single_witness(&polynomial, &row_dom_x_pts[col_index]);
			let commitment_to_witness = prover_key.commit(&witness).unwrap();
			let evaluated_point = ext_data_matrix[row_index + col_index * extended_rows_num];

			unsafe {
				std::ptr::copy(
					commitment_to_witness.to_bytes().as_ptr(),
					result_bytes.as_mut_ptr().add(cell_index * serialized_proof_size),
					config::PROOF_SIZE
				);

				std::ptr::copy(
					evaluated_point.to_bytes().as_ptr(),
					result_bytes.as_mut_ptr().add(cell_index * serialized_proof_size + config::PROOF_SIZE),
					config::SCALAR_SIZE
				);
			}

			cell_index += 1;
		}
	});

	unsafe {
		result_bytes.set_len(serialized_proof_size * cell_index);
	}

	info!(
		target: "system",
		"Time to build 1 row of proofs {:?}",
		total_start.elapsed()
	);

	Some(result_bytes)
}

// TODO @miguel Remove that param?
#[cfg(feature = "alloc")]
pub fn build_commitments(
	_public_params_data: &Vec<u8>,
	rows_num: usize,
	cols_num: usize,
	chunk_size: usize,
	extrinsics: &Vec<Vec<u8>>,
	header_hash: &[u8]
) -> (Vec<u8>, BlockDimensions) {
	let start= Instant::now();

	// generate data matrix first
	let (block, block_dims) = flatten_and_pad_block(
		rows_num,
		cols_num,
		chunk_size,
		extrinsics,
		header_hash
	);

	info!(
		target: "system",
		"Rows: {:?} Cols: {:?} Size: {:?}",
		block_dims.rows,
		block_dims.cols,
		block.len()
	);

	let ext_data_matrix = extend_data_matrix(
		block_dims,
		&block
	);
	let extended_rows_num = block_dims.rows * config::EXTENSION_FACTOR;

	info!(
		target: "system",
		"Time to prepare {:?}",
		start.elapsed()
	);

	// construct commitments in parallel
	let public_params = testnet::public_params(block_dims.cols);
	let (prover_key, _) = public_params.trim(block_dims.cols).unwrap();
	let row_eval_domain = EvaluationDomain::new(block_dims.cols).unwrap();

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
		let key_bytes = &prover_key.commit(&polynomial).unwrap().to_bytes();

		unsafe {
			std::ptr::copy(
				key_bytes.as_ptr(),
				result_bytes.as_mut_ptr().add(i * config::PROVER_KEY_SIZE),
				config::PROVER_KEY_SIZE
			);
		}
	}

	info!(
		target: "system",
		"Time to build a commitment {:?}",
		start.elapsed()
	);

	(result_bytes, block_dims)
}

extern crate criterion;
extern crate itertools;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use da_primitives::asdr::AppExtrinsic;
use itertools::Itertools;
use kate::{
	com::{build_proof, par_build_commitments, Cell},
	config::DATA_CHUNK_SIZE,
	testnet,
};
use kate_proof::kc_verify_proof;
use rand::prelude::*;
use rand_chacha::ChaCha20Rng;

fn variate_rc(rows: usize, cols: usize) -> Vec<(usize, usize)> {
	assert_eq!(rows >= 64, true);
	assert_eq!(cols >= 64, true);

	let mut dims = Vec::new();

	let mut i = 64;
	while i <= rows {
		dims.push((i, cols * (rows / i)));
		i <<= 1;
	}

	let mut i = 64;
	while i < cols {
		dims.push((rows * (cols / i), i));
		i <<= 1;
	}

	dims
}

fn generate_matrix_dimensions() -> Vec<(usize, usize)> {
	const MIN_ROWS: usize = 256;
	const MAX_ROWS: usize = 2048;

	const MIN_COLS: usize = 256;
	const MAX_COLS: usize = 2048;

	let mut dims = Vec::new();

	let mut r = MIN_ROWS;
	while r <= MAX_ROWS {
		let mut c = MIN_COLS;
		while c <= MAX_COLS {
			dims.extend(&variate_rc(r, c));
			c <<= 1;
		}
		r <<= 1;
	}

	dims.into_iter().unique().collect::<Vec<(usize, usize)>>()
}

// // Commitment builder routine candidate 1
// fn bench_build_commitments(c: &mut Criterion) {
// 	let mut rng = ChaCha20Rng::from_entropy();

// 	const CHUNK: usize = DATA_CHUNK_SIZE as usize + 1;
// 	let dims = generate_matrix_dimensions();

// 	for dim in dims {
// 		let dlen = dim.0 * dim.1 * (CHUNK - 2);

// 		let mut seed = [0u8; 32];
// 		let mut data = vec![0u8; dlen];

// 		rng.fill_bytes(&mut seed);
// 		rng.fill_bytes(&mut data);

// 		let tx = AppExtrinsic::from(data.to_vec());
// 		let txs = [tx];

// 		c.bench_function(
// 			&format!(
// 				"build_commitments/{}x{}/{} MB",
// 				dim.0,
// 				dim.1,
// 				(dim.0 * dim.1 * CHUNK) >> 20
// 			),
// 			|b| {
// 				b.iter(|| {
// 					let (_, _, _, _) = build_commitments(
// 						black_box(dim.0),
// 						black_box(dim.1),
// 						black_box(CHUNK),
// 						black_box(&txs),
// 						black_box(seed),
// 					)
// 					.unwrap();
// 				});
// 			},
// 		);
// 	}
// }

// Commitment builder routine candidate 2
fn bench_par_build_commitments(c: &mut Criterion) {
	let mut rng = ChaCha20Rng::from_entropy();

	const CHUNK: usize = DATA_CHUNK_SIZE as usize + 1;
	let dims = generate_matrix_dimensions();

	for dim in dims {
		let dlen = dim.0 * dim.1 * (CHUNK - 2);

		let mut seed = [0u8; 32];
		let mut data = vec![0u8; dlen];

		rng.fill_bytes(&mut seed);
		rng.fill_bytes(&mut data);

		let tx = AppExtrinsic::from(data.to_vec());
		let txs = [tx];

		c.bench_function(
			&format!(
				"par_build_commitments/{}x{}/{} MB",
				dim.0,
				dim.1,
				(dim.0 * dim.1 * CHUNK) >> 20
			),
			|b| {
				b.iter(|| {
					let (_, _, _, _) = par_build_commitments(
						black_box(dim.0),
						black_box(dim.1),
						black_box(CHUNK),
						black_box(&txs),
						black_box(seed),
					)
					.unwrap();
				});
			},
		);
	}
}

// // Commitment builder routine candidate 3
// fn bench_opt_par_build_commitments(c: &mut Criterion) {
// 	let mut rng = ChaCha20Rng::from_entropy();

// 	const CHUNK: usize = DATA_CHUNK_SIZE as usize + 1;
// 	let dims = generate_matrix_dimensions();

// 	for dim in dims {
// 		let dlen = dim.0 * dim.1 * (CHUNK - 2);

// 		let mut seed = [0u8; 32];
// 		let mut data = vec![0u8; dlen];

// 		rng.fill_bytes(&mut seed);
// 		rng.fill_bytes(&mut data);

// 		let tx = AppExtrinsic::from(data.to_vec());
// 		let txs = [tx];

// 		c.bench_function(
// 			&format!(
// 				"opt_par_build_commitments/{}x{}/{} MB",
// 				dim.0,
// 				dim.1,
// 				(dim.0 * dim.1 * CHUNK) >> 20
// 			),
// 			|b| {
// 				b.iter(|| {
// 					let (_, _, _, _) = opt_par_build_commitments(
// 						black_box(dim.0),
// 						black_box(dim.1),
// 						black_box(CHUNK),
// 						black_box(&txs),
// 						black_box(seed),
// 					)
// 					.unwrap();
// 				});
// 			},
// 		);
// 	}
// }

fn bench_build_proof(c: &mut Criterion) {
	let mut rng = ChaCha20Rng::from_entropy();

	const CHUNK: usize = DATA_CHUNK_SIZE as usize + 1;
	let mdims = generate_matrix_dimensions();

	for dim in mdims {
		let dlen = dim.0 * dim.1 * (CHUNK - 2);

		let mut seed = [0u8; 32];
		let mut data = vec![0u8; dlen];

		rng.fill_bytes(&mut seed);
		rng.fill_bytes(&mut data);

		let tx = AppExtrinsic::from(data.to_vec());
		let txs = [tx];

		let public_params = crate::testnet::public_params(dim.1);

		let (_, _, dims, mat) = par_build_commitments(dim.0, dim.1, CHUNK, &txs, seed).unwrap();

		c.bench_function(
			&format!(
				"build_proof/{}x{}/ {} MB",
				dim.0,
				dim.1,
				(dim.0 * dim.1 * CHUNK) >> 20
			),
			|b| {
				b.iter(|| {
					let cell = Cell {
						row: rng.next_u32() % dims.rows as u32,
						col: rng.next_u32() % dims.cols as u32,
					};

					let proof = build_proof(&public_params, dims, &mat, &[cell]).unwrap();
					assert_eq!(proof.len(), 80);
				});
			},
		);
	}
}

fn bench_verify_proof(c: &mut Criterion) {
	let mut rng = ChaCha20Rng::from_entropy();

	const CHUNK: usize = DATA_CHUNK_SIZE as usize + 1;
	let mdims = generate_matrix_dimensions();

	for dim in mdims {
		let dlen = dim.0 * dim.1 * (CHUNK - 2);

		let mut seed = [0u8; 32];
		let mut data = vec![0u8; dlen];

		rng.fill_bytes(&mut seed);
		rng.fill_bytes(&mut data);

		let tx = AppExtrinsic::from(data.to_vec());
		let txs = [tx];

		let pp = crate::testnet::public_params(dim.1);

		let (_, comms, dims, mat) = par_build_commitments(dim.0, dim.1, CHUNK, &txs, seed).unwrap();

		let row = rng.next_u32() % dims.rows as u32;
		let col = rng.next_u32() % dims.cols as u32;

		let proof = build_proof(&pp, dims, &mat, &[Cell { row, col }]).unwrap();
		assert_eq!(proof.len(), 80);

		c.bench_function(
			&format!(
				"verify_proof/{}x{}/ {} MB",
				dim.0,
				dim.1,
				(dim.0 * dim.1 * CHUNK) >> 20
			),
			|b| {
				b.iter(|| {
					let comm = &comms[row as usize * 48..(row as usize + 1) * 48];
					let flg = kc_verify_proof(col, &proof, comm, dims.rows, dims.cols, &pp);

					assert_eq!(flg.unwrap(), true);
				});
			},
		);
	}
}

criterion_group! {name = kzg; config = Criterion::default().sample_size(10); targets =   bench_par_build_commitments, bench_build_proof, bench_verify_proof}
criterion_main!(kzg);

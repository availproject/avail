extern crate criterion;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use da_primitives::asdr::AppExtrinsic;
use dusk_plonk::{
	fft::{EvaluationDomain, Evaluations},
	prelude::BlsScalar,
};
use kate::{
	com::{
		build_commitments, build_proof, extend_data_matrix, fft_on_commitments,
		flatten_and_pad_block, opt_par_build_commitments, par_build_commitments,
		par_extend_data_matrix, to_bls_scalar, Cell, Error,
	},
	config::{DATA_CHUNK_SIZE, EXTENSION_FACTOR, MAX_BLOCK_COLUMNS, MAX_BLOCK_ROWS},
	testnet,
};
use rand::prelude::*;
use rand_chacha::ChaCha20Rng;

fn bench_build_commitments(c: &mut Criterion) {
	const ROWS: usize = MAX_BLOCK_ROWS as usize;
	const COLS: usize = MAX_BLOCK_COLUMNS as usize;
	const CHUNK: usize = DATA_CHUNK_SIZE as usize + 1;
	const DLEN: usize = ROWS * COLS * (CHUNK - 2);

	let mut rng = ChaCha20Rng::from_entropy();

	let mut seed = [0u8; 32];
	let mut data = [0u8; DLEN];

	rng.fill_bytes(&mut seed);
	rng.fill_bytes(&mut data);

	let extrinsic = AppExtrinsic::from(data.to_vec());
	let extrinsics = [extrinsic];

	c.bench_function("build_commitments", |b| {
		b.iter(|| {
			let (_, _, dim, _) = build_commitments(
				black_box(ROWS),
				black_box(COLS),
				black_box(CHUNK),
				black_box(&extrinsics),
				black_box(seed),
			)
			.unwrap();

			assert_eq!(dim.rows, ROWS);
			assert_eq!(dim.cols, COLS);
		});
	});
}

fn bench_par_build_commitments(c: &mut Criterion) {
	const ROWS: usize = MAX_BLOCK_ROWS as usize;
	const COLS: usize = MAX_BLOCK_COLUMNS as usize;
	const CHUNK: usize = DATA_CHUNK_SIZE as usize + 1;
	const DLEN: usize = ROWS * COLS * (CHUNK - 2);

	let mut rng = ChaCha20Rng::from_entropy();

	let mut seed = [0u8; 32];
	let mut data = [0u8; DLEN];

	rng.fill_bytes(&mut seed);
	rng.fill_bytes(&mut data);

	let extrinsic = AppExtrinsic::from(data.to_vec());
	let extrinsics = [extrinsic];

	c.bench_function("par_build_commitments", |b| {
		b.iter(|| {
			let (_, _, dim, _) = par_build_commitments(
				black_box(ROWS),
				black_box(COLS),
				black_box(CHUNK),
				black_box(&extrinsics),
				black_box(seed),
			)
			.unwrap();

			assert_eq!(dim.rows, ROWS);
			assert_eq!(dim.cols, COLS);
		});
	});
}

fn bench_opt_par_build_commitments(c: &mut Criterion) {
	let mut rng = ChaCha20Rng::from_entropy();

	const CHUNK: usize = DATA_CHUNK_SIZE as usize + 1;
	const DIMS: [(usize, usize); 5] = [
		(1024, 64),
		(512, 128),
		(256, 256),
		(MAX_BLOCK_ROWS as usize, MAX_BLOCK_COLUMNS as usize),
		(64, 1024),
	];

	for dim in DIMS {
		let dlen = dim.0 * dim.1 * (CHUNK - 2);

		let mut seed = [0u8; 32];
		let mut data = vec![0u8; dlen];

		rng.fill_bytes(&mut seed);
		rng.fill_bytes(&mut data);

		let tx = AppExtrinsic::from(data.to_vec());
		let txs = [tx];

		c.bench_function(
			&format!("opt_par_build_commitments/{}x{}", dim.0, dim.1),
			|b| {
				b.iter(|| {
					let (_, _, _, _) = opt_par_build_commitments(
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

fn bench_build_proof(c: &mut Criterion) {
	let mut rng = ChaCha20Rng::from_entropy();

	const CHUNK: usize = DATA_CHUNK_SIZE as usize + 1;
	const DIMS: [(usize, usize); 5] = [
		(1024, 64),
		(512, 128),
		(256, 256),
		(MAX_BLOCK_ROWS as usize, MAX_BLOCK_COLUMNS as usize),
		(64, 1024),
	];

	for dim in DIMS {
		let dlen = dim.0 * dim.1 * (CHUNK - 2);

		let mut seed = [0u8; 32];
		let mut data = vec![0u8; dlen];

		rng.fill_bytes(&mut seed);
		rng.fill_bytes(&mut data);

		let tx = AppExtrinsic::from(data.to_vec());
		let txs = [tx];

		let public_params = crate::testnet::public_params(dim.1);

		let (_, _, dims, mat) = opt_par_build_commitments(dim.0, dim.1, CHUNK, &txs, seed).unwrap();

		c.bench_function(&format!("build_proof/{}x{}", dim.0, dim.1), |b| {
			b.iter(|| {
				let cell = Cell {
					row: rng.next_u32() % dims.rows as u32,
					col: rng.next_u32() % dims.cols as u32,
				};

				let proof = build_proof(&public_params, dims, &mat, &[cell]).unwrap();
				assert_eq!(proof.len(), 80);
			});
		});
	}
}

fn bench_extend_data_matrix(c: &mut Criterion) {
	const ROWS: usize = MAX_BLOCK_ROWS as usize;
	const COLS: usize = MAX_BLOCK_COLUMNS as usize;
	const CHUNK: usize = DATA_CHUNK_SIZE as usize + 1;
	const DLEN: usize = ROWS * COLS * (CHUNK - 2);

	let mut rng = ChaCha20Rng::from_entropy();

	let mut seed = [0u8; 32];
	let mut data = [0u8; DLEN];

	rng.fill_bytes(&mut seed);
	rng.fill_bytes(&mut data);

	let extrinsic = AppExtrinsic::from(data.to_vec());
	let extrinsics = [extrinsic];

	let (_, blk, dim) = flatten_and_pad_block(ROWS, COLS, CHUNK, &extrinsics, seed).unwrap();

	c.bench_function("extend_data_matrix", |b| {
		b.iter(|| {
			let extended = extend_data_matrix(black_box(dim), black_box(&blk)).unwrap();
			assert_eq!(extended.len(), ROWS * COLS * 2);
		});
	});
}

fn bench_par_extend_data_matrix(c: &mut Criterion) {
	const ROWS: usize = MAX_BLOCK_ROWS as usize;
	const COLS: usize = MAX_BLOCK_COLUMNS as usize;
	const CHUNK: usize = DATA_CHUNK_SIZE as usize + 1;
	const DLEN: usize = ROWS * COLS * (CHUNK - 2);

	let mut rng = ChaCha20Rng::from_entropy();

	let mut seed = [0u8; 32];
	let mut data = [0u8; DLEN];

	rng.fill_bytes(&mut seed);
	rng.fill_bytes(&mut data);

	let extrinsic = AppExtrinsic::from(data.to_vec());
	let extrinsics = [extrinsic];

	let (_, blk, dim) = flatten_and_pad_block(ROWS, COLS, CHUNK, &extrinsics, seed).unwrap();

	c.bench_function("par_extend_data_matrix", |b| {
		b.iter(|| {
			let extended = par_extend_data_matrix(black_box(dim), black_box(&blk)).unwrap();
			assert_eq!(extended.len(), ROWS * COLS * 2);
		});
	});
}

fn bench_ifft_on_commitments(c: &mut Criterion) {
	const ROWS: usize = MAX_BLOCK_ROWS as usize;
	const COLS: usize = MAX_BLOCK_COLUMNS as usize;
	const CHUNK: usize = DATA_CHUNK_SIZE as usize + 1;
	const DLEN: usize = ROWS * COLS * (CHUNK - 2);

	let mut rng = ChaCha20Rng::from_entropy();

	let mut seed = [0u8; 32];
	let mut data = [0u8; DLEN];

	rng.fill_bytes(&mut seed);
	rng.fill_bytes(&mut data);

	let extrinsic = AppExtrinsic::from(data.to_vec());
	let extrinsics = [extrinsic];

	let (_, blk, dims) = flatten_and_pad_block(ROWS, COLS, CHUNK, &extrinsics, seed).unwrap();
	let ext_rows = dims.rows * EXTENSION_FACTOR;

	let public_params = testnet::public_params(MAX_BLOCK_COLUMNS as usize);
	let (prover_key, _) = public_params.trim(dims.cols).map_err(Error::from).unwrap();

	let row_eval_domain = EvaluationDomain::new(dims.cols).unwrap();
	let col_eval_domain_ext = EvaluationDomain::new(ext_rows).unwrap();
	let col_eval_domain_red = EvaluationDomain::new(dims.rows).unwrap();

	let chunks = blk.chunks_exact(dims.chunk_size);
	assert!(chunks.remainder().is_empty());

	let chunk_elements = chunks
		.map(to_bls_scalar)
		.collect::<Result<Vec<BlsScalar>, Error>>()
		.unwrap();

	let mut commits = Vec::with_capacity(dims.rows);
	for i in 0..dims.rows {
		let mut row = Vec::with_capacity(dims.cols);

		for j in 0..dims.cols {
			row.push(chunk_elements[i + j * dims.rows]);
		}

		let poly = Evaluations::from_vec_and_domain(row, row_eval_domain).interpolate();
		let commit = prover_key.commit(&poly).unwrap();

		commits.push(commit);
	}

	c.bench_function("fft_on_commitments", |b| {
		b.iter(|| {
			let res = fft_on_commitments(
				black_box(commits.clone()),
				black_box(col_eval_domain_red),
				black_box(true),
			);
			assert_eq!(res.len(), dims.rows);
		});
	});

	let ifft = fft_on_commitments(commits.clone(), col_eval_domain_red, true);
	c.bench_function("ifft_on_commitments", |b| {
		b.iter(|| {
			let res = fft_on_commitments(
				black_box(ifft.clone()),
				black_box(col_eval_domain_ext),
				black_box(false),
			);
			assert_eq!(res.len(), ext_rows)
		});
	});

	c.bench_function("fft+ifft_on_commitments", |b| {
		b.iter(|| {
			let res = fft_on_commitments(
				black_box(fft_on_commitments(
					black_box(commits.clone()),
					black_box(col_eval_domain_red),
					black_box(true),
				)),
				black_box(col_eval_domain_ext),
				black_box(false),
			);
			assert_eq!(res.len(), ext_rows);
		});
	});
}

criterion_group! {name = kate_build_commitments; config = Criterion::default().sample_size(10); targets =  bench_build_commitments, bench_par_build_commitments, bench_opt_par_build_commitments, bench_build_proof, bench_extend_data_matrix, bench_par_extend_data_matrix, bench_ifft_on_commitments}
criterion_main!(kate_build_commitments);

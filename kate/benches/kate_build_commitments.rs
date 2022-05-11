extern crate criterion;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use da_primitives::asdr::AppExtrinsic;
use kate::{
	com::{
		build_commitments, extend_data_matrix, flatten_and_pad_block, par_build_commitments,
		par_extend_data_matrix,
	},
	config::{DATA_CHUNK_SIZE, MAX_BLOCK_COLUMNS, MAX_BLOCK_ROWS},
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

criterion_group! {name = kate_build_commitments; config = Criterion::default().sample_size(10); targets = bench_extend_data_matrix, bench_par_extend_data_matrix, bench_build_commitments, bench_par_build_commitments}
criterion_main!(kate_build_commitments);

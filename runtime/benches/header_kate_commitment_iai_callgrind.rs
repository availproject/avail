use iai_callgrind::{black_box, library_benchmark, library_benchmark_group, main};

include!("header_kate_commitment.rs");

fn commitment_builder(cols: BlockLengthColumns) {
	let txs = make_txs(cols);
	let block_length = block_length(cols);

	commitment_builder_with(txs, block_length);
}

#[library_benchmark]
fn commitment_builder_32() {
	black_box(commitment_builder(BlockLengthColumns(32)));
}

#[library_benchmark]
fn commitment_builder_64() {
	black_box(commitment_builder(BlockLengthColumns(64)));
}

#[library_benchmark]
fn commitment_builder_128() {
	black_box(commitment_builder(BlockLengthColumns(128)));
}

#[library_benchmark]
fn commitment_builder_256() {
	black_box(commitment_builder(BlockLengthColumns(256)));
}

library_benchmark_group!(
	name = commitment_builder;
	benchmarks = commitment_builder_32, commitment_builder_64, commitment_builder_128, commitment_builder_256
);

main!(library_benchmark_groups = commitment_builder);

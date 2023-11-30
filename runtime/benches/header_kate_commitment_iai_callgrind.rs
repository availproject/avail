include!("header_kate_commitment.rs");
use iai_callgrind::{black_box, library_benchmark, library_benchmark_group, main};

fn setup(cols: u32) -> (Vec<AppExtrinsic>, BlockLength) {
	let txs = make_txs(BlockLengthColumns(cols));
	let block_length = block_length(BlockLengthColumns(cols));

	(txs, block_length)
}

#[library_benchmark]
#[bench::columns_32(setup(32))]
#[bench::columns_64(setup(64))]
#[bench::columns_128(setup(128))]
#[bench::columns_256(setup(256))]
fn commitment_builder(input: (Vec<AppExtrinsic>, BlockLength)) {
	black_box(commitment_builder_with(input.0, input.1));
}

library_benchmark_group!(
	name = commitment_builder_group;
	benchmarks = commitment_builder
);

main!(library_benchmark_groups = commitment_builder_group,);

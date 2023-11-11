include!("header_kate_commitment.rs");
use divan;

fn main() {
	divan::main();
}

mod commitment_builder {
	use super::*;

	fn setup(cols: BlockLengthColumns) -> (Vec<AppExtrinsic>, BlockLength) {
		let txs = make_txs(cols);
		let block_length = block_length(cols);
		(txs, block_length)
	}

	#[divan::bench(max_time = 120.0, consts = [ 32, 64, 128, 256 ])]
	fn columns_count<const N: u32>(bencher: divan::Bencher) {
		bencher
			.counter(N)
			.with_inputs(|| setup(BlockLengthColumns(N)))
			.bench_values(|input| {
				commitment_builder_with(input.0, input.1);
			})
	}
}

use avail_core::{AppExtrinsic, BlockLengthColumns, BLOCK_CHUNK_SIZE, NORMAL_DISPATCH_RATIO};
use da_control::Config as DAConfig;
use da_runtime::Runtime;
use frame_support::traits::Get as _;
use frame_system::{header_builder::hosted_header_builder, limits::BlockLength};
use sp_core::H256;
use sp_std::iter::repeat;

fn make_txs(cols: BlockLengthColumns) -> Vec<AppExtrinsic> {
	let data_length: u32 = <Runtime as DAConfig>::MaxAppDataLength::get();
	let rows = <Runtime as DAConfig>::MaxBlockRows::get().0;

	let mut nb_tx = 4; // Value set depending on MaxAppDataLength (512 kb) to reach 2 mb
	let max_tx: u32 =
		rows * cols.0 * (BLOCK_CHUNK_SIZE.get().checked_sub(2).unwrap()) / data_length;
	if nb_tx > max_tx {
		nb_tx = max_tx;
	}

	let data: Vec<u8> = repeat(b'X')
		.take(usize::try_from(data_length).unwrap())
		.collect::<Vec<_>>();
	vec![AppExtrinsic::from(data); nb_tx as usize]
}

fn block_length(cols: BlockLengthColumns) -> BlockLength {
	let rows = <Runtime as DAConfig>::MaxBlockRows::get();
	BlockLength::with_normal_ratio(rows, cols, BLOCK_CHUNK_SIZE, NORMAL_DISPATCH_RATIO).unwrap()
}

fn commitment_builder_with(txs: Vec<AppExtrinsic>, block_length: BlockLength) {
	let seed = [0u8; 32];
	let root = H256::zero();
	let block_number: u32 = 0;

	let _ = hosted_header_builder::build(txs, root, block_length, block_number, seed);
}

fn commitment_builder(cols: BlockLengthColumns) {
	let txs = make_txs(cols);
	let block_length = block_length(cols);

	commitment_builder_with(txs, block_length);
}

fn commitment_builder_32() { commitment_builder(BlockLengthColumns(32)); }
fn commitment_builder_128() { commitment_builder(BlockLengthColumns(128)); }
fn commitment_builder_256() { commitment_builder(BlockLengthColumns(256)); }

iai::main! {commitment_builder_32, commitment_builder_128, commitment_builder_256} 

/*
mod iai_wrappers {
	pub fn commitment_builder_32() {
		let _ = $crate::black_box(super::commitment_builder_32());
	}
}
fn main() {
	z`

	let benchmarks: &[&(&'static str, fn())] =
		&[&("commitment_builder_32", iai_wrappers::commitment_builder_32)];

	iai::runner(benchmarks);
}
*/

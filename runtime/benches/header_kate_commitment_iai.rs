include!("header_kate_commitment.rs");

fn commitment_builder(cols: BlockLengthColumns) {
	let txs = make_txs(cols);
	let block_length = block_length(cols);

	commitment_builder_with(txs, block_length);
}

fn commitment_builder_32() {
	commitment_builder(BlockLengthColumns(32));
}

fn commitment_builder_64() {
	commitment_builder(BlockLengthColumns(64));
}

fn commitment_builder_128() {
	commitment_builder(BlockLengthColumns(128));
}
fn commitment_builder_256() {
	commitment_builder(BlockLengthColumns(256));
}

iai::main! {commitment_builder_32, commitment_builder_64, commitment_builder_128, commitment_builder_256 }

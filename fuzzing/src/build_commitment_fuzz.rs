use afl;
use da_primitives::asdr::AppExtrinsic;
use hex_literal::hex;

fn main() {
	println!("Starting build_commitment() fuzzer...");

	afl::fuzz!(|data: &[u8]| {
		let block_rows = 256;
		let block_cols = 256;
		let chunk_size = 32;
		let hash = hex!("4c29ae91bbc61204b6f95d1f3c3a5aa6ac2f29da18d4423e5bbbf815693").into();

		let (_, _, _, _) = kate::com::build_commitments(
			block_rows,
			block_cols,
			chunk_size,
			&[AppExtrinsic::from(data.to_vec())],
			&hash,
		)
		.unwrap();
	});
}

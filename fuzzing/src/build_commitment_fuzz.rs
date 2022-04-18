use afl;
use da_primitives::asdr::AppExtrinsic;

fn main() {
	println!("Starting build_commitment() fuzzer...");

	afl::fuzz!(|data: &[u8]| {
		let block_rows = 256;
		let block_cols = 256;
		let chunk_size = 32;
		let hash: Vec<u8> = vec![
			76, 41, 174, 145, 187, 12, 97, 32, 75, 111, 149, 209, 243, 195, 165, 10, 166, 172, 47,
			41, 218, 24, 212, 66, 62, 5, 187, 191, 129, 5, 105, 3,
		];

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

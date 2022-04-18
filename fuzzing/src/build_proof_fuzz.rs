use afl;
use da_primitives::asdr::AppExtrinsic;

fn main() {
	println!("Starting build_proof() fuzzer...");

	afl::fuzz!(|data: &[u8]| {
		let hash: Vec<u8> = vec![
			76, 41, 174, 145, 187, 12, 97, 32, 75, 111, 149, 209, 243, 195, 165, 10, 166, 172, 47,
			41, 218, 24, 212, 66, 62, 5, 187, 191, 129, 5, 105, 3,
		];

		let rows_num = 256;
		let cols_num = 256;
		let chunk_size = 32;

		let public_params = kate::testnet::public_params(kate::config::MAX_BLOCK_COLUMNS as usize);

		let mut cells = Vec::new();
		for i in 0..cols_num {
			for j in 0..rows_num {
				cells.push(kate::com::Cell {
					row: i as u32,
					col: j as u32,
				})
			}
		}

		kate::com::flatten_and_pad_block(
			rows_num,
			cols_num,
			chunk_size,
			&[AppExtrinsic::from(data.to_vec())],
			&hash,
		)
		.and_then(|(_, block, block_dims)| {
			kate::com::extend_data_matrix(block_dims, &block)
				.map(move |matrix| (block_dims, matrix))
		})
		.and_then(move |(block_dims, ext_data_matrix)| {
			kate::com::build_proof(&public_params, block_dims, &ext_data_matrix, &cells).unwrap();
			Ok(())
		});
	});
}

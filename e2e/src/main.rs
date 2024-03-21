#![warn(unused_extern_crates)]

fn main() {}

#[cfg(test)]
pub mod tests {
	use std::num::NonZeroU16;

	use avail_core::{AppExtrinsic, AppId};
	use avail_subxt::{
		avail::{GRawScalar, GRow, Rows},
		helpers::submitted_data_from,
		rpc::KateRpcClient as _,
		submit::submit_data,
		tx, AvailClient,
	};

	use anyhow::{anyhow, Result};
	use kate::gridgen::{AsBytes as _, EvaluationGrid};
	use kate_recovery::matrix::Dimensions;
	use subxt_signer::sr25519::dev;

	pub const MIN_WIDTH: usize = 4;

	async fn establish_a_connection() -> Result<AvailClient> {
		let ws = String::from("ws://127.0.0.1:9944");
		let client = AvailClient::new(ws).await?;
		Ok(client)
	}

	/*
	async fn get_block_app_extrinsics(
		submitted_block: &Block<AvailConfig, OnlineClient<AvailConfig>>,
	) -> Result<Vec<AppExtrinsic>> {
		let mut data: Vec<AppExtrinsic> = Vec::new();

		let transactions = submitted_block.extrinsics().await?;
		for tx in transactions.iter() {
			let tx = tx.unwrap();

			let mut app_id = 0;
			if let Some(signed_extensions) = tx.signed_extensions() {
				app_id = signed_extensions.find::<CheckAppId>().unwrap().unwrap().0;
			}

			data.push(AppExtrinsic::new(AppId(app_id), tx.bytes().into()));
		}

		Ok(data)
	}*/

	// async fn query_app_data(
	// 	rpc: &Rpc<AvailConfig>,
	// 	app_id: AppId,
	// 	block_hash: H256,
	// ) -> anyhow::Result<Vec<Option<Vec<u8>>>> {
	// 	let mut params = RpcParams::new();
	// 	params.push(app_id)?;
	// 	params.push(Some(block_hash))?;
	// 	let rows: Vec<Option<Vec<u8>>> = rpc.request("kate_queryAppData", params).await?;

	// 	Ok(rows)
	// }

	// async fn query_block_length(
	// 	rpc: &Rpc<AvailConfig>,
	// 	block_hash: H256,
	// ) -> anyhow::Result<BlockLength> {
	// 	let mut params = RpcParams::new();
	// 	params.push(Some(block_hash))?;
	// 	let block_length: BlockLength = rpc.request("kate_blockLength", params).await?;

	// 	Ok(block_length)
	// }

	// async fn query_data_proof(
	// 	rpc: &Rpc<AvailConfig>,
	// 	transaction_index: u32,
	// 	block_hash: H256,
	// ) -> anyhow::Result<ProofResponse> {
	// 	let mut params = RpcParams::new();
	// 	params.push(transaction_index)?;
	// 	params.push(Some(block_hash))?;
	// 	let data_proof: ProofResponse = rpc.request("kate_queryDataProof", params).await?;
	// 	Ok(data_proof)
	// }

	/*
	async fn query_proof(
		client: &AvailClient,
		cells: Vec<avail_subxt::Cell>,
		block_hash: H256,
	) -> anyhow::Result<Vec<GDataProof>> {
		let mut params = RpcParams::new();
		let cells = Cells::try_from(cells).expect("Valid bounds .qed");
		params.push(cells)?;
		params.push(Some(block_hash))?;
		let proof: Vec<GDataProof> = client.rpc().request("kate_queryProof", params).await?;

		Ok(proof)
	}*/

	#[async_std::test]
	pub async fn rpc_query_rows_test() -> Result<()> {
		let client = establish_a_connection().await?;
		let alice = dev::alice();

		println!("Data submitted...");
		let in_block =
			tx::in_finalized(submit_data(&client, &alice, b"ExampleData", AppId(1)).await?).await?;
		let block_hash = in_block.block_hash();
		println!("Submitted finalized at block hash: {block_hash:?}");

		let block_len = client.rpc_methods().query_block_length(block_hash).await?;
		let block = client.blocks().at(block_hash).await?;
		let extrinsics = block.extrinsics().await?;
		let submitted = submitted_data_from(&extrinsics);
		println!("Submitted data: {submitted:#?}");
		let seed = [0u8; 32];
		let max_width = block_len.cols.0 as usize;
		let max_height = block_len.rows.0 as usize;

		// Grid Creation
		println!("Generating Evaluation Grid ...");
		let app_ext = submitted
			.into_iter()
			.map(|s| AppExtrinsic::new(s.id, s.data))
			.collect::<Vec<_>>();
		let grid = EvaluationGrid::from_extrinsics(app_ext, MIN_WIDTH, max_width, max_height, seed)
			.map_err(|e| anyhow!("Eval grid failed {e:?}"))?;
		let extended_grid = grid.extend_columns(NonZeroU16::new(2).unwrap()).unwrap();

		assert_eq!(grid.dims(), Dimensions::new(1, 4).unwrap());
		assert_eq!(extended_grid.dims(), Dimensions::new(2, 4).unwrap());
		assert_eq!(grid.row(0), extended_grid.row(0));
		assert_eq!(grid.row(0), extended_grid.row(1));

		println!("RPC query rows ...");
		// RPC call: Querying non existing rows should fail
		let row_indexes = Rows::truncate_from(vec![2]);
		let rows = client
			.rpc_methods()
			.query_rows(row_indexes, block_hash)
			.await;
		assert!(rows.is_err());

		// RPC call: Querying existing rows should NOT fail
		let row_indexes = Rows::truncate_from(vec![0]);
		let actual_rows = client
			.rpc_methods()
			.query_rows(row_indexes, block_hash)
			.await?;

		let expected_rows = [extended_grid.row(0)]
			.into_iter()
			.filter_map(|row| {
				row.map(|r| {
					r.iter()
						.flat_map(|r| r.to_bytes().map(GRawScalar::from))
						.collect()
				})
			})
			.collect::<Vec<GRow>>();

		assert_eq!(actual_rows, expected_rows);
		Ok(())
	}

	// #[async_std::test]
	// pub async fn rpc_query_app_data_test() {
	// 	let client = establish_a_connection().await.unwrap();
	// 	let (txc, rpc) = (client.tx(), client.rpc());

	// 	let example_data = "ExampleData".as_bytes();
	// 	assert_eq!(example_data.len(), 11);

	// 	let block_hash = send_da_example_data(&txc, example_data).await.unwrap();
	// 	let submitted_block = get_submitted_block(rpc, block_hash).await.unwrap();
	// 	let app_extrinsics = get_block_app_extrinsics(&submitted_block).unwrap();

	// 	// Grid Creation
	// 	let grid = EvaluationGrid::from_extrinsics(app_extrinsics, 4, 256, 256, [0u8; 32]).unwrap();
	// 	assert_eq!(grid.dims(), Dimensions::new(1, 8).unwrap());

	// 	// RPC call
	// 	let actual_rows = query_app_data(rpc, AppId(0), block_hash).await.unwrap();

	// 	let row = grid.row(0).unwrap();
	// 	let flat_row: Vec<u8> = row.iter().flat_map(|r| r.to_bytes().unwrap()).collect();
	// 	let expected_rows: Vec<Option<Vec<u8>>> = vec![Some(flat_row)];

	// 	assert_eq!(actual_rows.len(), expected_rows.len());
	// 	for i in 0..actual_rows.len() {
	// 		assert_eq!(actual_rows[i], expected_rows[i]);
	// 	}
	// }

	/*
	#[async_std::test]
	pub async fn rpc_query_proof_test() {
		let client = establish_a_connection().await.unwrap();

		let example_data = "ExampleData".as_bytes();
		assert_eq!(example_data.len(), 11);

		let block_hash = send_da_example_data(&client, example_data).await.unwrap();
		let submitted_block = get_submitted_block(&client, block_hash).await.unwrap();
		let app_extrinsics = get_block_app_extrinsics(&submitted_block).await.unwrap();

		// Grid Creation
		let grid = EvaluationGrid::from_extrinsics(app_extrinsics, 4, 256, 256, [0u8; 32]).unwrap();
		let extended_grid = grid.extend_columns(NonZeroU16::new(2).unwrap()).unwrap();
		let poly_grid = extended_grid.make_polynomial_grid().unwrap();

		assert_eq!(grid.dims(), Dimensions::new(1, 8).unwrap());
		assert_eq!(extended_grid.dims(), Dimensions::new(2, 8).unwrap());
		assert_eq!(grid.row(0), extended_grid.row(0));
		assert_eq!(grid.row(0), extended_grid.row(1));

		let cells = vec![
			Cell::new(BlockLengthRows(0), BlockLengthColumns(0)),
			Cell::new(BlockLengthRows(0), BlockLengthColumns(1)),
			Cell::new(BlockLengthRows(0), BlockLengthColumns(2)),
			Cell::new(BlockLengthRows(0), BlockLengthColumns(3)),
			Cell::new(BlockLengthRows(0), BlockLengthColumns(4)),
			Cell::new(BlockLengthRows(0), BlockLengthColumns(5)),
			Cell::new(BlockLengthRows(0), BlockLengthColumns(6)),
			Cell::new(BlockLengthRows(0), BlockLengthColumns(7)),
		];

		let multiproof_srs = kate::couscous::multiproof_params();
		let expected_proof: Vec<Vec<u8>> = cells
			.iter()
			.map(|cell| {
				let row = usize::try_from(cell.row.0).unwrap();
				let col = usize::try_from(cell.col.0).unwrap();
				let data = extended_grid.get::<usize, usize>(row, col).unwrap();
				let proof = poly_grid.proof(&multiproof_srs, cell).unwrap();

				let data = data.to_bytes().expect("Ser cannot fail").to_vec();
				let proof = proof.to_bytes().expect("Ser cannot fail").to_vec();

				[proof, data].into_iter().flatten().collect::<Vec<u8>>()
			})
			.collect();
		let expected_proof: Vec<u8> = expected_proof.into_iter().flatten().collect();

		let cells = vec![
			avail_subxt::Cell::new(0, 0),
			avail_subxt::Cell::new(0, 1),
			avail_subxt::Cell::new(0, 2),
			avail_subxt::Cell::new(0, 3),
			avail_subxt::Cell::new(0, 4),
			avail_subxt::Cell::new(0, 5),
			avail_subxt::Cell::new(0, 6),
			avail_subxt::Cell::new(0, 7),
		];

		// RPC call
		let actual_proof = query_proof(&client, cells, block_hash)
			.await
			.unwrap()
			.into_iter()
			.map(|p| p.1)
			.map(|p| Vec::<u8>::from(p))
			.flatten()
			.collect::<Vec<u8>>();
		assert_eq!(actual_proof, expected_proof);
		dbg!(&actual_proof);
		dbg!(&expected_proof);
	}*/

	// #[async_std::test]
	// pub async fn rpc_query_proof_test_2() {
	// 	let client = establish_a_connection().await.unwrap();
	// 	let (txc, rpc) = (client.tx(), client.rpc());

	// 	let mut example_data = [0u8; 12_500];
	// 	example_data[..7].copy_from_slice(b"example");
	// 	assert_eq!(example_data.len(), 12_500);

	// 	let block_hash = send_da_example_data(&txc, &example_data).await.unwrap();

	// 	let cell = Cell {
	// 		row: BlockLengthRows(0),
	// 		col: BlockLengthColumns(0),
	// 	};
	// 	let cells = vec![cell.clone()];

	// 	// RPC call
	// 	let actual_proof = query_proof(client.rpc(), cells, block_hash).await.unwrap();
	// 	assert_eq!(actual_proof.len(), 80);

	// 	let pp = kate::couscous::public_params();

	// 	let submitted_block = get_submitted_block(rpc, block_hash).await.unwrap();
	// 	let (commitment, rows, cols) = match submitted_block.block.header.extension {
	// 		HeaderExtension::V3(ext) => (
	// 			ext.commitment.commitment,
	// 			ext.commitment.rows,
	// 			ext.commitment.cols,
	// 		),
	// 	};

	// 	let mut content = [0u8; 80];
	// 	content.copy_from_slice(&actual_proof);
	// 	let commitment: [u8; 48] = commitment[..48].try_into().unwrap();
	// 	let dcell = kate_recovery::data::Cell {
	// 		position: kate_recovery::matrix::Position { row: 0, col: 0 },
	// 		content,
	// 	};
	// 	let dim = Dimensions::new(rows, cols).unwrap();
	// 	let res = verify(&pp, dim, &commitment, &dcell).unwrap();
	// 	assert!(res);
	// }

	// #[async_std::test]
	// pub async fn rpc_query_block_length_test() {
	// 	use avail_subxt::api::runtime_types::avail_core::{BlockLengthColumns, BlockLengthRows};

	// 	let client = establish_a_connection().await.unwrap();
	// 	let (txc, rpc) = (client.tx(), client.rpc());

	// 	let example_data = "ExampleData".as_bytes();
	// 	assert_eq!(example_data.len(), 11);

	// 	let block_hash = send_da_example_data(&txc, example_data).await.unwrap();

	// 	// RPC call
	// 	let length = query_block_length(rpc, block_hash).await.unwrap();
	// 	assert_eq!(length.cols, BlockLengthColumns(256));
	// 	assert_eq!(length.rows, BlockLengthRows(256));
	// 	assert_eq!(length.chunk_size, 32);
	// }

	// #[async_std::test]
	// pub async fn rpc_query_data_proof_test() {
	// 	let client = establish_a_connection().await.unwrap();
	// 	let (txc, rpc) = (client.tx(), client.rpc());

	// 	// data hash: 729afe29f4e9fee2624d7ed311bcf57d24683fb78938bcb4e2a6a22c4968795e
	// 	let example_data = "ExampleData".as_bytes();
	// 	assert_eq!(example_data.len(), 11);
	// 	let block_hash = send_da_example_data(&txc, example_data).await.unwrap();
	// 	let expected_proof_root =
	// 		merkle_proof::<Keccak256, _, _>(vec![keccak_256(example_data)], 0);
	// 	let actual_proof = query_data_proof(rpc, 1, block_hash).await.unwrap();
	// 	// root is calculated keccak256(blob_root, bridge_root)
	// 	let mut root_data = vec![];
	// 	root_data.extend(expected_proof_root.root.as_bytes());
	// 	root_data.extend(H256::zero().as_bytes());
	// 	let expected_data_root = keccak_256(root_data.as_slice());

	// 	assert_eq!(actual_proof.data_proof.data_root, H256(expected_data_root));
	// 	assert_eq!(actual_proof.data_proof.proof, expected_proof_root.proof);
	// 	assert_eq!(actual_proof.data_proof.blob_root, expected_proof_root.root);
	// 	assert_eq!(actual_proof.data_proof.bridge_root, H256::zero());
	// }
}

#![warn(unused_extern_crates)]

fn main() {}

#[cfg(test)]
pub mod tests {
	use std::num::NonZeroU16;

	use avail_core::data_proof_v2::ProofResponse;
	use avail_core::DataProof;
	use avail_core::{AppExtrinsic, AppId, BlockLengthColumns, BlockLengthRows};
	use avail_subxt::{
		api::{
			self,
			runtime_types::{
				avail_core::header::extension::HeaderExtension,
				bounded_collections::bounded_vec::BoundedVec, frame_system::limits::BlockLength,
			},
		},
		avail::{AppUncheckedExtrinsic, PairSigner},
		build_client,
		primitives::AvailExtrinsicParams,
		AvailConfig, Opts,
	};
	use binary_merkle_tree::merkle_proof;
	use codec::Encode;
	use kate::com::Cell;
	use kate::gridgen::{AsBytes, EvaluationGrid};
	use kate_recovery::matrix::Dimensions;
	use kate_recovery::proof::verify;
	use sp_keyring::AccountKeyring;
	use subxt::ext::sp_core::keccak_256;
	use subxt::ext::sp_runtime::traits::Keccak256;
	use subxt::tx::TxClient;
	use subxt::{
		ext::sp_core::H256,
		rpc::{types::ChainBlockResponse, Rpc, RpcParams},
		OnlineClient,
	};

	async fn establish_a_connection() -> anyhow::Result<OnlineClient<AvailConfig>> {
		let args = Opts {
			ws: String::from("ws://127.0.0.1:9944"),
			validate_codegen: false,
		};
		let (client, _rpc) = build_client(args.ws, args.validate_codegen).await?;
		Ok(client)
	}

	async fn send_da_example_data(
		txc: &TxClient<AvailConfig, OnlineClient<AvailConfig>>,
		data: &[u8],
	) -> anyhow::Result<H256> {
		let signer = PairSigner::new(AccountKeyring::Alice.pair());

		let call = api::tx()
			.data_availability()
			.submit_data(BoundedVec(data.to_vec().clone()));
		let extrinsic_params = AvailExtrinsicParams::new_with_app_id(0.into());

		let tx_progress = txc
			.sign_and_submit_then_watch(&call, &signer, extrinsic_params)
			.await?;
		let events = tx_progress.wait_for_finalized_success().await?;

		Ok(events.block_hash())
	}

	async fn get_submitted_block(
		rpc: &Rpc<AvailConfig>,
		block_hash: H256,
	) -> anyhow::Result<ChainBlockResponse<AvailConfig>> {
		let maybe_block = rpc.block(Some(block_hash)).await;
		let maybe_block = maybe_block?;

		Ok(maybe_block.unwrap())
	}

	fn get_block_app_extrinsics(
		submitted_block: &ChainBlockResponse<AvailConfig>,
	) -> anyhow::Result<Vec<AppExtrinsic>> {
		let transactions = submitted_block.block.extrinsics.clone();
		let mut data: Vec<(AppId, Vec<u8>)> = Vec::new();
		for tx in transactions {
			let unchecked = AppUncheckedExtrinsic::try_from(tx)?;
			let encoded = unchecked.encode();
			let signature = unchecked.signature;
			let app_id = signature.map(|s| s.2.app_id.0).unwrap_or(0);

			data.push((AppId(app_id), encoded));
		}

		let app_extrinsics: Vec<AppExtrinsic> = data
			.into_iter()
			.map(|d| AppExtrinsic::new(d.0, d.1))
			.collect();

		Ok(app_extrinsics)
	}

	async fn query_rows(
		rpc: &Rpc<AvailConfig>,
		rows: &[usize],
		block_hash: H256,
	) -> anyhow::Result<Vec<Vec<u8>>> {
		let mut params = RpcParams::new();
		params.push(rows)?;
		params.push(Some(block_hash))?;
		let rows: Vec<Vec<u8>> = rpc.request("kate_queryRows", params).await?;

		Ok(rows)
	}

	async fn query_app_data(
		rpc: &Rpc<AvailConfig>,
		app_id: AppId,
		block_hash: H256,
	) -> anyhow::Result<Vec<Option<Vec<u8>>>> {
		let mut params = RpcParams::new();
		params.push(app_id)?;
		params.push(Some(block_hash))?;
		let rows: Vec<Option<Vec<u8>>> = rpc.request("kate_queryAppData", params).await?;

		Ok(rows)
	}

	async fn query_block_length(
		rpc: &Rpc<AvailConfig>,
		block_hash: H256,
	) -> anyhow::Result<BlockLength> {
		let mut params = RpcParams::new();
		params.push(Some(block_hash))?;
		let block_length: BlockLength = rpc.request("kate_blockLength", params).await?;

		Ok(block_length)
	}

	async fn query_data_proof(
		rpc: &Rpc<AvailConfig>,
		transaction_index: u32,
		block_hash: H256,
	) -> anyhow::Result<DataProof> {
		let mut params = RpcParams::new();
		params.push(transaction_index)?;
		params.push(Some(block_hash))?;
		let data_proof: DataProof = rpc.request("kate_queryDataProof", params).await?;

		Ok(data_proof)
	}

	async fn query_data_proof_v2(
		rpc: &Rpc<AvailConfig>,
		transaction_index: u32,
		block_hash: H256,
	) -> anyhow::Result<ProofResponse> {
		let mut params = RpcParams::new();
		params.push(transaction_index)?;
		params.push(Some(block_hash))?;
		let data_proof: ProofResponse = rpc.request("kate_queryDataProofV2", params).await?;
		Ok(data_proof)
	}

	async fn query_proof(
		rpc: &Rpc<AvailConfig>,
		cells: Vec<Cell>,
		block_hash: H256,
	) -> anyhow::Result<Vec<u8>> {
		let mut params = RpcParams::new();
		params.push(cells)?;
		params.push(Some(block_hash))?;
		let proof: Vec<u8> = rpc.request("kate_queryProof", params).await?;

		Ok(proof)
	}

	#[async_std::test]
	pub async fn rpc_query_rows_test() {
		let client = establish_a_connection().await.unwrap();
		let (txc, rpc) = (client.tx(), client.rpc());

		let example_data = "ExampleData".as_bytes();
		assert_eq!(example_data.len(), 11);

		let block_hash = send_da_example_data(&txc, example_data).await.unwrap();
		let submitted_block = get_submitted_block(rpc, block_hash).await.unwrap();
		let app_extrinsics = get_block_app_extrinsics(&submitted_block).unwrap();

		// Grid Creation
		let grid = EvaluationGrid::from_extrinsics(app_extrinsics, 4, 256, 256, [0u8; 32]).unwrap();
		let extended_grid = grid.extend_columns(NonZeroU16::new(2).unwrap()).unwrap();

		assert_eq!(grid.dims(), Dimensions::new(1, 8).unwrap());
		assert_eq!(extended_grid.dims(), Dimensions::new(2, 8).unwrap());
		assert_eq!(grid.row(0), extended_grid.row(0));
		assert_eq!(grid.row(0), extended_grid.row(1));

		// RPC call: Querying non existing rows should fail
		assert!(query_rows(rpc, &[2], block_hash).await.is_err());

		// RPC call: Querying existing rows should NOT fail
		let actual_rows = query_rows(rpc, &[0, 1], block_hash).await.unwrap();

		let mut expected_rows: Vec<Vec<u8>> = Vec::new();
		for row in [extended_grid.row(0), extended_grid.row(1)] {
			if let Some(row) = row {
				let flat_row: Vec<u8> = row.iter().flat_map(|r| r.to_bytes().unwrap()).collect();
				expected_rows.push(flat_row);
			}
		}

		assert_eq!(actual_rows.len(), expected_rows.len());
		for i in 0..actual_rows.len() {
			assert_eq!(actual_rows[i], expected_rows[i]);
		}
	}

	#[async_std::test]
	pub async fn rpc_query_app_data_test() {
		let client = establish_a_connection().await.unwrap();
		let (txc, rpc) = (client.tx(), client.rpc());

		let example_data = "ExampleData".as_bytes();
		assert_eq!(example_data.len(), 11);

		let block_hash = send_da_example_data(&txc, example_data).await.unwrap();
		let submitted_block = get_submitted_block(rpc, block_hash).await.unwrap();
		let app_extrinsics = get_block_app_extrinsics(&submitted_block).unwrap();

		// Grid Creation
		let grid = EvaluationGrid::from_extrinsics(app_extrinsics, 4, 256, 256, [0u8; 32]).unwrap();
		assert_eq!(grid.dims(), Dimensions::new(1, 8).unwrap());

		// RPC call
		let actual_rows = query_app_data(rpc, AppId(0), block_hash).await.unwrap();

		let row = grid.row(0).unwrap();
		let flat_row: Vec<u8> = row.iter().flat_map(|r| r.to_bytes().unwrap()).collect();
		let expected_rows: Vec<Option<Vec<u8>>> = vec![Some(flat_row)];

		assert_eq!(actual_rows.len(), expected_rows.len());
		for i in 0..actual_rows.len() {
			assert_eq!(actual_rows[i], expected_rows[i]);
		}
	}

	#[async_std::test]
	pub async fn rpc_query_proof_test() {
		let client = establish_a_connection().await.unwrap();
		let (txc, rpc) = (client.tx(), client.rpc());

		let example_data = "ExampleData".as_bytes();
		assert_eq!(example_data.len(), 11);

		let block_hash = send_da_example_data(&txc, example_data).await.unwrap();
		let submitted_block = get_submitted_block(rpc, block_hash).await.unwrap();
		let app_extrinsics = get_block_app_extrinsics(&submitted_block).unwrap();

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

		// RPC call
		let actual_proof = query_proof(rpc, cells, block_hash).await.unwrap();
		assert_eq!(actual_proof, expected_proof);
		dbg!(&actual_proof);
		dbg!(&expected_proof);
	}

	#[async_std::test]
	pub async fn rpc_query_proof_test_2() {
		let client = establish_a_connection().await.unwrap();
		let (txc, rpc) = (client.tx(), client.rpc());

		let mut example_data = [0u8; 12_500];
		example_data[..7].copy_from_slice(b"example");
		assert_eq!(example_data.len(), 12_500);

		let block_hash = send_da_example_data(&txc, &example_data).await.unwrap();

		let cell = Cell {
			row: BlockLengthRows(0),
			col: BlockLengthColumns(0),
		};
		let cells = vec![cell.clone()];

		// RPC call
		let actual_proof = query_proof(client.rpc(), cells, block_hash).await.unwrap();
		assert_eq!(actual_proof.len(), 80);

		let pp = kate::couscous::public_params();

		let submitted_block = get_submitted_block(rpc, block_hash).await.unwrap();
		let ext = if let HeaderExtension::V2(ref ext) = submitted_block.block.header.extension {
			ext
		} else {
			panic!("Unsupported header extension version")
		};

		let mut content = [0u8; 80];
		content.copy_from_slice(&actual_proof);
		let commitment: [u8; 48] = ext.commitment.commitment[..48].try_into().unwrap();
		let dcell = kate_recovery::data::Cell {
			position: kate_recovery::matrix::Position { row: 0, col: 0 },
			content,
		};
		let dim = Dimensions::new(ext.commitment.rows, ext.commitment.cols).unwrap();
		let res = verify(&pp, dim, &commitment, &dcell).unwrap();
		assert!(res);
	}

	#[async_std::test]
	pub async fn rpc_query_block_length_test() {
		use avail_subxt::api::runtime_types::avail_core::{BlockLengthColumns, BlockLengthRows};

		let client = establish_a_connection().await.unwrap();
		let (txc, rpc) = (client.tx(), client.rpc());

		let example_data = "ExampleData".as_bytes();
		assert_eq!(example_data.len(), 11);

		let block_hash = send_da_example_data(&txc, example_data).await.unwrap();

		// RPC call
		let length = query_block_length(rpc, block_hash).await.unwrap();
		assert_eq!(length.cols, BlockLengthColumns(256));
		assert_eq!(length.rows, BlockLengthRows(256));
		assert_eq!(length.chunk_size, 32);
	}

	// #[async_std::test]
	// pub async fn rpc_query_data_proof_test() {
	// 	let client = establish_a_connection().await.unwrap();
	// 	let (txc, rpc) = (client.tx(), client.rpc());

	// 	let example_data = "ExampleData".as_bytes();
	// 	assert_eq!(example_data.len(), 11);
	// 	let block_hash = send_da_example_data(&txc, example_data).await.unwrap();

	// 	let expected_proof_root = merkle_proof::<Keccak256, _, _>(vec![example_data.to_vec()], 0);
	// 	let actual_proof = query_data_proof(rpc, 1, block_hash).await.unwrap();
	// 	assert_eq!(actual_proof.root, expected_proof_root.root);
	// }

	#[async_std::test]
	pub async fn rpc_query_data_proof_v2_test() {
		let client = establish_a_connection().await.unwrap();
		let (txc, rpc) = (client.tx(), client.rpc());

		// data hash: 729afe29f4e9fee2624d7ed311bcf57d24683fb78938bcb4e2a6a22c4968795e
		let example_data = "ExampleData".as_bytes();
		assert_eq!(example_data.len(), 11);
		let block_hash = send_da_example_data(&txc, example_data).await.unwrap();
		let expected_proof_root =
			merkle_proof::<Keccak256, _, _>(vec![keccak_256(example_data)], 0);
		let actual_proof = query_data_proof_v2(rpc, 1, block_hash).await.unwrap();
		// root is calculated keccak256(blob_root, bridge_root)
		let mut root_data = vec![];
		root_data.extend(expected_proof_root.root.as_bytes());
		root_data.extend(H256::zero().as_bytes());
		let expected_data_root = keccak_256(root_data.as_slice());

		assert_eq!(actual_proof.data_proof.data_root, H256(expected_data_root));
		assert_eq!(actual_proof.data_proof.proof, expected_proof_root.proof);
		assert_eq!(actual_proof.data_proof.blob_root, expected_proof_root.root);
		assert_eq!(actual_proof.data_proof.bridge_root, H256::zero());
	}
}

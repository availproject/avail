#![warn(unused_extern_crates)]

fn main() {}

#[cfg(test)]
pub mod tests {
	use std::num::NonZeroU16;

	use avail_core::{AppExtrinsic, AppId, BlockLengthColumns, BlockLengthRows};
	use avail_core::{DataProof, Keccak256};
	use avail_subxt::{
		api::{
			self,
			runtime_types::{
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
	use sp_keyring::AccountKeyring;
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
		build_client(args.ws, args.validate_codegen).await
	}

	async fn send_da_example_data(
		client: &OnlineClient<AvailConfig>,
	) -> anyhow::Result<(H256, Vec<u8>)> {
		let signer = PairSigner::new(AccountKeyring::Alice.pair());
		let example_data = "ExampleData".as_bytes();
		assert_eq!(example_data.len(), 11);

		let call = api::tx()
			.data_availability()
			.submit_data(BoundedVec(example_data.to_vec().clone()));
		let extrinsic_params = AvailExtrinsicParams::new_with_app_id(0.into());

		let tx_progress = client
			.tx()
			.sign_and_submit_then_watch(&call, &signer, extrinsic_params)
			.await?;
		let events = tx_progress.wait_for_finalized_success().await?;

		Ok((events.block_hash(), example_data.to_vec()))
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

	async fn query_row(
		rpc: &Rpc<AvailConfig>,
		rows: &[usize],
		block_hash: H256,
	) -> anyhow::Result<Vec<Option<Vec<u8>>>> {
		let mut params = RpcParams::new();
		params.push(rows)?;
		params.push(Some(block_hash))?;
		let rows: Vec<Option<Vec<u8>>> = rpc.request("kate_queryRows", params).await?;

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
		let block_length_raw: Vec<u8> = rpc.request("kate_blockLength", params).await?;

		// It just works, deal with it.
		let block_length = unsafe { std::mem::transmute::<Vec<u8>, BlockLength>(block_length_raw) };

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

		let (block_hash, _) = send_da_example_data(&client).await.unwrap();
		let submitted_block = get_submitted_block(client.rpc(), block_hash).await.unwrap();
		let app_extrinsics = get_block_app_extrinsics(&submitted_block).unwrap();

		// Grid Creation
		let grid = EvaluationGrid::from_extrinsics(app_extrinsics, 4, 256, 256, [0u8; 32]).unwrap();
		let extended_grid = grid.extend_columns(NonZeroU16::new(2).unwrap()).unwrap();

		assert_eq!(grid.dims(), Dimensions::new(1, 8).unwrap());
		assert_eq!(extended_grid.dims(), Dimensions::new(2, 8).unwrap());
		assert_eq!(grid.row(0), extended_grid.row(0));
		assert_eq!(grid.row(0), extended_grid.row(1));

		// RPC call
		let actual_rows = query_row(client.rpc(), &[0, 1, 2], block_hash)
			.await
			.unwrap();

		let mut expected_rows: Vec<Option<Vec<u8>>> = Vec::new();
		let iter = [
			extended_grid.row(0),
			extended_grid.row(1),
			extended_grid.row(2),
		];
		for row in iter {
			if let Some(row) = row {
				let flat_row: Vec<u8> = row.iter().flat_map(|r| r.to_bytes().unwrap()).collect();
				expected_rows.push(Some(flat_row))
			} else {
				expected_rows.push(None);
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

		let (block_hash, _) = send_da_example_data(&client).await.unwrap();
		let submitted_block = get_submitted_block(client.rpc(), block_hash).await.unwrap();
		let app_extrinsics = get_block_app_extrinsics(&submitted_block).unwrap();

		// Grid Creation
		let grid = EvaluationGrid::from_extrinsics(app_extrinsics, 4, 256, 256, [0u8; 32]).unwrap();
		assert_eq!(grid.dims(), Dimensions::new(1, 8).unwrap());

		// RPC call
		let actual_rows = query_app_data(client.rpc(), AppId(0), block_hash)
			.await
			.unwrap();

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

		let (block_hash, _) = send_da_example_data(&client).await.unwrap();
		let submitted_block = get_submitted_block(client.rpc(), block_hash).await.unwrap();
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

		let multiproof_srs = kate::testnet::multiproof_params(256, 256);
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
		let actual_proof = query_proof(client.rpc(), cells, block_hash).await.unwrap();
		assert_eq!(actual_proof, expected_proof);
		dbg!(&actual_proof);
		dbg!(&expected_proof);
	}

	#[async_std::test]
	pub async fn rpc_query_block_length_test() {
		let client = establish_a_connection().await.unwrap();

		let (block_hash, _) = send_da_example_data(&client).await.unwrap();

		// RPC call
		let length = query_block_length(client.rpc(), block_hash).await.unwrap();
		dbg!(length);
	}

	#[async_std::test]
	pub async fn rpc_query_data_proof_test() {
		let client = establish_a_connection().await.unwrap();
		let (block_hash, ab) = send_da_example_data(&client).await.unwrap();

		let expected_proof_root = merkle_proof::<Keccak256, _, _>(vec![ab], 0);
		let actual_proof = query_data_proof(client.rpc(), 1, block_hash).await.unwrap();
		assert_eq!(actual_proof.root, expected_proof_root.root);
	}
}

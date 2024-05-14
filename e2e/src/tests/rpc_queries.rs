use super::{alice_nonce, local_connection, no_concurrency};

use avail_core::{AppExtrinsic, AppId, Keccak256};
use avail_subxt::{
	api::{
		self,
		runtime_types::avail_core::{
			header::extension::HeaderExtension, BlockLengthColumns, BlockLengthRows,
		},
	},
	avail::{Cells, GDataProof, GRawScalar, Rows},
	rpc::{GProof, KateRpcClient as _},
	submit::submit_data_with_nonce as submit_data,
	tx,
	utils::H256,
	AvailClient, Cell,
};
use kate::{
	com::Cell as KateCell,
	gridgen::{AsBytes as _, EvaluationGrid},
};
use kate_recovery::{matrix::Dimensions, proof::verify};
use subxt_signer::sr25519::dev;

use anyhow::{anyhow, Result};
use binary_merkle_tree::merkle_proof;
use codec::Encode;
use sp_core::keccak_256;
use std::{num::NonZeroU16, sync::atomic::Ordering::Relaxed};
use test_log::test;
use tracing::trace;

pub const MIN_WIDTH: usize = 4;
pub const DATA: &[u8] = b"ExampleData";

async fn eval_grid_from_block(client: &AvailClient, block_hash: H256) -> Result<EvaluationGrid> {
	let block = client.blocks().at(block_hash).await?;
	let extrinsics = block.extrinsics().await?;

	let mut app_extrinsics = Vec::new();
	for (i, e) in extrinsics.iter().enumerate() {
		let e = e.unwrap();
		if i == 0 || i == (extrinsics.len() - 1) {
			continue;
		}
		let raw_extrinsic = e.bytes().encode();
		let app_extrinsic = AppExtrinsic {
			app_id: AppId(1),
			data: raw_extrinsic,
		};
		app_extrinsics.push(app_extrinsic);
	}

	let block_len = client.rpc_methods().query_block_length(block_hash).await?;
	let max_width = block_len.cols.0 as usize;
	let max_height = block_len.rows.0 as usize;
	let seed = [0u8; 32];

	EvaluationGrid::from_extrinsics(app_extrinsics, MIN_WIDTH, max_width, max_height, seed)
		.map_err(|e| anyhow!("Eval grid failed {e:?}"))
}

#[test(tokio::test)]
pub async fn rpc_query_proof_test() -> Result<()> {
	let _cg = no_concurrency("rpc_queries::proof_test").await;
	let client = local_connection().await?;
	let alice = dev::alice();
	let app_id = AppId(1);

	let nonce = alice_nonce().await.fetch_add(1, Relaxed);
	let block_hash = tx::in_finalized(submit_data(&client, &alice, DATA, app_id, nonce).await?)
		.await?
		.block_hash();

	// Grid Creation
	let grid = eval_grid_from_block(&client, block_hash).await?;
	let extended_grid = grid.extend_columns(NonZeroU16::new(2).unwrap()).unwrap();
	let poly_grid = extended_grid.make_polynomial_grid().unwrap();

	assert_eq!(grid.dims(), Dimensions::new(1, 4).unwrap());
	assert_eq!(extended_grid.dims(), Dimensions::new(2, 4).unwrap());
	assert_eq!(grid.row(0), extended_grid.row(0));
	assert_eq!(grid.row(0), extended_grid.row(1));

	let cells = (0..grid.dims().cols().into())
		.map(|col| Cell::new(0, col.into()))
		.collect::<Vec<_>>();
	let multiproof_srs = kate::couscous::multiproof_params();
	let expected_proofs: Vec<GDataProof> = cells
		.iter()
		.map(|c| -> GDataProof {
			let data: GRawScalar = grid
				.get(c.row as usize, c.col as usize)
				.unwrap()
				.to_bytes()
				.map(GRawScalar::from)
				.unwrap();

			let cell = KateCell {
				row: avail_core::BlockLengthRows(c.row),
				col: avail_core::BlockLengthColumns(c.col),
			};
			let proof = poly_grid
				.proof(&multiproof_srs, &cell)
				.unwrap()
				.to_bytes()
				.map(GProof)
				.unwrap();

			(data, proof)
		})
		.collect();

	// RPC call
	let actual_proofs: Vec<GDataProof> = client
		.rpc_methods()
		.query_proof(Cells::try_from(cells).unwrap(), block_hash)
		.await?;

	let len = actual_proofs.len();
	assert_eq!(actual_proofs.len(), expected_proofs.len());
	for i in 0..len {
		assert_eq!(actual_proofs[i].0, expected_proofs[i].0);
		assert_eq!(actual_proofs[i].1 .0, expected_proofs[i].1 .0);
	}

	Ok(())
}

#[test(tokio::test)]
pub async fn rpc_query_proof_test_2() -> Result<()> {
	let _cg = no_concurrency("rpc_queries::proof_test_2").await;
	let client = local_connection().await?;
	let alice = dev::alice();

	let mut example_data = [0u8; 12_500];
	example_data[..7].copy_from_slice(b"example");
	assert_eq!(example_data.len(), 12_500);

	let nonce = alice_nonce().await.fetch_add(1, Relaxed);
	let block_hash =
		tx::in_finalized(submit_data(&client, &alice, &example_data, AppId(1), nonce).await?)
			.await?
			.block_hash();

	let cell = Cell { row: 0, col: 0 };
	let cells = Cells::try_from(vec![cell.clone()]).unwrap();

	// RPC call
	let actual_proof: Vec<GDataProof> = client.rpc_methods().query_proof(cells, block_hash).await?;
	let actual_proof: Vec<u8> = actual_proof
		.iter()
		.flat_map(|(raw_scalar, g_proof)| {
			let mut scalar_bytes = [0u8; 32];
			raw_scalar.to_big_endian(&mut scalar_bytes);
			let proof_bytes: Vec<u8> = Vec::from(*g_proof);

			[proof_bytes, scalar_bytes.to_vec()]
				.into_iter()
				.flatten()
				.collect::<Vec<u8>>()
		})
		.collect();
	assert_eq!(actual_proof.len(), 80);

	let pp = kate::couscous::public_params();

	let submitted_block = client.blocks().at(block_hash).await?;

	let (commitment, rows, cols) = match &submitted_block.header().extension {
		HeaderExtension::V3(ext) => (
			ext.commitment.commitment.clone(),
			ext.commitment.rows,
			ext.commitment.cols,
		),
	};

	let mut content = [0u8; 80];
	content.copy_from_slice(&actual_proof);
	let commitment: [u8; 48] = commitment[..48].try_into().unwrap();
	let dcell = kate_recovery::data::Cell {
		position: kate_recovery::matrix::Position { row: 0, col: 0 },
		content,
	};
	let dim = Dimensions::new(rows, cols).unwrap();
	let res = verify(&pp, dim, &commitment, &dcell).unwrap();
	assert!(res);

	// Fetch & verify extended cell
	let cell = Cell { row: 1, col: 1 };
	let cells = Cells::try_from(vec![cell.clone()]).unwrap();

	// RPC call
	let actual_proof: Vec<GDataProof> = client.rpc_methods().query_proof(cells, block_hash).await?;
	let actual_proof: Vec<u8> = actual_proof
		.iter()
		.flat_map(|(raw_scalar, g_proof)| {
			let mut scalar_bytes = [0u8; 32];
			raw_scalar.to_big_endian(&mut scalar_bytes);
			let proof_bytes: Vec<u8> = Vec::from(*g_proof);

			[proof_bytes, scalar_bytes.to_vec()]
				.into_iter()
				.flatten()
				.collect::<Vec<u8>>()
		})
		.collect();
	assert_eq!(actual_proof.len(), 80);

	let (commitment, rows, cols) = match &submitted_block.header().extension {
		HeaderExtension::V3(ext) => (
			ext.commitment.commitment.clone(),
			ext.commitment.rows,
			ext.commitment.cols,
		),
	};

	let mut content = [0u8; 80];
	content.copy_from_slice(&actual_proof);
	let commitment: [u8; 48] = commitment[48..96].try_into().unwrap();
	let dcell = kate_recovery::data::Cell {
		position: kate_recovery::matrix::Position { row: 1, col: 1 },
		content,
	};
	let dim = Dimensions::new(rows, cols).unwrap();
	let res = verify(&pp, dim, &commitment, &dcell).unwrap();
	assert!(res);
	Ok(())
}

#[test(tokio::test)]
pub async fn empty_commitments_test() -> Result<()> {
	let _cg = no_concurrency("rpc_queries::commitments_test").await;
	let client = local_connection().await?;
	let alice = dev::alice();

	// other than DA tx
	let call = api::tx().system().remark(b"Hi".to_vec());
	let nonce = alice_nonce().await.fetch_add(1, Relaxed);
	let tx = tx::send_with_nonce(&client, &call, &alice, AppId(0), nonce).await?;
	let block_hash = tx::in_finalized(tx).await?.block_hash();

	// query_rows should fail for block with empty commitments
	let row_indexes = Rows::truncate_from(vec![0]);
	let rows = client
		.rpc_methods()
		.query_rows(row_indexes, block_hash)
		.await;
	assert!(rows.is_err());

	// query_proof should fail for block with empty commitments
	let cell = Cell { row: 0, col: 0 };
	let cells = Cells::try_from(vec![cell.clone()]).unwrap();

	let proof = client.rpc_methods().query_proof(cells, block_hash).await;
	assert!(proof.is_err());
	Ok(())
}

#[test(tokio::test)]
pub async fn rpc_query_block_length_test() -> Result<()> {
	let _cg = no_concurrency("rpc_queries::block_lenght_test").await;
	let client = local_connection().await?;
	let alice = dev::alice();

	trace!("Data submitted...");
	let nonce = alice_nonce().await.fetch_add(1, Relaxed);
	let block_hash = tx::in_finalized(submit_data(&client, &alice, DATA, AppId(1), nonce).await?)
		.await?
		.block_hash();

	// RPC call
	let length = client.rpc_methods().query_block_length(block_hash).await?;
	assert_eq!(length.cols, BlockLengthColumns(256));
	assert_eq!(length.rows, BlockLengthRows(256));
	assert_eq!(length.chunk_size, 32);
	Ok(())
}

#[test(tokio::test)]
pub async fn rpc_query_data_proof_test() -> Result<()> {
	let _cg = no_concurrency("rpc_queries::data_proof_test").await;
	let client = local_connection().await?;
	let alice = dev::alice();

	// data hash: 729afe29f4e9fee2624d7ed311bcf57d24683fb78938bcb4e2a6a22c4968795e
	trace!("Data submitted...");
	let nonce = alice_nonce().await.fetch_add(1, Relaxed);
	let block_hash = tx::in_finalized(submit_data(&client, &alice, DATA, AppId(1), nonce).await?)
		.await?
		.block_hash();

	let expected_proof_root = merkle_proof::<Keccak256, _, _>(vec![keccak_256(DATA)], 0);

	let actual_proof = client.rpc_methods().query_data_proof(1, block_hash).await?;
	// root is calculated keccak256(blob_root, bridge_root)
	let mut root_data = vec![];
	root_data.extend(expected_proof_root.root.as_bytes());
	root_data.extend(H256::zero().as_bytes());
	let expected_data_root = keccak_256(root_data.as_slice());

	assert_eq!(
		actual_proof.data_proof.roots.data_root,
		H256(expected_data_root)
	);
	assert_eq!(actual_proof.data_proof.proof, expected_proof_root.proof);
	assert_eq!(
		actual_proof.data_proof.roots.blob_root,
		expected_proof_root.root
	);
	assert_eq!(actual_proof.data_proof.roots.bridge_root, H256::zero());
	Ok(())
}

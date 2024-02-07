#![warn(unused_extern_crates)]

fn main() {}

#[cfg(test)]
pub mod tests {
	use std::num::NonZeroU16;
	use std::str::FromStr;
	use std::sync::{Arc, Mutex};

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
	use kate::gridgen::{AsBytes, EvaluationGrid, PolynomialGrid};
	use kate::pmp::m1_blst::M1NoPrecomp;
	use kate_recovery::matrix::{Dimensions, Position};
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
			ws: String::from("wss://san1ty.avail.tools:443/ws"),
			validate_codegen: false,
		};
		let (client, _rpc) = build_client(args.ws, args.validate_codegen).await?;
		Ok(client)
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

	pub fn generate_proof(
		cells: &Vec<Cell>,
		extended_grid: &EvaluationGrid,
		poly_grid: &PolynomialGrid,
		multiproof_srs: &M1NoPrecomp,
	) -> Vec<u8> {
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

		expected_proof.into_iter().flatten().collect()
	}

	#[async_std::test]
	pub async fn rpc_query_proof_test() {
		let client = Arc::new(establish_a_connection().await.unwrap());

		let block_hash =
			H256::from_str("0x0eb7a9ba86d2f9c2ded9d24a28b75ee6b609f931fe5f72346dad1e06d04ba724")
				.unwrap();
		let submitted_block = get_submitted_block(client.rpc(), block_hash.clone())
			.await
			.unwrap();

		let header = client
			.rpc()
			.header(Some(block_hash))
			.await
			.unwrap()
			.unwrap();
		let app_extrinsics = get_block_app_extrinsics(&submitted_block).unwrap();

		// Grid Creation
		let grid = EvaluationGrid::from_extrinsics(app_extrinsics, 4, 256, 256, [0u8; 32]).unwrap();
		let extended_grid = grid.extend_columns(NonZeroU16::new(2).unwrap()).unwrap();
		let poly_grid = extended_grid.make_polynomial_grid().unwrap();

		assert_eq!(grid.dims(), Dimensions::new(128, 256).unwrap());
		assert_eq!(extended_grid.dims(), Dimensions::new(256, 256).unwrap());

		let multiproof_srs = kate::couscous::multiproof_params();

		const STEP: usize = 1;
		const MAX_ROWS: u32 = 1;
		const MAX_COLS: u32 = 1;
		let mut all_cells: Vec<Vec<Cell>> = Vec::new();
		for i in 0..MAX_ROWS {
			let mut cells = Vec::new();
			for j in 0..MAX_COLS {
				cells.push(Cell::new(BlockLengthRows(i), BlockLengthColumns(j)));
				if cells.len() == STEP {
					all_cells.push(cells);
					cells = Vec::new();
				}
			}
			if !cells.is_empty() {
				all_cells.push(cells);
			}
		}

		// Get all Proofs
		let tasks = all_cells.iter().map(|c| {
			let data = Arc::new((client.clone(), c.clone(), block_hash.clone()));
			async_std::task::spawn(async {
				let data = data;
				query_proof(data.0.rpc(), data.1.clone(), data.2)
					.await
					.unwrap()
			})
		});

		let mut proofs = Vec::new();
		let len = tasks.len();
		for (i, t) in tasks.enumerate() {
			println!("Waiting for Task: {}/{}", i + 1, len);
			proofs.push(t.await);
		}

		let (header_rows, header_cols, header_commitment, header_data_root) = {
			match header.extension {
				HeaderExtension::V2(a) => (
					a.commitment.rows,
					a.commitment.cols,
					a.commitment.commitment,
					a.commitment.data_root,
				),
				_ => panic!("Should Not Happen"),
			}
		};

		let header_commitments =
			kate_recovery::commitments::from_slice(&header_commitment).unwrap();

		pub struct ConstantData {
			extended_grid: EvaluationGrid,
			poly_grid: PolynomialGrid,
			multiproof_srs: M1NoPrecomp,
			header_commitments: Vec<[u8; 48]>,
		}

		pub struct MutableData {
			cells: Vec<Vec<Cell>>,
			proofs: Vec<Vec<u8>>,
		}

		let constant_data = Arc::new(ConstantData {
			extended_grid,
			poly_grid,
			multiproof_srs,
			header_commitments,
		});
		let mutable_data = Arc::new(Mutex::new(MutableData {
			cells: all_cells.clone(),
			proofs: proofs.clone(),
		}));
		// Check proofs in a multithreaded way.
		let threads: Vec<_> = (0..1)
			.into_iter()
			.map(|_| {
				let c = constant_data.clone();
				let v = mutable_data.clone();
				std::thread::spawn(move || loop {
					let (cells, actual_proof, i) = {
						let mut lock = v.lock().unwrap();
						if lock.cells.is_empty() {
							return;
						}

						let i = lock.cells.len();
						let cells = lock.cells.pop().unwrap();
						let actual_proof = lock.proofs.pop().unwrap();
						(cells, actual_proof, i)
					};

					println!("Checking Proof: {}/{}", i, len);
					let expected_proof =
						generate_proof(&cells, &c.extended_grid, &c.poly_grid, &c.multiproof_srs);
					assert_eq!(actual_proof, expected_proof);

					let p: [u8; 80] = actual_proof.try_into().unwrap();

					let commitments = c.poly_grid.commitments(&c.multiproof_srs).unwrap();
					let dim = Dimensions::new_from(256, 256).unwrap();
					let pp = kate::couscous::public_params();
					for cell in cells.iter() {
						let comm = commitments[cell.row.0 as usize].to_bytes().unwrap();
						let header_comm = c.header_commitments[cell.row.0 as usize];
						assert_eq!(comm, header_comm);

						let dcell = kate_recovery::data::Cell::new(
							Position {
								row: cell.row.0,
								col: cell.col.0 as u16,
							},
							p,
						);
						println!(
							"Cell: row: {}, col: {}, proof: {:?}, commitment: {:?}, DCell: {:?}",
							cell.row, cell.col, p, comm, dcell
						);

						// Verify Proof
						let verification = kate_recovery::proof::verify(&pp, dim, &comm, &dcell);
						assert_eq!(verification.is_ok(), true);
						verification.unwrap();
					}
				})
			})
			.collect();

		for t in threads {
			t.join().unwrap();
		}

		//dbg!(&actual_proof);
		//dbg!(&expected_proof);
	}

	/* 	#[async_std::test]
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
	} */
}

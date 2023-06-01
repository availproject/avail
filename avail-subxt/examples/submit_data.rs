use std::num::NonZeroUsize;

use anyhow::Result;
use avail_subxt::{
	api::{
		self,
		runtime_types::{
			da_control::pallet::Call as DaCall, da_primitives::header::extension::HeaderExtension,
			sp_core::bounded::bounded_vec::BoundedVec,
		},
	},
	avail::AppUncheckedExtrinsic,
	build_client,
	primitives::AvailExtrinsicParams,
	Call, Opts,
};
use kate::{
	grid::Dimensions as KDims,
	gridgen::ArkScalar,
	pmp::{
		merlin::Transcript,
		traits::{AsBytes, PolyMultiProofNoPrecomp},
	},
};
use kate_recovery::matrix::Dimensions;
use sp_keyring::AccountKeyring;
use structopt::StructOpt;
use subxt::{config::Header, rpc::RpcParams, tx::PairSigner};

/// This example submits an Avail data extrinsic, then retrieves the block containing the
/// extrinsic and matches the data.
#[async_std::main]
async fn main() -> Result<()> {
	let args = Opts::from_args();
	let client = build_client(args.ws, args.validate_codegen).await?;

	let signer = PairSigner::new(AccountKeyring::Alice.pair());
	let mut example_data = [0u8; 12_500];
	example_data[..7].copy_from_slice(b"example");
	let data_transfer = api::tx()
		.data_availability()
		.submit_data(BoundedVec(example_data.to_vec()));
	let extrinsic_params = AvailExtrinsicParams::new_with_app_id(1.into());

	println!("Sending example data...");
	let h = client
		.tx()
		.sign_and_submit_then_watch(&data_transfer, &signer, extrinsic_params)
		.await?
		.wait_for_finalized_success()
		.await?;

	let submitted_block = client.rpc().block(Some(h.block_hash())).await?.unwrap();

	let matched_xt = submitted_block
		.block
		.extrinsics
		.into_iter()
		.filter_map(|chain_block_ext| {
			AppUncheckedExtrinsic::try_from(chain_block_ext)
				.map(|ext| ext.function)
				.ok()
		})
		.find(|call| match call {
			Call::DataAvailability(da_call) => match da_call {
				DaCall::submit_data { data } => data.0 == example_data,
				_ => false,
			},
			_ => false,
		});

	assert!(matched_xt.is_some(), "Submitted data not found");

	// Grab and verify proof
	let mut params = RpcParams::new();
	let cell = kate::com::Cell {
		row: 0.into(),
		col: 0.into(),
	};
	params.push(vec![cell.clone()]).unwrap();
	params
		.push(Some(submitted_block.block.header.hash()))
		.unwrap();

	let res: [u8; 80] = client
		.rpc()
		.request::<Vec<u8>>("kate_queryProof", params)
		.await
		.unwrap()
		.try_into()
		.unwrap();

	let pp = kate::testnet::public_params(256.into());
	let ext = if let HeaderExtension::V1(ref ext) = submitted_block.block.header.extension {
        ext
    } else {
        panic!("Unsupported header extension version")
    };
	let commitment: [u8; 48] = ext.commitment.commitment[..48].try_into().unwrap();
	let dcell = kate_recovery::data::Cell {
		position: kate_recovery::matrix::Position { row: 0, col: 0 },
		content: res,
	};
	let res = kate_recovery::proof::verify(
		&pp,
		&Dimensions::new(ext.commitment.rows, ext.commitment.cols).unwrap(),
		&commitment,
		&dcell,
	)
	.unwrap();
	assert!(res);

	// ext.commitment contains un-extended dims, so we have to multiply rows by 2!
	let kdims = KDims::new(
		NonZeroUsize::new(ext.commitment.cols.into()).unwrap(),
		NonZeroUsize::new((2 * ext.commitment.rows).into()).unwrap(),
	);
	let target_dims = KDims::new_unchecked(16, 64);

	let mp_grid_dims = kate::gridgen::multiproof_dims(&kdims, &target_dims).unwrap();

	// Take every cell in `mp_grid_dims` for verification
	let cells = (0..mp_grid_dims.width() as u32)
		.flat_map(|col| {
			(0..mp_grid_dims.height() as u32).map(move |row| kate::com::Cell {
				row: row.into(),
				col: col.into(),
			})
		})
		.collect::<Vec<_>>();

	let mut params = RpcParams::new();
	params.push(cells.clone()).unwrap();
	params
		.push(Some(submitted_block.block.header.hash()))
		.unwrap();

	let res = client
		.rpc()
		.request::<Vec<MultiproofSer>>("kate_queryMultiProof", params)
		.await
		.unwrap();

	let commits = ext
		.commitment
		.commitment
		.chunks_exact(48)
		.map(|c| kate::pmp::Commitment::from_bytes(c.try_into().unwrap()).unwrap())
		.collect::<Vec<_>>();

	let pmp = kate::testnet::multiproof_params(256, 256);
	let points = kate::gridgen::domain_points(kdims.width())
		.unwrap()
		.into_iter()
		.collect::<Vec<_>>();

	for (mp, cell) in res.iter().zip(cells) {
		let mp_block = kate::gridgen::multiproof_block(
			cell.col.as_usize(),
			cell.row.as_usize(),
			&kdims,
			&KDims::new_unchecked(16, 64),
		)
		.unwrap();

		println!("Verifying multiproof of cells: {:?}", &mp_block);

		let evals: Vec<ArkScalar> = mp
			.evals
			.chunks_exact(32)
			.map(|c| {
				let mut arr = [0u8; 32];
				arr.copy_from_slice(c);
				kate::gridgen::ArkScalar::from_bytes(&arr).unwrap()
			})
			.collect::<Vec<_>>();

		let evals_grid = evals
			.chunks_exact(mp_block.end_x - mp_block.start_x)
			.collect::<Vec<_>>();

		let proof =
			kate::pmp::m1_blst::Proof::from_bytes(&mp.proof[..48].try_into().unwrap()).unwrap();

		let mut transcript = Transcript::new(b"avail-mp");
		assert!(pmp
			.verify(
				&mut transcript,
				&commits[mp_block.start_y..mp_block.end_y],
				&points[mp_block.start_x..mp_block.end_x],
				&evals_grid,
				&proof,
			)
			.unwrap());
	}

	Ok(())
}

use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct MultiproofSer {
	pub proof: Vec<u8>,
	pub evals: Vec<u8>,
}

use anyhow::Result;
use avail_subxt::{
	api::{
		self,
		runtime_types::{
			avail_core::header::extension::HeaderExtension,
			bounded_collections::bounded_vec::BoundedVec, da_control::pallet::Call as DaCall,
		},
	},
	avail::AppUncheckedExtrinsic,
	build_client,
	config::Header,
	primitives::AvailExtrinsicParams,
	rpc::RpcParams,
	Call, Opts,
};
use kate_recovery::matrix::Dimensions;
use sp_keyring::AccountKeyring;
use structopt::StructOpt;
use subxt::tx::PairSigner;

/// This example submits an Avail data extrinsic, then retrieves the block containing the
/// extrinsic and matches the data.
#[async_std::main]
async fn main() -> Result<()> {
	let args = Opts::from_args();
	let client = build_client(args.ws, args.validate_codegen).await?;

	let signer = PairSigner::new(AccountKeyring::Alice.pair());
	let mut example_data = [0u8; 12_500];
	example_data[..7].copy_from_slice(b"example");
	let example_data = example_data.to_vec();
	let data_transfer = api::tx()
		.data_availability()
		.submit_data(BoundedVec(example_data.clone()));
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
		row: avail_core::BlockLengthRows(0),
		col: avail_core::BlockLengthColumns(0),
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

	let pp = kate::testnet::public_params(avail_core::BlockLengthColumns(256));
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
		Dimensions::new(ext.commitment.rows, ext.commitment.cols).unwrap(),
		&commitment,
		&dcell,
	)
	.unwrap();
	assert!(res);

	Ok(())
}

use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct MultiproofSer {
	pub proof: Vec<u8>,
	pub evals: Vec<u8>,
}

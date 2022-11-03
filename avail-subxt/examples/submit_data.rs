use anyhow::Result;
use avail_subxt::{
	api::{self, runtime_types::frame_support::storage::bounded_vec::BoundedVec},
	build_client,
	primitives::AvailExtrinsicParams,
	Opts,
};
use sp_keyring::AccountKeyring;
use structopt::StructOpt;
use subxt::tx::PairSigner;

/// This example submits an Avail data extrinsic, then retrieves the block containing the
/// extrinsic and matches the data.
#[async_std::main]
async fn main() -> Result<()> {
	let args = Opts::from_args();
	let client = build_client(args.ws).await?;

	let signer = PairSigner::new(AccountKeyring::Alice.pair());
	let example_data = b"example".to_vec();
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

	let xts = submitted_block.block.extrinsics;
	println!("Submitted block extrinsic: {xts:?}");

	/*
	let matched_xt = xts.iter().find(move |e| match e {
		AvailExtrinsic::AvailDataExtrinsic {
			signature: _,
			data,
			address: _,
			extra_params,
		} => extra_params.app_id == 1 && data.eq(&example_data),
		_ => false,
	});

	assert!(matched_xt.is_some(), "Submitted data not found");
	*/

	Ok(())
}

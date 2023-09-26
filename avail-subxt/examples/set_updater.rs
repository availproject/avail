use anyhow::Result;
use avail_subxt::{api, build_client, primitives::AvailExtrinsicParams, Opts};
use structopt::StructOpt;
use subxt_signer::sr25519::dev;

/// This example submits an Avail data extrinsic, then retrieves the block containing the
/// extrinsic and matches the data.
#[async_std::main]
async fn main() -> Result<()> {
	let args = Opts::from_args();
	let client = build_client(args.ws, args.validate_codegen).await?;

	let new_updater = "0x14dC79964da2C08b23698B3D3cc7Ca32193d9955";
	let signer = dev::alice();

	// crate a new transaction with new updater
	let set_updater = api::tx()
		.nomad_home()
		.set_updater(new_updater.parse().unwrap());

	println!("Updating updater...");

	let params = AvailExtrinsicParams::default();
	client
		.tx()
		.sign_and_submit_then_watch(&set_updater, &signer, params)
		.await?
		.wait_for_finalized_success()
		.await?;

	println!("Updating done.");

	Ok(())
}

use anyhow::Result;
use avail_subxt::{
	api, api::runtime_types::bounded_collections::bounded_vec::BoundedVec, build_client,
	primitives::AvailExtrinsicParams, AvailConfig, Opts,
};
use structopt::StructOpt;
use subxt_signer::sr25519::dev;

#[async_std::main]
async fn main() -> Result<()> {
	let args = Opts::from_args();
	let client = build_client(args.ws, args.validate_codegen).await?;

	// Account
	let signer_a = dev::alice();

	let app_id = b"my_app_name".to_vec();
	let create_application_key = api::tx()
		.data_availability()
		.create_application_key(BoundedVec(app_id));

	let params = AvailExtrinsicParams::default();

	let res = client
		.tx()
		.sign_and_submit_then_watch(&create_application_key, &signer_a, params)
		.await?
		.wait_for_finalized_success()
		.await?;

	println!("Application key created, block hash: {}", res.block_hash());
	Ok(())
}

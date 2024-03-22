use avail_core::AppId;
use avail_subxt::{
	api, api::runtime_types::bounded_collections::bounded_vec::BoundedVec, tx, AvailClient, Opts,
};

use anyhow::Result;
use structopt::StructOpt;
use subxt_signer::sr25519::dev;

/// This example demonstrates creation of application key.
#[async_std::main]
async fn main() -> Result<()> {
	let args = Opts::from_args();
	let client = AvailClient::new(args.ws).await?;

	// Account
	let signer = dev::alice();

	let call = api::tx()
		.data_availability()
		.create_application_key(BoundedVec(b"my_app_name".to_vec()));

	let block_hash = tx::send_then_in_block(&client, &call, &signer, AppId(0))
		.await?
		.block_hash();

	println!("Application key created, block hash: {block_hash:?}");
	Ok(())
}

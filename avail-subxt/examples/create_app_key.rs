use anyhow::Result;
use avail_subxt::{
	api, api::runtime_types::bounded_collections::bounded_vec::BoundedVec, tx, AvailClient, Opts,
};
use structopt::StructOpt;
use subxt_signer::sr25519::dev;

/// This example demonstrates creation of application key.

#[async_std::main]
async fn main() -> Result<()> {
	let args = Opts::from_args();
	let client = AvailClient::new(args.ws).await?;

	// Account
	let alice = dev::alice();
	let alice_id = alice.public_key().into();
	let nonce = client.tx().account_nonce(&alice_id).await?;

	let call = api::tx()
		.data_availability()
		.create_application_key(BoundedVec(b"my_app_name".to_vec()));
	let progress = tx::send_with_nonce(&client, &call, &alice, 0, nonce).await?;

	let hash = tx::then_in_block(progress).await?.block_hash();

	println!("Application key created, block hash: {hash:?}");
	Ok(())
}

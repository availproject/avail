use super::local_connection;

use avail_core::AppId;
use avail_subxt::{api, api::runtime_types::bounded_collections::bounded_vec::BoundedVec, tx};

use anyhow::Result;
use subxt_signer::sr25519::dev;

/// This example demonstrates creation of application key.
#[async_std::main]
async fn main() -> Result<()> {
	let client = local_connection().await?;

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

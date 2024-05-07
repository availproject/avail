use super::{concurrent_controller, local_connection, ALICE_NONCE};

use avail_core::AppId;
use avail_subxt::{api, api::runtime_types::bounded_collections::bounded_vec::BoundedVec, tx};

use anyhow::Result;
use std::sync::atomic::Ordering::Relaxed;
use subxt_signer::sr25519::dev;

/// This example demonstrates creation of application key.
#[async_std::test]
async fn test() -> Result<()> {
	let _cg = concurrent_controller().allow_concurrency().await;
	let client = local_connection().await?;

	// Account
	let alice = dev::alice();
	let call = api::tx()
		.data_availability()
		.create_application_key(BoundedVec(b"my_app_name".to_vec()));

	let nonce = ALICE_NONCE.fetch_add(1, Relaxed);
	let tx_progress = tx::send_with_nonce(&client, &call, &alice, AppId(0), nonce).await?;
	let block_hash = tx::then_in_block(tx_progress).await?.block_hash();

	println!("Application key created, block hash: {block_hash:?}");
	Ok(())
}

use super::{alice_nonce, allow_concurrency, local_connection};

use avail_core::AppId;
use avail_subxt::{api, api::runtime_types::bounded_collections::bounded_vec::BoundedVec, tx};
use subxt_signer::sr25519::dev;

use anyhow::Result;
use std::sync::atomic::Ordering::Relaxed;
use test_log::test;
use tracing::trace;

/// This example demonstrates creation of application key.
#[test(tokio::test)]
async fn test() -> Result<()> {
	let _cg = allow_concurrency("create_app_key").await;
	let client = local_connection().await?;

	// Account
	let alice = dev::alice();
	let call = api::tx()
		.data_availability()
		.create_application_key(BoundedVec(b"my_app_name".to_vec()));

	let nonce = alice_nonce().await.fetch_add(1, Relaxed);
	let tx_progress = tx::send_with_nonce(&client, &call, &alice, AppId(0), nonce).await?;
	let block_hash = tx::in_finalized(tx_progress).await?.block_hash();

	trace!("Application key created, block hash: {block_hash:?}");
	Ok(())
}

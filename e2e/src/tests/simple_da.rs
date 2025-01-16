use super::{alice_nonce, allow_concurrency, local_connection};

use avail_core::AppId;
use avail_subxt::{api, api::runtime_types::bounded_collections::bounded_vec::BoundedVec, submit::submit_data_with_commitments,tx};
use subxt_signer::sr25519::dev;

use anyhow::Result;
use test_log::test;
use tracing::trace;

/// This example demonstrates submitting the data with commitments
#[test(tokio::test)]
async fn test() -> Result<()> {
	let _cg = allow_concurrency("simple_da").await;
	let client = local_connection().await?;

	// Account
	let alice = dev::alice();

	let data = b"some_data_for_testing".to_vec();
	let da_commitments = vec![0u8; 48];
	let tx_progress = submit_data_with_commitments(
		&client,
		&alice,
		data.as_slice(),
		AppId(1),
		&da_commitments,
	).await?;
	let block_hash = tx::in_finalized(tx_progress).await?.block_hash();

	trace!("DataSubmitted, block hash: {block_hash:?}");
	Ok(())
}

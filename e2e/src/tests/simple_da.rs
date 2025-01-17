use super::{alice_nonce, allow_concurrency, local_connection};

use avail_core::AppId;
use avail_subxt::{submit::submit_data_with_commitments,tx};
use subxt_signer::sr25519::dev;

use anyhow::Result;
use test_log::test;
use tracing::trace;
use hex_literal::hex;

/// This example demonstrates submitting the data with commitments
#[test(tokio::test)]
async fn test() -> Result<()> {
	let _cg = allow_concurrency("simple_da").await;
	let client = local_connection().await?;

	// Account
	let alice = dev::alice();

	let data = b"test".to_vec();
	let da_commitments =  hex!("884aa48a15ed57c2632f0c666490975c480bb8a0fc7b6b30ed68d5321e1c8855401730fa8f7d22648361124b56b6a708884aa48a15ed57c2632f0c666490975c480bb8a0fc7b6b30ed68d5321e1c8855401730fa8f7d22648361124b56b6a708");
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

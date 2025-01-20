use super::{allow_concurrency, local_connection};

use avail_core::AppId;
use avail_subxt::{submit::submit_data_with_commitments, tx};
use subxt_signer::sr25519::dev;

use super::build_da_commitments;
use anyhow::Result;
use kate::Seed;
use test_log::test;
use tracing::trace;

/// This example demonstrates submitting the data with commitments
#[test(tokio::test)]
async fn test() -> Result<()> {
	let _cg = allow_concurrency("simple_da").await;
	let client = local_connection().await?;

	// Account
	let alice = dev::alice();

	let data = b"test".to_vec();
	let da_commitments =
		build_da_commitments::build_da_commitments(data.clone(), 256, 256, Seed::default())
			.unwrap();

	let flattened_commitments: Vec<u8> = da_commitments.iter().flat_map(|c| c.to_vec()).collect();

	let tx_progress = submit_data_with_commitments(
		&client,
		&alice,
		&*data.as_slice(),
		AppId(1),
		&flattened_commitments,
	)
	.await?;
	let block_hash = tx::in_finalized(tx_progress).await?.block_hash();

	trace!("DataSubmitted, block hash: {block_hash:?}");
	Ok(())
}

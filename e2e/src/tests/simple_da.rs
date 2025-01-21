use super::{allow_concurrency, local_connection};

use avail_core::AppId;
use avail_subxt::{submit::submit_data_with_commitments, tx};
use subxt_signer::sr25519::dev;

use super::build_da_commitments;
use anyhow::Result;
use kate::Seed;
use test_log::test;
use tracing::trace;

const TX_MAX_SIZE: usize = 4 * 1024 * 1024;

/// This example demonstrates submitting the data with commitments
#[test(tokio::test)]
async fn test() -> Result<()> {
	let _cg = allow_concurrency("simple_da").await;
	let client = local_connection().await?;

	// Account
	let alice = dev::alice();

	let data = vec![200; TX_MAX_SIZE];
	let da_commitments =
		build_da_commitments::build_da_commitments(data.clone(), 2048, 2048, Seed::default())
			.unwrap();

	println!(
		"Data size: {}, DA Commitments size: {}",
		data.len(),
		da_commitments.len()
	);

	let tx_progress =
		submit_data_with_commitments(&client, &alice, &*data.as_slice(), AppId(1), da_commitments)
			.await?;
	let block_hash = tx::in_finalized(tx_progress).await?.block_hash();

	trace!("DataSubmitted, block hash: {block_hash:?}");
	Ok(())
}

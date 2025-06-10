use super::{alice_nonce, allow_concurrency, local_connection};

use avail_core::AppId;
use avail_subxt::{api, api::runtime_types::bounded_collections::bounded_vec::BoundedVec, tx};
use subxt_signer::sr25519::dev;

use super::build_da_commitments;
use anyhow::Result;
use kate::Seed;
use std::sync::atomic::Ordering::Relaxed;
use test_log::test;
use tracing::trace;

const TX_MAX_SIZE: usize = 4 * 1024 * 1024;

/// This example demonstrates submitting the data with commitments
#[test(tokio::test)]
async fn test() -> Result<()> {
	let _cg = allow_concurrency("simple_da").await;
	let client = local_connection().await?;

	let data = b"test".to_vec();
	// let data = vec![200; TX_MAX_SIZE];
	let da_commitments =
		build_da_commitments::build_da_commitments(data.clone(), 1024, 1024, Seed::default())
			.unwrap();

	println!(
		"Data size: {}, DA Commitments size: {}",
		data.len(),
		da_commitments.len()
	);

	let call = api::tx()
		.data_availability()
		.submit_data_with_commitments(BoundedVec(data), BoundedVec(da_commitments));

	let alice = dev::alice();
	let nonce = alice_nonce().await.fetch_add(1, Relaxed);
	let tx_progress = tx::send_with_nonce(&client, &call, &alice, AppId(1), nonce).await?;
	let block_hash = tx::in_finalized(tx_progress).await?.block_hash();

	trace!("DataSubmitted, block hash: {block_hash:?}");
	Ok(())
}

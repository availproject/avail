use super::{alice_nonce, allow_concurrency, local_connection};

use avail_core::{AppExtrinsic, AppId, BlockLengthColumns, BlockLengthRows};
use avail_subxt::{submit::submit_data_with_commitments,tx};
use subxt_signer::sr25519::dev;

use anyhow::Result;
use test_log::test;
use tracing::trace;
use hex_literal::hex;
use kate::{com::par_build_commitments_v2, metrics::IgnoreMetrics, Seed};

/// This example demonstrates submitting the data with commitments
#[test(tokio::test)]
async fn test() -> Result<()> {
	let _cg = allow_concurrency("simple_da").await;
	let client = local_connection().await?;

	// Account
	let alice = dev::alice();

	let data = b"test".to_vec();
	let ext = AppExtrinsic::from(data.to_vec());
	let metrics = IgnoreMetrics {};
	let (_, da_commitments, _, _) = par_build_commitments_v2::<32, _>(BlockLengthRows(256), BlockLengthColumns(256), &[ext], Seed::default(), &metrics).unwrap();
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

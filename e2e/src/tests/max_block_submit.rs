use super::{alice_nonce, local_connection, no_concurrency};

use avail_core::AppId;
use avail_subxt::{api, tx, BoundedVec};
use kate::Seed;
use subxt_signer::sr25519::dev;

use super::build_da_commitments::build_da_commitments;
use futures::stream::{FuturesOrdered, TryStreamExt as _};
use std::sync::atomic::Ordering::Relaxed;
use test_log::test;
use tracing::trace;

const BLOCK_SIZE: usize = 128 * 1024 * 1024;
const TX_MAX_SIZE: usize = 4 * 1024 * 1024;
const NUM_CHUNKS: u64 = (BLOCK_SIZE / TX_MAX_SIZE) as u64;

#[test(tokio::test)]
async fn max_block_submit() -> anyhow::Result<()> {
	let _cg = no_concurrency("max_block_submit").await;
	let client = local_connection().await?;

	let alice = dev::alice();
	let nonce = alice_nonce().await.fetch_add(NUM_CHUNKS, Relaxed);

	let start = std::time::Instant::now();
	let call = {
		let data = BoundedVec(vec![200; TX_MAX_SIZE]);
			let da_commitments =
				build_da_commitments(data.0.clone(), 2048, 2048, Seed::default()).unwrap();
			(
				api::tx().data_availability().submit_data(data),
				da_commitments,
			)
	};
	let txs = (0..NUM_CHUNKS).enumerate().map(|(_, i)| {
		tx::send_with_da_commits_and_nonce(
			&client,
			&call.0,
			&alice,
			AppId(1),
			call.1.to_vec(),
			nonce + i,
		)
	})
	.collect::<FuturesOrdered<_>>()
	.try_collect::<Vec<_>>()
	.await?;

	let submittions_end = start.elapsed();
	trace!("Submittions done in {submittions_end:?}");

	let in_block_txs = txs
		.into_iter()
		.map(tx::then_in_block)
		.collect::<FuturesOrdered<_>>()
		.try_collect::<Vec<_>>()
		.await?;
	for (i, in_block) in in_block_txs.into_iter().enumerate() {
		let hash = in_block.block_hash();
		trace!("Finalized {i} in block: {hash:?}");
	}
	let end = start.elapsed();
	trace!("Finalized in {end:?}");

	Ok(())
}

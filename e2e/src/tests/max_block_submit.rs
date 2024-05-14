use super::{alice_nonce, local_connection, no_concurrency};

use avail_core::AppId;
use avail_subxt::{api, tx, BoundedVec};
use subxt_signer::sr25519::dev;

use futures::stream::{FuturesOrdered, TryStreamExt as _};
use std::sync::atomic::Ordering::Relaxed;
use test_log::test;
use tracing::trace;

/// This example attempts to submit data to fill the entire block. Note that this doesn't guarantee
/// that the block will be filled, but if you submit more than a full block, then it will spill over
/// to the next block. The limit for the transaction is currently set to 512 kB, and limit for the block
/// is 2 MB, so this means 128 data transactions are needed to fill the block. Depending on the network,
/// it may not be possible to transfer so many in 20 s (the default block time)
const BLOCK_SIZE: usize = 2 * 1024 * 1024;
const TX_MAX_SIZE: usize = 512 * 1024;
const NUM_CHUNKS: u64 = (BLOCK_SIZE / TX_MAX_SIZE) as u64;

#[test(tokio::test)]
async fn max_block_submit() -> anyhow::Result<()> {
	let _cg = no_concurrency("max_block_submit").await;
	let client = local_connection().await?;

	let alice = dev::alice();
	let nonce = alice_nonce().await.fetch_add(NUM_CHUNKS, Relaxed);

	let start = std::time::Instant::now();
	let calls = (0..NUM_CHUNKS)
		.map(|i| {
			let data = BoundedVec(vec![(i & 255) as u8; TX_MAX_SIZE]);
			api::tx().data_availability().submit_data(data)
		})
		.collect::<Vec<_>>();
	let txs = calls
		.iter()
		.enumerate()
		.map(|(idx, call)| tx::send_with_nonce(&client, call, &alice, AppId(1), nonce + idx as u64))
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

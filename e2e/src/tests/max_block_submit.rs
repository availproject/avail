use super::{local_connection, no_concurrency};

use avail_core::AppId;
use avail_subxt::{api, tx, BoundedVec};
use kate::Seed;
use subxt_signer::sr25519::dev;

use super::build_da_commitments::build_da_commitments;

use futures::stream::{FuturesOrdered, TryStreamExt as _};
use test_log::test;
use tracing::trace;
use codec::{Compact, CompactLen as _};

const BLOCK_SIZE: u32 = 48 * 1024 * 1024;
const TX_MAX_SIZE: u32 = 31 * 1024 * 512;
const NUM_CHUNKS: u32 = BLOCK_SIZE / TX_MAX_SIZE;

#[test(tokio::test)]
async fn max_block_submit() -> anyhow::Result<()> {
	let _cg = no_concurrency("max_block_submit").await;
	let client = local_connection().await?;

	let sender = dev::alice();
	let nonce = avail_subxt::tx::nonce(&client, &sender).await.unwrap();

	let encoding_overhead = compact_len(&TX_MAX_SIZE).unwrap();
	let tx_size = TX_MAX_SIZE.saturating_sub(encoding_overhead) as usize;
	let start = std::time::Instant::now();
	let data = vec![200; tx_size];
	let da_commitments = build_da_commitments(data.clone(), 1024, 1024, Seed::default()).unwrap();

	let calls = (0..NUM_CHUNKS)
		.map(|_| {
			api::tx().data_availability().submit_data_with_commitments(
				BoundedVec(data.clone()),
				BoundedVec(da_commitments.clone()),
			)
		})
		.collect::<Vec<_>>();
	let txs = calls
		.iter()
		.enumerate()
		.map(|(idx, call)| {
			tx::send_with_nonce(&client, call, &sender, AppId(1), nonce + idx as u64)
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

fn compact_len(value: &u32) -> Option<u32> {
	let len = Compact::<u32>::compact_len(value);
	len.try_into().ok()
}

use super::{allow_concurrency, local_connection};
use avail_subxt::config::Header;
use sp_core::H256;

use anyhow::Result;
use core::mem::swap;
use test_log::test;
use tokio::time::{sleep, Duration};
use tracing::trace;

/// This example gets all the headers from testnet. It requests them in concurrently in batches of BATCH_NUM.
/// Fetching headers one by one is too slow for a large number of blocks.

#[test(tokio::test)]
async fn headers() -> Result<()> {
	let _cg = allow_concurrency("headers").await;
	let client = local_connection().await?;

	let genesis_hash = client.genesis_hash();

	// NOTE: Block cannot be the genesis block
	let mut block = None;
	for i in 0..6 {
		let try_block = client.blocks().at_latest().await?;
		if try_block.header().hash() != genesis_hash {
			block = Some(try_block);
			break;
		}
		trace!("Waiting first block, {i} of 6 ...");
		sleep(Duration::from_secs(10)).await;
	}

	let mut block = block.expect("Only genesis block found");
	let hash = block.header().hash();
	trace!("Current hash block: {hash:?} and genesis: {genesis_hash:?}");

	let mut headers = vec![hash];

	for _ in 1u32..=50_000 {
		let parent: H256 = block.header().parent_hash;
		headers.push(parent);
		if parent == genesis_hash {
			break;
		}
		let mut parent_block = client.blocks().at(parent).await?;
		swap(&mut block, &mut parent_block);
	}
	trace!("Headers: {}", headers.len());
	trace!("Header hashes: {headers:?}");

	Ok(())
}

use super::local_connection;
use avail_subxt::config::Header;
use sp_core::H256;

use anyhow::Result;
use core::mem::swap;

/// This example gets all the headers from testnet. It requests them in concurrently in batches of BATCH_NUM.
/// Fetching headers one by one is too slow for a large number of blocks.

#[async_std::test]
async fn headers() -> Result<()> {
	let client = local_connection().await?;

	let genesis_hash = client.genesis_hash();
	let mut block = client.blocks().at_latest().await?;

	let hash = block.header().hash();
	println!("Current hash block: {hash:?} and genesis: {genesis_hash:?}");

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
	println!("Headers: {}", headers.len());
	println!("Header hashes: {headers:?}");

	Ok(())
}

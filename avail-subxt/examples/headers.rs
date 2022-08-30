use avail_subxt::AvailConfig;
use futures::{future::join_all, TryFutureExt};
use subxt::{rpc::BlockNumber, OnlineClient};

/// This example gets all the headers from testnet. It requests them in concurrently in batches of BATCH_NUM.
/// Fetching headers one by one is too slow for a large number of blocks.

const BATCH_NUM: usize = 1000;
#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	let api = OnlineClient::<AvailConfig>::new().await?;

	let head_block = api.rpc().block(None).await.unwrap().unwrap();

	let block_num = head_block.block.header.number;
	println!("Current head: {block_num}");
	println!(
		"Current head block extrinsic: {block:?}",
		block = head_block.block.extrinsics
	);
	println!(
		"Current head block header: {header:?}",
		header = head_block.block.header
	);

	let mut headers = vec![];

	for batch in (1u32..=block_num)
		.collect::<Vec<_>>()
		.chunks(BATCH_NUM)
		.map(|e| {
			join_all(
				e.iter()
					.map(|n| {
						api.rpc()
							.block_hash(Some(BlockNumber::from(*n)))
							.and_then(|h| api.rpc().header(h))
					})
					.collect::<Vec<_>>(),
			)
		}) {
		headers.extend(batch.await);
	}
	println!("Headers: {num}", num = headers.len());

	assert_eq!(
		headers.len(),
		block_num as usize,
		"Didn't get the same number of block headers."
	);

	Ok(())
}

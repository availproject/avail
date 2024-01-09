use anyhow::Result;
use avail_subxt::{build_client, Opts};
use futures::future::{join_all, TryFutureExt};
use structopt::StructOpt;
use subxt::{config::Header as XtHeader, rpc::types::BlockNumber};

/// This example gets all the headers from testnet. It requests them in concurrently in batches of BATCH_NUM.
/// Fetching headers one by one is too slow for a large number of blocks.

const BATCH_NUM: usize = 1000;
#[async_std::main]
async fn main() -> Result<()> {
	let args = Opts::from_args();
	let (client, _) = build_client(args.ws, args.validate_codegen).await?;

	let head_block = client
		.rpc()
		.block(None)
		.await?
		.expect("Best block always exists .qed");

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
						let block_number = Some(BlockNumber::from(*n));
						client
							.rpc()
							.block_hash(block_number)
							.and_then(|h| client.rpc().header(h))
					})
					.collect::<Vec<_>>(),
			)
		}) {
		headers.extend(batch.await);
	}
	println!("Headers: {num}", num = headers.len());
	let header_hashes = headers
		.iter()
		.map(|result_maybe_header| {
			result_maybe_header
				.as_ref()
				.map(|maybe_header| maybe_header.as_ref().map(XtHeader::hash))
				.unwrap_or_default()
		})
		.collect::<Vec<_>>();
	println!("Header hashes: {:?}", header_hashes);

	assert_eq!(
		headers.len(),
		block_num as usize,
		"Didn't get the same number of block headers."
	);

	Ok(())
}

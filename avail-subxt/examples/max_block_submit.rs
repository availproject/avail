use avail_subxt::{api, tx, AvailClient, BoundedVec, Opts};

use structopt::StructOpt;
use subxt_signer::sr25519::dev;
use futures::stream::{FuturesOrdered, TryStreamExt as _};

/// This example attempts to submit data to fill the entire block. Note that this doesn't guarantee
/// that the block will be filled, but if you submit more than a full block, then it will spill over
/// to the next block. The limit for the transaction is currently set to 512 kB, and limit for the block
/// is 2 MB, so this means 128 data transactions are needed to fill the block. Depending on the network,
/// it may not be possible to transfer so many in 20 s (the default block time)
const BLOCK_SIZE: usize = 2 * 1024 * 1024;
const TX_MAX_SIZE: usize = 512 * 1024;
const NUM_CHUNKS: u64 = (BLOCK_SIZE / TX_MAX_SIZE) as u64;

#[async_std::main]
async fn main() -> anyhow::Result<()> {
	let args = Opts::from_args();
	let client = AvailClient::new(args.ws).await?;

	let alice = dev::alice();
	let nonce = tx::nonce(&client, &alice).await?;

	let start = std::time::Instant::now();
	let calls = (0..NUM_CHUNKS).map(|i| {
		let data = BoundedVec(vec![(i & 255) as u8; TX_MAX_SIZE]);
		api::tx().data_availability().submit_data(data)
	}).collect::<Vec<_>>();
	let txs = calls.iter().enumerate().map(|(idx,call)| {
		tx::send_with_nonce(&client, call, &alice, 1, nonce + idx as u64)
	})
		.collect::<FuturesOrdered<_>>()
		.try_collect::<Vec<_>>().await?;

	let submittions_end = start.elapsed();
	println!("Submittions done in {submittions_end:?}");

	let in_block_txs = txs.into_iter().map(tx::in_finalized)
		.collect::<FuturesOrdered<_>>()
		.try_collect::<Vec<_>>()
		.await?;
	for (i, in_block) in in_block_txs.into_iter().enumerate() {
		let hash = in_block.block_hash();
		println!("Finalized {i} in block: {hash:?}");
	}
	let end = start.elapsed();
	println!("Finalized in {end:?}");


	Ok(())
}

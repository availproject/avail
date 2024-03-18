use avail_subxt::{api, tx, AccountId, AvailClient, BoundedVec, Opts};

use structopt::StructOpt;
use subxt_signer::sr25519::dev;

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
	let alice_id: AccountId = alice.public_key().into();
	let nonce = client.tx().account_nonce(&alice_id).await?;
	let mut submittions = Vec::with_capacity(NUM_CHUNKS as usize);

	let start = std::time::Instant::now();
	for i in 0..NUM_CHUNKS {
		let data = BoundedVec(vec![(i & 255) as u8; TX_MAX_SIZE]);
		let call = api::tx().data_availability().submit_data(data);
		let progress = tx::send_with_nonce(&client, &call, &alice, 1, nonce + i).await?;
		submittions.push(progress);
	}
	let submittions_end = start.elapsed();
	println!("Submittions done in {submittions_end:?}");

	for (i, progress) in submittions.into_iter().enumerate() {
		let hash = tx::then_in_block(progress).await?.block_hash();
		println!("Finalized {i} in block: {hash:?} in {:?}", start.elapsed());
	}
	let end = start.elapsed();
	println!("Finalized in {end:?}");

	Ok(())
}

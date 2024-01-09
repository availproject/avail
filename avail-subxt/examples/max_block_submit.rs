use avail_subxt::{build_client, submit_data, Opts};

use sp_keyring::AccountKeyring;
use structopt::StructOpt;
use subxt::tx::PairSigner;

/// This example attempts to submit data to fill the entire block. Note that this doesn't guarantee
/// that the block will be filled, but if you submit more than a full block, then it will spill over
/// to the next block. The limit for the transaction is currently set to 512 kB, and limit for the block
/// is 2 MB, so this means 128 data transactions are needed to fill the block. Depending on the network,
/// it may not be possible to transfer so many in 20 s (the default block time)
const BLOCK_SIZE: usize = 2 * 1024 * 1024;
const TX_MAX_SIZE: usize = 512 * 1024;
const NUM_CHUNKS: usize = BLOCK_SIZE / TX_MAX_SIZE;

#[async_std::main]
async fn main() -> anyhow::Result<()> {
	let args = Opts::from_args();
	let (client, _) = build_client(args.ws, args.validate_codegen).await?;

	let signer = PairSigner::new(AccountKeyring::Alice.pair());
	let start = std::time::Instant::now();

	for i in 1..=NUM_CHUNKS {
		let data = vec![(i & 255) as u8; TX_MAX_SIZE];
		let h = submit_data(&client, &signer, data, 1).await?;
		println!("hash #{i}: {:?}", h);
	}
	let end = start.elapsed();

	println!("Done in {end:?}!");
	Ok(())
}

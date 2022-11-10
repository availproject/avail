use std::time::Instant;

use anyhow::Result;
use avail_subxt::{
	api::{self, runtime_types::frame_support::storage::bounded_vec::BoundedVec},
	build_client,
	primitives::AvailExtrinsicParams,
	Opts,
};
use sp_keyring::AccountKeyring;
use structopt::StructOpt;
use subxt::tx::PairSigner;

/// This example attempts to submit data to fill the entire block. Note that this doesn't guarantee
/// that the block will be filled, but if you submit more than a full block, then it will spill over
/// to the next block. The limit for the transaction is currently set to 16 kB, and limit for the block
/// is 2 MB, so this means 128 data transactions are needed to fill the block. Depending on the network,
/// it may not be possible to transfer so many in 20 s (the default block time)

#[async_std::main]
async fn main() -> Result<()> {
	let args = Opts::from_args();
	let client = build_client(args.ws).await?;

	let signer = PairSigner::new(AccountKeyring::Alice.pair());
	let size: usize = 2 * 1024 * 1024;
	let max_size: usize = 16 * 1024;
	let num_chunks = size / max_size;
	let extrinsic_params = AvailExtrinsicParams::new_with_app_id(1.into());
	let start = Instant::now();

	for i in 1..=num_chunks {
		let data_transfer =
			api::tx()
				.data_availability()
				.submit_data(BoundedVec(vec![(i & 255) as u8; max_size]));
		let h = client
			.tx()
			.sign_and_submit(&data_transfer, &signer, extrinsic_params.clone())
			.await?;
		println!("hash #{i}: {:?}", h);
	}
	let end = start.elapsed();

	println!("Done in {end:?}!");

	Ok(())
}

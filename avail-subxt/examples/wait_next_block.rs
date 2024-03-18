use avail_subxt::{AvailClient, Opts};

use futures::stream::StreamExt as _;
use std::time::Instant;
use structopt::StructOpt;
use subxt::Error;

#[async_std::main]
async fn main() -> Result<(), Error> {
	let args = Opts::from_args();
	let client = AvailClient::new(args.ws).await?;

	let start = Instant::now();
	println!("Waiting for next block...");

	let best_stream = client.blocks().subscribe_best().await?;

	// Skip current block, and wait for the next one.
	let block_hash = best_stream
		.skip(1)
		.next()
		.await
		.transpose()?
		.map(|b| b.hash());
	let end = start.elapsed();
	println!("\tNext block {block_hash:?}, finalized in {end:?}");

	Ok(())
}

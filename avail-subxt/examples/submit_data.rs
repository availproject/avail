use avail_subxt::{
	api::data_availability::calls::types::SubmitData, submit::submit_data, tx, AvailClient, Opts,
};

use anyhow::Result;
use structopt::StructOpt;
use subxt_signer::sr25519::dev;

const DATA: &[u8] = b"example";

/// This example submits an Avail data extrinsic, then retrieves the block containing the
/// extrinsic and matches the data.
#[async_std::main]
async fn main() -> Result<()> {
	let args = Opts::from_args();
	let client = AvailClient::new(args.ws).await?;

	let alice = dev::alice();
	let hash = tx::then_in_block(submit_data(&client, &alice, DATA, 1).await?)
		.await?
		.block_hash();
	println!("Submitted data in block hash: {hash:?}");

	let extrinsics = client.blocks().at(hash).await?.extrinsics().await?;
	let submit_call = extrinsics.find::<SubmitData>().next().unwrap()?;
	assert_eq!(submit_call.value.data.0.as_slice(), DATA);

	Ok(())
}

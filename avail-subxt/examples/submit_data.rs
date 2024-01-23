use anyhow::Result;
use avail_subxt::{
	api::runtime_types::da_control::pallet::Call as DaCall, avail::AppUncheckedExtrinsic,
	build_client, submit_data_in_block as submit, Call, Opts,
};
use sp_keyring::AccountKeyring;
use structopt::StructOpt;
use subxt::tx::PairSigner;

const DATA: &[u8] = b"example";

/// This example submits an Avail data extrinsic, then retrieves the block containing the
/// extrinsic and matches the data.
#[async_std::main]
async fn main() -> Result<()> {
	let args = Opts::from_args();
	let (client, _) = build_client(args.ws, args.validate_codegen).await?;
	let signer = PairSigner::new(AccountKeyring::Alice.pair());

	let block = submit(&client, &signer, DATA, 1).await?.block_hash();
	let submitted_block = client.rpc().block(Some(block)).await?.unwrap();

	let matched_xt = submitted_block
		.block
		.extrinsics
		.into_iter()
		.filter_map(|chain_block_ext| {
			AppUncheckedExtrinsic::try_from(chain_block_ext)
				.map(|ext| ext.function)
				.ok()
		})
		.find(|call| match call {
			Call::DataAvailability(DaCall::submit_data { data }) => data.0 == DATA,
			_ => false,
		});
	assert!(matched_xt.is_some(), "Submitted data not found");

	Ok(())
}

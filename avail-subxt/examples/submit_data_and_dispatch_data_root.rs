use anyhow::Result;
use avail_subxt::{
	api::{self, runtime_types::frame_support::storage::bounded_vec::BoundedVec},
	build_client,
	primitives::AvailExtrinsicParams,
	Opts,
};
use sp_keyring::AccountKeyring;
use structopt::StructOpt;
use subxt::{ext::sp_core::H160, tx::PairSigner};

const DESTINATION_DOMAIN: u32 = 1000;
const DA_BRIDGE_ROUTER_ADDRESS: &str = "0x3f28a3e66326c3aa494d4f8e9477d1397ee94432";

/// This example submits an Avail data extrinsic, then retrieves the block containing the
/// extrinsic and matches the data.
#[async_std::main]
async fn main() -> Result<()> {
	let args = Opts::from_args();
	let client = build_client(args.ws).await?;

	let signer = PairSigner::new(AccountKeyring::Alice.pair());
	let example_data = b"example".to_vec();
	let data_transfer = api::tx()
		.data_availability()
		.submit_data(BoundedVec(example_data.clone()));

	let extrinsic_params = AvailExtrinsicParams::new_with_app_id(1.into());
	println!("Sending example data...");
	let block_hash = client
		.tx()
		.sign_and_submit_then_watch(&data_transfer, &signer, extrinsic_params)
		.await?
		.wait_for_finalized_success()
		.await?
		.block_hash();

	let header = client
		.rpc()
		.header(Some(block_hash.clone()))
		.await?
		.expect("Valid block hash .qed");

	println!("Hash of block with example data: {:?}", &header);
	println!("Header data root: {:?}", header.data_root());
	println!("Block hash: {:?}", block_hash);

	let bridge_router_eth_addr: H160 = DA_BRIDGE_ROUTER_ADDRESS.parse().unwrap();
	let tx = api::tx().da_bridge().try_dispatch_data_root(
		DESTINATION_DOMAIN,
		bridge_router_eth_addr.into(),
		header.into(),
	);

	println!(
		"Sending finalized block header. Domain: {}. Recipient: {}",
		DESTINATION_DOMAIN, DA_BRIDGE_ROUTER_ADDRESS
	);
	let h = client
		.tx()
		.sign_and_submit_then_watch(&tx, &signer, Default::default())
		.await
		.unwrap()
		.wait_for_finalized_success()
		.await
		.unwrap();

	let submitted_block = client
		.rpc()
		.block(Some(h.block_hash()))
		.await
		.unwrap()
		.unwrap();

	let xts = submitted_block.block.extrinsics;
	println!("Submitted block extrinsic: {xts:?}");

	Ok(())
}

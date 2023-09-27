use anyhow::Result;
use avail_subxt::{
	api::{
		self, data_availability::calls::types::SubmitData,
		runtime_types::bounded_collections::bounded_vec::BoundedVec,
	},
	build_client, Opts,
};
use structopt::StructOpt;
use subxt::{tx::Payload, utils::H160};
use subxt_signer::sr25519::dev;

const DESTINATION_DOMAIN: u32 = 1000;
const DA_BRIDGE_ROUTER_ADDRESS: &str = "0x3f28a3e66326c3aa494d4f8e9477d1397ee94432";

fn submit_some_data() -> Result<Payload<SubmitData>> {
	let data = BoundedVec(b"Test Data".to_vec());
	let submit_data_tx = api::tx().data_availability().submit_data(data);
	Ok(submit_data_tx)
}

#[async_std::main]
async fn main() -> Result<()> {
	let args = Opts::from_args();
	let client = build_client(args.ws, args.validate_codegen).await?;

	let signer = dev::alice();
	let mut finalized_headers_subscription =
		client.rpc().subscribe_finalized_block_headers().await?;

	if let Some(header) = finalized_headers_subscription.next().await {
		let header = header?;
		if let Some(_block_hash) = client.rpc().block_hash(Some(header.number.into())).await? {
			// 1. Send some data.
			let block_hash = client
				.tx()
				.sign_and_submit_then_watch_default(&submit_some_data()?, &signer)
				.await?
				.wait_for_finalized_success()
				.await?
				.block_hash();
			println!("Block hash: {:?}", block_hash);
			let submit_data_header = client
				.rpc()
				.block(Some(block_hash))
				.await?
				.expect("Block exists .qed")
				.block
				.header;
			println!("Finalized block header: {:?}", &submit_data_header);
			println!("Header data root: {:?}", submit_data_header.data_root());

			// 2. Send the `DaBridge::try_dispatch_data_root`.
			let bridge_router_eth_addr: H160 = DA_BRIDGE_ROUTER_ADDRESS.parse()?;
			let tx = api::tx().nomad_da_bridge().try_dispatch_data_root(
				DESTINATION_DOMAIN,
				bridge_router_eth_addr.into(),
				submit_data_header.into(),
			);

			println!(
				"Sending finalized block header. Domain: {}. Recipient: {}",
				DESTINATION_DOMAIN, DA_BRIDGE_ROUTER_ADDRESS
			);
			let block_hash = client
				.tx()
				.sign_and_submit_then_watch(&tx, &signer, Default::default())
				.await?
				.wait_for_finalized_success()
				.await?
				.block_hash();

			// Get block
			let submitted_block = client
				.rpc()
				.block(Some(block_hash))
				.await?
				.expect("Block exists .qed");

			let xts = submitted_block.block.extrinsics;
			println!("Submitted block extrinsic: {xts:?}");
		}
	}

	Ok(())
}

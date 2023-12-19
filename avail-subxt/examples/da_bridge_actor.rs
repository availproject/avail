use anyhow::Result;
use avail_subxt::{
	api, build_client, submit_data_finalized as submit, tx_send_in_finalized, AvailConfig, Opts,
};
use sp_core::H256;
use sp_keyring::AccountKeyring::Alice;
use structopt::StructOpt;
use subxt::{rpc::types::ChainBlockResponse, tx::PairSigner, utils::H160, OnlineClient};

const DESTINATION_DOMAIN: u32 = 1000;
const DA_BRIDGE_ROUTER_ADDRESS: &str = "0x3f28a3e66326c3aa494d4f8e9477d1397ee94432";
const DATA: &[u8] = b"Test Data";

async fn fetch_block(
	client: &OnlineClient<AvailConfig>,
	hash: H256,
) -> ChainBlockResponse<AvailConfig> {
	client
		.rpc()
		.block(Some(hash))
		.await
		.ok()
		.flatten()
		.expect("Block exists .qed")
}

#[async_std::main]
async fn main() -> Result<()> {
	let args = Opts::from_args();
	let (client, _) = build_client(args.ws, args.validate_codegen).await?;

	let signer = PairSigner::new(Alice.pair());
	let mut finalized_headers_subscription =
		client.rpc().subscribe_finalized_block_headers().await?;

	if let Some(header) = finalized_headers_subscription.next().await {
		let header = header?;
		if let Some(_block_hash) = client.rpc().block_hash(Some(header.number.into())).await? {
			// 1. Send some data.
			let hash = submit(&client, &signer, DATA, 1).await?.block_hash();
			println!("Block hash: {hash:?}");
			let submit_data_header = fetch_block(&client, hash).await.block.header;
			println!("Finalized block header: {submit_data_header:?}");
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
			let hash = tx_send_in_finalized!(&client, &tx, &signer)
				.await?
				.block_hash();

			// Get block
			let submitted_block = fetch_block(&client, hash).await;
			println!(
				"Submitted block extrinsic: {:?}",
				submitted_block.block.extrinsics
			);
		}
	}

	Ok(())
}

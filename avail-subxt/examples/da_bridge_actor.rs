use std::{thread, time};

use avail_subxt::*;
use sp_keyring::AccountKeyring;
use subxt::{ext::sp_core::H160, tx::PairSigner, OnlineClient};

const DESTINATION_DOMAIN: u32 = 1000;
const DA_BRIDGE_ROUTER_ADDRESS: &str = "0x333d43c984b8d92f6c857e0bfbb29a4fbf42dbe9";

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	let api = OnlineClient::<AvailConfig>::from_url("ws://127.0.0.1:9944").await?;
	let signer = PairSigner::new(AccountKeyring::Alice.pair());

	let mut finalized_blocks = api.rpc().subscribe_finalized_blocks().await?;

	while let Some(finalized_block) = finalized_blocks.next().await {
		let header = finalized_block.unwrap();
		println!("Finalized block header: {:?}", &header);
		println!("Header data root: {:?}", header.extrinsics_root.data_root);

		let bridge_router_eth_addr: H160 = DA_BRIDGE_ROUTER_ADDRESS.parse().unwrap();
		let tx = avail::tx().da_bridge().try_enqueue_data_root(
			DESTINATION_DOMAIN,
			bridge_router_eth_addr.into(),
			header.into(),
		);

		println!(
			"Sending finalized block header. Domain: {}. Recipient: {}",
			DESTINATION_DOMAIN, DA_BRIDGE_ROUTER_ADDRESS
		);
		let h = api
			.tx()
			.sign_and_submit_then_watch(&tx, &signer, Default::default())
			.await
			.unwrap()
			.wait_for_finalized_success()
			.await
			.unwrap();

		let submitted_block = api
			.rpc()
			.block(Some(h.block_hash()))
			.await
			.unwrap()
			.unwrap();

		let xts = submitted_block.block.extrinsics;
		println!("Submitted block extrinsic: {xts:?}");

		println!("Sleeping for 10 sec...\n");
		thread::sleep(time::Duration::from_secs(10));
	}

	Ok(())
}

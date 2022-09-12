use avail_subxt::*;
use sp_keyring::AccountKeyring;
use subxt::{ext::sp_core::H160, tx::PairSigner, OnlineClient};

const DESTINATION_DOMAIN: u32 = 1000;
const DA_BRIDGE_ROUTER_ADDRESS: &str = "0x42a8ea235a8edb8c64c22d8c0d181301e9cf5051";

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	let api = OnlineClient::<AvailConfig>::from_url("ws://127.0.0.1:9944").await?;
	let signer = PairSigner::new(AccountKeyring::Alice.pair());

	let mut finalized_blocks = api.rpc().subscribe_finalized_blocks().await?;

	while let Some(finalized_block) = finalized_blocks.next().await {
		let header = finalized_block.unwrap();
		println!("Finalized block header: {:?}", &header);

		let bridge_router_eth_addr: H160 = DA_BRIDGE_ROUTER_ADDRESS.parse().unwrap();
		let tx = avail::tx().da_bridge().try_enqueue_data_root(
			DESTINATION_DOMAIN,
			bridge_router_eth_addr.into(),
			header.into(),
		);

		println!("Sending finalized block header...");
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
	}

	Ok(())
}

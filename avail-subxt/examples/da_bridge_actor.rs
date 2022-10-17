use avail_subxt::*;
use subxt::{
	ext::sp_core::{ecdsa::Pair, Pair as PairT, H160},
	tx::PairSigner,
	OnlineClient,
};

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	let rpc = std::env::var("RPC_URL").expect("missing RPC_URL");
	let priv_key = std::env::var("PRIVATE_KEY").expect("missing PRIVATE_KEY");
	let interval: u32 = std::env::var("INTERVAL_BLOCKS")
		.unwrap_or("10".into())
		.parse()
		.unwrap();
	let destination_domain = std::env::var("DESTINATION_DOMAIN")
		.expect("missing DESTINATION_DOMAIN")
		.parse()
		.unwrap();
	let da_bridge_router_addr =
		std::env::var("DA_BRIDGE_ROUTER_ADDRESS").expect("missing DA_BRIDGE_ROUTER_ADDRESS");

	let api = OnlineClient::<AvailConfig>::from_url(rpc).await?;

	let pair = Pair::from_string(&priv_key, None).unwrap();
	let signer = PairSigner::new(pair);
	println!("Tx signer AccountId: {}", signer.account_id());

	let mut finalized_blocks = api.rpc().subscribe_finalized_blocks().await?;

	while let Some(finalized_block) = finalized_blocks.next().await {
		let header = finalized_block.unwrap();

		// Want to submit every interval blocks
		let number: u32 = header.number.into();
		if number % interval != 0 {
			continue;
		}

		println!("Finalized block header: {:?}", &header);
		println!("Header data root: {:?}", header.extrinsics_root.data_root);

		if let Some(block_hash) = api.rpc().block_hash(Some(header.number.into())).await? {
			println!("Block hash: {:?}", block_hash);

			let bridge_router_eth_addr: H160 = da_bridge_router_addr.parse().unwrap();
			let tx = avail::tx().da_bridge().try_dispatch_data_root(
				destination_domain,
				bridge_router_eth_addr.into(),
				header.into(),
			);

			println!(
				"Sending finalized block header. Domain: {}. Recipient: {}",
				destination_domain, da_bridge_router_addr
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
		}
	}

	Ok(())
}

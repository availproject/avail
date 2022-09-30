use avail_subxt::{avail::runtime_types::frame_support::storage::bounded_vec::BoundedVec, *};
use sp_keyring::AccountKeyring;
use subxt::{
	ext::{sp_core::H160, sp_runtime::traits::Header},
	tx::PairSigner,
	OnlineClient,
};

const DESTINATION_DOMAIN: u32 = 1000;
const DA_BRIDGE_ROUTER_ADDRESS: &str = "0x77534486c6467fd24b1f7d60ca61d984d91f6a2a";

/// This example submits an Avail data extrinsic, then retrieves the block containing the
/// extrinsic and matches the data.
#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	let api = OnlineClient::<AvailConfig>::from_url("ws://127.0.0.1:9944").await?;
	let signer = PairSigner::new(AccountKeyring::Alice.pair());
	let example_data = b"example".to_vec();
	let data_transfer = avail::tx()
		.data_availability()
		.submit_data(BoundedVec(example_data.clone()));

	let extrinsic_params = AvailExtrinsicParams::new_with_app_id(1);
	println!("Sending example data...");
	let h = api
		.tx()
		.sign_and_submit_then_watch(&data_transfer, &signer, extrinsic_params)
		.await
		.unwrap()
		.wait_for_finalized_success()
		.await
		.unwrap();
	let block_hash = h.block_hash();

	let header: DaHeader = api
		.rpc()
		.header(Some(block_hash.clone()))
		.await
		.unwrap()
		.unwrap();

	println!("Hash of block with example data: {:?}", &header);
	println!("Header data root: {:?}", header.extrinsics_root.data_root);
	println!("Block hash: {:?}", block_hash);

	let bridge_router_eth_addr: H160 = DA_BRIDGE_ROUTER_ADDRESS.parse().unwrap();
	let tx = avail::tx().da_bridge().try_dispatch_data_root(
		DESTINATION_DOMAIN,
		bridge_router_eth_addr.into(),
		*header.number(),
		block_hash,
		header.data_root(),
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

	Ok(())
}

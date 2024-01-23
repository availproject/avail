use avail_subxt::{api, build_client, submit_data_finalized as submit, tx_send_in_finalized, Opts};
use sp_keyring::AccountKeyring;
use structopt::StructOpt;
use subxt::{tx::PairSigner, utils::H160};

const DESTINATION_DOMAIN: u32 = 1000;
const DA_BRIDGE_ROUTER_ADDRESS: &str = "0x3f28a3e66326c3aa494d4f8e9477d1397ee94432";
const DATA: &[u8] = b"example";

/// This example submits an Avail data extrinsic, then retrieves the block containing the
/// extrinsic and matches the data.
#[async_std::main]
async fn main() -> anyhow::Result<()> {
	let args = Opts::from_args();
	let (client, _) = build_client(args.ws, args.validate_codegen).await?;
	let signer = PairSigner::new(AccountKeyring::Alice.pair());

	println!("Sending example data...");
	let block_hash = submit(&client, &signer, DATA, 1).await?.block_hash();

	let header = client
		.rpc()
		.header(Some(block_hash.clone()))
		.await?
		.unwrap();

	println!("Hash of block with example data: {:?}", &header);
	println!("Header data root: {:?}", header.data_root());
	println!("Block hash: {:?}", block_hash);

	let bridge_router_eth_addr: H160 = DA_BRIDGE_ROUTER_ADDRESS.parse().unwrap();
	let tx = api::tx().nomad_da_bridge().try_dispatch_data_root(
		DESTINATION_DOMAIN,
		bridge_router_eth_addr.into(),
		header.into(),
	);

	println!(
		"Sending finalized block header. Domain: {}. Recipient: {}",
		DESTINATION_DOMAIN, DA_BRIDGE_ROUTER_ADDRESS
	);
	let h = tx_send_in_finalized!(&client, &tx, &signer).await?;

	let submitted_block = client.rpc().block(Some(h.block_hash())).await?.unwrap();

	let xts = submitted_block.block.extrinsics;
	println!("Submitted block extrinsic: {xts:?}");

	Ok(())
}

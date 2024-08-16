use avail_rust::{Keypair, SecretUri, WaitFor, SDK};
use core::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), String> {
	let sdk = SDK::new("ws://127.0.0.1:9944").await.unwrap();

	// Input
	let secret_uri = SecretUri::from_str("//Alice").unwrap();
	let account = Keypair::from_uri(&secret_uri).unwrap();
	let targets = [
		String::from("5GNJqTPyNqANBkUVMN1LPPrxXnFouWXoe2wNSmmEoLctxiZY"), // Alice Stash
		String::from("5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty"), // Bob;
	];

	let result = sdk
		.tx
		.staking
		.nominate(&targets, WaitFor::BlockInclusion, &account)
		.await?;

	println!("TxDataTargets={:?}", result.tx_data.targets);
	println!(
		"TxHash={:?}, BlockHash={:?}",
		result.tx_hash, result.block_hash
	);

	Ok(())
}

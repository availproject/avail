use avail_rust::{Keypair, SecretUri, WaitFor, SDK};
use core::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), String> {
	let sdk = SDK::new("ws://127.0.0.1:9944").await.unwrap();

	// Input
	let secret_uri = SecretUri::from_str("//Alice").unwrap();
	let account = Keypair::from_uri(&secret_uri).unwrap();
	let max_additional = 1_000_000_000_000_000_000u128; // 1 AVAIL

	let result = sdk
		.tx
		.staking
		.bond_extra(max_additional, WaitFor::BlockInclusion, &account)
		.await?;

	println!(
		"Stash={}, Amount={:?}",
		result.event.stash, result.event.amount
	);
	println!(
		"TxHash={:?}, BlockHash={:?}",
		result.tx_hash, result.block_hash
	);

	Ok(())
}

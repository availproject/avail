use avail_rust::{Keypair, SecretUri, WaitFor, SDK};
use core::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), String> {
	let sdk = SDK::new("ws://127.0.0.1:9944").await.unwrap();

	// Input
	let secret_uri = SecretUri::from_str("//Alice").unwrap();
	let account = Keypair::from_uri(&secret_uri).unwrap();
	let dest = "5HGjWAeFDfFCWPsjFQdVV2Msvz2XtMktvgocEZcCj68kUMaw"; // Eve
	let keep_alive = false;

	let result = sdk
		.tx
		.balances
		.transfer_all(dest, keep_alive, WaitFor::BlockInclusion, &account)
		.await?;

	println!(
		"From={}, To={}, Amount={}",
		result.event.from, result.event.to, result.event.amount
	);
	if let Some(event) = result.event2 {
		println!("Killed={}", event.account);
	}
	println!(
		"TxHash={:?}, BlockHash={:?}",
		result.tx_hash, result.block_hash
	);

	Ok(())
}

use avail_rust::{Keypair, SecretUri, WaitFor, SDK};
use core::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), String> {
	let sdk = SDK::new("ws://127.0.0.1:9944").await.unwrap();

	// Input
	let secret_uri = SecretUri::from_str("//Alice").unwrap();
	let account = Keypair::from_uri(&secret_uri).unwrap();
	let dest = "5HGjWAeFDfFCWPsjFQdVV2Msvz2XtMktvgocEZcCj68kUMaw"; // Eve
	let amount = 1_000_000_000_000_000_00u128; // 1 Avail

	let result = sdk
		.tx
		.balances
		.transfer_allow_death(dest, amount, WaitFor::BlockInclusion, &account, None)
		.await?;

	if let Some(event) = &result.event2 {
		println!("Killed={}", event.account);
	}

	dbg!(result);

	Ok(())
}

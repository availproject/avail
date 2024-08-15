use avail_rust::{Keypair, SecretUri, WaitFor, SDK};
use core::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), String> {
	let sdk = SDK::new("ws://127.0.0.1:9944").await.unwrap();

	// Input
	let secret_uri = SecretUri::from_str("//Alice").unwrap();
	let account = Keypair::from_uri(&secret_uri).unwrap();
	let commission = 100;
	let blocked = false;

	let result = sdk
		.tx
		.staking
		.validate(commission, blocked, WaitFor::BlockInclusion, &account)
		.await?;

	println!(
		"Stash={}, Commission={:?}, Blocked={:?}",
		result.event.stash, result.event.prefs.commission, result.event.prefs.blocked
	);
	println!(
		"TxHash={:?}, BlockHash={:?}",
		result.tx_hash, result.block_hash
	);

	Ok(())
}

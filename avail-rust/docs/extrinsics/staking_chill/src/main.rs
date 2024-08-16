use avail_rust::{Keypair, SecretUri, WaitFor, SDK};
use core::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), String> {
	let sdk = SDK::new("ws://127.0.0.1:9944").await.unwrap();

	// Input
	let secret_uri = SecretUri::from_str("//Alice//stash").unwrap();
	let account = Keypair::from_uri(&secret_uri).unwrap();

	let result = sdk
		.tx
		.staking
		.chill(WaitFor::BlockInclusion, &account)
		.await?;

	if let Some(event) = result.event {
		println!("Stash={}", event.stash);
	}

	println!(
		"TxHash={:?}, BlockHash={:?}",
		result.tx_hash, result.block_hash
	);

	Ok(())
}

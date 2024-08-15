use avail_rust::{Keypair, RewardDestination, SecretUri, WaitFor, SDK};
use core::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), String> {
	let sdk = SDK::new("ws://127.0.0.1:9944").await.unwrap();

	// Input
	let secret_uri = SecretUri::from_str("//Alice").unwrap();
	let account = Keypair::from_uri(&secret_uri).unwrap();
	let value = 1_000_000_000_000_000_000u128 * 100_000u128; // 100_000 Avail
	let payee = RewardDestination::Staked;

	let result = sdk
		.tx
		.staking
		.bond(value, payee, WaitFor::BlockInclusion, &account)
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

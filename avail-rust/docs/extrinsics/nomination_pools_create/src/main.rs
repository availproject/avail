use avail_rust::{Keypair, Nonce, Options, SecretUri, WaitFor, SDK};
use core::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), String> {
	let sdk = SDK::new("ws://127.0.0.1:9944").await.unwrap();

	// Input
	let secret_uri = SecretUri::from_str("//Alice").unwrap();
	let account = Keypair::from_uri(&secret_uri).unwrap();
	let amount = 1_000_000_000_000_000_000_000_000u128; // 1_000_000 Avail tokens
	let root = "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"; // Alice
	let nominator = "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"; // Alice
	let bouncer = "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"; // Alice

	let wait_for = WaitFor::BlockInclusion;
	let options = Options::new().nonce(Nonce::BestBlockAndTxPool);
	let result = sdk
		.tx
		.nomination_pools
		.create(
			amount,
			root,
			nominator,
			bouncer,
			wait_for,
			&account,
			Some(options),
		)
		.await?;

	dbg!(result);

	Ok(())
}

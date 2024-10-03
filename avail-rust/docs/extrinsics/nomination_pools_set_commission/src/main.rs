use avail_rust::{Keypair, NewCommission, Nonce, Options, Perbill, SecretUri, WaitFor, SDK};
use core::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), String> {
	let sdk = SDK::new("ws://127.0.0.1:9944").await.unwrap();

	// Input
	let secret_uri = SecretUri::from_str("//Alice").unwrap();
	let account = Keypair::from_uri(&secret_uri).unwrap();
	let pool_id = 1;
	let new_commission = NewCommission {
		payee: String::from("5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"), // Alice
		amount: Perbill(10_000_000u32),                                          // 1%
	};

	let wait_for = WaitFor::BlockInclusion;
	let options = Options::new().nonce(Nonce::BestBlockAndTxPool);
	let result = sdk
		.tx
		.nomination_pools
		.set_commission(
			pool_id,
			Some(new_commission),
			wait_for,
			&account,
			Some(options),
		)
		.await?;

	dbg!(result);

	Ok(())
}

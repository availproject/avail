use avail_rust::{Keypair, Nonce, Options, SecretUri, WaitFor, SDK};
use core::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), String> {
	let sdk = SDK::new("ws://127.0.0.1:9944").await.unwrap();

	// Input
	let secret_uri = SecretUri::from_str("//Alice").unwrap();
	let account = Keypair::from_uri(&secret_uri).unwrap();
	let keys = sdk.rpc.author.rotate_keys().await.unwrap();
	let keys = sdk.util.deconstruct_session_keys(keys)?;

	let wait_for = WaitFor::BlockInclusion;
	let options = Options::new().nonce(Nonce::BestBlockAndTxPool);
	let result = sdk
		.tx
		.session
		.set_keys(keys, wait_for, &account, Some(options))
		.await?;

	dbg!(result);

	Ok(())
}

use avail_rust::{Keypair, SecretUri, WaitFor, SDK};
use core::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), String> {
	let sdk = SDK::new("ws://127.0.0.1:9944").await.unwrap();

	// Input
	let secret_uri = SecretUri::from_str("//Alice").unwrap();
	let account = Keypair::from_uri(&secret_uri).unwrap();

	let keys = sdk.rpc.author.rotate_keys().await.unwrap();
	let keys = sdk.util.deconstruct_session_keys(keys)?;
	let result = sdk
		.tx
		.session
		.set_keys(keys, WaitFor::BlockInclusion, &account, None)
		.await?;

	dbg!(result);

	Ok(())
}

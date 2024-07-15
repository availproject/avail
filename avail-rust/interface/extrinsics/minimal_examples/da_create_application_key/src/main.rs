use avail_rust::{Key, Keypair, SecretUri, WaitFor, SDK};
use core::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), String> {
	let sdk = SDK::new("ws://127.0.0.1:9944").await.unwrap();

	// Input
	let secret_uri = SecretUri::from_str("//Alice").unwrap();
	let account = Keypair::from_uri(&secret_uri).unwrap();
	let key = String::from("MyAwesomeKey").as_bytes().to_vec();
	let key = Key { 0: key };

	let result = sdk
		.tx
		.data_availability
		.create_application_key(key, WaitFor::BlockInclusion, &account)
		.await?;

	println!(
		"Key={:?}, Owner={}, Id={:?}",
		result.event.key, result.event.owner, result.event.id
	);
	println!(
		"TxHash={:?}, BlockHash={:?}",
		result.tx_hash, result.block_hash
	);

	Ok(())
}

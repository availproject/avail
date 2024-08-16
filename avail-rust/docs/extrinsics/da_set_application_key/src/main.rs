use avail_rust::{Key, Keypair, SecretUri, WaitFor, SDK};
use core::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), String> {
	let sdk = SDK::new("ws://127.0.0.1:9944").await.unwrap();

	// Input
	let secret_uri = SecretUri::from_str("//Alice").unwrap();
	let account = Keypair::from_uri(&secret_uri).unwrap();

	let old_key = String::from("MyAwesomeKey").as_bytes().to_vec();
	let old_key = Key { 0: old_key };
	let new_key = String::from("MyAwesomeKey2").as_bytes().to_vec();
	let new_key = Key { 0: new_key };

	let result = sdk
		.tx
		.data_availability
		.set_application_key(old_key, new_key, WaitFor::BlockInclusion, &account)
		.await?;

	println!(
		"OldKey={:?}, NewKey={:?}",
		result.event.old_key, result.event.new_key
	);
	println!(
		"TxHash={:?}, BlockHash={:?}",
		result.tx_hash, result.block_hash
	);

	Ok(())
}

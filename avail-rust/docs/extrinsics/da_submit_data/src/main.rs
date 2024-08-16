use avail_rust::{Data, Keypair, SecretUri, WaitFor, SDK};
use core::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), String> {
	let sdk = SDK::new("ws://127.0.0.1:9944").await.unwrap();

	// Input
	let secret_uri = SecretUri::from_str("//Alice").unwrap();
	let account = Keypair::from_uri(&secret_uri).unwrap();
	let data = String::from("My Awesome Data").as_bytes().to_vec();
	let data = Data { 0: data };

	let result = sdk
		.tx
		.data_availability
		.submit_data(data, WaitFor::BlockInclusion, &account)
		.await?;

	println!(
		"Who={}, DataHash={:?}",
		result.event.who, result.event.data_hash
	);
	println!("TxData={:?}", result.tx_data.data);
	println!(
		"TxHash={:?}, BlockHash={:?}",
		result.tx_hash, result.block_hash
	);

	Ok(())
}

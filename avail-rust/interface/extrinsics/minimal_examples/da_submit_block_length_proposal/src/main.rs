use avail_rust::{Keypair, SecretUri, WaitFor, SDK};
use core::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), String> {
	let sdk = SDK::new("ws://127.0.0.1:9944").await.unwrap();

	// Input
	let secret_uri = SecretUri::from_str("//Alice").unwrap();
	let account = Keypair::from_uri(&secret_uri).unwrap();

	let rows = 128;
	let cols = 128;

	let result = sdk
		.tx
		.data_availability
		.submit_block_length_proposal(rows, cols, WaitFor::BlockInclusion, &account)
		.await?;

	println!("Rows={:?}, Cols={:?}", result.event.rows, result.event.cols);
	println!(
		"TxHash={:?}, BlockHash={:?}",
		result.tx_hash, result.block_hash
	);

	Ok(())
}

use avail_rust::SDK;

#[tokio::main]
async fn main() -> Result<(), String> {
	let sdk = SDK::new("ws://127.0.0.1:9944")
		.await
		.map_err(|e| e.to_string())?;

	let block_hash = sdk
		.rpc
		.chain
		.get_block_hash(None)
		.await
		.map_err(|e| e.to_string())?;
	println!("BlockHash={:?}", block_hash);

	Ok(())
}

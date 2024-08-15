use avail_rust::SDK;

#[tokio::main]
async fn main() -> Result<(), String> {
	let sdk = SDK::new("ws://127.0.0.1:9944")
		.await
		.map_err(|e| e.to_string())?;

	let block_hash = sdk
		.rpc
		.chain
		.get_finalized_head()
		.await
		.map_err(|e| e.to_string())?;
	println!("FinalizedHeadBlockHash={:?}", block_hash);

	Ok(())
}

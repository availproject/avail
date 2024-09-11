use avail_rust::{avail, SDK};

#[tokio::main]
async fn main() -> Result<(), String> {
	let sdk = SDK::new("ws://127.0.0.1:9944").await.unwrap();

	let storage_query = avail::storage().staking().active_era();
	let best_block_hash = sdk
		.rpc
		.chain
		.get_block_hash(None)
		.await
		.map_err(|e| e.to_string())?;
	let result = sdk
		.api
		.storage()
		.at(best_block_hash)
		.fetch(&storage_query)
		.await
		.map_err(|e| e.to_string())?;

	dbg!(result);

	Ok(())
}

use avail_rust::{avail, SDK};

#[tokio::main]
async fn main() -> Result<(), String> {
	let sdk = SDK::new("ws://127.0.0.1:9944").await.unwrap();

	let storage_query = avail::storage().data_availability().app_keys_iter();
	let best_block_hash = sdk
		.rpc
		.chain
		.get_block_hash(None)
		.await
		.map_err(|e| e.to_string())?;
	let mut results = sdk
		.api
		.storage()
		.at(best_block_hash)
		.iter(storage_query)
		.await
		.map_err(|e| e.to_string())?;

	while let Some(Ok(kv)) = results.next().await {
		let key = (&kv.key_bytes[49..]).to_vec();
		let key = String::from_utf8(key).unwrap();

		println!("Key: {:?}", key);
		println!("Value: {:?}", kv.value);
	}

	Ok(())
}

use avail_rust::{avail, AccountId, SDK};

#[tokio::main]
async fn main() -> Result<(), String> {
	let sdk = SDK::new("ws://127.0.0.1:9944").await.unwrap();

	let storage_query = avail::storage().system().account_iter();
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
		let key: [u8; 32] = (&kv.key_bytes[48..]).try_into().unwrap();
		let key = AccountId::from(key);

		println!("Key: 0x{:?}", key.to_string());
		println!("Value: {:?}", kv.value);
	}

	Ok(())
}

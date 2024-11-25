use avail_rust::{avail, error::ClientError, rpcs, AccountId, SDK};

pub async fn account_iter() -> Result<(), ClientError> {
	let sdk = SDK::new(SDK::local_endpoint()).await?;
	let (online_client, rpc_client) = (&sdk.online_client, &sdk.rpc_client);

	let storage_query = avail::storage().system().account_iter();
	let best_block_hash = rpcs::get_best_block_hash(rpc_client).await?;
	let storage = online_client.storage().at(best_block_hash);
	let mut results = storage.iter(storage_query).await?;

	while let Some(Ok(kv)) = results.next().await {
		let key: [u8; 32] = (&kv.key_bytes[48..]).try_into().unwrap();
		let key = AccountId::from(key);

		println!("Key: 0x{:?}", key.to_string());
		println!("Value: {:?}", kv.value);
	}

	Ok(())
}

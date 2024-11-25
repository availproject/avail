use avail_rust::{avail, error::ClientError, rpcs, utils::account_id_from_str, AccountId, SDK};

pub async fn active_era() -> Result<(), ClientError> {
	let sdk = SDK::new(SDK::local_endpoint()).await?;
	let (online_client, rpc_client) = (&sdk.online_client, &sdk.rpc_client);

	let storage_query = avail::storage().staking().active_era();
	let best_block_hash = rpcs::get_best_block_hash(rpc_client).await?;
	let storage = online_client.storage().at(best_block_hash);
	let result = storage.fetch(&storage_query).await?;

	dbg!(result);

	Ok(())
}

pub async fn bonded() -> Result<(), ClientError> {
	let sdk = SDK::new(SDK::local_endpoint()).await?;
	let (online_client, rpc_client) = (&sdk.online_client, &sdk.rpc_client);

	let account_id = account_id_from_str("5GNJqTPyNqANBkUVMN1LPPrxXnFouWXoe2wNSmmEoLctxiZY")?; // Alice_Stash

	let storage_query = avail::storage().staking().bonded(account_id);
	let best_block_hash = rpcs::get_best_block_hash(rpc_client).await?;
	let storage = online_client.storage().at(best_block_hash);
	let result = storage.fetch(&storage_query).await?;

	dbg!(result);

	Ok(())
}

pub async fn bonded_iter() -> Result<(), ClientError> {
	let sdk = SDK::new(SDK::local_endpoint()).await?;
	let (online_client, rpc_client) = (&sdk.online_client, &sdk.rpc_client);

	let storage_query = avail::storage().staking().bonded_iter();
	let best_block_hash = rpcs::get_best_block_hash(rpc_client).await?;
	let storage = online_client.storage().at(best_block_hash);
	let mut results = storage.iter(storage_query).await?;

	while let Some(Ok(kv)) = results.next().await {
		let key: [u8; 32] = (&kv.key_bytes[40..]).try_into().unwrap();
		let key = AccountId::from(key);

		println!("Key: 0x{:?}", key.to_string());
		println!("Value: {:?}", kv.value);
	}

	Ok(())
}

use avail_rust::{
	avail::{self, runtime_types::bounded_collections::bounded_vec::BoundedVec},
	error::ClientError,
	rpcs,
	utils::account_id_from_str,
	AccountId, Block, SDK,
};

pub async fn da_app_keys() -> Result<(), ClientError> {
	let sdk = SDK::new(SDK::local_endpoint()).await?;
	let (online_client, rpc_client) = (&sdk.online_client, &sdk.rpc_client);

	let key = String::from("Reserved-1");
	let key = BoundedVec(key.as_bytes().to_vec());
	let storage_query = avail::storage().data_availability().app_keys(key);

	let best_block_hash = Block::fetch_best_block_hash(rpc_client).await?;
	let storage = online_client.storage().at(best_block_hash);
	let result = storage.fetch(&storage_query).await?;

	dbg!(result);

	Ok(())
}

pub async fn da_app_keys_iter() -> Result<(), ClientError> {
	let sdk = SDK::new(SDK::local_endpoint()).await?;
	let (online_client, rpc_client) = (&sdk.online_client, &sdk.rpc_client);

	let storage_query = avail::storage().data_availability().app_keys_iter();

	let best_block_hash = Block::fetch_best_block_hash(rpc_client).await?;
	let storage = online_client.storage().at(best_block_hash);
	let mut results = storage.iter(storage_query).await?;

	while let Some(Ok(kv)) = results.next().await {
		let key = (&kv.key_bytes[49..]).to_vec();
		let key = String::from_utf8(key).unwrap();

		println!("Key: {:?}", key);
		println!("Value: {:?}", kv.value);
	}

	Ok(())
}

pub async fn da_next_app_id() -> Result<(), ClientError> {
	let sdk = SDK::new(SDK::local_endpoint()).await?;
	let (online_client, rpc_client) = (&sdk.online_client, &sdk.rpc_client);

	let storage_query = avail::storage().data_availability().next_app_id();

	let best_block_hash = Block::fetch_best_block_hash(rpc_client).await?;
	let storage = online_client.storage().at(best_block_hash);
	let result = storage.fetch_or_default(&storage_query).await?;

	dbg!(result);

	Ok(())
}

pub async fn staking_active_era() -> Result<(), ClientError> {
	let sdk = SDK::new(SDK::local_endpoint()).await?;
	let (online_client, rpc_client) = (&sdk.online_client, &sdk.rpc_client);

	let storage_query = avail::storage().staking().active_era();
	let best_block_hash = Block::fetch_best_block_hash(rpc_client).await?;
	let storage = online_client.storage().at(best_block_hash);
	let result = storage.fetch(&storage_query).await?;

	dbg!(result);

	Ok(())
}

pub async fn staking_bonded() -> Result<(), ClientError> {
	let sdk = SDK::new(SDK::local_endpoint()).await?;
	let (online_client, rpc_client) = (&sdk.online_client, &sdk.rpc_client);

	let account_id = account_id_from_str("5GNJqTPyNqANBkUVMN1LPPrxXnFouWXoe2wNSmmEoLctxiZY")?; // Alice_Stash

	let storage_query = avail::storage().staking().bonded(account_id);
	let best_block_hash = Block::fetch_best_block_hash(rpc_client).await?;
	let storage = online_client.storage().at(best_block_hash);
	let result = storage.fetch(&storage_query).await?;

	dbg!(result);

	Ok(())
}

pub async fn staking_bonded_iter() -> Result<(), ClientError> {
	let sdk = SDK::new(SDK::local_endpoint()).await?;
	let (online_client, rpc_client) = (&sdk.online_client, &sdk.rpc_client);

	let storage_query = avail::storage().staking().bonded_iter();
	let best_block_hash = Block::fetch_best_block_hash(rpc_client).await?;
	let storage = online_client.storage().at(best_block_hash);
	let mut results = storage.iter(storage_query).await?;

	while let Some(Ok(kv)) = results.next().await {
		let key: [u8; 32] = (&kv.key_bytes[40..]).try_into().unwrap();
		let key = AccountId::from(key);

		println!("Key: {:?}", key.to_string());
		println!("Value: {:?}", kv.value);
	}

	Ok(())
}

pub async fn system_account_iter() -> Result<(), ClientError> {
	let sdk = SDK::new(SDK::local_endpoint()).await?;
	let (online_client, rpc_client) = (&sdk.online_client, &sdk.rpc_client);

	let storage_query = avail::storage().system().account_iter();
	let best_block_hash = Block::fetch_best_block_hash(rpc_client).await?;
	let storage = online_client.storage().at(best_block_hash);
	let mut results = storage.iter(storage_query).await?;

	while let Some(Ok(kv)) = results.next().await {
		let key: [u8; 32] = (&kv.key_bytes[48..]).try_into().unwrap();
		let key = AccountId::from(key);

		println!("Key: {:?}", key.to_string());
		println!("Value: {:?}", kv.value);
	}

	Ok(())
}

pub async fn run() -> Result<(), ClientError> {
	println!("da_app_keys");
	da_app_keys().await?;
	println!("da_app_keys_iter");
	da_app_keys_iter().await?;
	println!("da_next_app_id");
	da_next_app_id().await?;
	println!("staking_active_era");
	staking_active_era().await?;
	println!("staking_bonded");
	staking_bonded().await?;
	println!("staking_bonded_iter");
	staking_bonded_iter().await?;
	println!("system_account_iter");
	system_account_iter().await?;

	Ok(())
}

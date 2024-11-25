use avail_rust::{
	avail::{self, runtime_types::bounded_collections::bounded_vec::BoundedVec},
	error::ClientError,
	rpcs, SDK,
};

pub async fn app_keys() -> Result<(), ClientError> {
	let sdk = SDK::new(SDK::local_endpoint()).await?;
	let (online_client, rpc_client) = (&sdk.online_client, &sdk.rpc_client);

	let key = String::from("Reserved-1");
	let key = BoundedVec(key.as_bytes().to_vec());
	let storage_query = avail::storage().data_availability().app_keys(key);

	let best_block_hash = rpcs::get_best_block_hash(rpc_client).await?;
	let storage = online_client.storage().at(best_block_hash);
	let result = storage.fetch(&storage_query).await?;

	dbg!(result);

	Ok(())
}

pub async fn app_keys_iter() -> Result<(), ClientError> {
	let sdk = SDK::new(SDK::local_endpoint()).await?;
	let (online_client, rpc_client) = (&sdk.online_client, &sdk.rpc_client);

	let storage_query = avail::storage().data_availability().app_keys_iter();

	let best_block_hash = rpcs::get_best_block_hash(rpc_client).await?;
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

pub async fn next_app_id() -> Result<(), ClientError> {
	let sdk = SDK::new(SDK::local_endpoint()).await?;
	let (online_client, rpc_client) = (&sdk.online_client, &sdk.rpc_client);

	let storage_query = avail::storage().data_availability().next_app_id();

	let best_block_hash = rpcs::get_best_block_hash(rpc_client).await?;
	let storage = online_client.storage().at(best_block_hash);
	let result = storage.fetch_or_default(&storage_query).await?;

	dbg!(result);

	Ok(())
}

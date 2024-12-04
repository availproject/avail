use avail_rust::{
	avail::{self, runtime_types::bounded_collections::bounded_vec::BoundedVec},
	error::ClientError,
	utils, Cell, Options, PopulatedOptions, SDK,
};

pub async fn run() -> Result<(), ClientError> {
	let sdk = SDK::new(SDK::local_endpoint()).await?;

	// author_rotate_keys
	let value = sdk.rpc.author.rotate_keys().await?;
	let value = utils::deconstruct_session_keys(value)?;
	dbg!(value);

	// chain_get_block
	let value = sdk.rpc.chain.get_block(None).await?;
	dbg!(value);

	// chain_get_block_hash
	let value = sdk.rpc.chain.get_block_hash(None).await?;
	dbg!(value);

	// chain_get_finalized_head
	let value = sdk.rpc.chain.get_finalized_head().await?;
	dbg!(value);

	// chain_get_header
	let value = sdk.rpc.chain.get_header(None).await?;
	dbg!(value);

	// system_account_next_index
	let account = String::from("5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY");
	let value = sdk.rpc.system.account_next_index(account).await?;
	dbg!(value);

	// system_chain
	let value = sdk.rpc.system.chain().await?;
	dbg!(value);

	// system_chain_type
	let value = sdk.rpc.system.chain_type().await?;
	dbg!(value);

	// system_health
	let value = sdk.rpc.system.health().await?;
	dbg!(value);

	// system_local_listen_addresses
	let value = sdk.rpc.system.local_listen_addresses().await?;
	dbg!(value);

	// system_local_peer_id
	let value = sdk.rpc.system.local_peer_id().await?;
	dbg!(value);

	// system_name
	let value = sdk.rpc.system.name().await?;
	dbg!(value);

	// system_node_roles
	let value = sdk.rpc.system.node_roles().await?;
	dbg!(value);

	// system_peers
	let value = sdk.rpc.system.peers().await?;
	dbg!(value);

	// system_properties
	let value = sdk.rpc.system.properties().await?;
	dbg!(value);

	// system_system_sync_state
	let value = sdk.rpc.system.sync_state().await?;
	dbg!(value);

	// system_version
	let value = sdk.rpc.system.version().await?;
	dbg!(value);

	// TransactionPaymentApi_query_info
	let payload = avail::tx()
		.data_availability()
		.submit_data(BoundedVec(vec![1]));
	let keypair = SDK::alice()?;
	let account = keypair.public_key().to_account_id();

	let options = Some(Options::new().app_id(1));
	let populated_options = options
		.unwrap_or_default()
		.build(&sdk.online_client, &sdk.rpc_client, &account)
		.await?;

	let params = populated_options.build(&sdk.rpc_client).await?;
	let tx = sdk
		.online_client
		.tx()
		.create_signed(&payload, &keypair, params)
		.await?;
	let partial_fee_estimate = tx.partial_fee_estimate().await?;
	dbg!(partial_fee_estimate);

	// TransactionPaymentApi_query_fee_details
	let len_bytes: [u8; 4] = (tx.encoded().len() as u32).to_le_bytes();
	let encoded_with_len = [tx.encoded(), &len_bytes[..]].concat();

	let fee_details = sdk
		.rpc
		.payment
		.query_fee_details(encoded_with_len.into(), None)
		.await?;
	dbg!(fee_details);

	// kate_block_length
	let value = sdk.rpc.kate.block_length(None).await?;
	dbg!(value);

	// kate_query_data_proof
	let data = String::from("My Data").into_bytes();
	let tx = sdk.tx.data_availability.submit_data(data);
	let result = tx.execute_wait_for_finalization(&keypair, options).await?;
	let (tx_index, block_hash) = (result.tx_index, Some(result.block_hash));
	let value = sdk.rpc.kate.query_data_proof(tx_index, block_hash).await?;
	dbg!(value);

	// kate_query_proof
	let cells = vec![Cell::from((0u32, 0u32))];
	let value = sdk.rpc.kate.query_proof(cells, block_hash).await?;
	dbg!(value);

	// kate_query_rows
	let rows = vec![0u32];
	let value = sdk.rpc.kate.query_rows(rows, block_hash).await?;
	dbg!(value);

	Ok(())
}

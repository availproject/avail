use avail_rust::{avail, error::ClientError, utils, Block, Options, SDK};

pub async fn run() -> Result<(), ClientError> {
	let sdk = SDK::new(SDK::local_endpoint()).await?;

	let account = SDK::alice()?;

	let dest = utils::account_id_from_str("5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty")?;
	let options = Some(Options::new().nonce(avail_rust::Nonce::BestBlockAndTxPool));
	let tx = sdk.tx.balances.transfer_keep_alive(dest, SDK::one_avail());
	let res = tx.execute_wait_for_inclusion(&account, options).await?;

	let block = Block::new(&sdk.online_client, res.block_hash).await?;

	// transaction_all_static, transaction_count, transaction_by_signer, transaction_by_signer_static
	// transaction_by_index, transaction_by_index_static, transaction_by_hash,
	// transaction_by_hash_static, transaction_by_app_id, transaction_by_app_id_static
	for tx in block.transactions.iter() {
		println!(
			"Tx Pallet name: {}, Tx Name: {}, Tx Hash: {:?}",
			tx.pallet_name()?,
			tx.variant_name()?,
			tx.hash()
		);

		for event in tx.events().await?.iter() {
			let Ok(event) = event else {
				return Ok(());
			};

			println!(
				"\tEvent Pallet name: {}, Event Name: {}",
				event.pallet_name(),
				event.variant_name()
			);
		}

		let balance_tx = tx.as_extrinsic::<avail::balances::calls::types::TransferKeepAlive>();
		if let Some(tx) = balance_tx.ok().flatten() {
			println!("Transfer dest: {:?}, value: {}", tx.dest, tx.value);
		}
	}

	Ok(())
}

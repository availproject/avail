use avail_rust::{error::ClientError, rpcs::get_best_block, utils, Nonce, Options, SDK};
use std::time::Duration;

pub async fn run() -> Result<(), ClientError> {
	let sdk = SDK::new(SDK::local_endpoint()).await?;

	let account = SDK::alice()?;
	let dest = utils::account_id_from_str("5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty")?;
	let mut options = Options::new();
	let tx = sdk.tx.balances.transfer_keep_alive(dest, SDK::one_avail());

	/*
		The default behavior is for the transaction to use the nonce from the finalized block.
		This behavior is different from the one in avail-js.
	*/
	options = options.nonce(Nonce::FinalizedBlock);
	tx.execute_and_forget(&account, Some(options)).await?;
	tx.execute_and_forget(&account, Some(options))
		.await
		.expect_err("qed");
	wait_n_blocks(&sdk, 3).await?;

	/*
		Using best block nonce will not take into consideration existing transactions in the
		tx pool.
	*/
	options = options.nonce(Nonce::BestBlock);
	tx.execute_and_forget(&account, Some(options)).await?;
	tx.execute_and_forget(&account, Some(options))
		.await
		.expect_err("qed");
	wait_n_blocks(&sdk, 1).await?;

	/*
		This is the most commonly used nonce. If correctness is needed, use `Nonce::FinalizedBlock`
	*/
	options = options.nonce(Nonce::BestBlockAndTxPool);
	tx.execute_and_forget(&account, Some(options)).await?;
	tx.execute_and_forget(&account, Some(options)).await?;

	/*
		Managing the nonce manually
	*/
	let nonce = utils::get_nonce_node(
		&sdk.rpc_client,
		&account.public_key().to_account_id().to_string(),
	)
	.await?;

	options = options.nonce(Nonce::Custom(nonce));
	tx.execute_and_forget(&account, Some(options)).await?;
	options = options.nonce(Nonce::Custom(nonce + 1));
	tx.execute_and_forget(&account, Some(options)).await?;

	Ok(())
}

async fn wait_n_blocks(sdk: &SDK, n: u32) -> Result<(), ClientError> {
	let current_block = get_best_block(&sdk.rpc_client).await?.block.header.number;
	let expected_block = current_block + n;

	loop {
		let current_block = get_best_block(&sdk.rpc_client).await?.block.header.number;
		if current_block >= expected_block {
			break;
		}

		tokio::time::sleep(Duration::from_secs(3)).await
	}

	Ok(())
}

use std::time::Duration;

use avail_rust::prelude::*;

use crate::wait_for_new_block;

/// This example attempts to submit data to fill the entire block. Note that this doesn't guarantee
/// that the block will be filled, but if you submit more than a full block, then it will spill over
/// to the next block. The limit for the transaction is currently set to 1 MB, and limit for the block
/// is 4 MB, so this means 4 data transactions are needed to fill the block. Depending on the network,
/// it may not be possible to transfer so many in 20 s (the default block time)
const ONE_MIB: usize = 1024 * 1024;
const ONE_MB: usize = 1000 * 1000;
const BLOCK_SIZE: usize = 4 * ONE_MIB;
const TX_MAX_SIZE: usize = ONE_MIB;
const GROUP_TX_MAX_SIZE: usize = ONE_MB;
const NUM_CHUNKS: usize = (BLOCK_SIZE / TX_MAX_SIZE) as usize;
const TAG: &str = "MAX_BLOCK_SUBMIT";

pub async fn run() -> Result<(), ClientError> {
	let sdk = SDK::new(SDK::local_endpoint()).await?;

	let alice = account::alice();
	let options = Options::new().app_id(5);

	// Testing if MAX_TX_SIZE is viable
	println!("{}: Submitting transaction...", TAG);
	let tx = sdk.tx.data_availability.submit_data(vec![0; TX_MAX_SIZE]);
	let res = tx.execute_and_watch_inclusion(&alice, options).await?;
	assert_eq!(res.is_successful(), Some(true));

	// Testing if we can fit NUM_CHUNKS * GROUP_TX_MAX_SIZE bytes into a block.
	let mut nonce = account::nonce(
		&sdk.client,
		&std::format!("{}", alice.public_key().to_account_id()),
	)
	.await?;
	let tx = sdk
		.tx
		.data_availability
		.submit_data(vec![0; GROUP_TX_MAX_SIZE]);

	println!("{}: Submitting transactions...", TAG);
	let mut tx_hashes = Vec::with_capacity(NUM_CHUNKS);
	// Execute Txs
	for _ in 0..NUM_CHUNKS {
		let hash = tx.execute(&alice, options.nonce(nonce)).await?;
		nonce += 1;
		tx_hashes.push(hash);
	}

	// Wait for new block
	println!("{}: Waiting for new block...", TAG);
	wait_for_new_block(&sdk).await?;
	tokio::time::sleep(Duration::from_secs(5)).await;

	println!("{}: Checking transactions...", TAG);
	// Get details
	let mut expected_block_number = None;
	for tx_hash in tx_hashes {
		let result = sdk.client.transaction_state(&tx_hash, false).await.unwrap();
		let result = result.get(0).unwrap();
		if expected_block_number.is_none() {
			expected_block_number = Some(result.block_height)
		}

		assert_eq!(expected_block_number, Some(result.block_height));
		assert_eq!(result.tx_success, true);
	}

	println!("{}: Done", TAG);
	Ok(())
}

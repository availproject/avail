#![allow(dead_code)]

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

pub async fn run() -> Result<(), Error> {
	let client = Client::new(LOCAL_ENDPOINT).await?;

	let alice = alice();
	let options = Options::new(5);

	// Testing if MAX_TX_SIZE is viable
	println!("{}: Submitting transaction...", TAG);
	let tx = client
		.tx()
		.data_availability()
		.submit_data(5, vec![0; TX_MAX_SIZE]);
	let res = tx.sign_and_submit(&alice, options).await?;
	let receipt = res.receipt(true).await.unwrap();
	assert!(receipt.is_some());

	// Testing if we can fit NUM_CHUNKS * GROUP_TX_MAX_SIZE bytes into a block.
	let mut nonce = client.chain().account_nonce(alice.account_id()).await?;
	let tx = client
		.tx()
		.data_availability()
		.submit_data(5, vec![0; GROUP_TX_MAX_SIZE]);

	println!("{}: Submitting transactions...", TAG);
	let mut submitted_txs = Vec::with_capacity(NUM_CHUNKS);
	// Execute Txs
	for _ in 0..NUM_CHUNKS {
		let hash = tx.sign_and_submit(&alice, options.nonce(nonce)).await?;
		nonce += 1;
		submitted_txs.push(hash);
	}

	// Wait for new block
	println!("{}: Waiting for new block...", TAG);
	wait_for_new_block(&client).await?;
	tokio::time::sleep(Duration::from_secs(5)).await;

	println!("{}: Checking transactions...", TAG);
	// Get details
	let mut expected_block_id = None;
	for submitted_tx in submitted_txs {
		let receipt = submitted_tx.receipt(true).await.unwrap().unwrap();
		if expected_block_id.is_none() {
			expected_block_id = Some(receipt.block_ref)
		}
		let expected_id = expected_block_id.unwrap();

		assert_eq!(expected_id.height, receipt.block_ref.height);
		assert_eq!(expected_id.hash, receipt.block_ref.hash);
	}

	println!("{}: Done", TAG);
	Ok(())
}

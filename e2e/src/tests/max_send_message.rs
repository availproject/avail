use super::{alice_nonce, local_connection, no_concurrency};

use avail_core::{
	data_proof::{BoundedData, Message},
	AppId,
};
use avail_subxt::{
	api::{self, vector::calls::types::FailedSendMessageTxs},
	tx, AccountId,
};
use subxt::utils::H256;
use subxt_signer::sr25519::dev;

use futures::stream::{FuturesUnordered, TryStreamExt as _};
use std::{collections::HashMap, sync::atomic::Ordering::Relaxed};
use test_log::test;
use tracing::trace;

const INVALID_DOMAIN: u32 = 0;
const CALL_COUNT: usize = 136;

#[test(tokio::test)]
async fn max_send_message() -> anyhow::Result<()> {
	let _cg = no_concurrency("submit_data").await;
	let client = local_connection().await?;

	let alice = dev::alice();
	let to_bob = H256(AccountId::from(dev::bob().public_key()).0);
	let message = Message::ArbitraryMessage(BoundedData::truncate_from(vec![]));

	let send_call = api::tx()
		.vector()
		.send_message(message.into(), to_bob, INVALID_DOMAIN);
	let nonce = alice_nonce().await.fetch_add(CALL_COUNT as u64, Relaxed);
	trace!("Minumum `vector::send_message` call created (invalid domain), reserved nonces {CALL_COUNT}");

	// Send Txs.
	let txs_progress = (0..CALL_COUNT)
		.map(|call_idx| {
			tx::send_with_nonce(
				&client,
				&send_call,
				&alice,
				AppId(0),
				nonce + call_idx as u64,
			)
		})
		.collect::<FuturesUnordered<_>>()
		.try_collect::<Vec<_>>()
		.await?;

	// Wait until they are in one block.
	let txs_in_block = txs_progress
		.into_iter()
		.map(tx::then_in_block)
		.collect::<FuturesUnordered<_>>()
		.try_collect::<Vec<_>>()
		.await?;

	// Get Txs per block.
	let mut block_hashes = HashMap::<H256, usize>::new();
	let _ = txs_in_block
		.into_iter()
		.scan(&mut block_hashes, |acc, tx| {
			acc.entry(tx.block_hash())
				.and_modify(|count| *count += 1)
				.or_insert(1);
			Some(true)
		})
		.collect::<Vec<_>>();
	trace!("Transactions per block: {block_hashes:?}");

	// Ensure post inherent for blocks.
	for block_hash in block_hashes.into_keys() {
		if let Ok(block) = client.blocks().at(block_hash).await {
			let number = block.number();
			let extrinsics = block.extrinsics().await?;
			let found_exts = extrinsics
				.find::<FailedSendMessageTxs>()
				.collect::<Result<Vec<_>, _>>()?;
			for ext in found_exts.iter() {
				let len = ext.details.bytes().len();
				let failed_txs = &ext.value.failed_txs;
				trace!("Failed Send Message Txs on {block_hash:?} ({number}) is {len} bytes: {failed_txs:?}");
			}
		}
	}
	Ok(())
}

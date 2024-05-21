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

use anyhow::ensure;
use futures::stream::{FuturesUnordered, TryStreamExt as _};
use std::{collections::HashMap, sync::atomic::Ordering::Relaxed};
use test_log::test;
use tracing::trace;

const INVALID_DOMAIN: u32 = 0;
const MAX_ARBITRATY: usize = 136 + 1;
const MAX_FUNGIBLE: usize = 132 + 1;

const ENABLE_LOG_MSG: &str =
	"Re-run with `RUST_LOG=\"e2e::tests::max_send_message=trace\" to display logs`";

#[test(tokio::test)]
async fn max_fungible_message() -> anyhow::Result<()> {
	let _cg = no_concurrency("submit_data").await;
	let message = Message::FungibleToken {
		asset_id: H256::default(),
		amount: 0,
	};

	max_send_message(message, MAX_FUNGIBLE, 2).await
}

#[test(tokio::test)]
async fn max_arbitrary_message() -> anyhow::Result<()> {
	let _cg = no_concurrency("submit_data").await;
	let message = Message::ArbitraryMessage(BoundedData::truncate_from(vec![]));

	max_send_message(message, MAX_ARBITRATY, 2).await
}

async fn max_send_message(
	message: Message,
	max_calls: usize,
	expected_blocks: usize,
) -> anyhow::Result<()> {
	let alice = dev::alice();
	let to_bob = H256(AccountId::from(dev::bob().public_key()).0);
	let client = local_connection().await?;

	let send_call = api::tx()
		.vector()
		.send_message(message.into(), to_bob, INVALID_DOMAIN);
	let nonce = alice_nonce().await.fetch_add(max_calls as u64, Relaxed);
	trace!("Minumum `vector::send_message` call created (invalid domain), reserved nonces {max_calls} from {nonce}");

	// Send Txs.
	let txs_progress = (0..max_calls)
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

	// Ensure block does not contain more than expected maximum.
	ensure!(
		block_hashes.len() == expected_blocks,
		"Generated blocks are not expected({expected_blocks}), {ENABLE_LOG_MSG}"
	);
	for (block_hash, count) in block_hashes.iter() {
		ensure!(
			*count <= max_calls,
			"Block {block_hash} contains more TXs({count}) than expected({max_calls}), {ENABLE_LOG_MSG}"
		);
	}

	// Ensure post inherent for blocks.
	let mut total_failed_txs = 0usize;
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
				total_failed_txs += failed_txs.len();
				trace!("Failed Send Message Txs on {block_hash:?} ({number}) is {len} bytes: {failed_txs:?}");
			}
		}
	}
	ensure!(
		total_failed_txs == max_calls,
		"Missing some failed TXs in post inherent, {ENABLE_LOG_MSG}"
	);

	Ok(())
}

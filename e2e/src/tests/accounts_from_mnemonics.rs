use super::{alice_nonce, allow_concurrency, free_balance_of, local_connection};

use avail_core::AppId;
use avail_subxt::{api, avail::AVAIL, tx, AccountId};
use sp_core::H256;
use subxt::blocks::BlockRef;
use subxt_signer::sr25519::dev;

use anyhow::Result;
use std::sync::atomic::Ordering::Relaxed;
use test_log::test;
use tracing::{debug, trace};

const AMOUNT: u128 = 2 * AVAIL;

#[test(tokio::test)]
async fn account_from_mnemonics() -> Result<()> {
	let _cg = allow_concurrency("account_from_mnemonics").await;
	let client = local_connection().await?;

	// Accounts
	let alice = dev::alice();
	let bob = dev::bob();
	let bob_id: AccountId = bob.public_key().into();

	// Transfer and wait finalized
	let pre_alice_bal = free_balance_of(&client, &alice, None).await?;
	debug!("Alice pre balance: {pre_alice_bal:#?}");
	let pre_bob_bal = free_balance_of(&client, &bob, None).await?;
	debug!("Bob pre balance: {pre_bob_bal:#?}");

	let call = api::tx()
		.balances()
		.transfer_keep_alive(bob_id.into(), AMOUNT);
	let nonce = alice_nonce().await.fetch_add(1, Relaxed);
	let tx_progress = tx::send_with_nonce(&client, &call, &alice, AppId(0), nonce).await?;
	let tx_in_block: BlockRef<H256> = tx::then_in_block(tx_progress).await?.block_hash().into();
	trace!("Transfer {AMOUNT} from Alice to Bob at {tx_in_block:?}");

	// Check post balance
	let alice_bal = free_balance_of(&client, &alice, Some(tx_in_block.clone())).await?;
	debug!("Alice post balance: {alice_bal:#?}");
	let bob_bal = free_balance_of(&client, &bob, Some(tx_in_block)).await?;
	debug!("Bob post balance: {bob_bal:#?}");

	assert!(pre_alice_bal - alice_bal >= AMOUNT);
	//assert!(bob_bal - pre_bob_bal, AMOUNT);

	Ok(())
}

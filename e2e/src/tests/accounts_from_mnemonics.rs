use super::{free_balance_of, local_connection};

use avail_core::AppId;
use avail_subxt::{api, avail::AVAIL, tx, AccountId};

use anyhow::Result;
use subxt_signer::sr25519::dev;

const AMOUNT: u128 = 2 * AVAIL;

#[async_std::test]
async fn account_from_mnemonics() -> Result<()> {
	let client = local_connection().await?;

	// Accounts
	let alice = dev::alice();
	let bob = dev::bob();
	let bob_id: AccountId = bob.public_key().into();

	// Transfer and wait finalized
	let pre_alice_bal = free_balance_of(&client, &alice).await?;
	println!("Alice pre balance: {pre_alice_bal:#?}");
	let pre_bob_bal = free_balance_of(&client, &bob).await?;
	println!("Bob pre balance: {pre_bob_bal:#?}");

	let call = api::tx()
		.balances()
		.transfer_keep_alive(bob_id.into(), AMOUNT);
	let tx_in_block = tx::send_then_finalized(&client, &call, &alice, AppId(0))
		.await?
		.block_hash();
	println!("Transfer {AMOUNT} from Alice to Bob at {tx_in_block:?}");

	// Check post balance
	let alice_bal = free_balance_of(&client, &alice).await?;
	println!("Alice post balance: {alice_bal:#?}");
	let bob_bal = free_balance_of(&client, &bob).await?;
	println!("Bob post balance: {bob_bal:#?}");

	assert!(pre_alice_bal - alice_bal >= AMOUNT);
	assert_eq!(bob_bal - pre_bob_bal, AMOUNT);

	Ok(())
}

use avail_core::currency::Balance;
use avail_subxt::{api, avail::AVAIL, tx, AccountId, AvailClient, AvailConfig, Opts};

use anyhow::Result;
use structopt::StructOpt;
use subxt::{Error, OnlineClient};
use subxt_signer::sr25519::dev;

async fn free_balance_of(
	client: &OnlineClient<AvailConfig>,
	acc: &AccountId,
) -> Result<Balance, Error> {
	let query = api::storage().system().account(acc);
	let acc_info = client
		.storage()
		.at_latest()
		.await?
		.fetch(&query)
		.await?
		.ok_or_else(|| Error::Other("Missing account info".to_string()))?;

	Ok(acc_info.data.free)
}

#[async_std::main]
async fn main() -> Result<()> {
	let args = Opts::from_args();
	let client = AvailClient::new(args.ws).await?;
	let amount = 2 * AVAIL;

	// Accounts
	let alice = dev::alice();
	let alice_id: AccountId = alice.public_key().into();
	let bob = dev::bob().public_key().into();
	let bob_id: AccountId = dev::bob().public_key().into();

	// Transfer and wait finalized
	let pre_alice_bal = free_balance_of(&client, &alice_id).await?;
	println!("Alice pre balance: {pre_alice_bal:#?}");
	let pre_bob_bal = free_balance_of(&client, &bob_id).await?;
	println!("Bob pre balance: {pre_bob_bal:#?}");

	let call = api::tx().balances().transfer_keep_alive(bob, amount);
	let tx_in_block = tx::send_then_finalized(&client, &call, &alice, 0)
		.await?
		.block_hash();
	println!("Transfer {amount} from Alice to Bob at {tx_in_block:?}");

	// Check post balance
	let alice_bal = free_balance_of(&client, &alice_id).await?;
	println!("Alice post balance: {alice_bal:#?}");
	let bob_bal = free_balance_of(&client, &bob_id).await?;
	println!("Bob post balance: {bob_bal:#?}");

	assert!(pre_alice_bal - alice_bal >= amount);
	assert_eq!(bob_bal - pre_bob_bal, amount);

	Ok(())
}

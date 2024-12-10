use avail_rust::{
	avail, error::ClientError, transactions::Transaction, utils, Nonce::BestBlockAndTxPool,
	Options, SDK,
};

use avail::runtime_types::pallet_balances::pallet::Call::transfer_keep_alive as TransferKeepAlive;
use avail::utility::events as UtilityEvents;

pub async fn run() -> Result<(), ClientError> {
	let sdk = SDK::new(SDK::local_endpoint()).await?;

	let account = SDK::alice()?;
	let options = Some(Options::new().nonce(BestBlockAndTxPool));

	let value_1 = SDK::one_avail();
	let value_2 = SDK::one_avail() * 100_000_000;
	let dest_bob = utils::account_id_from_str("5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty")?;
	let dest_charlie =
		utils::account_id_from_str("5FLSigC9HGRKVhB9FiEo4Y3koPsNmBmLJbpXg2mp1hXcS59Y")?;

	let call_1 = TransferKeepAlive {
		dest: dest_bob.into(),
		value: value_1,
	};
	let call_2 = TransferKeepAlive {
		dest: dest_charlie.into(),
		value: value_2,
	};
	let calls = vec![call_1.into(), call_2.into()];

	// Batch
	// This will return `Ok` in all circumstances. To determine the success of the batch, an
	// event is deposited. If a call failed and the batch was interrupted, then the
	// `BatchInterrupted` event is deposited, along with the number of successful calls made
	// and the error of the failed call. If all were successful, then the `BatchCompleted`
	// event is deposited.
	let payload = avail::tx().utility().batch(calls.clone());
	let tx = Transaction::new(sdk.online_client.clone(), sdk.rpc_client.clone(), payload);
	let res = tx.execute_wait_for_inclusion(&account, options).await?;
	println!("-- Batch Call --");

	let batch_interrupted = res.find_event::<UtilityEvents::BatchInterrupted>();
	if batch_interrupted.len() > 0 {
		println!("At least one call has failed");
	}

	let batch_completed = res.find_first_event::<UtilityEvents::BatchCompleted>();
	if batch_completed.is_some() {
		println!("All calls were successful");
	}

	// Batch All
	// Send a batch of dispatch calls and atomically execute them.
	// The whole transaction will rollback and fail if any of the calls failed.
	let payload = avail::tx().utility().batch_all(calls.clone());
	let tx = Transaction::new(sdk.online_client.clone(), sdk.rpc_client.clone(), payload);
	let res = tx.execute_wait_for_inclusion(&account, options).await?;
	res.check_if_transaction_was_successful(&sdk.online_client)
		.expect_err("It should fail");

	// Force Batch
	// Send a batch of dispatch calls.
	// Unlike `batch`, it allows errors and won't interrupt.
	let payload = avail::tx().utility().force_batch(calls.clone());
	let tx = Transaction::new(sdk.online_client.clone(), sdk.rpc_client.clone(), payload);
	let res = tx.execute_wait_for_inclusion(&account, options).await?;
	println!("-- Force Batch Call --");

	let item_failed = res.find_event::<UtilityEvents::ItemFailed>();
	if item_failed.len() > 0 {
		println!("At least one call has failed");
	}

	let batch_completed_with_error =
		res.find_first_event::<UtilityEvents::BatchCompletedWithErrors>();
	if batch_completed_with_error.is_some() {
		println!("Batch completed even though one or more calls have failed.");
	}

	let batch_completed = res.find_first_event::<UtilityEvents::BatchCompleted>();
	if batch_completed.is_some() {
		println!("All calls were successful");
	}

	Ok(())
}

use avail_rust::{avail, Data, Keypair, SecretUri, WaitFor, SDK};
use core::str::FromStr;

use avail::transaction_payment::events as TransactionPaymentEvents;

#[tokio::main]
async fn main() -> Result<(), String> {
	let sdk = SDK::new("ws://127.0.0.1:9944").await.unwrap();

	// Input
	let secret_uri = SecretUri::from_str("//Alice").unwrap();
	let account = Keypair::from_uri(&secret_uri).unwrap();
	let data = String::from("My Awesome Data").as_bytes().to_vec();
	let data = Data { 0: data };

	let tx_api = sdk.api.tx();
	let call = avail::tx().data_availability().submit_data(data);

	let maybe_tx_progress = tx_api
		.sign_and_submit_then_watch_default(&call, &account)
		.await;

	let tx_in_block = sdk
		.util
		.progress_transaction(maybe_tx_progress, WaitFor::BlockInclusion)
		.await?;

	let events = match tx_in_block.wait_for_success().await {
		Ok(e) => e,
		Err(error) => return Err(error.to_string()),
	};

	let event = events
		.find_first::<TransactionPaymentEvents::TransactionFeePaid>()
		.map_err(|err| err.to_string())?;

	match event {
		Some(event) => println!("Event found: {:?}", event),
		None => println!("Event was not found"),
	};

	Ok(())
}

use avail_rust::{avail, Data, Keypair, SecretUri, WaitFor, SDK};
use core::str::FromStr;

use avail::data_availability::calls::types as DataAvailabilityCalls;
use avail::transaction_payment::events as TransactionPaymentEvents;

#[tokio::main]
async fn main() -> Result<(), String> {
	let sdk = SDK::new("ws://127.0.0.1:9944").await.unwrap();

	// Input
	let secret_uri = SecretUri::from_str("//Alice").unwrap();
	let account = Keypair::from_uri(&secret_uri).unwrap();
	let data = String::from("My Awesome Data").as_bytes().to_vec();
	let data = Data { 0: data };

	let result = sdk
		.tx
		.data_availability
		.submit_data(data, WaitFor::BlockInclusion, &account, None)
		.await?;

	let tx = sdk
		.util
		.fetch_transaction::<DataAvailabilityCalls::SubmitData>(result.block_hash, result.tx_hash)
		.await;
	let tx = tx.map_err(|err| err.to_string())?;

	let events = tx.details.events().await.map_err(|err| err.to_string())?;
	let event = events
		.find_first::<TransactionPaymentEvents::TransactionFeePaid>()
		.map_err(|err| err.to_string())?;

	match event {
		Some(event) => println!("Event found: {:?}", event),
		None => println!("Event was not found"),
	};

	Ok(())
}

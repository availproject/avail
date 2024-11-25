use avail_rust::error::ClientError;

pub async fn run() -> Result<(), ClientError> {
	println!("da_submit_data");
	submit_data::run().await?;
	println!("da_create_application_key");
	create_application_key::run().await?;

	Ok(())
}

mod submit_data {
	use avail_rust::{
		error::ClientError,
		transactions::{DataAvailabilityCalls, DataAvailabilityEvents},
		Keypair, Nonce, Options, SecretUri, SDK,
	};
	use core::str::FromStr;

	pub async fn run() -> Result<(), ClientError> {
		let sdk = SDK::new(SDK::local_endpoint()).await?;

		// Input
		let secret_uri = SecretUri::from_str("//Alice").unwrap();
		let account = Keypair::from_uri(&secret_uri).unwrap();
		let data = String::from("My Awesome Data").as_bytes().to_vec();

		let options = Some(Options::new().nonce(Nonce::BestBlockAndTxPool).app_id(1));
		let tx = sdk.tx.data_availability.submit_data(data);
		let result = tx.execute_wait_for_inclusion(&account, options).await?;

		dbg!(&result);
		if let Some(event) = result.find_first_event::<DataAvailabilityEvents::DataSubmitted>() {
			dbg!(event);
		}
		if let Some(data) = result
			.get_data::<DataAvailabilityCalls::SubmitData>(&sdk.online_client)
			.await
		{
			dbg!(data);
		}

		Ok(())
	}
}

mod create_application_key {
	use avail_rust::{
		error::ClientError, transactions::DataAvailabilityEvents, Keypair, Nonce, Options,
		SecretUri, SDK,
	};
	use core::str::FromStr;

	pub async fn run() -> Result<(), ClientError> {
		let sdk = SDK::new(SDK::local_endpoint()).await?;

		// Input
		let secret_uri = SecretUri::from_str("//Alice").unwrap();
		let account = Keypair::from_uri(&secret_uri).unwrap();
		let key = String::from("MyAwesomeKey").as_bytes().to_vec();

		let options = Some(Options::new().nonce(Nonce::BestBlockAndTxPool));
		let tx = sdk.tx.data_availability.create_application_key(key);
		let result = tx.execute_wait_for_inclusion(&account, options).await?;

		dbg!(&result);
		if let Some(event) =
			result.find_first_event::<DataAvailabilityEvents::ApplicationKeyCreated>()
		{
			dbg!(event);
		}

		Ok(())
	}
}

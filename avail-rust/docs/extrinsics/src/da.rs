use avail_rust::error::ClientError;

pub async fn run() -> Result<(), ClientError> {
	println!("da_submit_data");
	submit_data::run().await?;
	println!("da_create_application_key");
	create_application_key::run().await?;

	Ok(())
}

mod submit_data {
	use avail_rust::{error::ClientError, Keypair, Nonce, Options, SecretUri, WaitFor, SDK};
	use core::str::FromStr;

	pub async fn run() -> Result<(), ClientError> {
		let sdk = SDK::new(SDK::local_endpoint()).await?;

		// Input
		let secret_uri = SecretUri::from_str("//Alice").unwrap();
		let account = Keypair::from_uri(&secret_uri).unwrap();
		let data = String::from("My Awesome Data").as_bytes().to_vec();

		let wait_for = WaitFor::BlockInclusion;
		let options = Options::new().nonce(Nonce::BestBlockAndTxPool);
		let result = sdk
			.tx
			.data_availability
			.submit_data(data, wait_for, &account, Some(options))
			.await;
		let result = result.map_err(|e| e.reason)?;

		dbg!(result);

		Ok(())
	}
}

mod create_application_key {
	use avail_rust::{error::ClientError, Key, Keypair, Nonce, Options, SecretUri, WaitFor, SDK};
	use core::str::FromStr;

	pub async fn run() -> Result<(), ClientError> {
		let sdk = SDK::new(SDK::local_endpoint()).await?;

		// Input
		let secret_uri = SecretUri::from_str("//Alice").unwrap();
		let account = Keypair::from_uri(&secret_uri).unwrap();
		let key = String::from("MyAwesomeKey").as_bytes().to_vec();
		let key = Key { 0: key };

		let wait_for = WaitFor::BlockInclusion;
		let options = Options::new().nonce(Nonce::BestBlockAndTxPool);
		let result = sdk
			.tx
			.data_availability
			.create_application_key(key, wait_for, &account, Some(options))
			.await;
		let result = result.map_err(|e| e.reason)?;

		dbg!(result);

		Ok(())
	}
}

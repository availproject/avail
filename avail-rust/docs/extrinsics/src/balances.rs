use avail_rust::error::ClientError;

pub async fn run() -> Result<(), ClientError> {
	println!("balances_transfer_all");
	transfer_all::run().await?;
	transfer_all::clean().await;
	println!("balances_transfer_allow_death");
	transfer_allow_death::run().await?;
	println!("balances_transfer_keep_alive");
	transfer_keep_alive::run().await?;

	Ok(())
}

mod transfer_all {
	use avail_rust::{
		error::ClientError, utils::account_id_from_str, Keypair, Nonce, Options, SecretUri,
		WaitFor, SDK,
	};
	use core::str::FromStr;

	pub async fn run() -> Result<(), ClientError> {
		let sdk = SDK::new(SDK::local_endpoint()).await?;

		// Input
		let secret_uri = SecretUri::from_str("//Alice").unwrap();
		let account = Keypair::from_uri(&secret_uri).unwrap();
		let dest = account_id_from_str("5HGjWAeFDfFCWPsjFQdVV2Msvz2XtMktvgocEZcCj68kUMaw")?; // Eve
		let keep_alive = false;

		let wait_for = WaitFor::BlockInclusion;
		let options = Options::new().nonce(Nonce::BestBlockAndTxPool);
		let result = sdk
			.tx
			.balances
			.transfer_all(dest, keep_alive, wait_for, &account, Some(options))
			.await;
		let result = result.map_err(|e| e.reason)?;

		if let Some(event) = &result.event2 {
			println!("Killed={}", event.account);
		}

		dbg!(result);

		Ok(())
	}

	pub async fn clean() {
		let sdk = SDK::new(SDK::local_endpoint()).await.unwrap();

		// Input
		let secret_uri = SecretUri::from_str("//Eve").unwrap();
		let account = Keypair::from_uri(&secret_uri).unwrap();
		let dest = account_id_from_str("5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY").unwrap(); // Alice
		let value = SDK::one_avail() * 900_000;

		let wait_for = WaitFor::BlockInclusion;
		sdk.tx
			.balances
			.transfer_keep_alive(dest, value, wait_for, &account, None)
			.await
			.unwrap();
	}
}

mod transfer_allow_death {
	use avail_rust::{
		error::ClientError, utils::account_id_from_str, Keypair, Nonce, Options, SecretUri,
		WaitFor, SDK,
	};
	use core::str::FromStr;

	pub async fn run() -> Result<(), ClientError> {
		let sdk = SDK::new(SDK::local_endpoint()).await?;

		// Input
		let secret_uri = SecretUri::from_str("//Alice").unwrap();
		let account = Keypair::from_uri(&secret_uri).unwrap();
		let dest = account_id_from_str("5HGjWAeFDfFCWPsjFQdVV2Msvz2XtMktvgocEZcCj68kUMaw")?; // Eve
		let amount = SDK::one_avail();

		let wait_for = WaitFor::BlockInclusion;
		let options = Options::new().nonce(Nonce::BestBlockAndTxPool);
		let result = sdk
			.tx
			.balances
			.transfer_allow_death(dest, amount, wait_for, &account, Some(options))
			.await;
		let result = result.map_err(|e| e.reason)?;

		if let Some(event) = &result.event2 {
			println!("Killed={}", event.account);
		}

		dbg!(result);

		Ok(())
	}
}

mod transfer_keep_alive {
	use avail_rust::{
		error::ClientError, utils::account_id_from_str, Keypair, Nonce, Options, SecretUri,
		WaitFor, SDK,
	};
	use core::str::FromStr;

	pub async fn run() -> Result<(), ClientError> {
		let sdk = SDK::new(SDK::local_endpoint()).await?;

		// Input
		let secret_uri = SecretUri::from_str("//Alice").unwrap();
		let account = Keypair::from_uri(&secret_uri).unwrap();
		let dest = account_id_from_str("5HGjWAeFDfFCWPsjFQdVV2Msvz2XtMktvgocEZcCj68kUMaw")?; // Eve
		let amount = SDK::one_avail();

		let wait_for = WaitFor::BlockInclusion;
		let options = Options::new().nonce(Nonce::BestBlockAndTxPool);
		let result = sdk
			.tx
			.balances
			.transfer_keep_alive(dest, amount, wait_for, &account, Some(options))
			.await;
		let result = result.map_err(|e| e.reason)?;

		dbg!(result);

		Ok(())
	}
}

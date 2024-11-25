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
		error::ClientError,
		transactions::{BalancesEvents, SystemEvents},
		utils::account_id_from_str,
		Keypair, Nonce, Options, SecretUri, SDK,
	};
	use core::str::FromStr;

	pub async fn run() -> Result<(), ClientError> {
		let sdk = SDK::new(SDK::local_endpoint()).await?;

		// Input
		let secret_uri = SecretUri::from_str("//Alice").unwrap();
		let account = Keypair::from_uri(&secret_uri).unwrap();
		let dest = account_id_from_str("5HGjWAeFDfFCWPsjFQdVV2Msvz2XtMktvgocEZcCj68kUMaw")?; // Eve
		let keep_alive = false;

		let options = Some(Options::new().nonce(Nonce::BestBlockAndTxPool));
		let tx = sdk.tx.balances.transfer_all(dest, keep_alive);
		let result = tx.execute_wait_for_inclusion(&account, options).await?;

		dbg!(&result);
		if let Some(event) = result.find_first_event::<BalancesEvents::Transfer>() {
			dbg!(event);
		}
		if let Some(event) = result.find_first_event::<SystemEvents::KilledAccount>() {
			dbg!(event);
		}

		Ok(())
	}

	pub async fn clean() {
		let sdk = SDK::new(SDK::local_endpoint()).await.unwrap();

		// Input
		let secret_uri = SecretUri::from_str("//Eve").unwrap();
		let account = Keypair::from_uri(&secret_uri).unwrap();
		let dest = account_id_from_str("5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY").unwrap(); // Alice
		let value = SDK::one_avail() * 900_000;
		let options = Some(Options::new().nonce(Nonce::BestBlockAndTxPool));

		let tx = sdk.tx.balances.transfer_keep_alive(dest, value);
		tx.execute_wait_for_inclusion(&account, options)
			.await
			.unwrap();
	}
}

mod transfer_allow_death {
	use avail_rust::{
		error::ClientError,
		transactions::{BalancesEvents, SystemEvents},
		utils::account_id_from_str,
		Keypair, Nonce, Options, SecretUri, SDK,
	};
	use core::str::FromStr;

	pub async fn run() -> Result<(), ClientError> {
		let sdk = SDK::new(SDK::local_endpoint()).await?;

		// Input
		let secret_uri = SecretUri::from_str("//Alice").unwrap();
		let account = Keypair::from_uri(&secret_uri).unwrap();
		let dest = account_id_from_str("5HGjWAeFDfFCWPsjFQdVV2Msvz2XtMktvgocEZcCj68kUMaw")?; // Eve
		let amount = SDK::one_avail();

		let options = Some(Options::new().nonce(Nonce::BestBlockAndTxPool));
		let tx = sdk.tx.balances.transfer_allow_death(dest, amount);
		let result = tx.execute_wait_for_inclusion(&account, options).await?;

		dbg!(&result);
		if let Some(event) = result.find_first_event::<BalancesEvents::Transfer>() {
			dbg!(event);
		}
		if let Some(event) = result.find_first_event::<SystemEvents::KilledAccount>() {
			dbg!(event);
		}

		Ok(())
	}
}

mod transfer_keep_alive {
	use avail_rust::{
		error::ClientError, transactions::BalancesEvents, utils::account_id_from_str, Keypair,
		Nonce, Options, SecretUri, SDK,
	};
	use core::str::FromStr;

	pub async fn run() -> Result<(), ClientError> {
		let sdk = SDK::new(SDK::local_endpoint()).await?;

		// Input
		let secret_uri = SecretUri::from_str("//Alice").unwrap();
		let account = Keypair::from_uri(&secret_uri).unwrap();
		let dest = account_id_from_str("5HGjWAeFDfFCWPsjFQdVV2Msvz2XtMktvgocEZcCj68kUMaw")?; // Eve
		let amount = SDK::one_avail();

		let options = Some(Options::new().nonce(Nonce::BestBlockAndTxPool));
		let tx = sdk.tx.balances.transfer_keep_alive(dest, amount);
		let result = tx.execute_wait_for_inclusion(&account, options).await?;

		dbg!(&result);
		if let Some(event) = result.find_first_event::<BalancesEvents::Transfer>() {
			dbg!(event);
		}

		Ok(())
	}
}

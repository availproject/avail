use avail_rust::error::ClientError;

use crate::wait_for_new_era;

pub async fn run() -> Result<(), ClientError> {
	println!("staking_bond");
	bond::run().await?;
	println!("staking_bond_extra");
	bond_extra::run().await?;
	println!("staking_nominate");
	nominate::run().await?;
	println!("staking_chill");
	chill::run().await?;
	println!("staking_chill_other");
	chill_other::prepare().await;
	chill_other::run().await?;
	println!("staking_unbond");
	unbond::run().await?;
	println!("staking_validate");
	validate::run().await?;
	validate::clean().await;

	wait_for_new_era(None).await?;

	println!("staking_payout_stakers");
	payout_stakers::run().await?;

	Ok(())
}

mod bond {
	use avail_rust::{
		error::ClientError, Keypair, Nonce, Options, RewardDestination, SecretUri, WaitFor, SDK,
	};
	use core::str::FromStr;

	pub async fn run() -> Result<(), ClientError> {
		let sdk = SDK::new(SDK::local_endpoint()).await?;

		// Input
		let secret_uri = SecretUri::from_str("//Alice")?;
		let account = Keypair::from_uri(&secret_uri)?;
		let value = SDK::one_avail() * 100_000u128;
		let payee = RewardDestination::Staked;

		let wait_for = WaitFor::BlockInclusion;
		let options = Options::new().nonce(Nonce::BestBlockAndTxPool);
		let result = sdk
			.tx
			.staking
			.bond(value, payee, wait_for, &account, Some(options))
			.await;
		let result = result.map_err(|e| e.reason)?;

		dbg!(result);

		Ok(())
	}
}

mod bond_extra {
	use avail_rust::{error::ClientError, Keypair, Nonce, Options, SecretUri, WaitFor, SDK};
	use core::str::FromStr;

	pub async fn run() -> Result<(), ClientError> {
		let sdk = SDK::new(SDK::local_endpoint()).await?;

		// Input
		let secret_uri = SecretUri::from_str("//Alice")?;
		let account = Keypair::from_uri(&secret_uri)?;
		let max_additional = SDK::one_avail();

		let wait_for = WaitFor::BlockInclusion;
		let options = Options::new().nonce(Nonce::BestBlockAndTxPool);
		let result = sdk
			.tx
			.staking
			.bond_extra(max_additional, wait_for, &account, Some(options))
			.await;
		let result = result.map_err(|e| e.reason)?;

		dbg!(result);

		Ok(())
	}
}

mod nominate {
	use avail_rust::{
		error::ClientError, utils::account_id_from_str, Keypair, Nonce, Options, SecretUri,
		WaitFor, SDK,
	};
	use core::str::FromStr;

	pub async fn run() -> Result<(), ClientError> {
		let sdk = SDK::new(SDK::local_endpoint()).await?;

		// Input
		let secret_uri = SecretUri::from_str("//Alice")?;
		let account = Keypair::from_uri(&secret_uri)?;
		let targets = [
			account_id_from_str("5GNJqTPyNqANBkUVMN1LPPrxXnFouWXoe2wNSmmEoLctxiZY")?, // Alice Stash
		];

		let wait_for = WaitFor::BlockInclusion;
		let options = Options::new().nonce(Nonce::BestBlockAndTxPool);
		let result = sdk
			.tx
			.staking
			.nominate(&targets, wait_for, &account, Some(options))
			.await;
		let result = result.map_err(|e| e.reason)?;

		dbg!(result);

		Ok(())
	}
}

mod chill {
	use avail_rust::{error::ClientError, Keypair, Nonce, Options, SecretUri, WaitFor, SDK};
	use core::str::FromStr;

	pub async fn run() -> Result<(), ClientError> {
		let sdk = SDK::new(SDK::local_endpoint()).await?;

		// Input
		let secret_uri = SecretUri::from_str("//Alice")?;
		let account = Keypair::from_uri(&secret_uri)?;

		let wait_for = WaitFor::BlockInclusion;
		let options = Options::new().nonce(Nonce::BestBlockAndTxPool);
		let result = sdk
			.tx
			.staking
			.chill(wait_for, &account, Some(options))
			.await;
		let result = result.map_err(|e| e.reason)?;

		dbg!(result);

		Ok(())
	}
}

mod chill_other {
	use avail_rust::{
		error::ClientError, utils::account_id_from_str, Keypair, Nonce, Options, SecretUri,
		WaitFor, SDK,
	};
	use core::str::FromStr;

	pub async fn prepare() {
		let sdk = SDK::new(SDK::local_endpoint()).await.unwrap();

		// Input
		let secret_uri = SecretUri::from_str("//Alice").unwrap();
		let account = Keypair::from_uri(&secret_uri).unwrap();
		let targets = [
			account_id_from_str("5GNJqTPyNqANBkUVMN1LPPrxXnFouWXoe2wNSmmEoLctxiZY").unwrap(), // Alice Stash
		];

		let options = Options::new().nonce(Nonce::BestBlockAndTxPool);
		_ = sdk
			.tx
			.staking
			.nominate(&targets, WaitFor::BlockInclusion, &account, Some(options))
			.await
			.unwrap();
	}

	pub async fn run() -> Result<(), ClientError> {
		let sdk = SDK::new(SDK::local_endpoint()).await?;

		// Input
		let secret_uri = SecretUri::from_str("//Alice")?;
		let account = Keypair::from_uri(&secret_uri)?;
		let stash = account_id_from_str("5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY")?;

		let wait_for = WaitFor::BlockInclusion;
		let options = Options::new().nonce(Nonce::BestBlockAndTxPool);
		let result = sdk
			.tx
			.staking
			.chill_other(stash, wait_for, &account, Some(options))
			.await;
		let result = result.map_err(|e| e.reason)?;

		dbg!(result);

		Ok(())
	}
}

mod unbond {
	use avail_rust::{error::ClientError, Keypair, Nonce, Options, SecretUri, WaitFor, SDK};
	use core::str::FromStr;

	pub async fn run() -> Result<(), ClientError> {
		let sdk = SDK::new(SDK::local_endpoint()).await?;

		// Input
		let secret_uri = SecretUri::from_str("//Alice")?;
		let account = Keypair::from_uri(&secret_uri)?;
		let value = SDK::one_avail();

		let wait_for = WaitFor::BlockInclusion;
		let options = Options::new().nonce(Nonce::BestBlockAndTxPool);
		let result = sdk
			.tx
			.staking
			.unbond(value, wait_for, &account, Some(options))
			.await;
		let result = result.map_err(|e| e.reason)?;

		dbg!(result);

		Ok(())
	}
}

mod validate {
	use avail_rust::{error::ClientError, Keypair, Nonce, Options, SecretUri, WaitFor, SDK};
	use core::str::FromStr;

	pub async fn run() -> Result<(), ClientError> {
		let sdk = SDK::new(SDK::local_endpoint()).await?;

		// Input
		let secret_uri = SecretUri::from_str("//Alice")?;
		let account = Keypair::from_uri(&secret_uri)?;
		let commission = 100;
		let blocked = false;

		let wait_for = WaitFor::BlockInclusion;
		let options = Options::new().nonce(Nonce::BestBlockAndTxPool);
		let result = sdk
			.tx
			.staking
			.validate(commission, blocked, wait_for, &account, Some(options))
			.await;
		let result = result.map_err(|e| e.reason)?;

		dbg!(result);

		Ok(())
	}

	pub async fn clean() {
		let sdk = SDK::new(SDK::local_endpoint()).await.unwrap();

		// Input
		let secret_uri = SecretUri::from_str("//Alice").unwrap();
		let account = Keypair::from_uri(&secret_uri).unwrap();

		let wait_for = WaitFor::BlockInclusion;
		let options = Options::new().nonce(Nonce::BestBlockAndTxPool);
		_ = sdk
			.tx
			.staking
			.chill(wait_for, &account, Some(options))
			.await
			.unwrap();
	}
}

mod payout_stakers {
	use avail_rust::{
		avail, error::ClientError, utils::account_id_from_str, Keypair, Nonce, Options, SecretUri,
		WaitFor, SDK,
	};
	use core::str::FromStr;

	pub async fn run() -> Result<(), ClientError> {
		let sdk = SDK::new(SDK::local_endpoint()).await?;

		// Input
		let secret_uri = SecretUri::from_str("//Alice")?;
		let account = Keypair::from_uri(&secret_uri)?;
		let validator_stash =
			account_id_from_str("5GNJqTPyNqANBkUVMN1LPPrxXnFouWXoe2wNSmmEoLctxiZY")?; // Alice Stash
		let era_storage = avail::storage().staking().active_era();
		let storage = sdk.online_client.storage().at_latest().await?;
		let era = storage.fetch(&era_storage).await?;
		let mut era = era.map(|e| e.index).unwrap_or(0);
		if era > 0 {
			era = era - 1
		};

		let wait_for = WaitFor::BlockInclusion;
		let options = Options::new().nonce(Nonce::BestBlockAndTxPool);
		let result = sdk
			.tx
			.staking
			.payout_stakers(validator_stash, era, wait_for, &account, Some(options))
			.await;
		let result = result.map_err(|e| e.reason)?;

		dbg!(result);

		Ok(())
	}
}

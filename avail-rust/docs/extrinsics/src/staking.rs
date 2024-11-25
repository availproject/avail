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
		error::ClientError, transactions::StakingEvents, Keypair, Nonce, Options,
		RewardDestination, SecretUri, SDK,
	};
	use core::str::FromStr;

	pub async fn run() -> Result<(), ClientError> {
		let sdk = SDK::new(SDK::local_endpoint()).await?;

		// Input
		let secret_uri = SecretUri::from_str("//Alice")?;
		let account = Keypair::from_uri(&secret_uri)?;
		let value = SDK::one_avail() * 100_000u128;
		let payee = RewardDestination::Staked;

		let options = Some(Options::new().nonce(Nonce::BestBlockAndTxPool));
		let tx = sdk.tx.staking.bond(value, payee);
		let result = tx.execute_wait_for_inclusion(&account, options).await?;

		result.print_debug();
		if let Some(event) = result.find_first_event::<StakingEvents::Bonded>() {
			dbg!(event);
		}

		Ok(())
	}
}

mod bond_extra {
	use avail_rust::{
		error::ClientError, transactions::StakingEvents, Keypair, Nonce, Options, SecretUri, SDK,
	};
	use core::str::FromStr;

	pub async fn run() -> Result<(), ClientError> {
		let sdk = SDK::new(SDK::local_endpoint()).await?;

		// Input
		let secret_uri = SecretUri::from_str("//Alice")?;
		let account = Keypair::from_uri(&secret_uri)?;
		let max_additional = SDK::one_avail();

		let options = Some(Options::new().nonce(Nonce::BestBlockAndTxPool));
		let tx = sdk.tx.staking.bond_extra(max_additional);
		let result = tx.execute_wait_for_inclusion(&account, options).await?;

		result.print_debug();
		if let Some(event) = result.find_first_event::<StakingEvents::Bonded>() {
			dbg!(event);
		}

		Ok(())
	}
}

mod nominate {
	use avail_rust::{
		error::ClientError, transactions::StakingCalls, utils::account_id_from_str, Keypair, Nonce,
		Options, SecretUri, SDK,
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

		let options = Some(Options::new().nonce(Nonce::BestBlockAndTxPool));
		let tx = sdk.tx.staking.nominate(&targets);
		let result = tx.execute_wait_for_inclusion(&account, options).await?;

		result.print_debug();
		if let Some(data) = result
			.get_data::<StakingCalls::Nominate>(&sdk.online_client)
			.await
		{
			dbg!(data);
		}

		Ok(())
	}
}

mod chill {
	use avail_rust::{
		error::ClientError, transactions::StakingEvents, Keypair, Nonce, Options, SecretUri, SDK,
	};
	use core::str::FromStr;

	pub async fn run() -> Result<(), ClientError> {
		let sdk = SDK::new(SDK::local_endpoint()).await?;

		// Input
		let secret_uri = SecretUri::from_str("//Alice")?;
		let account = Keypair::from_uri(&secret_uri)?;

		let options = Some(Options::new().nonce(Nonce::BestBlockAndTxPool));
		let tx = sdk.tx.staking.chill();
		let result = tx.execute_wait_for_inclusion(&account, options).await?;

		result.print_debug();
		if let Some(event) = result.find_first_event::<StakingEvents::Chilled>() {
			dbg!(event);
		}

		Ok(())
	}
}

mod chill_other {
	use avail_rust::{
		error::ClientError, transactions::StakingEvents, utils::account_id_from_str, Keypair,
		Nonce, Options, SecretUri, SDK,
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

		let options = Some(Options::new().nonce(Nonce::BestBlockAndTxPool));
		let tx = sdk.tx.staking.nominate(&targets);
		tx.execute_wait_for_inclusion(&account, options)
			.await
			.unwrap();
	}

	pub async fn run() -> Result<(), ClientError> {
		let sdk = SDK::new(SDK::local_endpoint()).await?;

		// Input
		let secret_uri = SecretUri::from_str("//Alice")?;
		let account = Keypair::from_uri(&secret_uri)?;
		let stash = account_id_from_str("5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY")?;

		let options = Some(Options::new().nonce(Nonce::BestBlockAndTxPool));
		let tx = sdk.tx.staking.chill_other(stash);
		let result = tx.execute_wait_for_inclusion(&account, options).await?;

		result.print_debug();
		if let Some(event) = result.find_first_event::<StakingEvents::Chilled>() {
			dbg!(event);
		}

		Ok(())
	}
}

mod unbond {
	use avail_rust::{
		error::ClientError, transactions::StakingEvents, Keypair, Nonce, Options, SecretUri, SDK,
	};
	use core::str::FromStr;

	pub async fn run() -> Result<(), ClientError> {
		let sdk = SDK::new(SDK::local_endpoint()).await?;

		// Input
		let secret_uri = SecretUri::from_str("//Alice")?;
		let account = Keypair::from_uri(&secret_uri)?;
		let value = SDK::one_avail();

		let options = Some(Options::new().nonce(Nonce::BestBlockAndTxPool));
		let tx = sdk.tx.staking.unbond(value);
		let result = tx.execute_wait_for_inclusion(&account, options).await?;

		result.print_debug();
		if let Some(event) = result.find_first_event::<StakingEvents::Unbonded>() {
			dbg!(event);
		}

		Ok(())
	}
}

mod validate {
	use avail_rust::{
		error::ClientError,
		transactions::{staking::Commission, StakingEvents},
		Keypair, Nonce, Options, SecretUri, SDK,
	};
	use core::str::FromStr;

	pub async fn run() -> Result<(), ClientError> {
		let sdk = SDK::new(SDK::local_endpoint()).await?;

		// Input
		let secret_uri = SecretUri::from_str("//Alice")?;
		let account = Keypair::from_uri(&secret_uri)?;
		let commission = Commission::new(50).unwrap();
		let blocked = false;

		let options = Some(Options::new().nonce(Nonce::BestBlockAndTxPool));
		let tx = sdk.tx.staking.validate(commission, blocked);
		let result = tx.execute_wait_for_inclusion(&account, options).await?;

		result.print_debug();
		if let Some(event) = result.find_first_event::<StakingEvents::ValidatorPrefsSet>() {
			dbg!(event);
		}

		Ok(())
	}

	pub async fn clean() {
		let sdk = SDK::new(SDK::local_endpoint()).await.unwrap();

		// Input
		let secret_uri = SecretUri::from_str("//Alice").unwrap();
		let account = Keypair::from_uri(&secret_uri).unwrap();

		let options = Some(Options::new().nonce(Nonce::BestBlockAndTxPool));
		let tx = sdk.tx.staking.chill();
		tx.execute_wait_for_inclusion(&account, options)
			.await
			.unwrap();
	}
}

mod payout_stakers {
	use avail_rust::{
		avail, error::ClientError, utils::account_id_from_str, Keypair, Nonce, Options, SecretUri,
		SDK,
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

		let options = Some(Options::new().nonce(Nonce::BestBlockAndTxPool));
		let tx = sdk.tx.staking.payout_stakers(validator_stash, era);
		let result = tx.execute_wait_for_inclusion(&account, options).await?;

		result.print_debug();

		Ok(())
	}
}

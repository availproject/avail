mod balances;
mod da;
mod nomination_pools;
mod session;
mod staking;

use std::time::Duration;

use avail_rust::error::ClientError;
use avail_rust::{avail, SDK};

#[tokio::main]
async fn main() -> Result<(), ClientError> {
	balances::run().await?;
	da::run().await?;
	session::run().await?;
	staking::run().await?;
	nomination_pools::run().await?;

	Ok(())
}

pub async fn wait_for_new_era(mut target_era: Option<u32>) -> Result<(), ClientError> {
	println!("Waiting for new era...");

	let sdk = SDK::new(SDK::local_endpoint()).await?;

	let era_storage = avail::storage().staking().active_era();
	loop {
		let storage = sdk.online_client.storage().at_latest().await?;
		let era = storage.fetch(&era_storage).await?;
		let era = era.map(|e| e.index).unwrap_or(0);
		if target_era.is_none() {
			target_era = Some(era + 1)
		}

		if target_era == Some(era) {
			break;
		};
		tokio::time::sleep(Duration::from_secs(3)).await
	}

	println!("Waiting for new era... Done");

	Ok(())
}

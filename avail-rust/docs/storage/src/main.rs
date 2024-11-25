use avail_rust::error::ClientError;

mod da;
mod staking;
mod system;

#[tokio::main]
async fn main() -> Result<(), ClientError> {
	println!("da::app_keys()");
	da::app_keys().await?;
	println!("da::app_keys_iter()");
	da::app_keys_iter().await?;
	println!("da::next_app_id()");
	da::next_app_id().await?;

	println!("staking::active_era()");
	staking::active_era().await?;
	println!("staking::bonded()");
	staking::bonded().await?;
	println!("system::bonded_iter()");
	staking::bonded_iter().await?;
	println!("system::account_iter()");
	system::account_iter().await?;

	Ok(())
}

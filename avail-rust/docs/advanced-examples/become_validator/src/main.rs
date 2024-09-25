use avail_rust::{avail, Keypair, RewardDestination, SecretUri, WaitFor, SDK};
use core::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), String> {
	let sdk = SDK::new("ws://127.0.0.1:9944").await.unwrap();

	let secret_uri = SecretUri::from_str("//Alice").unwrap();
	let account = Keypair::from_uri(&secret_uri).unwrap();

	// Bond min_validator_bond or 1 AVAIL token
	let storage = sdk.api.storage().at_latest().await.unwrap();
	let min_validator_bond = storage
		.fetch(&avail::storage().staking().min_validator_bond())
		.await
		.unwrap()
		.unwrap_or_else(|| 1_000_000_000_000_000_000u128);

	let wait_for = WaitFor::BlockFinalization;
	let payee = RewardDestination::Staked;

	// Bond
	sdk.tx
		.staking
		.bond(min_validator_bond, payee, wait_for, &account, None)
		.await
		.unwrap();

	// Generate Session Keys
	let keys = sdk.rpc.author.rotate_keys().await.unwrap();
	let keys = sdk.util.deconstruct_session_keys(keys).unwrap();

	// Set Keys
	sdk.tx
		.session
		.set_keys(keys, wait_for, &account, None)
		.await
		.unwrap();

	// Validate
	sdk.tx
		.staking
		.validate(0, false, wait_for, &account, None)
		.await
		.unwrap();

	Ok(())
}

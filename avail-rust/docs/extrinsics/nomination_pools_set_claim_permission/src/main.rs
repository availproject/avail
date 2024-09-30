use avail_rust::{
	nomination_pools_types::Permission, Keypair, Nonce, Options, SecretUri, WaitFor, SDK,
};
use core::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), String> {
	let sdk = SDK::new("ws://127.0.0.1:9944").await.unwrap();

	// Input
	let secret_uri = SecretUri::from_str("//Alice").unwrap();
	let account = Keypair::from_uri(&secret_uri).unwrap();
	let permission = Permission::PermissionlessAll;

	let wait_for = WaitFor::BlockInclusion;
	let options = Options::new().nonce(Nonce::BestBlockAndTxPool);
	let result = sdk
		.tx
		.nomination_pools
		.set_claim_permission(permission, wait_for, &account, Some(options))
		.await?;

	dbg!(result);

	Ok(())
}

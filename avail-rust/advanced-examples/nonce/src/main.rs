use avail_rust::{avail, AccountId, AvailExtrinsicParamsBuilder, Keypair, SecretUri, SDK};
use core::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), String> {
	let sdk = SDK::new("ws://127.0.0.1:9944").await.unwrap();

	let secret_uri = SecretUri::from_str("//Alice").unwrap();
	let account = Keypair::from_uri(&secret_uri).unwrap();
	let account_id = account.public_key().to_account_id();
	let mut nonce = sdk.api.tx().account_nonce(&account_id).await.unwrap();

	let dest: avail_rust::subxt::utils::AccountId32 =
		match AccountId::from_str("5HGjWAeFDfFCWPsjFQdVV2Msvz2XtMktvgocEZcCj68kUMaw") {
			Ok(dest) => dest,
			Err(error) => return Err(std::format!("{:?}", error)),
		};
	let value = 1_000_000_000_000_000_000u128; // 1 Avail
	let call = avail::tx()
		.balances()
		.transfer_keep_alive(dest.into(), value);

	for _ in 0..10 {
		let params = AvailExtrinsicParamsBuilder::new().nonce(nonce).build();
		let result = sdk.api.tx().sign_and_submit(&call, &account, params).await;
		result.map_err(|e| e.to_string())?;
		nonce += 1;
	}

	Ok(())
}

use avail_rust::{avail, AvailExtrinsicParamsBuilder, Data, Keypair, SecretUri, SDK};
use core::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), String> {
	let sdk = SDK::new("ws://127.0.0.1:9944").await.unwrap();

	let secret_uri = SecretUri::from_str("//Alice").unwrap();
	let account = Keypair::from_uri(&secret_uri).unwrap();
	let account_id = account.public_key().to_account_id();
	let mut nonce = sdk.api.tx().account_nonce(&account_id).await.unwrap();
	let mut app_id = 0;

	let data = String::from("My Awesome Data").as_bytes().to_vec();
	let data = Data { 0: data };

	let call = avail::tx().data_availability().submit_data(data);

	for _ in 0..10 {
		let params = AvailExtrinsicParamsBuilder::new()
			.nonce(nonce)
			.app_id(app_id)
			.build();
		let result = sdk.api.tx().sign_and_submit(&call, &account, params).await;
		result.map_err(|e| e.to_string())?;
		app_id += 1;
		nonce += 1;
	}

	Ok(())
}

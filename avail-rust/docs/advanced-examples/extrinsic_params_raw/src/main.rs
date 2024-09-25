use avail_rust::{avail, AvailExtrinsicParamsBuilder, Data, Key, Keypair, SecretUri, SDK};
use core::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), String> {
	let sdk = SDK::new("ws://127.0.0.1:9944").await.unwrap();

	let secret_uri = SecretUri::from_str("//Alice").unwrap();
	let account = Keypair::from_uri(&secret_uri).unwrap();
	let account_id = account.public_key().to_account_id();

	let key = String::from("ThisIsMyKey").as_bytes().to_vec();
	let key = Key { 0: key };
	let result = sdk
		.tx
		.data_availability
		.create_application_key(key, avail_rust::WaitFor::BlockFinalization, &account, None)
		.await?;

	let block = sdk.api.blocks().at_latest().await.unwrap();
	let app_id = result.event.id;
	let mut nonce = sdk.api.tx().account_nonce(&account_id).await.unwrap();

	let data = String::from("My Awesome Data").as_bytes().to_vec();
	let data = Data { 0: data };
	let call = avail::tx().data_availability().submit_data(data);

	for _ in 0..10 {
		let options = AvailExtrinsicParamsBuilder::new()
			.nonce(nonce)
			.app_id(app_id.0)
			.tip(1_000_000_000_000_000_000u128)
			.mortal(block.header(), 400)
			.build();
		let result = sdk.api.tx().sign_and_submit(&call, &account, options).await;
		result.map_err(|e| e.to_string())?;
		nonce += 1;
	}

	Ok(())
}

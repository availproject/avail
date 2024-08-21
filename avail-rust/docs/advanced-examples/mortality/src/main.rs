use avail_rust::{avail, AvailExtrinsicParamsBuilder, Data, Keypair, SecretUri, SDK};
use core::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), String> {
	let sdk = SDK::new("ws://127.0.0.1:9944").await.unwrap();

	let secret_uri = SecretUri::from_str("//Alice").unwrap();
	let account = Keypair::from_uri(&secret_uri).unwrap();

	let data = String::from("My Awesome Data").as_bytes().to_vec();
	let data = Data { 0: data };

	let call = avail::tx().data_availability().submit_data(data);

	let block = sdk.api.blocks().at_latest().await.unwrap();
	let block_header = block.header();

	let params = AvailExtrinsicParamsBuilder::new()
		.mortal(block_header, 420)
		.build();
	let result = sdk.api.tx().sign_and_submit(&call, &account, params).await;
	result.map_err(|e| e.to_string())?;

	Ok(())
}

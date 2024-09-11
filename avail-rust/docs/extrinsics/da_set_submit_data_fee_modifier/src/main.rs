use avail_rust::{DispatchFeeModifier, Keypair, Nonce, Options, SecretUri, WaitFor, SDK};
use core::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), String> {
	let sdk = SDK::new("ws://127.0.0.1:9944").await.unwrap();

	// Input
	let secret_uri = SecretUri::from_str("//Alice").unwrap();
	let account = Keypair::from_uri(&secret_uri).unwrap();

	let modifier = DispatchFeeModifier {
		weight_maximum_fee: None,
		weight_fee_divider: Some(2),
		weight_fee_multiplier: None,
	};

	let wait_for = WaitFor::BlockInclusion;
	let options = Options::new().nonce(Nonce::BestBlockAndTxPool);
	let result = sdk
		.tx
		.data_availability
		.set_submit_data_fee_modifier(modifier, wait_for, &account, Some(options))
		.await?;

	dbg!(result);

	Ok(())
}

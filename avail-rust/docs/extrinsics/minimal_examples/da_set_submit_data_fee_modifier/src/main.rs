use avail_rust::{DispatchFeeModifier, Keypair, SecretUri, WaitFor, SDK};
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

	let result = sdk
		.tx
		.data_availability
		.set_submit_data_fee_modifier(modifier, WaitFor::BlockInclusion, &account)
		.await?;

	println!(
		"WeightMaximumFee={:?}, WeightFeeMultiplier={:?}, WeightFeeDivider={:?}",
		result.event.value.weight_maximum_fee,
		result.event.value.weight_fee_multiplier,
		result.event.value.weight_fee_divider
	);
	println!(
		"TxHash={:?}, BlockHash={:?}",
		result.tx_hash, result.block_hash
	);

	Ok(())
}

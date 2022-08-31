use avail_subxt::*;
use sp_keyring::sr25519::sr25519;
use subxt::{
	ext::{sp_core::Pair, sp_runtime::MultiAddress},
	tx::PairSigner,
	OnlineClient,
};

/// This example demonstrates using mnemonic seed for generating signer pairs. It creates Alice and Bob
/// from seeds, but could also be used for an arbitrary account.

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	let signer_a: PairSigner<AvailConfig, sr25519::Pair> = PairSigner::new(
		Pair::from_string_with_seed(
			"bottom drive obey lake curtain smoke basket hold race lonely fit walk//Alice",
			None,
		)
		.unwrap()
		.0,
	);
	let signer_b: PairSigner<AvailConfig, sr25519::Pair> = PairSigner::new(
		Pair::from_string_with_seed(
			"bottom drive obey lake curtain smoke basket hold race lonely fit walk//Bob",
			None,
		)
		.unwrap()
		.0,
	);
	let api = OnlineClient::<AvailConfig>::new().await?;
	let balance_transfer = avail::tx()
		.balances()
		.transfer(MultiAddress::Id(signer_b.account_id().clone()), 2);
	let _ = api
		.tx()
		.sign_and_submit_then_watch(
			&balance_transfer,
			&signer_a,
			AvailExtrinsicParams::new_with_app_id(0),
		)
		.await
		.unwrap()
		.wait_for_finalized_success()
		.await
		.unwrap();

	Ok(())
}

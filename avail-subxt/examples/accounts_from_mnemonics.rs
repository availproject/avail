use anyhow::Result;
use avail_subxt::{api, build_client, primitives::AvailExtrinsicParams, AvailConfig, Opts};
use sp_keyring::sr25519::sr25519;
use structopt::StructOpt;
use subxt::{
	ext::{sp_core::Pair, sp_runtime::MultiAddress},
	tx::PairSigner,
};

/// This example demonstrates using mnemonic seed for generating signer pairs. It creates Alice and Bob
/// from seeds, but could also be used for an arbitrary account.
const ALICE_SEED: &str =
	"bottom drive obey lake curtain smoke basket hold race lonely fit walk//Alice";
const BOB_SEED: &str = "bottom drive obey lake curtain smoke basket hold race lonely fit walk//Bob";

#[async_std::main]
async fn main() -> Result<()> {
	let args = Opts::from_args();
	let client = build_client(args.ws).await?;

	// Accounts
	let pair_a = Pair::from_string_with_seed(ALICE_SEED, None).unwrap();
	let signer_a = PairSigner::<AvailConfig, sr25519::Pair>::new(pair_a.0);
	let pair_b = Pair::from_string_with_seed(BOB_SEED, None).unwrap();
	let signer_b = PairSigner::<AvailConfig, sr25519::Pair>::new(pair_b.0);

	// Transfer and wait finalized
	let balance_transfer = api::tx()
		.balances()
		.transfer(MultiAddress::Id(signer_b.account_id().clone()), 2);
	let _ = client
		.tx()
		.sign_and_submit_then_watch(
			&balance_transfer,
			&signer_a,
			AvailExtrinsicParams::new_with_app_id(0.into()),
		)
		.await?
		.wait_for_finalized_success()
		.await?;

	Ok(())
}

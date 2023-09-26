use anyhow::Result;
use avail_subxt::{api, build_client, primitives::AvailExtrinsicParams, Opts};
use structopt::StructOpt;
use subxt::utils::MultiAddress;
use subxt_signer::{bip39, sr25519::Keypair, DeriveJunction};

/// This example demonstrates using mnemonic seed for generating signer pairs. It creates Alice and Bob
/// from seeds, but could also be used for an arbitrary account.
const DEV_SEED: &str = "bottom drive obey lake curtain smoke basket hold race lonely fit walk";

#[async_std::main]
async fn main() -> Result<()> {
	let args = Opts::from_args();
	let client = build_client(args.ws, args.validate_codegen).await?;

	// Accounts
	let phrase = bip39::Mnemonic::parse(DEV_SEED).unwrap();
	let keypair = Keypair::from_phrase(&phrase, None).unwrap();
	let signer_a = keypair.derive([DeriveJunction::hard("Alice")]);
	let signer_b = keypair.derive([DeriveJunction::hard("Bob")]);

	// Transfer and wait finalized
	let balance_transfer = api::tx()
		.balances()
		.transfer(MultiAddress::Id(signer_b.public_key().into()), 2);
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

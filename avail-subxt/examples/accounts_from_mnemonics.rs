use anyhow::Result;
use avail_subxt::{api, build_client, tx_send_in_finalized, AvailConfig, Opts};
use structopt::StructOpt;
use subxt::{
	ext::sp_core::{sr25519::Pair, Pair as _},
	tx::PairSigner,
	utils::MultiAddress,
};

/// This example demonstrates using mnemonic seed for generating signer pairs. It creates Alice and Bob
/// from seeds, but could also be used for an arbitrary account.
const ALICE_SEED: &str =
	"bottom drive obey lake curtain smoke basket hold race lonely fit walk//Alice";
const BOB_SEED: &str = "bottom drive obey lake curtain smoke basket hold race lonely fit walk//Bob";

#[async_std::main]
async fn main() -> Result<()> {
	let args = Opts::from_args();
	let (client, _) = build_client(args.ws, args.validate_codegen).await?;

	// Accounts
	let alice = Pair::from_string_with_seed(ALICE_SEED, None).unwrap();
	let alice = PairSigner::<AvailConfig, Pair>::new(alice.0);
	let bob = Pair::from_string_with_seed(BOB_SEED, None).unwrap();
	let bob = PairSigner::<AvailConfig, Pair>::new(bob.0);

	// Transfer and wait finalized
	let call = api::tx()
		.balances()
		.transfer(MultiAddress::Id(bob.account_id().clone()), 2);
	let _ = tx_send_in_finalized!(&client, &call, &alice).await?;

	Ok(())
}

use anyhow::Result;
use avail_subxt::{
	api, api::runtime_types::sp_core::bounded::bounded_vec::BoundedVec, build_client,
	primitives::AvailExtrinsicParams, AvailConfig, Opts,
};
use sp_core::crypto::Pair as _;
use sp_keyring::sr25519::sr25519::{self, Pair};
use structopt::StructOpt;
use subxt::{tx::PairSigner, utils::MultiAddress};

/// This example demonstrates creation of application key.
const ALICE_SEED: &str =
	"bottom drive obey lake curtain smoke basket hold race lonely fit walk//Alice";

#[async_std::main]
async fn main() -> Result<()> {
	let args = Opts::from_args();
	let client = build_client(args.ws).await?;

	// Account
	let pair_a = Pair::from_string_with_seed(ALICE_SEED, None).unwrap();
	let signer_a = PairSigner::<AvailConfig, sr25519::Pair>::new(pair_a.0);

	// app key to submit
	let app_id = b"1".to_vec();
	let create_application_key = api::tx()
		.data_availability()
		.create_application_key(BoundedVec(app_id));

	let params = AvailExtrinsicParams::default();

	let res = client
		.tx()
		.sign_and_submit_then_watch(&create_application_key, &signer_a, params)
		.await?
		.wait_for_finalized_success()
		.await?;

	println!("Application key created, block hash: {}", res.block_hash());
	Ok(())
}

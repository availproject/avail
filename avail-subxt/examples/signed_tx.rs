use anyhow::Result;
use avail_subxt::{
	api::{
		self,
		runtime_types::{
			da_control::pallet::Call as DaCall, sp_core::bounded::bounded_vec::BoundedVec,
		},
	},
	avail::AppUncheckedExtrinsic,
	build_client,
	primitives::AvailExtrinsicParams,
	Call, Opts,AvailConfig
};
use sp_keyring::AccountKeyring;
use structopt::StructOpt;
use subxt::tx::PairSigner;
use sp_core::{Pair, sr25519};
/// This example submits an Avail data extrinsic, then retrieves the block containing the
/// extrinsic and matches the data.
#[async_std::main]
async fn main() -> Result<()> {
	let args = Opts::from_args();
	let client = build_client("wss://kate.avail.tools:443/ws", args.validate_codegen).await?;
	//change bounce organ nurse genuine bargain issue blush near lumber manage reject

	let mnemonic_phrase = "change bounce organ nurse genuine bargain issue blush near lumber manage reject";
	let pair = sp_core::sr25519::Pair::from_phrase(mnemonic_phrase, None).unwrap();
	let signer = PairSigner::<AvailConfig, sp_core::sr25519::Pair>::new(pair.0);
	
	// let signer = PairSigner::new(AccountKeyring::Alice.pair());
	let example_data = b"exampleP_data".to_vec();
	let data_transfer = api::tx()
		.data_availability()
		.submit_data(BoundedVec(example_data.clone()));
	let extrinsic_params = AvailExtrinsicParams::new_with_app_id(1.into());

	println!("Sending example data...");
	let h = client
		.tx()
		.create_signed(&data_transfer, &signer, extrinsic_params)
		.await?;
	let enc = h.into_encoded();
	println!("Encoded extrinsic: {:?}", enc);
	let hex_enc = hex::encode(&enc);
	// h.submit_and_watch()
	// 	.await?;
	println!("Encoded extrinsic: {:?}", hex_enc);

	Ok(())
}

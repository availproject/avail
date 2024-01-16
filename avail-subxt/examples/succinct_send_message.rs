use anyhow::Result;
use avail_subxt::{
	api,
	api::runtime_types::{
		avail_core::data_proof_v2::MessageType, bounded_collections::bounded_vec::BoundedVec,
	},
	build_client, tx_send_in_block, AvailConfig, Opts,
};
use hex_literal::hex;
use sp_core::{crypto::Pair as _, H256};
use sp_keyring::sr25519::sr25519::{self, Pair};
use structopt::StructOpt;
use subxt::tx::PairSigner;

/// This example demonstrates submission of arbitrary & token message on succinct bridge.
const ALICE_SEED: &str =
	"bottom drive obey lake curtain smoke basket hold race lonely fit walk//Alice";

#[async_std::main]
async fn main() -> Result<()> {
	let args = Opts::from_args();
	let (client, _) = build_client(args.ws, args.validate_codegen).await?;

	// Account
	let alice = Pair::from_string_with_seed(ALICE_SEED, None).unwrap();
	let signer = PairSigner::<AvailConfig, sr25519::Pair>::new(alice.0);

	let to = H256(hex!(
		"af44af6890508b3b7f6910d4a4570a0d524769a23ce340b2c7400e140ad168ab"
	));

	// Submit Arbitrary Message
	let call = api::tx().succinct().send_message(
		MessageType::ArbitraryMessage,
		to,
		2,
		None,
		None,
		Some(BoundedVec(b"some_data".to_vec())),
	);
	let block = tx_send_in_block!(&client, &call, &signer).block_hash();
	println!("Succinct bridge arbitrary message submitted to block hash: {block:?}");

	// Submit Token Message
	let value = Some(1_000_000_000_000_000_000u128);
	let asset_id = Some(H256(hex!(
		"4554480000000000000000000000000000000000000000000000000000000000"
	)));
	let call =
		api::tx()
			.succinct()
			.send_message(MessageType::FungibleToken, to, 2, value, asset_id, None);
	let block = tx_send_in_block!(&client, &call, &signer).block_hash();
	println!("Succinct bridge token message submitted to block hash: {block:?}");

	Ok(())
}

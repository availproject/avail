use avail_subxt::{
	api::{self, runtime_types::avail_core::data_proof::message::Message},
	tx, AvailClient, BoundedVec, Opts,
};
use hex_literal::hex;
use structopt::StructOpt;
use subxt::{utils::H256, Error};
use subxt_signer::sr25519::dev;

#[async_std::main]
async fn main() -> Result<(), Error> {
	let args = Opts::from_args();
	let client = AvailClient::new(args.ws).await?;

	// Account
	let alice = dev::alice();
	let alice_id = alice.public_key().into();
	let nonce = client.tx().account_nonce(&alice_id).await?;
	let to = hex!("af44af6890508b3b7f6910d4a4570a0d524769a23ce340b2c7400e140ad168ab");

	// Submit Arbitrary Message
	let amount = 1_000_000_000_000_000_000u128;
	let asset_id = H256(hex!(
		"4554480000000000000000000000000000000000000000000000000000000000"
	));
	let m1 = Message::ArbitraryMessage(BoundedVec(b"some_data".to_vec()));
	let m2 = Message::FungibleToken { asset_id, amount };

	let mut in_block_futures = vec![];
	for (idx, (message, desc)) in [(m1, "arbitrary"), (m2, "fungible")]
		.into_iter()
		.enumerate()
	{
		let call = api::tx().vector().send_message(message, to.into(), 2);
		let progress = tx::send_with_nonce(&client, &call, &alice, 0, nonce + idx as u64).await?;
		let block_fut = tx::then_in_block(progress);
		in_block_futures.push((block_fut, desc));
	}

	for (in_block_fut, desc) in in_block_futures.into_iter() {
		let hash = in_block_fut.await?.block_hash();
		println!("Vector bridge message {desc} submitted to block hash: {hash:?}");
	}

	Ok(())
}

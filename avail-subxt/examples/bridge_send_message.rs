use avail_subxt::{
	api::{self, runtime_types::avail_core::data_proof::message::Message},
	tx, AvailClient, BoundedVec, Opts,
};
use futures::stream::{FuturesOrdered, TryStreamExt as _};
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
	let nonce = tx::nonce(&client, &alice).await?;
	let to = hex!("af44af6890508b3b7f6910d4a4570a0d524769a23ce340b2c7400e140ad168ab");

	// Submit Arbitrary Message
	let amount = 1_000_000_000_000_000_000u128;
	let asset_id: H256 =
		hex!("4554480000000000000000000000000000000000000000000000000000000000").into();
	let m1 = Message::ArbitraryMessage(BoundedVec(b"some_data".to_vec()));
	let m2 = Message::FungibleToken { asset_id, amount };

	let descriptions = ["arbitrary", "fungible"];
	let calls = [m1, m2]
		.into_iter()
		.map(|message| api::tx().vector().send_message(message, to.into(), 2))
		.collect::<Vec<_>>();

	let txs = calls
		.iter()
		.enumerate()
		.map(|(idx, call)| tx::send_with_nonce(&client, call, &alice, 0, nonce + idx as u64))
		.collect::<FuturesOrdered<_>>()
		.try_collect::<Vec<_>>()
		.await?;

	let in_block_txs = txs
		.into_iter()
		.map(tx::in_finalized)
		.collect::<FuturesOrdered<_>>()
		.try_collect::<Vec<_>>()
		.await?;

	for (in_block, desc) in in_block_txs.iter().zip(descriptions.into_iter()) {
		let hash = in_block.block_hash();
		println!("Vector bridge message {desc} submitted to block hash: {hash:?}");
	}

	Ok(())
}

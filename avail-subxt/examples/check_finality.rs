use avail_subxt::{
	api::{self},
	build_client,
	primitives::Header,
};
use codec::{Decode, Encode};
use serde::{de::Error, Deserialize};
use sp_core::{
	blake2_256, bytes,
	ed25519::{self, Public as EdPublic, Signature},
	Pair, H256,
};
use subxt::rpc_params;
use tokio::sync::mpsc::unbounded_channel;

#[derive(Debug, Encode)]
pub enum SignerMessage {
	DummyMessage(u32),
	PrecommitMessage(Precommit),
}

#[derive(Clone, Debug, Decode, Encode, Deserialize)]
pub struct Precommit {
	pub target_hash: H256,
	/// The target block's number
	pub target_number: u32,
}

#[derive(Clone, Debug, Decode, Deserialize)]
pub struct SignedPrecommit {
	pub precommit: Precommit,
	/// The signature on the message.
	pub signature: Signature,
	/// The Id of the signer.
	pub id: EdPublic,
}
#[derive(Clone, Debug, Decode, Deserialize)]
pub struct Commit {
	pub target_hash: H256,
	/// The target block's number.
	pub target_number: u32,
	/// Precommits for target block or any block after it that justify this commit.
	pub precommits: Vec<SignedPrecommit>,
}

#[derive(Clone, Debug, Decode)]
pub struct GrandpaJustification {
	pub round: u64,
	pub commit: Commit,
	pub votes_ancestries: Vec<Header>,
}

impl<'de> Deserialize<'de> for GrandpaJustification {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		let encoded = bytes::deserialize(deserializer)?;
		Self::decode(&mut &encoded[..])
			.map_err(|codec_err| D::Error::custom(format!("Invalid decoding: {:?}", codec_err)))
	}
}

#[derive(Clone, Debug, Decode)]
pub enum Messages {
	Justification(GrandpaJustification),
	ValidatorSetChange((Vec<EdPublic>, u64)),
	NewHeader((H256, u32)),
}

#[tokio::main]
async fn main() {
	let url = "ws://localhost:9944";
	let subxt_client = build_client(url).await.unwrap();

	// Subscribe to finalized headers (actually, finalized chain's head headers, some final headers may be skipped).
	let mut header_subscription = subxt_client
		.rpc()
		.subscribe_finalized_block_headers()
		.await
		.unwrap();

	// Current set of authorities, implicitly trusted, fetched from grandpa runtime.
	let grandpa_valset_raw = subxt_client
		.runtime_api()
		.at(None)
		.await
		.unwrap()
		.call_raw("GrandpaApi_grandpa_authorities", None)
		.await
		.unwrap();

	// Decode result to proper type - ed25519 public key and u64 weight.
	let grandpa_valset: Vec<(EdPublic, u64)> =
		Decode::decode(&mut &grandpa_valset_raw[..]).unwrap();

	// Drop weights, as they are not currently used.
	let mut validator_set: Vec<EdPublic> = grandpa_valset.iter().map(|e| e.0).collect();

	// Set ID is acquired in a separate storage query. It is necessary, because it is a part of message being signed.

	// Form a query key for storage
	let set_id_key = api::storage().grandpa().current_set_id();
	// Fetch the set ID from storage at current height
	let mut set_id = subxt_client
		.storage()
		// None means current height
		.at(None)
		.await
		.unwrap()
		.fetch(&set_id_key)
		.await
		.unwrap()
		.unwrap();

	println!("Current set: {:?}", validator_set);

	// Forming a channel for sending any relevant events gathered asynchronously through Substrate WS API.
	let (msg_sender, mut msg_receiver) = unbounded_channel::<Messages>();

	// Task that produces headers and new validator sets
	tokio::spawn({
		let subxt_client = subxt_client.clone();
		let msg_sender = msg_sender.clone();
		async move {
			while let Some(Ok(header)) = header_subscription.next().await {
				let head_hash: H256 = Encode::using_encoded(&header, blake2_256).into();
				msg_sender
					.send(Messages::NewHeader((head_hash, header.number)))
					.unwrap();
				// Fetch all events at the incoming header hight.
				let events = subxt_client.events().at(Some(head_hash)).await.unwrap();

				// Filter out just new authorities event.
				let new_auths =
					events.find_last::<avail_subxt::api::grandpa::events::NewAuthorities>();

				// If the event exists, send the new auths over the message channel.
				if let Ok(Some(auths)) = new_auths {
					// Fetch set ID at the incoming header hight (needed to verify justification).
					let set_id = subxt_client
						.storage()
						.at(Some(head_hash))
						.await
						.unwrap()
						.fetch(&set_id_key)
						.await
						.unwrap()
						.unwrap();
					// Drop weights and re-pack into appropriate type.
					let new_valset: Vec<EdPublic> = auths
						.authority_set
						.into_iter()
						.map(|(a, _)| EdPublic::from_raw(a.0 .0))
						.collect();
					// Send it.
					msg_sender
						.send(Messages::ValidatorSetChange((new_valset, set_id)))
						.unwrap();
				}
			}
		}
	});

	// Subscribe to justifications.
	let j: Result<subxt::rpc::Subscription<GrandpaJustification>, subxt::Error> = subxt_client
		.rpc()
		.subscribe(
			"grandpa_subscribeJustifications",
			rpc_params![],
			"grandpa_unsubscribeJustifications",
		)
		.await;
	let mut justification_subscription = j.unwrap();

	// Task that produces justifications concurrently and just passes the justification to the main task.
	tokio::spawn(async move {
		while let Some(Ok(justification)) = justification_subscription.next().await {
			msg_sender
				.send(Messages::Justification(justification))
				.unwrap();
		}
	});

	// An accumulated collection of unverified headers and justifications that are matched one by one as headers/justifications arrive.
	let mut unverified_headers: Vec<H256> = vec![];
	let mut justifications: Vec<GrandpaJustification> = vec![];

	// Main loop, gathers blocks, justifications and validator sets and checks finality
	loop {
		match msg_receiver.recv().await.unwrap() {
			Messages::Justification(justification) => {
				println!(
					"New justification at block no.: {}, hash: {:?}",
					justification.commit.target_number, justification.commit.target_hash
				);
				justifications.push(justification);
			},
			Messages::ValidatorSetChange(valset) => {
				println!("######################");
				println!("New validator set: {valset:?}");
				println!("######################");
				(validator_set, set_id) = valset;
			},
			Messages::NewHeader((hash, number)) => {
				println!("Header no.: {}, hash: {hash:?}", number);
				unverified_headers.push(hash);
			},
		}

		while let Some(hash) = unverified_headers.pop() {
			//let hash = Encode::using_encoded(&header, blake2_256).into();

			// Iterate through justifications and try to find a matching one.
			if let Some(pos) = justifications
				.iter()
				.position(|e| e.commit.target_hash == hash)
			{
				// Basically, pop it out of the collection.
				let justification = justifications.swap_remove(pos);
				// Form a message which is signed in the justification, it's a triplet of a Precommit, round number and set_id (taken from Substrate code).
				let signed_message = Encode::encode(&(
					&SignerMessage::PrecommitMessage(
						justification.commit.precommits[0].clone().precommit,
					),
					&justification.round,
					&set_id, // Set ID is needed here.
				));

				// Verify all the signatures of the justification signs the hash of the block and extract all the signer addreses.
				let signer_addresses = justification
					.commit
					.precommits
					.iter()
					.map(|precommit| {
						let is_ok = <ed25519::Pair as Pair>::verify_weak(
							&precommit.clone().signature.0[..],
							signed_message.as_slice(),
							precommit.clone().id,
						);
						// On first failure to verify signature, we exit.
						assert!(is_ok, "Not signed by this signature!");
						precommit.clone().id
					})
					.collect::<Vec<_>>();

				// Match all the signer addresses to the current validator set.
				let num_matched_addresses = signer_addresses
					.iter()
					.filter(|x| validator_set.iter().any(|e| e.0.eq(&x.0)))
					.count();

				println!(
					"Number of matching signatures: {num_matched_addresses}/{}",
					validator_set.len()
				);
				assert!(
					num_matched_addresses >= (validator_set.len() * 2 / 3),
					"Not signed by the supermajority of the validator set."
				);
			} else {
				eprintln!("Matched pair of header/justification not found.");
			}
		}
	}
}

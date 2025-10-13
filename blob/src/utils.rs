use crate::telemetry::TelemetryOperator;
use crate::traits::CommitmentQueueApiT;
use crate::{
	p2p::BlobHandle,
	store::{RocksdbBlobStore, StorageApiT},
	types::{BlobHash, BlobMetadata, BlobSignatureData, BlobTxSummary, OwnershipEntry},
};
use anyhow::{anyhow, Context, Result};
use base64::Engine;
use codec::{Decode, Encode};
use da_commitment::build_da_commitments::build_commitments_from_polynomial_grid;
use da_control::{BlobRuntimeParameters, Call};
use da_runtime::{apis::BlobApi, RuntimeCall, UncheckedExtrinsic};
use kate::gridgen::core::PolynomialGrid;
use sc_client_api::{HeaderBackend, StorageKey};
use sc_keystore::{Keystore, LocalKeystore};
use sc_transaction_pool_api::TransactionSource;
use sp_api::ProvideRuntimeApi;
use sp_authority_discovery::AuthorityId;
use sp_core::H256;
use sp_core::{crypto::KeyTypeId, sr25519, twox_128};
use sp_runtime::MultiAddress;
use sp_runtime::{
	key_types,
	traits::{Block as BlockT, Verify},
	transaction_validity::{InvalidTransaction, TransactionValidityError},
	AccountId32,
};
use sp_transaction_pool::runtime_api::TaggedTransactionQueue;
use std::collections::BTreeSet;
use std::time::Duration;
use std::{collections::BTreeMap, io::Write, sync::Arc};
use tokio::sync::{mpsc, oneshot};

pub fn get_my_validator_public_account(
	keystore: &Arc<LocalKeystore>,
) -> std::option::Option<(AuthorityId, KeyTypeId)> {
	let key_type = key_types::BABE;

	// Get keys from the keystore
	let keys = keystore.sr25519_public_keys(key_type);

	// Return None if no keys are in the keystore
	if keys.len() == 0 {
		return None;
	}
	let k = keys[keys.len() - 1];

	Some((k.into(), key_type))
}

/// Get this node's Address
pub fn get_my_validator_id<Block, Client>(
	keystore: &Arc<LocalKeystore>,
	client: &Arc<Client>,
	at: &[u8],
) -> Option<(AccountId32, AuthorityId)>
where
	Block: BlockT,
	Client: ProvideRuntimeApi<Block>,
	Client::Api: BlobApi<Block>,
{
	let key_type = key_types::BABE;

	// Get keys from the keystore
	let keys = keystore.sr25519_public_keys(key_type);

	// Return None if no keys are in the keystore
	if keys.len() == 0 {
		return None;
	}
	let k = keys[keys.len() - 1];

	get_validator_id_from_key(&k.into(), client, at)
}

pub fn get_validator_id_from_key<Block, Client>(
	key: &AuthorityId,
	client: &Arc<Client>,
	at: &[u8],
) -> Option<(AccountId32, AuthorityId)>
where
	Block: BlockT,
	Client: ProvideRuntimeApi<Block>,
	Client::Api: BlobApi<Block>,
{
	let key_type = key_types::BABE;

	let Some(at) = Block::Hash::decode(&mut &*at).ok() else {
		log::error!("Could not convert bytes to at hash");
		return None;
	};
	if let Ok(owner_opt) =
		client
			.runtime_api()
			.get_validator_from_key(at.clone(), key_type, key.encode())
	{
		if let Some(owner) = owner_opt {
			return Some((owner, key.clone()));
		}
	}

	None
}

/// Get active validator addresses
pub fn get_active_validators<Block, Client>(client: &Arc<Client>, at: &[u8]) -> Vec<AccountId32>
where
	Block: BlockT,
	Client: ProvideRuntimeApi<Block>,
	Client::Api: BlobApi<Block>,
{
	let Some(at) = Block::Hash::decode(&mut &*at).ok() else {
		log::error!("Could not convert bytes to 'at' hash");
		return Vec::new();
	};
	match client.runtime_api().get_active_validators(at.clone()) {
		Ok(validators) => validators,
		Err(e) => {
			log::error!("Failed to fetch active validators at {:?}: {:?}", at, e);
			Vec::new()
		},
	}
}

/// Get the number of validator that need to store a blob.
pub fn get_validator_per_blob<Block, Client>(
	client: &Arc<Client>,
	at: &[u8],
	nb_validators: u32,
) -> (u32, u32)
where
	Block: BlockT,
	Client: ProvideRuntimeApi<Block>,
	Client::Api: BlobApi<Block>,
{
	let Some(at) = Block::Hash::decode(&mut &*at).ok() else {
		log::error!("Could not convert bytes to 'at' hash");
		return (nb_validators, nb_validators);
	};
	let blob_params = match client.runtime_api().get_blob_runtime_parameters(at) {
		Ok(p) => p,
		Err(e) => {
			log::error!("Could get blob runtime params: {e:?}");
			return (nb_validators, nb_validators);
		},
	};
	get_validator_per_blob_inner(blob_params, nb_validators)
}

pub fn get_validator_per_blob_inner(
	blob_params: BlobRuntimeParameters,
	nb_validators: u32,
) -> (u32, u32) {
	if nb_validators <= blob_params.min_blob_holder_count {
		return (nb_validators, nb_validators);
	}

	let threshold = blob_params
		.min_blob_holder_percentage
		.mul_ceil(nb_validators)
		.max(blob_params.min_blob_holder_count);

	let diff = nb_validators.saturating_sub(threshold);

	// Add up to 10% of the diff, capped
	let margin = (diff / 10).min(3);

	let nb_validators_per_blob = threshold + margin;

	(nb_validators_per_blob, threshold)
}

pub fn check_if_wait_next_block<C, Block>(
	client: &Arc<C>,
	blob_database: &Arc<RocksdbBlobStore>,
	encoded: Vec<u8>,
	submit_blob_metadata_calls: &mut Vec<(RuntimeCall, u32)>,
	blob_metadata: &mut BTreeMap<BlobHash, (BlobMetadata, Vec<OwnershipEntry>)>,
	tx_index: u32,
) -> (bool, bool)
where
	Block: BlockT,
	C: HeaderBackend<Block> + ProvideRuntimeApi<Block>,
	C::Api: TaggedTransactionQueue<Block> + BlobApi<Block>,
{
	let mut should_submit = true;
	let mut is_submit_blob_metadata = false;

	// Decode once
	let extrinsic_data = match Decode::decode(&mut &encoded[..]) {
		Ok(UncheckedExtrinsic { function, .. }) => function,
		// If we can't decode, just give up on it
		Err(_) => return (should_submit, is_submit_blob_metadata),
	};

	// Only care about submit_blob_metadata calls
	let blob_hash =
		if let RuntimeCall::DataAvailability(Call::submit_blob_metadata { blob_hash, .. }) =
			&extrinsic_data
		{
			blob_hash
		} else {
			return (should_submit, is_submit_blob_metadata);
		};

	is_submit_blob_metadata = true;

	match blob_database.get_blob_metadata(blob_hash) {
		Ok(Some(meta)) => {
			let Ok(ownerships) = blob_database.get_blob_ownerships(&blob_hash) else {
				log::error!("Failed to read from db");
				should_submit = check_retries_for_blob(client, blob_hash, blob_database);
				return (should_submit, is_submit_blob_metadata);
			};
			// Store it for later
			blob_metadata.insert(meta.hash, (meta.clone(), ownerships.clone()));

			if meta.is_notified && ownerships.len() >= meta.nb_validators_per_blob as usize {
				// Fully finalized (either valid or errored) → submit now
				should_submit = true;
			} else {
				let Ok(tx) = codec::Decode::decode(&mut &encoded[..]) else {
					should_submit = true;
					submit_blob_metadata_calls.push((extrinsic_data, tx_index));
					return (should_submit, is_submit_blob_metadata);
				};
				let Ok(validity_res) = client.runtime_api().validate_transaction(
					client.info().best_hash,
					TransactionSource::External,
					tx,
					client.info().best_hash,
				) else {
					should_submit = true;
					submit_blob_metadata_calls.push((extrinsic_data, tx_index));
					return (should_submit, is_submit_blob_metadata);
				};

				// TODO hack
				let encoded_h256 = meta.finalized_block_hash.encode();
				let mut slice = encoded_h256.as_slice();
				// TODO handle panic
				let generic_h256 = <Block as BlockT>::Hash::decode(&mut slice)
					.expect("Should not fail. h256 to h256");

				let blob_runtime_params = match client
					.runtime_api()
					.get_blob_runtime_parameters(generic_h256)
				{
					Ok(p) => p,
					Err(e) => {
						log::error!("Could not get blob_params: {e:?}");
						BlobRuntimeParameters::default()
					},
				};

				match validity_res {
					Ok(v) => {
						if v.longevity < blob_runtime_params.min_transaction_validity
							|| v.longevity > blob_runtime_params.max_transaction_validity
						{
							// longevity out of our acceptable window → submit & drop
							should_submit = true;
						} else {
							// still valid and longevity in-bounds → wait another block
							// But check the number of retried
							should_submit =
								check_retries_for_blob(client, blob_hash, blob_database);
						}
					},
					Err(e) => match e {
						TransactionValidityError::Invalid(InvalidTransaction::Future) => {
							// The transaction is supposed to go in, but it's waiting for a tx with a lower nonce.
							should_submit =
								check_retries_for_blob(client, blob_hash, blob_database);
						},
						_ => {
							// Anything went wrong → submit so it disappears
							should_submit = true;
						},
					},
				};
			}
		},

		// No metadata yet (or DB error) → maybe we just haven't seen the blob announcement
		_ => {
			should_submit = check_retries_for_blob(client, blob_hash, blob_database);
		},
	}

	if should_submit {
		submit_blob_metadata_calls.push((extrinsic_data, tx_index));
	}

	(should_submit, is_submit_blob_metadata)
}

pub fn check_retries_for_blob<Block, C>(
	client: &Arc<C>,
	blob_hash: &BlobHash,
	blob_database: &Arc<RocksdbBlobStore>,
) -> bool
where
	Block: BlockT,
	C: HeaderBackend<Block> + ProvideRuntimeApi<Block>,
	C::Api: BlobApi<Block>,
{
	let finalized_hash = client.info().finalized_hash;
	let blob_runtime_params = match client
		.runtime_api()
		.get_blob_runtime_parameters(finalized_hash)
	{
		Ok(p) => p,
		Err(e) => {
			log::error!("Could not get blob_params: {e:?}");
			BlobRuntimeParameters::default()
		},
	};
	let tried = blob_database.get_blob_retry(blob_hash).unwrap_or(0);
	if tried <= blob_runtime_params.max_blob_retry_before_discarding {
		// bump retry count and wait
		let _ = blob_database.insert_blob_retry(blob_hash, tried + 1);
		log::info!(
			"BLOB - RPC check_retries_for_blob - it lives - {:?}",
			blob_hash
		);
		false
	} else {
		// give up: submit to get it dropped quickly
		log::info!(
			"BLOB - RPC check_retries_for_blob - it dies - {:?}",
			blob_hash
		);
		true
	}
}

pub async fn get_blob_txs_summary(
	submit_blob_metadata_calls: &Vec<(RuntimeCall, u32)>,
	blob_metadata: BTreeMap<BlobHash, (BlobMetadata, Vec<OwnershipEntry>)>,
) -> (Vec<BlobTxSummary>, u64) {
	let mut blob_txs_summary: Vec<BlobTxSummary> = Vec::new();
	let mut total_size = 0;

	for (tx, tx_index) in submit_blob_metadata_calls.iter().cloned() {
		if let RuntimeCall::DataAvailability(Call::submit_blob_metadata {
			size,
			blob_hash,
			commitment,
			..
		}) = tx
		{
			let maybe_blob_metadata = blob_metadata.get(&blob_hash);
			let blob_summary =
				get_block_tx_summary(blob_hash, commitment, maybe_blob_metadata, tx_index, size);
			if blob_summary.success {
				total_size += size;
			}
			blob_txs_summary.push(blob_summary);
		}
	}

	(blob_txs_summary, total_size)
}

fn get_block_tx_summary(
	blob_hash: BlobHash,
	commitment: Vec<u8>,
	blob_metadata: Option<&(BlobMetadata, Vec<OwnershipEntry>)>,
	tx_index: u32,
	size: u64,
) -> BlobTxSummary {
	let mut blob_summary = BlobTxSummary {
		hash: blob_hash,
		finalized_block_hash_checkpoint: H256::zero(),
		tx_index,
		success: false,
		reason: None,
		missing_validators: Vec::new(),
		ownership: Vec::new(),
	};

	let (meta, ownerships) = match blob_metadata {
		Some(m) => m,
		None => {
			blob_summary.reason =
				Some("Blob metadata not found in the store to check for ownerships".into());
			return blob_summary;
		},
	};

	if meta.size != size || meta.commitment != commitment {
		blob_summary.reason =
			Some("Blob metadata from the store did not match the one from the transaction".into());
		return blob_summary;
	}

	if !meta.is_notified {
		blob_summary.reason =
			Some("Blob metadata from the store is not notified, discarding it".into());
		return blob_summary;
	}

	blob_summary.ownership = ownerships.to_vec();

	if ownerships.len() < meta.nb_validators_per_blob_threshold as usize {
		// We store this only in case we have an issue, if the nb_validators_per_blob_threshold is respected, we can't forget this
		// Compute the list of missing validators so everyone can check
		let missing_validators: Vec<AccountId32> = {
			let owned: BTreeSet<_> = ownerships.iter().map(|o| o.address.clone()).collect();
			meta.storing_validator_list
				.iter()
				.cloned()
				.filter(|v| !owned.contains(v))
				.collect()
		};

		blob_summary.missing_validators = missing_validators;
		blob_summary.reason = Some("Not enough validators vouched for this block".into());
		return blob_summary;
	}

	blob_summary.success = true;

	return blob_summary;
}

pub fn build_signature_payload(blob_hash: BlobHash, additional: Vec<u8>) -> Vec<u8> {
	let mut payload = Vec::new();
	payload.extend(blob_hash.encode());
	payload.extend(additional.encode());
	payload
}

pub fn sign_blob_data<Block: BlockT>(
	blob_handle: &BlobHandle<Block>,
	payload: Vec<u8>,
) -> Result<BlobSignatureData> {
	let Some(keystore) = blob_handle.keystore.get() else {
		return Err(anyhow!("Keystore is not yet registered"));
	};

	sign_blob_data_inner(keystore, payload)
}

pub fn sign_blob_data_inner(
	keystore: &Arc<LocalKeystore>,
	payload: Vec<u8>,
) -> Result<BlobSignatureData> {
	let key_type = key_types::BABE;
	let keys = keystore.sr25519_public_keys(key_type);

	if keys.len() == 0 {
		return Err(anyhow!(
			"No keys found in the store when trying to sign data"
		));
	}
	let public = keys[keys.len() - 1];
	let signature_result = keystore.sr25519_sign(key_type, &public, &payload);

	let signature = match signature_result {
		Ok(maybe_sig) => match maybe_sig {
			Some(sig) => sig,
			None => {
				return Err(anyhow!(
						"An error has occured while trying to sign data: the given `key_type` and `public` combination doesn't exist in the keystore"
					));
			},
		},
		Err(e) => {
			return Err(anyhow!(
				"An error has occured while trying to sign data: {e}"
			));
		},
	};

	let pubkey_bytes = public.0.to_vec();
	let sig_bytes = signature.0.to_vec();
	Ok(BlobSignatureData {
		signer: pubkey_bytes,
		signature: sig_bytes,
	})
}

pub fn verify_signed_blob_data(
	signature_data: BlobSignatureData,
	payload: Vec<u8>,
) -> Result<bool> {
	let public: [u8; 32] = signature_data
		.signer
		.as_slice()
		.try_into()
		.context(format!(
			"Public key had wrong length: expected 32 bytes, got {}",
			signature_data.signer.len()
		))?;
	let public = sr25519::Public::from_raw(public);

	let signature: [u8; 64] = signature_data
		.signature
		.as_slice()
		.try_into()
		.context(format!(
			"Signature had wrong length: expected 64 bytes, got {}",
			signature_data.signature.len()
		))?;
	let signature = sr25519::Signature::from_raw(signature.try_into().unwrap());

	let valid = signature.verify(payload.as_slice(), &public);
	Ok(valid)
}

pub fn get_dynamic_blocklength_key() -> StorageKey {
	let mut key = Vec::new();
	key.extend(&twox_128(b"System"));
	key.extend(&twox_128(b"DynamicBlockLength"));
	let storage_key = StorageKey(key);
	storage_key
}

pub struct SmartStopwatch {
	span: String,
	extra_information: Vec<String>,
	created: std::time::Instant,
	tracking: Vec<(String, std::time::Instant)>,
	finished: Vec<(String, std::time::Duration, String)>,
}

impl SmartStopwatch {
	pub fn new(span: impl Into<String>) -> Self {
		Self {
			span: span.into(),
			tracking: Vec::with_capacity(20),
			finished: Vec::with_capacity(20),
			created: std::time::Instant::now(),
			extra_information: Vec::with_capacity(3),
		}
	}

	pub fn add_extra_information(&mut self, value: impl Into<String>) {
		self.extra_information.push(value.into());
	}

	pub fn start(&mut self, name: impl Into<String>) {
		self.tracking.push((name.into(), std::time::Instant::now()));
	}

	pub fn stop(&mut self, name: &str) -> Duration {
		let Some(index) = self.tracking.iter().position(|x| x.0 == name) else {
			return Duration::from_secs(0);
		};
		let value = self.tracking.swap_remove(index);
		let elapsed = value.1.elapsed();
		self.finished.push((value.0, elapsed, "".into()));
		elapsed
	}
}

impl Drop for SmartStopwatch {
	fn drop(&mut self) {
		use std::{fmt::Write, mem::take};

		let now = std::time::Instant::now();

		let still_active = take(&mut self.tracking);
		let mut finished = take(&mut self.finished);
		for sa in still_active {
			finished.push((sa.0, now.duration_since(sa.1), String::new()));
		}

		finished.sort_by(|x, y| y.1.cmp(&x.1));

		let mut msg = String::with_capacity(500);
		msg.push_str(self.span.as_str());
		msg.push_str(" -- ");

		let extra_information = take(&mut self.extra_information);
		for ei in extra_information {
			let _ = write!(msg, "{}. ", ei);
		}
		let _ = write!(
			msg,
			"Total duration: {} ms. ",
			now.duration_since(self.created).as_millis()
		);
		for f in finished {
			if f.2.is_empty() {
				let _ = write!(msg, "{}: {} ms. ", f.0, f.1.as_millis());
			} else {
				let _ = write!(msg, "{}: {} ms, {}. ", f.0, f.1.as_millis(), f.2);
			}
		}

		log::info!("{}", msg)
	}
}

pub struct CommitmentQueueMessage {
	hash: H256,
	size: usize,
	grid: PolynomialGrid,
	request: oneshot::Sender<Vec<u8>>,
}

impl CommitmentQueueMessage {
	pub fn new(
		hash: H256,
		size: usize,
		grid: PolynomialGrid,
	) -> (Self, oneshot::Receiver<Vec<u8>>) {
		let (tx, rx) = oneshot::channel();
		let s = Self {
			hash,
			size,
			grid,
			request: tx,
		};
		(s, rx)
	}
}

pub struct CommitmentQueue {
	tx: mpsc::Sender<CommitmentQueueMessage>,
}

impl CommitmentQueueApiT for CommitmentQueue {
	fn send(&self, value: CommitmentQueueMessage) -> bool {
		return self.tx.try_send(value).is_ok();
	}
}

impl CommitmentQueue {
	pub fn new(channel_size: usize) -> (Self, mpsc::Receiver<CommitmentQueueMessage>) {
		let (tx, rx) = mpsc::channel(channel_size);
		(Self { tx }, rx)
	}

	pub fn spawn_background_task(
		rx: mpsc::Receiver<CommitmentQueueMessage>,
		telemetry_operator: Option<TelemetryOperator>,
	) {
		std::thread::spawn(move || {
			Self::run_task(rx, telemetry_operator);
		});
	}

	pub fn run_task(
		mut rx: mpsc::Receiver<CommitmentQueueMessage>,
		telemetry_operator: Option<TelemetryOperator>,
	) {
		let mut stop_watch = SmartStopwatch::new("Commitment queue stopwatch.");
		while let Some(msg) = rx.blocking_recv() {
			stop_watch.start("Building commitment");
			let commitment = build_commitments_from_polynomial_grid(msg.grid);
			let duration = stop_watch.stop("Building commitment");
			if let Some(ref telemetry_operator) = telemetry_operator {
				telemetry_operator.blob_commitment(msg.size, msg.hash, duration);
			}
			_ = msg.request.send(commitment);
		}
	}
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct B64Param(#[serde(deserialize_with = "deserialize_base64_to_vec")] pub Vec<u8>);

fn deserialize_base64_to_vec<'de, D>(d: D) -> Result<Vec<u8>, D::Error>
where
	D: serde::Deserializer<'de>,
{
	struct B64Visitor;

	impl<'de> serde::de::Visitor<'de> for B64Visitor {
		type Value = Vec<u8>;

		fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.write_str("a base64-encoded string")
		}

		// This path borrows the incoming &str directly; no intermediate String allocation.
		fn visit_borrowed_str<E: serde::de::Error>(self, v: &'de str) -> Result<Self::Value, E> {
			base64::engine::general_purpose::STANDARD
				.decode(v)
				.map_err(E::custom)
		}

		// Fallback if the JSON parser doesn’t give us a borrowed str.
		fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
			base64::engine::general_purpose::STANDARD
				.decode(v)
				.map_err(E::custom)
		}
	}

	d.deserialize_str(B64Visitor)
}

pub fn zstd_compress(data: &[u8], level: i32) -> Result<Vec<u8>, std::io::Error> {
	let mut out = Vec::with_capacity(data.len() / 3);
	let mut encoder = zstd::Encoder::new(&mut out, level)?;
	// Improves performance
	encoder.set_pledged_src_size(Some(data.len() as u64))?;
	encoder.write_all(data)?;
	encoder.finish()?;

	Ok(out)
}

pub fn zstd_decompress(data: &[u8]) -> Result<Vec<u8>, std::io::Error> {
	zstd::decode_all(data)
}

pub fn extract_signer_and_nonce(uxt: &UncheckedExtrinsic) -> Option<(AccountId32, u32)> {
	let (address, _sig, extra) = uxt.signature.as_ref()?;

	let who: AccountId32 = match address {
		MultiAddress::Id(id) => id.clone(),
		MultiAddress::Address32(data) => AccountId32::new(*data),
		_ => return None,
	};

	let check_nonce = &extra.5;
	let nonce = check_nonce.0;

	Some((who, nonce))
}

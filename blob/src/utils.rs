use crate::{
	p2p::BlobHandle,
	store::{BlobStore, RocksdbBlobStore},
	types::{BlobHash, BlobMetadata, BlobSignatureData, BlobTxSummary, OwnershipEntry},
};
use anyhow::{anyhow, Context, Result};
use codec::{Decode, Encode};
use da_control::{BlobRuntimeParameters, Call};
use da_runtime::{apis::BlobApi, RuntimeCall, UncheckedExtrinsic};
use sc_client_api::HeaderBackend;
use sc_keystore::{Keystore, LocalKeystore};
use sc_transaction_pool_api::TransactionSource;
use sp_api::ProvideRuntimeApi;
use sp_authority_discovery::AuthorityId;
use sp_core::sr25519;
use sp_runtime::{
	key_types,
	traits::{Block as BlockT, Verify},
	transaction_validity::{InvalidTransaction, TransactionValidityError},
	AccountId32, SaturatedConversion,
};
use sp_transaction_pool::runtime_api::TaggedTransactionQueue;
use std::{collections::BTreeMap, sync::Arc};

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
) -> u32
where
	Block: BlockT,
	Client: ProvideRuntimeApi<Block>,
	Client::Api: BlobApi<Block>,
{
	let Some(at) = Block::Hash::decode(&mut &*at).ok() else {
		log::error!("Could not convert bytes to 'at' hash");
		return nb_validators;
	};
	let blob_params = match client.runtime_api().get_blob_runtime_parameters(at) {
		Ok(p) => p,
		Err(e) => {
			log::error!("Could get blob runtime params: {e:?}");
			return nb_validators;
		},
	};
	get_validator_per_blob_inner(blob_params, nb_validators)
}

pub fn get_validator_per_blob_inner(blob_params: BlobRuntimeParameters, nb_validators: u32) -> u32 {
	if nb_validators <= blob_params.min_blob_holder_count {
		return nb_validators;
	} else {
		let percentage = blob_params
			.min_blob_holder_percentage
			.mul_ceil(nb_validators);
		return percentage.max(blob_params.min_blob_holder_count);
	}
}

/// Generate pseudo deterministic index based on given values
pub fn generate_base_index(
	blob_hash: BlobHash,
	block_hash_bytes: &[u8],
	ring_size: usize,
	additional: Option<Vec<u8>>,
) -> Result<usize> {
	let ring_size: u64 = ring_size.saturated_into();

	let hash_bytes = blob_hash.as_bytes();
	let truncated = hash_bytes
		.get(..8)
		.ok_or(anyhow!("Blob hash is too short, expected at least 8 bytes"))?;
	let array: [u8; 8] = truncated
		.try_into()
		.map_err(|_| anyhow!("Failed to convert first 8 bytes of blob hash"))?;
	let blob_seed = u64::from_le_bytes(array);
	let blob_index = blob_seed % ring_size;

	let truncated = block_hash_bytes.get(..8).ok_or(anyhow!(
		"Block hash is too short, expected at least 8 bytes"
	))?;
	let array: [u8; 8] = truncated
		.try_into()
		.map_err(|_| anyhow!("Failed to convert first 8 bytes of block hash"))?;
	let block_seed = u64::from_le_bytes(array);
	let block_index = block_seed % ring_size;

	let additional_index = match additional {
		Some(additional) => {
			let truncated = additional.get(..8).ok_or(anyhow!(
				"Additional hash is too short, expected at least 8 bytes"
			))?;
			let array: [u8; 8] = truncated
				.try_into()
				.map_err(|_| anyhow!("Failed to convert first 8 bytes of blob hash"))?;
			let additional_seed = u64::from_le_bytes(array);
			let additional_index = additional_seed % ring_size;
			additional_index
		},
		None => 0,
	};

	let index = blob_index
		.wrapping_add(block_index)
		.wrapping_add(additional_index)
		% ring_size;

	Ok(index as usize)
}

/// Decide deterministically whether this node should fetch/store a blob based on the blob hash
/// given the full sorted list of validators.
pub fn check_store_blob(
	blob_hash: BlobHash,
	validators: &Vec<AccountId32>,
	my_id: &AccountId32,
	block_hash_bytes: &[u8],
	nb_validators_per_blob: u32,
) -> Result<bool> {
	let nb_validators = validators.len() as u32;
	log::info!("Check store blob start - nb_validators:{nb_validators:?} - nb_validators_per_blob: {nb_validators_per_blob:?}");

	if nb_validators == 0 || nb_validators_per_blob == 0 {
		return Ok(false);
	}

	let my_pos = match validators.iter().position(|v| v == my_id) {
		Some(p) => p,
		None => {
			// We're not in the validator set
			log::info!(
				"Check store blob - Could not find my pos. Validators:{:?}. Me: {:?}",
				validators.clone(),
				my_id.clone()
			);
			return Ok(false);
		},
	};

	log::info!("Check store blob - My Pos in the validator set:{my_pos:?}");

	let base_index =
		generate_base_index(blob_hash, block_hash_bytes, nb_validators as usize, None)?;

	log::info!("Check store blob - base_index:{base_index:?}");
	for i in 0..nb_validators_per_blob {
		let index = ((base_index as u32) + i) % nb_validators;
		log::info!(
			"Validator: {:?}, should store blob: {:?}",
			validators.get(index as usize),
			blob_hash
		);
		if index as usize == my_pos {
			log::info!("I should store blob: {:?}", blob_hash);
			return Ok(true);
		}
	}
	log::info!("Check store blob end");

	Ok(false)
}

pub fn check_if_wait_next_block<C, Block>(
	client: &Arc<C>,
	blob_store: &Arc<RocksdbBlobStore<Block>>,
	encoded: Vec<u8>,
	submit_blob_metadata_calls: &mut Vec<(RuntimeCall, u32)>,
	blob_metadata: &mut BTreeMap<BlobHash, (BlobMetadata<Block>, Vec<OwnershipEntry>)>,
	tx_index: u32,
) -> (bool, bool)
where
	Block: BlockT,
	C: HeaderBackend<Block> + ProvideRuntimeApi<Block>,
	C::Api: TaggedTransactionQueue<Block> + BlobApi<Block>,
{
	let timer = std::time::Instant::now();
	log::info!(
		"BLOB - RPC check_if_wait_next_block - START - {:?}",
		timer.elapsed()
	);
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

	match blob_store.get_blob_metadata(blob_hash) {
		Ok(Some(meta)) => {
			let Ok(ownerships) = blob_store.get_blob_ownerships(&blob_hash) else {
				log::error!("Failed to read from db");
				should_submit = check_retries_for_blob(client, blob_hash, blob_store);
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

				let blob_runtime_params = match client
					.runtime_api()
					.get_blob_runtime_parameters(meta.finalized_block_hash)
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
							should_submit = check_retries_for_blob(client, blob_hash, blob_store);
						}
					},
					Err(e) => match e {
						TransactionValidityError::Invalid(InvalidTransaction::Future) => {
							// The transaction is supposed to go in, but it's waiting for a tx with a lower nonce.
							should_submit = check_retries_for_blob(client, blob_hash, blob_store);
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
			should_submit = check_retries_for_blob(client, blob_hash, blob_store);
		},
	}

	if should_submit {
		submit_blob_metadata_calls.push((extrinsic_data, tx_index));
	}
	log::info!(
		"BLOB - RPC check_if_wait_next_block - END - {:?}",
		timer.elapsed()
	);

	(should_submit, is_submit_blob_metadata)
}

pub fn check_retries_for_blob<Block, C>(
	client: &Arc<C>,
	blob_hash: &BlobHash,
	blob_store: &Arc<RocksdbBlobStore<Block>>,
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
	let tried = blob_store.get_blob_retry(blob_hash).unwrap_or(0);
	if tried <= blob_runtime_params.max_blob_retry_before_discarding {
		// bump retry count and wait
		let _ = blob_store.insert_blob_retry(blob_hash, tried + 1);
		log::info!(
			"BLOB - RPC check_retries_for_blob - it lives - {:?}",
			blob_hash
		);
		false
	} else {
		// give up: submit to get it dropped quickly
		log::info!(
			"BLOB - RPC check_retries_for_blob - it lives - {:?}",
			blob_hash
		);
		true
	}
}

pub async fn get_blob_txs_summary<Block: BlockT>(
	submit_blob_metadata_calls: &Vec<(RuntimeCall, u32)>,
	blob_metadata: BTreeMap<BlobHash, (BlobMetadata<Block>, Vec<OwnershipEntry>)>,
) -> (Vec<BlobTxSummary>, u64) {
	let timer = std::time::Instant::now();
	log::info!(
		"BLOB - get_blob_txs_summary - START - {:?}",
		timer.elapsed()
	);
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
			match get_block_tx_summary(blob_hash, size, commitment, maybe_blob_metadata, tx_index) {
				Ok((tx_summary, size)) => {
					total_size += size;
					blob_txs_summary.push(tx_summary);
				},
				Err(reason) => {
					blob_txs_summary.push(BlobTxSummary {
						hash: blob_hash,
						success: false,
						reason: Some(reason),
						ownership: Vec::new(),
						tx_index,
					});
				},
			}
		}
	}
	log::info!("BLOB - get_blob_txs_summary - END - {:?}", timer.elapsed());

	(blob_txs_summary, total_size)
}

fn get_block_tx_summary<Block: BlockT>(
	blob_hash: BlobHash,
	size: u64,
	commitment: Vec<u8>,
	blob_metadata: Option<&(BlobMetadata<Block>, Vec<OwnershipEntry>)>,
	tx_index: u32,
) -> Result<(BlobTxSummary, u64), String> {
	let (meta, ownerships) = match blob_metadata {
		Some(m) => m,
		None => {
			return Err("Blob metadata not found in the store to sample the blob".into());
		},
	};

	if meta.size != size || meta.commitment != commitment {
		return Err(
			"Blob metadata from the store did not match the one from the transaction".into(),
		);
	}

	if !meta.is_notified {
		return Err("Blob metadata from the store is not notified, discarding it".into());
	}

	if ownerships.len() < meta.nb_validators_per_blob as usize {
		return Err("Not enough validators vouched for this block".into());
	}

	return Ok((
		BlobTxSummary {
			hash: blob_hash,
			tx_index,
			success: true,
			reason: None,
			ownership: ownerships.to_vec(),
		},
		meta.size as u64,
	));
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

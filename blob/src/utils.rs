use crate::{
	p2p::BlobHandle,
	send_cell_request,
	store::{BlobStore, RocksdbBlobStore},
	types::{
		BlobHash, BlobMetadata, BlobSignatureData, BlobTxSummary, CellCoordinate, CellRequest,
	},
	CELL_COUNT, LOG_TARGET, MAX_BLOB_RETRY_BEFORE_DISCARDING, MAX_TRANSACTION_VALIDITY,
	MIN_BLOB_HOLDER_COUNT, MIN_BLOB_HOLDER_PERCENTAGE, MIN_TRANSACTION_VALIDITY,
};
use anyhow::{anyhow, Context, Result};
use codec::{Decode, Encode};
use da_commitment::build_da_commitments::build_da_commitments;
use da_control::Call;
use da_runtime::{kate::GDataProof, RuntimeCall, UncheckedExtrinsic};
use kate::{couscous::multiproof_params, Seed};
use kate_recovery::{
	commons::ArkPublicParams,
	data::SingleCell,
	matrix::{Dimensions, Position},
	proof::verify_v2,
};
use rand::{thread_rng, Rng, SeedableRng};
use rand_chacha::ChaCha20Rng;
use sc_client_api::HeaderBackend;
use sc_keystore::{Keystore, LocalKeystore};
use sc_network::{NetworkService, NetworkStateInfo, PeerId};
use sc_transaction_pool_api::TransactionSource;
use sp_api::ProvideRuntimeApi;
use sp_authority_discovery::AuthorityId;
use sp_core::{keccak_256, sr25519};
use sp_runtime::{
	key_types,
	traits::{Block as BlockT, Verify},
	transaction_validity::{InvalidTransaction, TransactionValidityError},
	SaturatedConversion,
};
use sp_transaction_pool::runtime_api::TaggedTransactionQueue;
use std::{
	collections::{BTreeMap, HashSet},
	str::FromStr,
	sync::Arc,
};
static PP: std::sync::OnceLock<ArkPublicParams> = std::sync::OnceLock::new();

/// Get this node Authority ID
pub fn get_my_validator_id(keystore: &Arc<LocalKeystore>) -> Option<AuthorityId> {
	let key_type = key_types::BABE;

	// Get keys from the keystore
	let keys = keystore.sr25519_public_keys(key_type);

	// Return None if no keys are in the keystore
	if keys.len() == 0 {
		return None;
	}

	Some(keys[keys.len() - 1].into())
}

/// Get the number of validator that need to store a blob.
pub fn get_validator_per_blob(nb_validators: u32) -> u32 {
	if nb_validators <= MIN_BLOB_HOLDER_COUNT {
		return nb_validators;
	} else {
		let percentage = MIN_BLOB_HOLDER_PERCENTAGE.mul_ceil(nb_validators);
		return percentage.max(MIN_BLOB_HOLDER_COUNT);
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
	validators: &Vec<AuthorityId>,
	my_id: &AuthorityId,
	block_hash_bytes: &[u8],
	nb_validators_per_blob: u32,
) -> Result<bool> {
	let nb_validators = validators.len() as u32;

	if nb_validators == 0 || nb_validators_per_blob == 0 {
		return Ok(false);
	}

	let my_pos = match validators.iter().position(|v| v == my_id) {
		Some(p) => p,
		None => return Ok(false), // We're not in the validator set
	};

	let base_index =
		generate_base_index(blob_hash, block_hash_bytes, nb_validators as usize, None)?;
	for i in 0..nb_validators_per_blob {
		let index = ((base_index as u32) + i) % nb_validators;
		if index as usize == my_pos {
			return Ok(true);
		}
	}

	Ok(false)
}

pub fn check_if_wait_next_block<C, Block>(
	client: &Arc<C>,
	blob_store: &Arc<RocksdbBlobStore<Block>>,
	encoded: Vec<u8>,
	submit_blob_metadata_calls: &mut Vec<(RuntimeCall, u32)>,
	blob_metadata: &mut BTreeMap<BlobHash, BlobMetadata<Block>>,
	tx_index: u32,
) -> (bool, bool)
where
	Block: BlockT,
	C: HeaderBackend<Block> + ProvideRuntimeApi<Block>,
	C::Api: TaggedTransactionQueue<Block>,
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

	match blob_store.get_blob_metadata(blob_hash) {
		Ok(Some(meta)) => {
			// Store it for later
			blob_metadata.insert(meta.hash, meta.clone());

			if meta.is_validated || meta.error_reason.is_some() {
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

				match validity_res {
					Ok(v) => {
						if v.longevity < MIN_TRANSACTION_VALIDITY
							|| v.longevity > MAX_TRANSACTION_VALIDITY
						{
							// longevity out of our acceptable window → submit & drop
							should_submit = true;
						} else {
							// still valid and longevity in-bounds → wait another block
							// But check the number of retried
							should_submit = check_retries_for_blob(blob_hash, blob_store);
						}
					},
					Err(e) => match e {
						TransactionValidityError::Invalid(InvalidTransaction::Future) => {
							// The transaction is supposed to go in, but it's waiting for a tx with a lower nonce.
							should_submit = check_retries_for_blob(blob_hash, blob_store);
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
			should_submit = check_retries_for_blob(blob_hash, blob_store);
		},
	}

	if should_submit {
		submit_blob_metadata_calls.push((extrinsic_data, tx_index));
	}

	(should_submit, is_submit_blob_metadata)
}

pub fn check_retries_for_blob<Block: BlockT>(
	blob_hash: &BlobHash,
	blob_store: &Arc<RocksdbBlobStore<Block>>,
) -> bool {
	let tried = blob_store.get_blob_retry(blob_hash).unwrap_or(0);
	if tried <= MAX_BLOB_RETRY_BEFORE_DISCARDING {
		// bump retry count and wait
		let _ = blob_store.insert_blob_retry(blob_hash, tried + 1);
		false
	} else {
		// give up: submit to get it dropped quickly
		true
	}
}

pub async fn check_blob_validity<Block: BlockT>(
	network: &Arc<NetworkService<Block, Block::Hash>>,
	keystore: &Arc<LocalKeystore>,
	blob_store: &Arc<RocksdbBlobStore<Block>>,
	blob_metadata: &BlobMetadata<Block>,
	finalized_hash_bytes: &[u8],
	my_validator_address: AuthorityId,
) {
	if !blob_metadata.is_validated
		&& blob_metadata.error_reason.is_none()
		&& blob_metadata.is_notified
		&& blob_metadata.ownership.len() > 0
		&& blob_metadata.ownership.len() >= (blob_metadata.nb_validators_per_blob as usize)
	{
		let start_time = std::time::Instant::now();
		let elapsed = start_time.elapsed();
		log::info!("check_blob_validity start {:.2?}", elapsed);

		let mut blob_metadata = blob_metadata.clone();

		let i_have_blob = blob_metadata
			.ownership
			.iter()
			.find(|o| o.address == my_validator_address);
		if let Some(_) = i_have_blob {
			// I have the blob in my own store, i can check the commitment
			let (blob_valid, reason) = check_commitment_validity(blob_store, &blob_metadata);
			blob_metadata.is_validated = blob_valid;
			log::info!(
				"Hash: {:?}, valid: {:?}: error: {:?}",
				blob_metadata.hash,
				blob_valid,
				reason
			);
			blob_metadata.error_reason = reason;
		} else {
			// I don't have the blob in my store, i need to sample using the peer list
			match cell_sample_check_validity(
				network,
				keystore,
				&blob_metadata,
				finalized_hash_bytes,
				my_validator_address,
			)
			.await
			{
				Ok(()) => {
					log::info!("Hash: {} is valid", blob_metadata.hash);
					blob_metadata.is_validated = true;
					blob_metadata.error_reason = None;
				},
				Err(e) => {
					log::info!("Hash: {} is invalid: {}", blob_metadata.hash, e);
					blob_metadata.is_validated = false;
					blob_metadata.error_reason = Some(e);
				},
			};
		}

		if let Err(e) = blob_store.insert_blob_metadata(&blob_metadata) {
			log::error!(
				target: LOG_TARGET,
				"An error has occured while trying to save blob_metadata in the store: {e}"
			);
		}

		let elapsed = start_time.elapsed();
		log::info!("check_blob_validity end {:.2?}", elapsed);
	}
}

pub fn check_commitment_validity<Block: BlockT>(
	blob_store: &Arc<RocksdbBlobStore<Block>>,
	blob_metadata: &BlobMetadata<Block>,
) -> (bool, Option<String>) {
	let blob = match blob_store.get_blob(&blob_metadata.hash) {
		Ok(b) => match b {
			Some(b) => b,
			None => return (false, Some(format!("Blob not found in the store."))),
		},
		Err(e) => {
			return (
				false,
				Some(format!(
					"An issue occured while trying to get the blob from the store: {e:?}"
				)),
			)
		},
	};

	// TODO Blob - get values from runtime
	let generated_commitment =
		build_da_commitments(blob.data.to_vec(), 1024, 4096, Seed::default());

	if blob_metadata.commitment != generated_commitment {
		return (
			false,
			Some(format!(
				"submitted blob commitment: {:?} does not correspond to generated commitment {:?}",
				blob_metadata.commitment, generated_commitment
			)),
		);
	}

	(true, None)
}

pub async fn cell_sample_check_validity<Block: BlockT>(
	network: &Arc<NetworkService<Block, Block::Hash>>,
	keystore: &Arc<LocalKeystore>,
	blob_metadata: &BlobMetadata<Block>,
	finalized_hash_bytes: &[u8],
	my_validator_address: AuthorityId,
) -> Result<(), String> {
	let start_time = std::time::Instant::now();
	let elapsed = start_time.elapsed();
	log::info!("cell_sample start {:.2?}", elapsed);

	let extended_commitment_bytes = blob_metadata.extended_commitment.clone();
	let commitments_bytes: Vec<[u8; 48]> = extended_commitment_bytes
		.chunks_exact(48)
		.map(|c| {
			c.try_into()
				.map_err(|_| "Could not convert chunk to [u8; 48]")
		})
		.collect::<Result<_, _>>()?;

	let elapsed = start_time.elapsed();
	log::info!("cell_sample after having extended comms {:.2?}", elapsed);

	// TODO Blobs: Take values from runtime
	let cols = 1024;
	let rows = (blob_metadata.commitment.len() / 48) as u16;
	let dimension = Dimensions::new(rows, cols).ok_or_else(|| format!("Invalid dimension"))?;
	let cells = generate_random_cells(dimension, CELL_COUNT);

	let signature_payload = build_signature_payload(blob_metadata.hash, cells.clone().encode());
	let signature_data = sign_blob_data_inner(&keystore, signature_payload)
		.map_err(|e| format!("Signing failed: {}", e))?;

	let req = CellRequest {
		hash: blob_metadata.hash,
		cells,
		signature_data,
	};

	let peers: Vec<PeerId> = blob_metadata
		.ownership
		.clone()
		.into_iter()
		.filter_map(|o| PeerId::from_str(&o.peer_id_encoded).ok())
		.collect();

	if peers.is_empty() {
		return Err("No peers available".into());
	}

	let peers_len = peers.len();
	let local_peer_id = network.local_peer_id();

	let base_index = match generate_base_index(
		blob_metadata.hash,
		&finalized_hash_bytes,
		peers_len,
		Some(my_validator_address.encode()),
	) {
		Ok(i) => i,
		Err(e) => {
			return Err(format!(
				"Could not generate pseudo random peer index: {e:?}"
			));
		},
	};

	for i in 0..(MAX_BLOB_RETRY_BEFORE_DISCARDING as usize) {
		let Some(target_peer) = peers.get((base_index + i) % peers_len).cloned() else {
			log::warn!("Invalid index for peer list, continuing");
			continue;
		};

		if target_peer == local_peer_id {
			log::warn!(
				"Invalid peer id: Found this node local peer_id when it should not be the case."
			);
			continue;
		}

		let elapsed = start_time.elapsed();
		log::info!("cell_sample until request in {:.2?}", elapsed);

		let cell_response = match send_cell_request(req.clone(), &network, target_peer).await {
			Ok(r) => r,
			Err(e) => {
				log::warn!(
					target: LOG_TARGET,
					"Failed to get cell proof from peer {:?} for blob {:?}: {}",
					target_peer,
					blob_metadata.hash,
					e
				);
				continue;
			},
		};
		let elapsed = start_time.elapsed();
		log::info!("cell_sample before verify in {:.2?}", elapsed);

		if verify_cell_proofs(
			cell_response.cell_proofs,
			commitments_bytes.clone(),
			req.clone(),
			dimension,
		)
		.is_ok()
		{
			let elapsed = start_time.elapsed();
			log::info!("cell_sample completed in {:.2?} : ok", elapsed);
			return Ok(());
		} else {
			log::warn!(
				target: LOG_TARGET,
				"Invalid cell proof from peer {:?} for blob {:?}",
				target_peer,
				blob_metadata.hash
			);
			let elapsed = start_time.elapsed();
			log::info!("cell_sample completed in {:.2?} : Not ok", elapsed);
		}
	}

	Err(format!("All tried peers failed to return valid cell proof"))
}

pub async fn get_blob_txs_summary<Block: BlockT>(
	submit_blob_metadata_calls: &Vec<(RuntimeCall, u32)>,
	blob_metadata: BTreeMap<BlobHash, BlobMetadata<Block>>,
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
			let blob_metadata = blob_metadata.get(&blob_hash);
			match get_block_tx_summary(blob_hash, size, commitment, blob_metadata, tx_index) {
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

	(blob_txs_summary, total_size)
}

fn get_block_tx_summary<Block: BlockT>(
	blob_hash: BlobHash,
	size: u64,
	commitment: Vec<u8>,
	blob_metadata: Option<&BlobMetadata<Block>>,
	tx_index: u32,
) -> Result<(BlobTxSummary, u64), String> {
	let meta = match blob_metadata {
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

	if meta.is_validated {
		return Ok((
			BlobTxSummary {
				hash: blob_hash,
				tx_index,
				success: true,
				reason: None,
				ownership: meta.ownership.clone(),
			},
			meta.size as u64,
		));
	} else {
		return Err(meta.error_reason.clone().unwrap_or(format!(
			"Blob metadata were not filled before transaction expiry"
		)));
	}
}

fn verify_cell_proofs(
	proofs: Vec<GDataProof>,
	commitments: Vec<[u8; 48]>,
	request: CellRequest,
	dimension: Dimensions,
) -> Result<()> {
	let pp = PP.get_or_init(multiproof_params);

	for (coord, proof) in request.cells.iter().zip(proofs.iter()) {
		let mut data_proof = [0u8; 80];
		data_proof[..48].copy_from_slice(&proof.1.encode());
		let mut data_bytes = [0u8; 32];
		proof.0.to_big_endian(&mut data_bytes);
		data_proof[48..].copy_from_slice(&data_bytes);

		let cell = SingleCell::new(
			Position {
				row: coord.row,
				col: coord.col as u16,
			},
			data_proof,
		);

		let commitment = commitments
			.get(coord.row as usize)
			.ok_or_else(|| anyhow::anyhow!("Missing commitment for row {}", coord.row))?;

		if let Err(e) = verify_v2(pp, dimension, commitment, &cell) {
			log::warn!(
				target: LOG_TARGET,
				"Cell proof verification failed for {:?}: {e}",
				coord
			);
			return Err(anyhow::anyhow!(
				"Cell proof verification failed at row {}, col {}",
				coord.row,
				coord.col
			));
		}
	}
	log::debug!(
		target: LOG_TARGET,
		"Verified random cell proofs for blob {:?}",
		request.hash
	);
	Ok(())
}

fn generate_random_cells(dimensions: Dimensions, cell_count: u32) -> Vec<CellCoordinate> {
	let (max_cells, row_limit) = (dimensions.extended_size(), dimensions.extended_rows());
	let count = max_cells.min(cell_count);

	if max_cells < cell_count {
		log::debug!("Max cells {max_cells} < requested {cell_count}");
	}

	let mut rng = thread_rng();
	let mut indices = HashSet::with_capacity(count as usize);

	while indices.len() < count as usize {
		indices.insert(CellCoordinate {
			row: rng.gen_range(0..row_limit),
			col: rng.gen_range(0..dimensions.cols().into()) as u32,
		});
	}

	indices.into_iter().collect()
}

fn generate_pseudo_random_cells(
	dimensions: Dimensions,
	cell_count: u32,
	finalized_hash_encoded: &[u8],
	blob_hash: BlobHash,
	validator_address_encoded: &[u8],
) -> Vec<CellCoordinate> {
	let (max_cells, cols) = (dimensions.extended_size(), dimensions.cols());
	let count = max_cells.min(cell_count);

	if max_cells < cell_count {
		log::debug!("Max cells {max_cells} < requested {cell_count}");
	}

	let mut seed_input = Vec::new();
	seed_input.extend_from_slice(finalized_hash_encoded);
	seed_input.extend_from_slice(blob_hash.as_bytes());
	seed_input.extend_from_slice(validator_address_encoded);
	let seed = keccak_256(&seed_input);
	let mut rng = ChaCha20Rng::from_seed(seed);

	let mut selected_indices = HashSet::with_capacity(count as usize);
	while selected_indices.len() < count as usize {
		let idx = rng.gen_range(0..max_cells);
		selected_indices.insert(idx);
	}

	let cols = cols.get() as u32;
	selected_indices
		.into_iter()
		.map(|i| CellCoordinate {
			row: i / cols,
			col: i % cols,
		})
		.collect()
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

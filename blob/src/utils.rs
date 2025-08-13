use crate::{
	p2p::BlobHandle,
	store::{BlobStore, RocksdbBlobStore},
	types::{Blob, BlobHash, BlobMetadata, BlobSignatureData, BlobTxSummary, CellCoordinate},
	CELL_COUNT, LOG_TARGET, MAX_BLOB_RETRY_BEFORE_DISCARDING, MAX_TRANSACTION_VALIDITY,
	MIN_BLOB_HOLDER_COUNT, MIN_BLOB_HOLDER_PERCENTAGE, MIN_TRANSACTION_VALIDITY,
};
use anyhow::{anyhow, Context, Result};
use avail_core::{BlockLengthColumns, BlockLengthRows};
use codec::{Decode, Encode};
use da_control::Call;
use da_runtime::{
	kate::{GDataProof, GProof, GRawScalar},
	RuntimeCall, UncheckedExtrinsic,
};
use kate::{
	com::Cell,
	couscous::multiproof_params,
	gridgen::core::{AsBytes as _, EvaluationGrid as EGrid},
	M1NoPrecomp, Seed,
};
use kate_recovery::{
	commons::ArkPublicParams,
	data::SingleCell,
	matrix::{Dimensions, Position},
	proof::verify_v2,
};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha20Rng;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use sc_client_api::HeaderBackend;
use sc_keystore::{Keystore, LocalKeystore};
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
	num::NonZeroU16,
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

			if meta.is_notified && meta.ownership.len() >= meta.nb_validators_per_blob as usize {
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
							log::info!(
								"INVALID BECAUSE OF NONCE, should_submit: {should_submit:?}"
							);
						},
						_ => {
							// Anything went wrong → submit so it disappears
							log::info!("INVALID BECAUSE OF SOMETHING ELSE, e: {e:?}");
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
	log::info!("Check retries for blob: {:?}", blob_hash);
	let tried = blob_store.get_blob_retry(blob_hash).unwrap_or(0);
	if tried <= MAX_BLOB_RETRY_BEFORE_DISCARDING {
		// bump retry count and wait
		let _ = blob_store.insert_blob_retry(blob_hash, tried + 1);
		log::info!("WE LET IT LIVE: {:?}", blob_hash);
		false
	} else {
		// give up: submit to get it dropped quickly
		log::info!("IT DIES: {:?}", blob_hash);
		true
	}
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
					let ownership = if let Some(b) = blob_metadata {
						b.ownership.clone()
					} else {
						Vec::new()
					};
					blob_txs_summary.push(BlobTxSummary {
						hash: blob_hash,
						success: false,
						reason: Some(reason),
						ownership,
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

	if !meta.is_notified {
		return Err("Blob metadata from the store is not notified, discarding it".into());
	}

	if meta.ownership.len() < meta.nb_validators_per_blob as usize {
		return Err("Not enough validators vouched for this block".into());
	}

	// Check if we have enough ownership entries with validated cells
	let valid_proof_count = get_valid_validators_proof_count(&meta);
	if valid_proof_count < meta.nb_validators_per_blob {
		return Err(format!(
			"Not enough valid proof count, expected:{} - found:{}",
			meta.nb_validators_per_blob, valid_proof_count
		));
	}

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
}

pub fn generate_pseudo_random_cells(
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

pub fn build_cell_proofs<Block: BlockT>(
	blob_meta: &BlobMetadata<Block>,
	blob: &Blob,
	my_validator_id: &AuthorityId,
) -> Result<BTreeMap<CellCoordinate, (GDataProof, Option<bool>)>> {
	// Compute the cells coordinates required for this blob
	// TODO Blobs: Take values from runtime
	let cols = 1024;
	let rows = (blob_meta.commitment.len() / 48) as u16;
	let Some(dimensions) = Dimensions::new(rows, cols) else {
		return Err(anyhow!(
			"Invalid dimensions rows: {} - cols: {}",
			rows,
			cols
		));
	};

	let cell_coordinates = generate_pseudo_random_cells(
		dimensions,
		CELL_COUNT,
		&blob_meta.finalized_block_hash.encode(),
		blob_meta.hash,
		&my_validator_id.encode(),
	);

	// TODO Blob: Fetch the configured tx grid dimension
	let rows = 4096;

	let pp = PP.get_or_init(multiproof_params);
	let grid = EGrid::from_data(
		blob.data.clone(),
		cols as usize,
		cols as usize,
		rows,
		Seed::default(),
	)
	.map_err(|e| anyhow!("construction from data failed: {e:?}"))?;
	let grid = EGrid::merge_with_padding(vec![grid])
		.map_err(|e| anyhow!("Merging grid failed: {e:?}"))?
		.extend_columns(NonZeroU16::new(2).expect("2 > 0"))
		.map_err(|e| anyhow!("Extension of grid failed even if dimensions are correct: {e:?}"))?;
	let poly = grid
		.make_polynomial_grid()
		.map_err(|e| anyhow!("polynomial grid construction failed: {e:?}"))?;

	let cell_proofs: BTreeMap<CellCoordinate, (GDataProof, Option<bool>)> = match cell_coordinates
		.into_par_iter()
		.map(|cell_coordinate| {
			let cell = grid.get(cell_coordinate.row as usize, cell_coordinate.col as usize);
			if cell.is_none() {
				return Err(anyhow!(
					"Missing cell at row: {}, col: {}",
					cell_coordinate.row,
					cell_coordinate.col
				));
			}

			let scalar = match cell.unwrap().to_bytes().map(GRawScalar::from) {
				Ok(s) => s,
				Err(e) => {
					return Err(anyhow!(
						"Invalid scalar at row: {}: {}",
						cell_coordinate.row,
						e
					));
				},
			};

			let raw_proof = match poly.proof(
				pp,
				&Cell::new(
					BlockLengthRows(cell_coordinate.row),
					BlockLengthColumns(cell_coordinate.col),
				),
			) {
				Ok(p) => p,
				Err(e) => {
					return Err(anyhow!(
						"Proof generation failed at ({}, {}): {:?}",
						cell_coordinate.row,
						cell_coordinate.col,
						e
					));
				},
			};

			let proof_bytes = match raw_proof.to_bytes() {
				Ok(b) => b.to_vec(),
				Err(e) => {
					return Err(anyhow!(
						"Failed to convert proof to bytes at row: {}, col: {}: {e}",
						cell_coordinate.row,
						cell_coordinate.col
					));
				},
			};

			let gproof = match GProof::try_from(proof_bytes) {
				Ok(p) => p,
				Err(e) => {
					return Err(anyhow!(
						"Invalid GProof encoding at row: {}, col: {}: {e}",
						cell_coordinate.row,
						cell_coordinate.col
					));
				},
			};

			Ok((cell_coordinate, ((scalar, gproof), Some(true))))
		})
		.collect::<Result<BTreeMap<_, _>, _>>()
	{
		Ok(proofs) => proofs,
		Err(e) => {
			return Err(anyhow!(
				"Proof generation failed for blob: {:?} - error: {:?}",
				blob_meta.hash,
				e,
			));
		},
	};

	Ok(cell_proofs)
}

pub fn validate_cell_proofs<Block: BlockT>(meta: &mut BlobMetadata<Block>) {
	if !meta.is_notified
		|| meta.ownership.iter().all(|o| {
			o.data_proofs
				.iter()
				.all(|(_, (_, maybe_valid))| maybe_valid.is_some())
		}) {
		// Nothing to do here
		return;
	}
	let extended_commitments: Vec<[u8; 48]> = match meta
		.extended_commitment
		.chunks_exact(48)
		.map(|c| <[u8; 48]>::try_from(c).map_err(|_| ()))
		.collect::<Result<_, _>>()
	{
		Ok(v) => v,
		Err(e) => {
			log::error!(
				"Could not build extended commitment for blob {:?}: {:?}",
				meta.hash,
				e
			);
			return;
		},
	};

	// TODO Blobs: Take values from runtime
	let cols = 1024;
	let rows = (meta.commitment.len() / 48) as u16;
	let dimensions = match Dimensions::new(rows, cols) {
		Some(d) => d,
		None => {
			log::error!("Invalid dimension for blob {:?}", meta.hash,);
			return;
		},
	};

	let pp = PP.get_or_init(multiproof_params);

	for entry in &mut meta.ownership {
		if entry.data_proofs.is_empty() {
			log::error!("Empty data proofs for ownership");
			continue;
		}

		if entry
			.data_proofs
			.iter()
			.all(|(_, (_, maybe_valid))| maybe_valid.is_some())
		{
			// Nothing to do here
			continue;
		}

		let expected_cells = generate_pseudo_random_cells(
			dimensions,
			CELL_COUNT,
			&meta.finalized_block_hash.encode(),
			meta.hash,
			&entry.address.encode(),
		);

		for cell_coordinate in expected_cells {
			let Some((proof, valid)) = entry.data_proofs.get_mut(&cell_coordinate) else {
				log::error!("Did not found expected coordinates in ownership");
				continue;
			};
			if !valid.is_some() {
				if let Err(e) = verify_cell_proof(
					&extended_commitments,
					dimensions,
					pp,
					cell_coordinate,
					proof,
				) {
					log::error!("Invalid cell proof: {e:?}");
					*valid = Some(false);
				} else {
					*valid = Some(true);
				}
			}
		}
	}
}

fn verify_cell_proof(
	extended_commitments: &Vec<[u8; 48]>,
	dimensions: Dimensions,
	pp: &M1NoPrecomp,
	cell_coordinate: CellCoordinate,
	proof: &GDataProof,
) -> Result<()> {
	let mut data_proof = [0u8; 80];
	data_proof[..48].copy_from_slice(&proof.1.encode());
	let mut data_bytes = [0u8; 32];
	proof.0.to_big_endian(&mut data_bytes);
	data_proof[48..].copy_from_slice(&data_bytes);

	let cell = SingleCell::new(
		Position {
			row: cell_coordinate.row,
			col: cell_coordinate.col as u16,
		},
		data_proof,
	);

	let commitment = extended_commitments
		.get(cell_coordinate.row as usize)
		.ok_or_else(|| anyhow::anyhow!("Missing commitment for row {}", cell_coordinate.row))?;

	if let Err(e) = verify_v2(pp, dimensions, commitment, &cell) {
		log::warn!(
			target: LOG_TARGET,
			"Cell proof verification failed for {:?}: {e}",
			cell_coordinate
		);
		return Err(anyhow::anyhow!(
			"Cell proof verification failed at row {}, col {}",
			cell_coordinate.row,
			cell_coordinate.col
		));
	}

	Ok(())
}

fn get_valid_validators_proof_count<Block: BlockT>(meta: &BlobMetadata<Block>) -> u32 {
	let mut valid_validator_count = 0;
	for entry in &meta.ownership {
		let valid_proof_count = entry
			.data_proofs
			.values()
			.filter(|(_, valid)| *valid == Some(true))
			.count();

		if valid_proof_count >= CELL_COUNT as usize {
			valid_validator_count += 1;
		}
	}
	valid_validator_count
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

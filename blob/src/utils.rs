use crate::{
	p2p::BlobHandle,
	process_cell_request_inner, send_cell_request,
	store::{RocksdbShardStore, ShardStore},
	types::{
		BlobHash, BlobMetadata, BlobSignatureData, BlobTxSummary, CellCoordinate, CellRequest,
		OwnershipEntry, Shard, ShardRequest,
	},
	CELL_COUNT, LOG_TARGET, MAX_BLOB_RETRY_BEFORE_DISCARDING, MAX_TRANSACTION_VALIDITY,
	MIN_SHARD_HOLDER_COUNT, MIN_SHARD_HOLDER_PERCENTAGE, MIN_TRANSACTION_VALIDITY, SHARD_SIZE,
};
use anyhow::{anyhow, Context, Result};
use codec::{Decode, Encode};
use da_control::Call;
use da_runtime::{kate::GDataProof, RuntimeCall, UncheckedExtrinsic};
use futures::{stream::FuturesOrdered, StreamExt};
use kate::{
	couscous::multiproof_params,
	gridgen::core::AsBytes,
	pmp::{ark_bls12_381::Bls12_381, Commitment},
};
use kate_recovery::{
	commons::ArkPublicParams,
	data::SingleCell,
	matrix::{Dimensions, Position},
	proof::verify_v2,
};
use rand::{thread_rng, Rng};
use sc_client_api::HeaderBackend;
use sc_keystore::{Keystore, LocalKeystore};
use sc_network::{NetworkService, NetworkStateInfo, PeerId};
use sc_transaction_pool_api::TransactionSource;
use sp_api::ProvideRuntimeApi;
use sp_authority_discovery::AuthorityId;
use sp_core::sr25519;
use sp_runtime::{
	key_types,
	traits::{Block as BlockT, Verify},
	SaturatedConversion,
};
use sp_transaction_pool::runtime_api::TaggedTransactionQueue;
use std::{
	collections::{BTreeMap, HashSet},
	str::FromStr,
	sync::Arc,
};
type ArkCommitment = Commitment<Bls12_381>;

static PP: std::sync::OnceLock<ArkPublicParams> = std::sync::OnceLock::new();

/// Get the number of shard for a blob based on its size
pub fn get_nb_shards_from_blob_size(size: usize) -> u16 {
	let shard_size = SHARD_SIZE as usize;
	((size + shard_size - 1) / shard_size).saturated_into()
}

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

/// Get the number of validator that need to store a single shard.
pub fn get_validator_per_shard(nb_validators: u32) -> u32 {
	if nb_validators <= MIN_SHARD_HOLDER_COUNT {
		return nb_validators;
	} else {
		let percentage = MIN_SHARD_HOLDER_PERCENTAGE.mul_ceil(nb_validators);
		return percentage.max(MIN_SHARD_HOLDER_COUNT);
	}
}

/// Decide deterministically whether this node should fetch/store shard `shard_index`
/// of blob `blob_hash`, given the full sorted list of validators.
/// Returns `true` if I am one of the `num_replicas` replicas for that shard.
pub fn get_shards_to_store(
	blob_hash: BlobHash,
	nb_shards: u16,
	validators: &Vec<AuthorityId>,
	my_id: &AuthorityId,
) -> Result<Vec<u16>> {
	let nb_validators = validators.len() as u32;
	let nb_validators_per_shard = get_validator_per_shard(nb_validators);

	if nb_validators == 0 || nb_validators_per_shard == 0 {
		return Ok(Vec::new());
	}

	let my_pos = match validators.iter().position(|v| v == my_id) {
		Some(p) => p,
		None => return Ok(Vec::new()), // We're not in the validator set
	};

	let hash_bytes = blob_hash.as_bytes();
	let truncated = hash_bytes
		.get(..8)
		.ok_or(anyhow!("Blob hash is too short, expected at least 8 bytes"))?;
	let array: [u8; 8] = truncated
		.try_into()
		.map_err(|_| anyhow!("Failed to convert first 8 bytes of blob hash"))?;
	let seed = u64::from_le_bytes(array);

	let mut shards_to_store = Vec::new();

	let ring_size = nb_validators as u64;
	for shard_index in 0..nb_shards {
		let shard_seed = seed.wrapping_add(shard_index as u64);
		let base_index = (shard_seed % ring_size) as usize;

		for i in 0..nb_validators_per_shard {
			let index = ((base_index as u32) + i) % nb_validators;
			if index as usize == my_pos {
				shards_to_store.push(shard_index);
				break;
			}
		}
	}

	log::info!(target: LOG_TARGET, "Validator: {my_id:?} should store shards: {shards_to_store:?}");

	Ok(shards_to_store)
}

pub fn fetch_shards<Block: BlockT>(
	store: &RocksdbShardStore<Block>,
	shard_request: &ShardRequest,
) -> Result<Vec<Shard>> {
	let shards = shard_request
		.shard_ids
		.iter()
		.try_fold(Vec::new(), |mut acc, &shard_id| {
			match store
				.get_shard(&shard_request.hash, shard_id)
				.map_err(|e| anyhow!("Could not decode shard from db: {e}"))?
			{
				Some(shard) => acc.push(shard),
				None => {},
			}
			Ok(acc)
		});

	shards
}

pub fn check_if_wait_next_block<C, Block>(
	client: &Arc<C>,
	shard_store: &Arc<RocksdbShardStore<Block>>,
	encoded: Vec<u8>,
	submit_blob_metadata_calls: &mut Vec<RuntimeCall>,
	blob_metadata: &mut BTreeMap<BlobHash, BlobMetadata<Block>>,
	nb_validators_per_shard: usize,
) -> (bool, bool)
where
	Block: BlockT,
	C: HeaderBackend<Block> + ProvideRuntimeApi<Block>,
	C::Api: TaggedTransactionQueue<Block>,
{
	let mut should_continue = false;
	let mut submit_blob_metadata_pushed = false;
	if let Some(UncheckedExtrinsic {
		function: extrinsic_data,
		..
	}) = Decode::decode(&mut &encoded[..]).ok()
	{
		if let RuntimeCall::DataAvailability(Call::submit_blob_metadata { blob_hash, .. }) =
			&extrinsic_data
		{
			let mut should_submit = true;
			let mut should_check_validity = false;
			match shard_store.get_blob_metadata(blob_hash) {
				// If we have metadata…
				Ok(Some(meta)) => {
					blob_metadata.insert(meta.hash, meta.clone());
					// check if every shard has non-empty ownership
					let metadata_valid = meta.is_notified
						&& (0..meta.nb_shards).all(|i| {
							meta.ownership
								.get(&i)
								.map_or(false, |v| v.len() >= nb_validators_per_shard)
						});

					// If ownership is complete, no changes, if not, we whould recheck validity to see if we can wait for ownership data to arrive
					if !metadata_valid {
						should_check_validity = true;
					}
				},
				// Err or Ok(None), definitely submit the tx so it fails and disappear.
				// We still let a chance in case the transaction was only tried once, valid blob metadata may arrive soon.
				_ => {
					let tried_count = shard_store.get_blob_retry(blob_hash).unwrap_or(0);
					if tried_count < MAX_BLOB_RETRY_BEFORE_DISCARDING {
						log::info!(target: LOG_TARGET, "A transaction will be retried after being tried {tried_count} - blob hash: {blob_hash}");
						should_check_validity = true;
						let _ = shard_store.insert_blob_retry(blob_hash, tried_count + 1);
					} else {
						// Transaction will be submitted and discarded
						log::info!(target: LOG_TARGET, "A transaction will be discarded after being tried {tried_count} - blob hash: {blob_hash}");
					}
				},
			};

			if should_check_validity {
				// Ownership incomplete or the blob was not notified yet or we don't have the blob yet
				// Re-check transaction validity to see if we can wait next block
				let still_valid = codec::Decode::decode(&mut &encoded[..])
					.ok()
					.and_then(|tx| {
						client
							.runtime_api()
							.validate_transaction(
								client.info().best_hash,
								TransactionSource::External,
								tx,
								client.info().best_hash,
							)
							.ok()
							.and_then(Result::ok)
					});

				// if we failed any step → assume invalid and submit so it disappears;
				// otherwise only submit if longevity is out of your bounds (to discard the tx)
				should_submit = still_valid.map_or(true, |v| {
					v.longevity < MIN_TRANSACTION_VALIDITY || v.longevity > MAX_TRANSACTION_VALIDITY
				});
			}

			if should_submit {
				submit_blob_metadata_calls.push(extrinsic_data);
				submit_blob_metadata_pushed = true;
			} else {
				should_continue = true;
			}
		}
	}
	(should_continue, submit_blob_metadata_pushed)
}

pub async fn sample_and_get_failed_blobs<Block: BlockT>(
	submit_blob_metadata_calls: &Vec<RuntimeCall>,
	network: Arc<NetworkService<Block, Block::Hash>>,
	keystore: &Arc<LocalKeystore>,
	shard_store: &Arc<RocksdbShardStore<Block>>,
	blob_metadata: BTreeMap<BlobHash, BlobMetadata<Block>>,
) -> (Vec<BlobTxSummary>, u64) {
	let mut tx_futures = FuturesOrdered::new();

	for tx in submit_blob_metadata_calls.iter().cloned() {
		if let RuntimeCall::DataAvailability(Call::submit_blob_metadata {
			size,
			blob_hash,
			commitments,
		}) = tx
		{
			let network = Arc::clone(&network);
			let keystore = Arc::clone(keystore);
			let blob_metadata = blob_metadata.clone();
			log::debug!(
				target: LOG_TARGET,
				"Verifying random cell proofs for blob {:?}",
				blob_hash
			);
			tx_futures.push_back(async move {
				handle_blob_sample_request(
					blob_hash,
					size,
					commitments,
					network,
					keystore,
					&blob_metadata,
					shard_store,
				)
				.await
			});
		}
	}

	let mut blob_txs_summary: Vec<BlobTxSummary> = Vec::new();
	let mut total_size = 0;
	while let Some(res) = tx_futures.next().await {
		match res {
			Ok((entry, size)) => {
				total_size += size;
				blob_txs_summary.push(entry);
			},
			Err((blob_hash, reason)) => {
				log::error!(
					target: LOG_TARGET,
					"Failed to verify cell proofs for {:?}: {:?}",
					blob_hash, reason
				);
				blob_txs_summary.push(BlobTxSummary {
					hash: blob_hash,
					success: false,
					reason: Some(reason),
					ownership: BTreeMap::new(),
				});
			},
		}
	}

	(blob_txs_summary, total_size)
}

async fn handle_blob_sample_request<Block: BlockT>(
	blob_hash: BlobHash,
	size: u64,
	commitments: Vec<u8>,
	network: Arc<NetworkService<Block, Block::Hash>>,
	keystore: Arc<LocalKeystore>,
	blob_metadata: &BTreeMap<BlobHash, BlobMetadata<Block>>,
	shard_store: &Arc<RocksdbShardStore<Block>>,
) -> Result<(BlobTxSummary, u64), (BlobHash, String)> {
	let meta = match blob_metadata.get(&blob_hash) {
		Some(m) => m,
		None => {
			return Err((
				blob_hash,
				"Blob metadata not found in the store to sample the blob".into(),
			));
		},
	};

	if meta.size != size || meta.commitments != commitments {
		return Err((
			blob_hash,
			"Blob metadata from the store did not match the one from the transaction".into(),
		));
	}

	let original_commitments: Vec<ArkCommitment> = commitments
		.chunks_exact(48)
		.enumerate()
		.map(|(i, chunk)| {
			let chunk_array: [u8; 48] = chunk.try_into().map_err(|_| {
				(
					blob_hash,
					format!("Chunk at index {i} is not 48 bytes long"),
				)
			})?;

			ArkCommitment::from_bytes(&chunk_array).map_err(|e| {
				log::error!(target: LOG_TARGET, "Invalid commitment at index {i}: {e}");
				(blob_hash, format!("Invalid commitment at index {i}: {e}"))
			})
		})
		.collect::<Result<_, _>>()?;

	let extended_commitments =
		ArkCommitment::extend_commitments(&original_commitments, original_commitments.len() * 2)
			.expect("extending commitments should work if dimensions are valid");
	let commitments_bytes: Vec<_> = extended_commitments
		.into_iter()
		.map(|c| {
			c.to_bytes()
				.expect("Valid commitments should be serialisable")
		})
		.collect();
	let rows = original_commitments.len();
	let cols = 1024;

	let dimension = Dimensions::new(rows as u16, cols)
		.ok_or_else(|| (blob_hash, "Invalid dimension".into()))?;

	let cells = generate_random_cells(dimension, CELL_COUNT);

	let signature_payload = build_cell_signature_payload(blob_hash, cells.clone());
	let signature_data = sign_blob_data_inner(&keystore, signature_payload)
		.map_err(|e| (blob_hash, format!("Signing failed: {}", e)))?;

	let req = CellRequest {
		hash: blob_hash,
		cells,
		signature_data,
	};

	let peers = get_blob_owners(meta.ownership.clone());
	if peers.is_empty() {
		return Err((blob_hash, "No peers available".into()));
	}
	let local_peer_id = network.local_peer_id();
	if peers.contains(&local_peer_id) {
		log::debug!(target: LOG_TARGET, "Trying to verify cell proofs locally for blob {:?}", blob_hash);
		match process_cell_request_inner(req.clone(), shard_store) {
			Ok(response) => {
				if verify_cell_proofs(
					response.cell_proofs,
					commitments_bytes.clone(),
					req.clone(),
					dimension,
				)
				.is_ok()
				{
					let entry = BlobTxSummary {
						hash: blob_hash,
						success: true,
						reason: None,
						ownership: meta.ownership.clone(),
					};
					return Ok((entry, size));
				} else {
					log::warn!(
						target: LOG_TARGET,
						"Invalid cell proof from peer {:?} for blob {:?}",
						local_peer_id,
						blob_hash
					);
				}
			},
			Err(e) => {
				log::warn!(
					target: LOG_TARGET,
					"Failed to get cell proof from peer {:?} for blob {:?}: {}",
					local_peer_id,
					blob_hash,
					e
				);
			},
		};
	}

	for peer in peers {
		if peer == local_peer_id {
			continue; // Already tried local
		}
		match send_cell_request(req.clone(), &network, peer).await {
			Ok(response) => {
				if verify_cell_proofs(
					response.cell_proofs,
					commitments_bytes.clone(),
					req.clone(),
					dimension,
				)
				.is_ok()
				{
					let entry = BlobTxSummary {
						hash: blob_hash,
						success: true,
						reason: None,
						ownership: meta.ownership.clone(),
					};
					return Ok((entry, size));
				} else {
					log::warn!(
						target: LOG_TARGET,
						"Invalid cell proof from peer {:?} for blob {:?}",
						peer,
						blob_hash
					);
				}
			},
			Err(e) => {
				log::warn!(
					target: LOG_TARGET,
					"Failed to get cell proof from peer {:?} for blob {:?}: {}",
					peer,
					blob_hash,
					e
				);
			},
		}
	}

	// All peers failed or returned invalid/missing proofs
	Err((
		blob_hash,
		"All peers failed to return valid cell proof".into(),
	))
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

fn get_blob_owners(owners: BTreeMap<u16, Vec<OwnershipEntry>>) -> Vec<PeerId> {
	let mut unique_peers = HashSet::new();

	for entries in owners.values() {
		for entry in entries {
			if let Ok(peer_id) = PeerId::from_str(&entry.peer_id_encoded) {
				unique_peers.insert(peer_id);
			} else {
				log::warn!(target: "blob", "Invalid PeerId string: {}", entry.peer_id_encoded);
			}
		}
	}

	unique_peers.into_iter().collect()
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

pub fn build_signature_payload(
	blob_hash: BlobHash,
	shard_ids: Vec<u16>,
	additional: Vec<u8>,
) -> Vec<u8> {
	let mut payload = Vec::new();
	payload.extend(blob_hash.encode());
	payload.extend(shard_ids.encode());
	payload.extend(additional.encode());
	payload
}

pub fn build_cell_signature_payload(blob_hash: BlobHash, cells: Vec<CellCoordinate>) -> Vec<u8> {
	let mut payload = Vec::new();
	payload.extend(blob_hash.encode());
	payload.extend(cells.encode());
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

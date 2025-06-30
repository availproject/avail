use crate::{
	send_cell_request,
	store::{RocksdbShardStore, ShardStore},
	types::{
		BlobHash, BlobMetadata, CellRequest, CellUnitRequest, FullClient, Shard, ShardRequest,
	},
	LOG_TARGET, MAX_TRANSACTION_VALIDITY, MIN_SHARD_HOLDER_COUNT, MIN_SHARD_HOLDER_PERCENTAGE,
	MIN_TRANSACTION_VALIDITY, SHARD_SIZE,
};
use anyhow::{anyhow, Result};
use codec::Decode;
use da_control::Call;
use da_runtime::{RuntimeCall, UncheckedExtrinsic};
use sc_authority_discovery::AuthorityDiscovery;
use sc_client_api::HeaderBackend;
use sc_keystore::{Keystore, LocalKeystore};
use sc_network::{NetworkService, NetworkStateInfo, PeerId};
use sc_transaction_pool_api::TransactionSource;
use sp_api::ProvideRuntimeApi;
use sp_authority_discovery::AuthorityId;
use sp_runtime::{key_types, traits::Block as BlockT, SaturatedConversion};
use sp_transaction_pool::runtime_api::TaggedTransactionQueue;
use std::{collections::BTreeMap, str::FromStr, sync::Arc};

/// Get the number of shard for a blob based on its size
pub fn get_nb_shards_from_blob_size(size: usize) -> u16 {
	let shard_size = SHARD_SIZE as usize;
	((size + shard_size - 1) / shard_size).saturated_into()
}

/// Get the current elected validators
pub async fn get_validators(client: &Arc<FullClient>) -> Result<Vec<AuthorityId>> {
	let best_hash = client.info().best_hash;
	client.authorities(best_hash).await.map_err(|e| {
		return anyhow!("Could not get validators: {e:?}");
	})
}

/// Get this node Authority ID
pub fn get_my_validator_id(keystore: &Arc<LocalKeystore>) -> Result<Option<AuthorityId>> {
	let key_type = key_types::BABE;

	// Try to get keys from the keystore
	let Ok(keys) = keystore.keys(key_type) else {
		return Err(anyhow!(
			"Could not get keys from keystore, not storing blobs"
		));
	};

	// Return None if no keys are in the keystore
	if keys.len() == 0 {
		return Ok(None);
	}

	// Try to get the last created key from the keystore
	let Some(key) = keys.get(keys.len() - 1) else {
		return Err(anyhow!(
			"An error has occured while trying to get the key from the keystore"
		));
	};

	let Ok(my_address) = AuthorityId::decode(&mut &key[..]) else {
		return Err(anyhow!(
			"Could not decode malformed BABE key from the keystore"
		));
	};

	Ok(Some(my_address))
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

pub fn fetch_shards(store: &RocksdbShardStore, shard_request: &ShardRequest) -> Result<Vec<Shard>> {
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
	shard_store: &Arc<RocksdbShardStore>,
	encoded: Vec<u8>,
	submit_blob_metadata_calls: &mut Vec<RuntimeCall>,
	blob_metadata: &mut BTreeMap<BlobHash, BlobMetadata>,
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
			let should_submit = match shard_store.get_blob_metadata(blob_hash) {
				// If we have metadata…
				Ok(Some(meta)) => {
					blob_metadata.insert(meta.hash, meta.clone());
					// check if every shard has non-empty ownership
					let ownership_valid = (0..meta.nb_shards)
						.all(|i| meta.ownership.get(&i).map_or(false, |v| !v.is_empty()));

					if ownership_valid {
						// ownership already complete, we can submit it properly
						true
					} else {
						// ownership incomplete, re-check transaction validity to see if we can wait next block
						let still_valid =
							codec::Decode::decode(&mut &encoded[..])
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
						// otherwise only submit if longevity is out of your bounds
						still_valid.map_or(true, |v| {
							v.longevity < MIN_TRANSACTION_VALIDITY
								|| v.longevity > MAX_TRANSACTION_VALIDITY
						})
					}
				},
				// Err or Ok(None), definitely submit the tx so it fails and disappear.
				_ => true,
			};
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

pub async fn sample_and_get_failed_blobs<Block>(
	submit_blob_metadata_calls: &Vec<RuntimeCall>,
	network: Arc<NetworkService<Block, <Block as BlockT>::Hash>>,
	blob_metadata: BTreeMap<BlobHash, BlobMetadata>,
) -> Vec<(BlobHash, String)>
where
	Block: BlockT,
{
	let mut failed_txs: Vec<(BlobHash, String)> = Vec::new();
	for tx in submit_blob_metadata_calls {
		if let RuntimeCall::DataAvailability(Call::submit_blob_metadata {
			size,
			blob_hash,
			commitments,
		}) = tx
		{
			// Get blob metadata from storage
			let Some(blob_metadata) = blob_metadata.get(blob_hash) else {
				failed_txs.push((
					*blob_hash,
					"Blob metadata not found in the store to sample the blob".to_string(),
				));
				continue;
			};

			// Check that values match
			if blob_metadata.size != *size || blob_metadata.commitments != *commitments {
				failed_txs.push((
					*blob_hash,
					"Blob metadata from the store did not match the one from the transaction"
						.to_string(),
				));
				continue;
			}

			// TODO Blob GET REAL SHARDS NEEDED FOR SAMPLING, for now we sample a pseudo random blob for fun
			let some_bytes: [u8; 8] = blob_hash.0[0..8].try_into().unwrap();
			let some_number: u64 = u64::from_le_bytes(some_bytes);
			let some_index: u16 = (some_number % (blob_metadata.nb_shards as u64))
				.try_into()
				.unwrap();

			// TODO Blob Select peer to target with request, for now we select the first and fail if no response, will need retry mechanism and timeout.
			let Some(peers) = blob_metadata.ownership.get(&some_index) else {
				failed_txs.push((*blob_hash, format!("No ownership find in the blob_metadata for the shard {some_index} of blob hash {blob_hash}")));
				continue;
			};
			if peers.len() == 0 {
				failed_txs.push((*blob_hash, format!("Ownership is empty in the blob_metadata for the shard {some_index} of blob hash {blob_hash}")));
				continue;
			}
			let peer_base58 = &peers[peers.len() - 1].1;
			let Ok(peer) = PeerId::from_str(&peer_base58) else {
				failed_txs.push((*blob_hash, format!("Could not parse the peer Id for the shard {some_index} of blob hash {blob_hash}")));
				continue;
			};

			// TODO Blob we still need to sample our own shards if needed, for now ignore
			let my_peer_id = network.local_peer_id();
			if my_peer_id != peer {
				// TODO Blob Send request to get shards for each shard needed of the blobs, for now we get one random
				// TODO Blob Make sure we don't do a request to ourselves which is not required
				let cell_request = CellRequest {
					hash: *blob_hash,
					cell_units: vec![CellUnitRequest {
						shard_id: some_index,
						start: 0,
						end: if blob_metadata.size == 0 {
							0
						} else {
							(blob_metadata.size as f64 * 0.1).ceil() as u64
						},
					}],
				};

				// TODO Blob here we should keep track of already failed blobs so we don't request cells for nothing
				let _cell_response = match send_cell_request(cell_request, &network, peer).await {
					Ok(c) => c,
					Err(e) => {
						log::error!(target: LOG_TARGET, "Could not get response from peer {peer_base58} for shard {some_index} of blob hash {blob_hash}: {e}");
						failed_txs.push((*blob_hash, format!("Could not get response from peer {peer_base58} for shard {some_index} of blob hash {blob_hash}: {e}")));
						continue;
					},
				};

				// TODO Blob sampling
				log::info!("Should sample: size-{size:?} blob_hash-{blob_hash:?}");

				// TODO Blob For now we randomly put failed tx for fun
				if some_index % 2 == 1 {
					// Say if failed or not with a reason
					failed_txs.push((*blob_hash, format!("Fake reason for fun")));
				}
			}
		}
	}

	failed_txs
}

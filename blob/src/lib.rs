use crate::{p2p::NetworkHandle, types::BlobHash};
use anyhow::{anyhow, Result};
use codec::{Decode, Encode};
use da_control::Call;
use da_runtime::{RuntimeCall, UncheckedExtrinsic};
use once_cell::sync::OnceCell;
use sc_authority_discovery::AuthorityDiscovery;
use sc_client_api::HeaderBackend;
use sc_keystore::{Keystore, LocalKeystore};
use sc_network::{
	config::{IncomingRequest, NonDefaultSetConfig, OutgoingResponse, RequestResponseConfig, Role},
	IfDisconnected, NetworkRequest, NetworkService, ObservedRole, PeerId,
};
use sp_authority_discovery::AuthorityId;
use sp_runtime::{key_types, traits::Block as BlockT, Perbill};
use std::{path::Path, sync::Arc, time::Duration};
use store::{RocksdbShardStore, ShardStore};
use tempfile::TempDir;
use types::{
	BlobMetadata, BlobNotification, BlobRequest, BlobResponse, FullClient, Shard, ShardRequest,
	BLOB_NOTIF_PROTO, BLOB_REQ_PROTO,
};
pub mod p2p;
pub mod rpc;
pub mod store;
pub mod types;

/***** Make CLI args from this *****/
/// Maximum notification size, 1kb
pub const NOTIFICATION_MAX_SIZE: u64 = 1 * 1024;
/// Maximum request size, 1kb
pub const REQUEST_MAX_SIZE: u64 = 1 * 1024; // 1 kb
/// Maximum response size, 32mb
pub const RESPONSE_MAX_SIZE: u64 = 32 * 1024 * 1024; // 32 mb
/// Maximum request time
pub const REQUEST_TIME_OUT: Duration = Duration::from_secs(5);

/***** Make runtime variable from this *****/
/// Maximum size of a blob
pub const MAX_BLOB_SIZE: u64 = 1024 * 1024 * 1024; // 1 gb
/// Maximum shard size
pub const SHARD_SIZE: u64 = 16 * 1024 * 1024; // 16mb
/// Minimum percentage of validator that needs to store a shard, we take the maximum between this and MIN_SHARD_HOLDER_COUNT
pub const MIN_SHARD_HOLDER_PERCENTAGE: Perbill = Perbill::from_percent(10);
/// Minimum amount of validator that needs to store a shard, if we have less than minimum, everyone stores it.
/// We take the maximum between this and MIN_SHARD_HOLDER_COUNT
pub const MIN_SHARD_HOLDER_COUNT: u32 = 2; // TODO Put like 10 validators
/// Amount of block for which we need to store the blob metadata and shards.
pub const BLOB_TTL: u32 = 120_960; // 28 days
/// Amount of block for which the receiving rpc node need to store the blob metadata and shards.
pub const TMP_BLOB_TTL: u32 = 180; // 1 hour
/// Amount of block for which the transaction submitted through RPC is valid.
/// This value needs to handle the time to upload the blob to the store
pub const MIN_TRANSACTION_VALIDITY: u64 = 15; // 5 mn

pub fn start_blob_service<Block>(
	path: &Path,
	role: Role,
) -> (
	NonDefaultSetConfig,
	RequestResponseConfig,
	Arc<NetworkHandle<Block>>,
	Arc<RocksdbShardStore>,
)
where
	Block: BlockT,
{
	// Initialize the shard store
	let _db_path = path.join("blob_store"); // Todo use this one
	let blob_tmp_dir = TempDir::new().expect("failed to create temp dir for blob store");
	let db_path = blob_tmp_dir.path().join("blob_store");
	let shard_store =
		Arc::new(RocksdbShardStore::open(db_path).expect("opening RocksDB blob store failed"));

	// Initialize the blob annoucement notification protocol config
	let (blob_notif_cfg, blob_notif_service) = NonDefaultSetConfig::new(
		BLOB_NOTIF_PROTO,
		Vec::default(),
		NOTIFICATION_MAX_SIZE,
		None,
		Default::default(),
	);

	// Initialize the blob shard req/res protocol config
	let (request_sender, request_receiver) = async_channel::unbounded();
	let blob_req_res_cfg = RequestResponseConfig {
		name: BLOB_REQ_PROTO,
		fallback_names: vec![],
		max_request_size: REQUEST_MAX_SIZE,
		max_response_size: RESPONSE_MAX_SIZE,
		request_timeout: REQUEST_TIME_OUT,
		inbound_queue: Some(request_sender),
	};

	// Spawn the async task for notification and req/res protocols

	let blob_handle = Arc::new(p2p::spawn(
		shard_store.clone(),
		blob_notif_service,
		request_receiver,
		role,
	));

	(blob_notif_cfg, blob_req_res_cfg, blob_handle, shard_store)
}

pub async fn decode_blob_notification<Block>(
	data: &[u8],
	network: &Arc<OnceCell<Arc<NetworkService<Block, <Block as BlockT>::Hash>>>>,
	peer: PeerId,
	store: &RocksdbShardStore,
	keystore: &Arc<OnceCell<Arc<LocalKeystore>>>,
	my_role: &Role,
	client: &Arc<OnceCell<Arc<FullClient>>>,
) where
	Block: BlockT,
{
	match BlobNotification::decode(&mut &data[..]) {
		Ok(notification) => {
			log::info!("Decoded Blob notification {:?}", notification);
			let Some(net) = network.get() else {
				log::error!("Network not yet registered");
				return;
			};

			match notification {
				BlobNotification::Announce(blob_metadata) => {
					if !my_role.is_authority() {
						log::info!("Got a blob notification but ignoring it since this node is not an authority.");
						return;
					}

					let Some(keystore) = keystore.get() else {
						log::error!("Keystore not yet registered");
						return;
					};

					let my_validator_id = match get_my_validator_id(&keystore) {
						Ok(maybe_validator_id) => match maybe_validator_id {
							Some(v) => v,
							None => {
								log::info!("This node is not a current authority, skipping recording blobs");
								return;
							},
						},
						Err(e) => {
							log::error!("Error happened while trying to get this node's id: {e}");
							return;
						},
					};

					let Some(client) = client.get() else {
						log::error!("Client not yet registered");
						return;
					};

					let validators = match get_validators(client).await {
						Ok(v) => v,
						Err(e) => {
							log::error!("Could not get validator from the runtime: {e}");
							return;
						},
					};

					let shards_to_store = match get_shards_to_store(
						blob_metadata.clone(),
						&validators,
						my_validator_id,
					) {
						Ok(s) => s,
						Err(e) => {
							log::error!("Could not get shards to store: {e}");
							return;
						},
					};

					if shards_to_store.len() > 0 {
						let shard_request = BlobRequest::ShardRequest(ShardRequest {
							hash: blob_metadata.hash,
							shard_ids: shards_to_store,
						});
						let response = net
							.request(
								peer,
								BLOB_REQ_PROTO,
								shard_request.encode(),
								None,
								IfDisconnected::TryConnect,
							)
							.await;

						if let Ok((data, _proto)) = response {
							let mut buf: &[u8] = &data;
							match BlobResponse::decode(&mut buf) {
								Ok(response) => {
									log::info!("Got A response : {response:?}");
									match response {
										BlobResponse::ShardResponse(shards) => {
											match store.insert_shards(&shards){
												Ok(()) => log::info!("Stored {} shards in the db", shards.len()),
												Err(e) => log::error!("An error occured while trying to store shards: {e}"),
											};
										},
									}
								},
								Err(err) => {
									log::error!(
										"Failed to decode Blob response ({} bytes): {:?}",
										data.len(),
										err
									);
								},
							}
						}
					}
				},
			}
		},
		Err(err) => {
			log::error!(
				"Failed to decode Blob notification ({} bytes): {:?}",
				data.len(),
				err
			);
		},
	};
	let _ = store.log_all_entries();
}

pub async fn decode_blob_request(
	request: IncomingRequest,
	store: &RocksdbShardStore,
	peer_role: &Option<ObservedRole>,
) {
	let data = request.payload;
	let response_tx = request.pending_response;
	if !matches!(peer_role, Some(ObservedRole::Authority)) {
		log::info!("Got a blob request from a peer but the requester is not an authority, dropping the request.");
		let _ = response_tx.send(OutgoingResponse {
			result: Err(()),
			reputation_changes: Vec::default(),
			sent_feedback: None,
		});
		return;
	}

	let mut buf: &[u8] = &data;
	match BlobRequest::decode(&mut buf) {
		Ok(blob_request) => match blob_request {
			BlobRequest::ShardRequest(shard_request) => {
				log::info!("Decoded Blob request {:?}", shard_request);

				let shards_to_return = match fetch_shards(&store, &shard_request) {
					Ok(s) => s,
					Err(e) => {
						log::error!("Could not get shards: {e}");
						let _ = response_tx.send(OutgoingResponse {
							result: Err(()),
							reputation_changes: Vec::default(),
							sent_feedback: None,
						});
						return;
					},
				};

				let blob_response = BlobResponse::ShardResponse(shards_to_return);
				let res = OutgoingResponse {
					result: Ok(blob_response.encode()),
					reputation_changes: Vec::default(),
					sent_feedback: None,
				};

				let _ = response_tx.send(res);
			},
		},
		Err(err) => {
			log::error!(
				"Failed to decode Blob request ({} bytes): {:?}",
				data.len(),
				err
			);
		},
	}
	let _ = store.log_all_entries();
}

/// Get the current elected validators
async fn get_validators(client: &Arc<FullClient>) -> Result<Vec<AuthorityId>> {
	let best_hash = client.info().best_hash;
	client.authorities(best_hash).await.map_err(|e| {
		log::error!("Could not get validators: {e:?}");
		e.into()
	})
}

/// Get this node Authority ID
fn get_my_validator_id(keystore: &Arc<LocalKeystore>) -> Result<Option<AuthorityId>> {
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
fn get_validator_per_shard(nb_validators: u32) -> u32 {
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
fn get_shards_to_store(
	blob_metadata: BlobMetadata,
	validators: &Vec<AuthorityId>,
	my_id: AuthorityId,
) -> Result<Vec<u16>> {
	let nb_validators = validators.len() as u32;
	let nb_validators_per_shard = get_validator_per_shard(nb_validators);

	if nb_validators == 0 || nb_validators_per_shard == 0 {
		return Ok(Vec::new());
	}

	let my_pos = match validators.iter().position(|v| *v == my_id) {
		Some(p) => p,
		None => return Ok(Vec::new()), // We're not in the validator set
	};

	let hash_bytes = blob_metadata.hash.as_bytes();
	let truncated = hash_bytes
		.get(..8)
		.ok_or(anyhow!("Blob hash is too short, expected at least 8 bytes"))?;
	let array: [u8; 8] = truncated
		.try_into()
		.map_err(|_| anyhow!("Failed to convert first 8 bytes of blob hash"))?;
	let seed = u64::from_le_bytes(array);

	let mut shards_to_store = Vec::new();

	let ring_size = nb_validators as u64;
	for shard_index in 0..blob_metadata.nb_shard {
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

	log::info!("Validator: {my_id:?} should store shards: {shards_to_store:?}");

	Ok(shards_to_store)
}

fn fetch_shards(store: &RocksdbShardStore, shard_request: &ShardRequest) -> Result<Vec<Shard>> {
	shard_request
		.shard_ids
		.iter()
		.map(|&shard_id| {
			let maybe_bytes = store.get_shard(&shard_request.hash, shard_id)?;
			let bytes = maybe_bytes.ok_or_else(|| anyhow::anyhow!("Shard {shard_id} not found"))?;
			let mut buf: &[u8] = &bytes;
			Shard::decode(&mut buf).map_err(|e| anyhow!("Could not decode shard from db: {e}"))
		})
		.collect()
}

pub async fn check_and_sample_blobs(submit_blob_metadata_calls: &Vec<RuntimeCall>) -> Vec<RuntimeCall>
{
	let mut failed_txs: Vec<RuntimeCall> = Vec::new();
	for tx in submit_blob_metadata_calls {
		if let RuntimeCall::DataAvailability(Call::submit_blob_metadata { size, blob_hash, commitments }) = tx {
			// Todo sampling
		}
	}
	failed_txs
}

pub fn create_post_inherent(blob_hashes: Vec<BlobHash>) {
	// TODO CREATE INHERENT
	let call = RuntimeCall::DataAvailability(da_control::Call::failed_submit_blob_txs {
		failed_txs: blob_hashes,
	});
}

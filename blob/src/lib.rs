use crate::{
	p2p::BlobHandle,
	types::{
		BlobHash, BlobMetadata, BlobReceived, CellRequest, CellResponse, CellUnitResponse, Shard,
		ShardReceived, ShardResponse,
	},
	utils::{
		build_signature_payload, fetch_shards, get_my_validator_id, get_shards_to_store,
		sign_blob_data, verify_signed_blob_data,
	},
};
use anyhow::{anyhow, Result};
use codec::{Decode, Encode};
use futures::{channel::oneshot, stream::FuturesUnordered, StreamExt};
use sc_authority_discovery::AuthorityDiscovery;
use sc_client_api::HeaderBackend;
use sc_network::{
	config::{IncomingRequest, OutgoingResponse},
	IfDisconnected, NetworkPeers, NetworkRequest, NetworkService, NetworkStateInfo, ObservedRole,
	PeerId,
};
use sp_authority_discovery::AuthorityId;
use sp_runtime::{traits::Block as BlockT, Perbill, SaturatedConversion};
use std::{
	collections::BTreeMap, future::Future, pin::Pin, str::FromStr, sync::Arc, time::Duration,
};
use store::{RocksdbShardStore, ShardStore};
use types::{BlobNotification, BlobRequest, BlobResponse, ShardRequest, BLOB_REQ_PROTO};

pub mod p2p;
pub mod rpc;
pub mod store;
pub mod types;
pub mod utils;

pub(crate) const LOG_TARGET: &str = "avail::blob";

/***** TODO Blob: Make CLI / Runtime args from this, must be compatible with rpc flags *****/
/// Maximum notification size, 128kb
const NOTIFICATION_MAX_SIZE: u64 = 128 * 1024;
/// Maximum request size, 32kb
const REQUEST_MAX_SIZE: u64 = 32 * 1024;
/// Maximum response size, 65mb
const RESPONSE_MAX_SIZE: u64 = 65 * 1024 * 1024;
/// Maximum request time
const REQUEST_TIME_OUT: Duration = Duration::from_secs(5);
/// The maximum number of allowed concurrent request processing or notification processing
const CONCURRENT_REQUESTS: usize = 100;

// Business logic
/// Maximum size of a blob, need to change the rpc request and response size to handle this
const MAX_BLOB_SIZE: u64 = 64 * 1024 * 1024;
/// Maximum shard size
const SHARD_SIZE: u64 = 32 * 1024 * 1024;
/// Minimum percentage of validator that needs to store a shard, we take the maximum between this and MIN_SHARD_HOLDER_COUNT
const MIN_SHARD_HOLDER_PERCENTAGE: Perbill = Perbill::from_percent(10);
/// Minimum amount of validator that needs to store a shard, if we have less than minimum, everyone stores it.
/// We take the maximum between this and MIN_SHARD_HOLDER_COUNT
const MIN_SHARD_HOLDER_COUNT: u32 = 2; // TODO Blob Put like 10 validators
/// Amount of block for which we need to store the blob metadata and shards.
const BLOB_TTL: u64 = 120_960; // 28 days
/// Amount of block for which we need to store the blob metadata if the blob is not notified yet.
const TEMP_BLOB_TTL: u64 = 180;
/// Amount of blocks used to periodically check wether we should remove expired blobs or not.
const BLOB_EXPIRATION_CHECK_PERIOD: u64 = 180; // 1 hour
/// Min Amount of block for which the transaction submitted through RPC is valid.
/// This value needs to handle the time to upload the blob to the store
const MIN_TRANSACTION_VALIDITY: u64 = 15; // 5 mn
/// Max Amount of block for which the transaction submitted through RPC is valid.
/// This value is used so a transaction won't be stuck in the mempool.
const MAX_TRANSACTION_VALIDITY: u64 = 150; // 50 mn
/// The number of time we'll allow trying to fetch internal blob metadata before letting the transaction go through to get discarded
const MAX_BLOB_RETRY_BEFORE_DISCARDING: u16 = 1;
/// The number of block for which a notification is considered valid
const NOTIFICATION_EXPIRATION_PERIOD: u64 = 180;

pub async fn decode_blob_notification<Block>(data: &[u8], blob_handle: &BlobHandle<Block>)
where
	Block: BlockT,
{
	match BlobNotification::decode(&mut &data[..]) {
		Ok(notification) => match notification {
			BlobNotification::BlobReceived(blob_received) => {
				match PeerId::from_str(&blob_received.original_peer_id.clone()) {
					Ok(original_peer_id) => {
						handle_blob_received_notification::<Block>(
							blob_received,
							blob_handle,
							original_peer_id,
						)
						.await;
					},
					Err(e) => {
						log::error!(target: LOG_TARGET, "Could not decode peer id from Blob received notification: {e}");
					},
				}
			},
			BlobNotification::ShardReceived(shard_received) => {
				handle_shard_received_notification(shard_received, &blob_handle);
			},
		},
		Err(err) => {
			log::error!(
				target: LOG_TARGET,
				"Failed to decode Blob notification ({} bytes): {:?}",
				data.len(),
				err
			);
		},
	};
}

async fn handle_blob_received_notification<Block>(
	blob_received: BlobReceived<Block>,
	blob_handle: &BlobHandle<Block>,
	original_peer_id: PeerId,
) where
	Block: BlockT,
{
	log::info!(target: LOG_TARGET, "1 - Got a blob notification");

	let Some(client) = blob_handle.client.get() else {
		log::error!(target: LOG_TARGET, "Client not yet registered");
		return;
	};

	let Some(network) = blob_handle.network.get() else {
		log::error!(target: LOG_TARGET, "Network not yet registered");
		return;
	};

	let is_authority = blob_handle.role.is_authority();

	let announced_finalized_hash = blob_received.finalized_block_hash;
	let announced_finalized_number = blob_received.finalized_block_number;

	let Ok(Some(finalized_hash)) = client.hash(announced_finalized_number.saturated_into()) else {
		log::error!(target: LOG_TARGET, "Could not get announced block hash from backend");
		return;
	};

	if finalized_hash.encode() != announced_finalized_hash.encode() {
		log::error!(target: LOG_TARGET, "Invalid finalized block hash.");
		return;
	}

	let client_info = client.info();
	let finalized_block_number = client_info.finalized_number.saturated_into::<u64>();
	if announced_finalized_number > finalized_block_number
		|| finalized_block_number - announced_finalized_number > NOTIFICATION_EXPIRATION_PERIOD
	{
		log::error!(target: LOG_TARGET, "Invalid announced finalized block number.");
		return;
	}

	let maybe_metadata = match blob_handle
		.shard_store
		.get_blob_metadata(&blob_received.hash)
	{
		Ok(maybe_meta) => maybe_meta,
		Err(e) => {
			log::error!("Failed to check data from blob storage: {e}");
			return;
		},
	};

	// Get the existing blob or create a new one
	let expires_at = announced_finalized_number.saturating_add(BLOB_TTL);
	let mut blob_meta = maybe_metadata.unwrap_or_else(|| BlobMetadata {
		hash: blob_received.hash,
		size: blob_received.size,
		nb_shards: blob_received.nb_shards,
		commitments: blob_received.commitments.clone(),
		ownership: BTreeMap::new(),
		is_notified: true,
		expires_at: 0,
		finalized_block_hash: announced_finalized_hash,
		finalized_block_number: announced_finalized_number,
	});

	// If we already had the blob metadata but incomplete (is notified == false) we fill the missing data.
	if !blob_meta.is_notified {
		blob_meta.size = blob_received.size;
		blob_meta.nb_shards = blob_received.nb_shards;
		blob_meta.commitments = blob_received.commitments;
		blob_meta.is_notified = true;
	}
	// Here, no matter if it's a new blob or a duplicate, we set the new expiration time and update the ownership.
	blob_meta.expires_at = expires_at;
	blob_meta.merge_ownerships(blob_received.ownership);

	if is_authority {
		let Some(keystore) = blob_handle.keystore.get() else {
			log::error!(target: LOG_TARGET, "Keystore not yet registered");
			return;
		};

		let my_validator_id = match get_my_validator_id(&keystore) {
			Some(v) => v,
			None => {
				log::error!(target: LOG_TARGET, "No keys found while trying to get this node's id");
				return;
			},
		};

		let validators = match client.authorities(finalized_hash).await {
			Ok(v) => v,
			Err(e) => {
				log::error!(target: LOG_TARGET, "Could not get validator from the runtime: {e}");
				return;
			},
		};

		let shards_to_store = match get_shards_to_store(
			blob_received.hash,
			blob_received.nb_shards,
			&validators,
			&my_validator_id,
		) {
			Ok(s) => s,
			Err(e) => {
				log::error!(target: LOG_TARGET, "Could not get shards to store: {e}");
				return;
			},
		};

		if shards_to_store.len() > 0 {
			let my_peer_id = network.local_peer_id();
			let my_peer_id_base58 = my_peer_id.to_base58();
			let mut shard_to_request = Vec::new();
			let mut shard_already_stored = Vec::new();

			for shard_id in &shards_to_store {
				let already_stored =
					blob_meta.insert_in_ownership(shard_id, &my_validator_id, &my_peer_id_base58);
				if already_stored {
					shard_already_stored.push(*shard_id);
				} else {
					shard_to_request.push(*shard_id)
				}
			}

			send_shard_request(
				blob_received.hash,
				shard_to_request,
				shard_already_stored,
				blob_handle,
				network,
				original_peer_id,
				my_validator_id,
			)
			.await;
		}
	}

	match blob_handle
		.shard_store
		.insert_blob_metadata(&blob_meta.hash, &blob_meta)
	{
		Ok(()) => {
			log::info!(
				target: LOG_TARGET,
				"Stored blob metadata {} in the store db",
				blob_meta.hash
			)
		},
		Err(e) => {
			log::error!(
				target: LOG_TARGET,
				"An error occured while trying to store blob metadata {}: {}",
				blob_meta.hash,
				e
			)
		},
	};
}

async fn send_shard_request<Block>(
	blob_hash: BlobHash,
	shard_ids_to_request: Vec<u16>,
	shard_ids_already_stored: Vec<u16>,
	blob_handle: &BlobHandle<Block>,
	network: &Arc<NetworkService<Block, Block::Hash>>,
	original_peer_id: PeerId,
	my_validator_id: AuthorityId,
) where
	Block: BlockT,
{
	let shards_per_request = (RESPONSE_MAX_SIZE / SHARD_SIZE) as usize;
	if shards_per_request == 0 {
		log::error!(
			target: LOG_TARGET,
			"RESPONSE_MAX_SIZE ({}) is smaller than SHARD_SIZE ({}), cannot request any shard.",
			RESPONSE_MAX_SIZE,
			SHARD_SIZE
		);
		return;
	}

	let futures: FuturesUnordered<Pin<Box<dyn Future<Output = ()> + Send>>> =
		FuturesUnordered::new();

	for shard_ids_chunk in shard_ids_to_request.chunks(shards_per_request) {
		let signature_payload =
			build_signature_payload(blob_hash, shard_ids_chunk.to_vec(), b"shard".to_vec());
		let signature_data = match sign_blob_data(blob_handle, signature_payload) {
			Ok(s) => s,
			Err(e) => {
				log::error!(target: LOG_TARGET, "An error has occured while trying to sign data, exiting the function: {e}");
				return;
			},
		};

		let chunk_shard_request = ShardRequest {
			hash: blob_hash,
			shard_ids: shard_ids_chunk.to_vec(),
			signature_data,
		};
		let blob_request = BlobRequest::ShardRequest(chunk_shard_request);
		let my_validator_id = my_validator_id.clone();
		let blob_handle = blob_handle.clone();
		let network = network.clone();

		let fut = async move {
			log::info!(target: LOG_TARGET, "2 - Sending a shard request for {} shards", shard_ids_chunk.len());

			let response = network
				.request(
					original_peer_id,
					BLOB_REQ_PROTO,
					blob_request.encode(),
					None,
					IfDisconnected::TryConnect,
				)
				.await;

			match response {
				Ok((data, _proto)) => {
					let mut buf: &[u8] = &data;
					match BlobResponse::decode(&mut buf) {
						Ok(response) => match response {
							BlobResponse::ShardResponse(shard_response) => {
								process_shard_response(
									shard_response,
									&blob_handle,
									my_validator_id,
								)
								.await;
							},
							BlobResponse::CellResponse(_) => {
								log::error!(
									target: LOG_TARGET,
									"Invalid response in send shard request, got CellResponse, expected ShardResponse"
								);
							},
						},
						Err(err) => {
							log::error!(
								target: LOG_TARGET,
								"Failed to decode Blob response ({} bytes): {:?}",
								data.len(),
								err
							);
						},
					}
				},
				Err(e) => {
					log::error!(target: LOG_TARGET, "An error has occured while trying to send shard request for a chunk: {e}");
				},
			}
		};

		futures.push(Box::pin(fut));
	}

	if shard_ids_already_stored.len() > 0 {
		let my_validator_id = my_validator_id.clone();
		let blob_handle = blob_handle.clone();
		let shard_response = ShardResponse {
			hash: blob_hash,
			shards: shard_ids_already_stored
				.into_iter()
				.map(|shard_id| Shard {
					blob_hash,
					shard_id,
					data: Vec::new(), // We don't care about those values as we just want the ID to send the response.
					size: 0,
				})
				.collect(),
		};
		futures.push(Box::pin(async move {
			send_shard_received_notification(shard_response, &blob_handle, my_validator_id).await;
		}));
	}

	futures.collect::<()>().await;
}

pub fn handle_incoming_blob_request<Block: BlockT>(
	request: IncomingRequest,
	store: &RocksdbShardStore<Block>,
	network: &Arc<NetworkService<Block, Block::Hash>>,
) where
	Block: BlockT,
{
	log::info!(target: LOG_TARGET, "X - In handle incoming blob request");
	let data = request.payload;
	let response_tx = request.pending_response;
	let peer_id = request.peer;
	let role = network.peer_role(peer_id, Vec::new());
	if role != Some(ObservedRole::Authority) {
		log::error!(
			target: LOG_TARGET,
			"Not answering to {peer_id:?} as it's not an authority.",
		);
	}

	let mut buf: &[u8] = &data;
	match BlobRequest::decode(&mut buf) {
		Ok(blob_request) => match blob_request {
			BlobRequest::ShardRequest(shard_request) => {
				process_shard_request(shard_request, store, response_tx);
			},
			BlobRequest::CellRequest(cell_request) => {
				process_cell_request(cell_request, store, response_tx);
			},
		},
		Err(err) => {
			log::error!(
				target: LOG_TARGET,
				"Failed to decode Blob request ({} bytes): {:?}",
				data.len(),
				err
			);
			let _ = response_tx.send(OutgoingResponse {
				result: Err(()),
				reputation_changes: Vec::default(),
				sent_feedback: None,
			});
		},
	}
}

fn process_shard_request<Block: BlockT>(
	shard_request: ShardRequest,
	store: &RocksdbShardStore<Block>,
	response_tx: oneshot::Sender<OutgoingResponse>,
) {
	log::info!(target: LOG_TARGET, "3 - processing incoming shard_request");
	let expected_payload = build_signature_payload(
		shard_request.hash,
		shard_request.shard_ids.clone(),
		b"shard".to_vec(),
	);
	match verify_signed_blob_data(shard_request.signature_data.clone(), expected_payload) {
		Ok(valid) => {
			if !valid {
				log::error!(target: LOG_TARGET, "An error has occured in process_shard_request: Invalid signature");
				let _ = response_tx.send(OutgoingResponse {
					result: Err(()),
					reputation_changes: Vec::default(),
					sent_feedback: None,
				});
				return;
			}
		},
		Err(e) => {
			log::error!(target: LOG_TARGET, "An error has occured while checking the signature: {e}");
			let _ = response_tx.send(OutgoingResponse {
				result: Err(()),
				reputation_changes: Vec::default(),
				sent_feedback: None,
			});
			return;
		},
	}

	let shards = match fetch_shards(&store, &shard_request) {
		Ok(s) => s,
		Err(e) => {
			log::error!(target: LOG_TARGET, "Could not get shards: {e}");
			let _ = response_tx.send(OutgoingResponse {
				result: Err(()),
				reputation_changes: Vec::default(),
				sent_feedback: None,
			});
			return;
		},
	};

	let blob_response = BlobResponse::ShardResponse(ShardResponse {
		hash: shard_request.hash,
		shards,
	});
	let res = OutgoingResponse {
		result: Ok(blob_response.encode()),
		reputation_changes: Vec::default(),
		sent_feedback: None,
	};

	if let Err(e) = response_tx.send(res) {
		log::error!(target: LOG_TARGET, "An error has occured in process_shard_request: {e:?}");
	}
}

async fn process_shard_response<Block>(
	shard_response: ShardResponse,
	blob_handle: &BlobHandle<Block>,
	my_validator_id: AuthorityId,
) where
	Block: BlockT,
{
	log::info!(target: LOG_TARGET, "4 - processing shard_response");
	match blob_handle
		.shard_store
		.insert_shards(&shard_response.shards)
	{
		Ok(()) => {
			log::info!(target: LOG_TARGET, "Stored {} shards in the db", shard_response.shards.len());
			send_shard_received_notification(shard_response, blob_handle, my_validator_id).await;
		},
		Err(e) => {
			log::error!(target: LOG_TARGET, "An error occured while trying to store shards: {e}");
		},
	};
}

async fn send_shard_received_notification<Block>(
	shard_response: ShardResponse,
	blob_handle: &BlobHandle<Block>,
	my_validator_id: AuthorityId,
) where
	Block: BlockT,
{
	log::info!(target: LOG_TARGET, "5 - Sending shard received notification");
	let Some(network) = blob_handle.network.get() else {
		log::error!(target: LOG_TARGET, "Network not yet registered, could not send shard received notification");
		return;
	};

	let shard_ids: Vec<u16> = shard_response
		.shards
		.into_iter()
		.map(|shard| shard.shard_id)
		.collect();

	let signature_payload =
		build_signature_payload(shard_response.hash, shard_ids.clone(), b"received".to_vec());
	let signature_data = match sign_blob_data(blob_handle, signature_payload) {
		Ok(s) => s,
		Err(e) => {
			log::error!(target: LOG_TARGET, "An error has occured while trying to sign data, exiting the function: {e}");
			return;
		},
	};

	let shard_received = ShardReceived {
		hash: shard_response.hash,
		shard_ids,
		address: my_validator_id,
		original_peer_id: network.local_peer_id().to_base58(),
		signature_data,
	};

	if shard_received.shard_ids.len() > 0 {
		let Some(gossip_cmd_sender) = blob_handle.gossip_cmd_sender.get() else {
			log::error!(target: LOG_TARGET, "Could not send gossip notification since gossip_cmd_sender was not initialized");
			return;
		};

		if let Err(e) = gossip_cmd_sender
			.send(BlobNotification::ShardReceived(shard_received))
			.await
		{
			log::error!(target: LOG_TARGET, "Could not send ShardReceived notification: {e}")
		}
	}
}

fn handle_shard_received_notification<Block>(
	shard_received: ShardReceived,
	blob_handle: &BlobHandle<Block>,
) where
	Block: BlockT,
{
	log::info!(target: LOG_TARGET, "6 - Handling incoming shard received notification");

	let Some(client) = blob_handle.client.get() else {
		log::error!(target: LOG_TARGET, "Client not yet registered");
		return;
	};

	let expected_payload = build_signature_payload(
		shard_received.hash,
		shard_received.shard_ids.clone(),
		b"received".to_vec(),
	);
	match verify_signed_blob_data(shard_received.signature_data.clone(), expected_payload) {
		Ok(valid) => {
			if !valid {
				log::error!(target: LOG_TARGET, "An error has occured in handle_shard_received_notification: Invalid signature");
				return;
			}
		},
		Err(e) => {
			log::error!(target: LOG_TARGET, "An error has occured while checking the signature: {e}");
			return;
		},
	}

	let Ok(maybe_meta) = blob_handle
		.shard_store
		.get_blob_metadata(&shard_received.hash)
	else {
		log::error!(target: LOG_TARGET, "An error has occured while trying to get blob from the store");
		return;
	};

	let mut blob_metadata = maybe_meta.unwrap_or(BlobMetadata {
		hash: shard_received.hash,
		size: 0,
		nb_shards: 0,
		commitments: Vec::new(),
		ownership: BTreeMap::new(),
		is_notified: false,
		finalized_block_hash: Block::Hash::default(),
		finalized_block_number: 0,
		expires_at: client
			.info()
			.finalized_number
			.saturated_into::<u64>()
			.saturating_add(TEMP_BLOB_TTL),
	});
	if !blob_metadata.is_notified {
		log::info!(target: LOG_TARGET, "A Shard Received notification was received before Blob Received, creating empty blob metadata.");
	}

	for shard_id in shard_received.shard_ids {
		blob_metadata.insert_in_ownership(
			&shard_id,
			&shard_received.address,
			&shard_received.original_peer_id,
		);
	}

	if let Err(e) = blob_handle
		.shard_store
		.insert_blob_metadata(&blob_metadata.hash, &blob_metadata)
	{
		log::error!(
			target: LOG_TARGET,
			"An error has occured while trying to save blob_metadata in the store: {e}"
		);
	}
}

async fn send_cell_request<Block>(
	cell_request: CellRequest,
	network: &Arc<NetworkService<Block, Block::Hash>>,
	target_peer: PeerId,
) -> Result<CellResponse>
where
	Block: BlockT,
{
	log::info!(target: LOG_TARGET, "7 - Sending cell request");
	let blob_request = BlobRequest::CellRequest(cell_request);
	let response = network
		.request(
			target_peer,
			BLOB_REQ_PROTO,
			blob_request.encode(),
			None,
			IfDisconnected::TryConnect,
		)
		.await;

	match response {
		Ok((data, _proto)) => {
			let mut buf: &[u8] = &data;
			match BlobResponse::decode(&mut buf) {
				Ok(response) => match response {
					BlobResponse::ShardResponse(_) => {
						Err(anyhow!("Invalid response in send cell request, got ShardResponse, expected CellResponse"))
					},
					BlobResponse::CellResponse(cell_response) => {
						Ok(process_cell_response(cell_response))
					},
				},
				Err(err) => {
					Err(anyhow!("Failed to decode Blob response ({} bytes): {:?}", data.len(), err))
				},
			}
		},
		Err(e) => {
			log::error!(target: LOG_TARGET, "An error has occured while trying to send cell request: {e}");
			Err(anyhow!(format!(
				"An error has occured while trying to send cell request: {e}"
			)))
		},
	}
}

pub fn process_cell_request_inner<Block: BlockT>(
	cell_request: CellRequest,
	store: &RocksdbShardStore<Block>,
) -> CellResponse {
	log::info!(target: LOG_TARGET, "8 - Processing cell request");
	let mut cell_response = CellResponse {
		hash: cell_request.hash,
		cell_units_response: Vec::new(),
	};
	// Use CellRequest and store to get the required data
	for cell_unit in cell_request.cell_units {
		let mut cell_unit_response = CellUnitResponse {
			shard_id: cell_unit.shard_id,
			start: cell_unit.start,
			end: cell_unit.end,
			data: Vec::new(),
			failed_reason: None,
		};

		if cell_unit.start > cell_unit.end {
			cell_unit_response.failed_reason = Some(format!(
				"Invalid cell start and end index: (start:{} - end:{})",
				cell_unit.start, cell_unit.end
			));
			cell_response.cell_units_response.push(cell_unit_response);
			continue;
		}

		let Ok(shard_data) = store.get_shard(&cell_request.hash, cell_unit.shard_id) else {
			cell_unit_response.failed_reason = Some(format!(
				"Failed to get shard_data from store for shard: {}",
				cell_unit.shard_id
			));
			cell_response.cell_units_response.push(cell_unit_response);
			continue;
		};

		let Some(shard_data) = shard_data else {
			cell_unit_response.failed_reason = Some(format!(
				"Could not find shard_data in store for shard: {}",
				cell_unit.shard_id
			));
			cell_response.cell_units_response.push(cell_unit_response);
			continue;
		};

		if cell_unit.end > shard_data.size {
			cell_unit_response.failed_reason = Some(format!(
				"Invalid cell end index: (end:{} - total size:{})",
				cell_unit.end, shard_data.size
			));
			cell_response.cell_units_response.push(cell_unit_response);
			continue;
		}

		let data = shard_data.data[cell_unit.start as usize..cell_unit.end as usize].to_vec();

		cell_unit_response.data = data;

		cell_response.cell_units_response.push(cell_unit_response);
	}
	cell_response
}

pub fn process_cell_request<Block: BlockT>(
	cell_request: CellRequest,
	store: &RocksdbShardStore<Block>,
	response_tx: oneshot::Sender<OutgoingResponse>,
) {
	let shard_ids: Vec<u16> = cell_request
		.cell_units
		.iter()
		.map(|cell_unit| cell_unit.shard_id)
		.collect();
	let expected_payload = build_signature_payload(
		cell_request.hash,
		shard_ids,
		b"cell".to_vec(),
	);
	match verify_signed_blob_data(cell_request.signature_data.clone(), expected_payload) {
		Ok(valid) => {
			if !valid {
				log::error!(target: LOG_TARGET, "An error has occured in process_cell_request: Invalid signature");
				let _ = response_tx.send(OutgoingResponse {
					result: Err(()),
					reputation_changes: Vec::default(),
					sent_feedback: None,
				});
				return;
			}
		},
		Err(e) => {
			log::error!(target: LOG_TARGET, "An error has occured while checking the signature: {e}");
			let _ = response_tx.send(OutgoingResponse {
				result: Err(()),
				reputation_changes: Vec::default(),
				sent_feedback: None,
			});
			return;
		},
	}

	let cell_response = process_cell_request_inner(cell_request, store);

	// Return the data as a CellResponse
	let res = OutgoingResponse {
		result: Ok(BlobResponse::CellResponse(cell_response.clone()).encode()),
		reputation_changes: Vec::default(),
		sent_feedback: None,
	};

	if let Err(e) = response_tx.send(res) {
		log::error!(target: LOG_TARGET, "An error has occured while trying to return cells to requester: {e:?}")
	};
}

pub fn process_cell_response(cell_response: CellResponse) -> CellResponse {
	log::info!(target: LOG_TARGET, "9 - Processing cell response");
	cell_response
}

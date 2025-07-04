use crate::{
	p2p::BlobHandle,
	types::{
		BlobHash, BlobMetadata, BlobReceived, CellRequest, CellResponse, CellUnitResponse,
		ShardReceived, ShardResponse,
	},
	utils::{fetch_shards, get_my_validator_id, get_shards_to_store, get_validators},
};
use anyhow::{anyhow, Result};
use codec::{Decode, Encode};
use futures::{channel::oneshot, stream::FuturesUnordered, StreamExt};
use sc_client_api::HeaderBackend;
use sc_network::{
	config::{IncomingRequest, OutgoingResponse},
	IfDisconnected, NetworkRequest, NetworkService, NetworkStateInfo, PeerId,
};
use sp_authority_discovery::AuthorityId;
use sp_runtime::{traits::Block as BlockT, Perbill, SaturatedConversion};
use std::{collections::BTreeMap, str::FromStr, sync::Arc, time::Duration};
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
/// Amount of block for which the receiving rpc node need to store the blob metadata and shards.
const TMP_BLOB_TTL: u64 = 180; // 1 hour
/// Amount of blocks used to periodically check wether we should remove expired blobs or not.
const BLOB_EXPIRATION_CHECK_PERIOD: u64 = TMP_BLOB_TTL;
/// Min Amount of block for which the transaction submitted through RPC is valid.
/// This value needs to handle the time to upload the blob to the store
const MIN_TRANSACTION_VALIDITY: u64 = 15; // 5 mn
/// Max Amount of block for which the transaction submitted through RPC is valid.
/// This value is used so a transaction won't be stuck in the mempool.
const MAX_TRANSACTION_VALIDITY: u64 = 150; // 50 mn
/// The number of time we'll allow trying to fetch internal blob metadata before letting the transaction go through to get discarded
const MAX_BLOB_RETRY_BEFORE_DISCARDING: u16 = 1;

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
	blob_received: BlobReceived,
	blob_handle: &BlobHandle<Block>,
	original_peer_id: PeerId,
) where
	Block: BlockT,
{
	log::info!(target: LOG_TARGET, "1 - Got a blob notification");
	if !blob_handle.role.is_authority() {
		log::info!(target: LOG_TARGET, "Got a blob notification but ignoring it since this node is not an authority.");
		return;
	}

	let Some(keystore) = blob_handle.keystore.get() else {
		log::error!(target: LOG_TARGET, "Keystore not yet registered");
		return;
	};

	let my_validator_id = match get_my_validator_id(&keystore) {
		Ok(maybe_validator_id) => match maybe_validator_id {
			Some(v) => v,
			None => {
				log::info!(target: LOG_TARGET, "This node is not a current authority, skipping recording blobs");
				return;
			},
		},
		Err(e) => {
			log::error!(target: LOG_TARGET, "Error happened while trying to get this node's id: {e}");
			return;
		},
	};

	let Some(client) = blob_handle.client.get() else {
		log::error!(target: LOG_TARGET, "Client not yet registered");
		return;
	};

	let Some(network) = blob_handle.network.get() else {
		log::error!(target: LOG_TARGET, "Network not yet registered");
		return;
	};

	let validators = match get_validators(client).await {
		Ok(v) => v,
		Err(e) => {
			log::error!(target: LOG_TARGET, "Could not get validator from the runtime: {e}");
			return;
		},
	};

	let mut blob_metadata = BlobMetadata {
		hash: blob_received.hash,
		size: blob_received.size,
		nb_shards: blob_received.nb_shards,
		commitments: blob_received.commitments,
		ownership: blob_received.ownership,
		is_notified: true,
		expires_at: client
			.info()
			.finalized_number
			.saturated_into::<u64>()
			.saturating_add(BLOB_TTL),
	};

	// Check existence of blob metadata in case we had the shard received notification before blob received.
	let existing_ownership = match blob_handle
		.shard_store
		.get_blob_metadata(&blob_received.hash)
	{
		Ok(Some(meta)) => {
			if !meta.is_notified {
				Some(meta.ownership)
			} else {
				None
			}
		},
		_ => None,
	};
	if let Some(existing_ownership) = existing_ownership {
		for (shard_id, mut owners) in existing_ownership {
			let entry = blob_metadata.ownership.entry(shard_id).or_default();
			entry.append(&mut owners);
			entry.sort_unstable();
			entry.dedup();
		}
	}

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
		for shard_id in &shards_to_store {
			blob_metadata.insert_in_ownership(shard_id, &my_validator_id, &my_peer_id_base58);
		}

		send_shard_request(
			blob_received.hash,
			shards_to_store,
			blob_handle,
			network,
			original_peer_id,
			my_validator_id,
		)
		.await;
	}

	match blob_handle
		.shard_store
		.insert_blob_metadata(&blob_metadata.hash, &blob_metadata)
	{
		Ok(()) => {
			log::info!(
				target: LOG_TARGET,
				"Stored blob metadata {} in the store db",
				blob_metadata.hash
			)
		},
		Err(e) => {
			log::error!(
				target: LOG_TARGET,
				"An error occured while trying to store blob metadata {}: {}",
				blob_metadata.hash,
				e
			)
		},
	};
}

async fn send_shard_request<Block>(
	blob_hash: BlobHash,
	shard_ids: Vec<u16>,
	blob_handle: &BlobHandle<Block>,
	network: &Arc<NetworkService<Block, <Block as BlockT>::Hash>>,
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

	let futures = FuturesUnordered::new();

	for shard_ids_chunk in shard_ids.chunks(shards_per_request) {
		let chunk_shard_request = ShardRequest {
			hash: blob_hash,
			shard_ids: shard_ids_chunk.to_vec(),
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
									"Invalid response in send_shard_request, got CellResponse, expected ShardResponse"
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

		futures.push(fut);
	}

	futures.collect::<()>().await;
}

pub fn handle_incoming_blob_request(request: IncomingRequest, store: &RocksdbShardStore) {
	log::info!(target: LOG_TARGET, "X - In handle incoming blob request");
	let data = request.payload;
	let response_tx = request.pending_response;

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

fn process_shard_request(
	shard_request: ShardRequest,
	store: &RocksdbShardStore,
	response_tx: oneshot::Sender<OutgoingResponse>,
) {
	log::info!(target: LOG_TARGET, "3 - processing incoming shard_request");
	let shards = match fetch_shards(&store, &shard_request) {
		Ok(s) => s,
		Err(e) => {
			log::error!(target: LOG_TARGET, "Could not get shards: {e}");
			if let Err(e) = response_tx.send(OutgoingResponse {
				result: Err(()),
				reputation_changes: Vec::default(),
				sent_feedback: None,
			}) {
				log::error!(target: LOG_TARGET, "An error has occured in process_shard_request: {e:?}");
			}
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

	let shard_received = ShardReceived {
		hash: shard_response.hash,
		shard_ids: shard_response
			.shards
			.into_iter()
			.map(|shard| shard.shard_id)
			.collect(),
		address: my_validator_id,
		original_peer_id: network.local_peer_id().to_base58(),
	};

	if shard_received.shard_ids.len() > 0 {
		log::info!(target: LOG_TARGET, "Sending ShardReceived notification");
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
		expires_at: client
			.info()
			.finalized_number
			.saturated_into::<u64>()
			.saturating_add(TMP_BLOB_TTL),
	});
	if !blob_metadata.is_notified {
		log::info!(target: LOG_TARGET, "A ShardReceived was received before BlobReceived, creating empty blob metadata.");
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
	network: &Arc<NetworkService<Block, <Block as BlockT>::Hash>>,
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
						Err(anyhow!("Invalid response in send_cell_request, got ShardResponse, expected CellResponse"))
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

pub fn process_cell_request_inner(
	cell_request: CellRequest,
	store: &RocksdbShardStore,
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

pub fn process_cell_request(
	cell_request: CellRequest,
	store: &RocksdbShardStore,
	response_tx: oneshot::Sender<OutgoingResponse>,
) {
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

use crate::{
	p2p::BlobHandle,
	types::{
		Blob, BlobHash, BlobMetadata, BlobNotification, BlobQueryRequest, BlobReceived,
		BlobRequest, BlobRequestEnum, BlobResponse, BlobResponseEnum, BlobSignatureData,
		BlobStored, OwnershipEntry, BLOB_REQ_PROTO,
	},
	utils::{
		build_signature_payload, check_store_blob, generate_base_index, get_active_validators,
		get_my_validator_id, get_validator_id_from_key, get_validator_per_blob, sign_blob_data,
		verify_signed_blob_data,
	},
};
use anyhow::{anyhow, Result};
use codec::{Decode, Encode};
use futures::channel::oneshot;
use sc_client_api::HeaderBackend;
use sc_network::{
	config::{IncomingRequest, OutgoingResponse},
	IfDisconnected, NetworkPeers, NetworkRequest, NetworkService, NetworkStateInfo, ObservedRole,
	PeerId,
};
use sp_runtime::{traits::Block as BlockT, SaturatedConversion};
use std::{str::FromStr, sync::Arc, time::Duration};
use store::{BlobStore, RocksdbBlobStore};

pub mod p2p;
pub mod rpc;
pub mod store;
pub mod types;
pub mod utils;

pub(crate) const LOG_TARGET: &str = "avail::blob";

/***** TODO Blob: Make CLI / Runtime args from this, must be compatible with rpc flags *****/
/// Maximum notification size
const NOTIFICATION_MAX_SIZE: u64 = 2 * 1024 * 1024;
/// Maximum request size
const REQUEST_MAX_SIZE: u64 = 128 * 1024;
/// Maximum response size
const RESPONSE_MAX_SIZE: u64 = 65 * 1024 * 1024;
/// Maximum request time
const REQUEST_TIME_OUT: Duration = Duration::from_secs(30);
/// The maximum number of allowed parallel request processing or notification processing
const CONCURRENT_REQUESTS: usize = 256;

// Business logic
/// Amount of block for which we need to store the blob metadata and blob.
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
/// The number of time we'll allow trying to fetch internal blob metadata or blob data before letting the transaction go through to get discarded
const MAX_BLOB_RETRY_BEFORE_DISCARDING: u16 = 3;
/// The number of block for which a notification is considered valid
const NOTIFICATION_EXPIRATION_PERIOD: u64 = 180;
/// The number of retries the validator is going to make before stopping a blob request
const MAX_REQUEST_RETRIES: u8 = 3;
/// The number of retries the RPC is going to make before dropping a user blob request
const MAX_RPC_RETRIES: u8 = 3;

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
			BlobNotification::BlobStored(blob_stored) => {
				handle_blob_stored_notification(blob_stored, &blob_handle).await;
			},
			BlobNotification::ClearBlob => {
				let _ = blob_handle.blob_store.clear_blob_storage();
				let _ = blob_handle.blob_data_store.clear_blob_storage();
				log::info!(target: LOG_TARGET, "Everything was deleted from storage");
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
	let timer = std::time::Instant::now();
	log::info!(
		"BLOB - handle_blob_received_notification - START - {:?} - {:?}",
		blob_received.hash,
		timer.elapsed()
	);
	let is_authority = blob_handle.role.is_authority();
	if !is_authority {
		// Received a blob notification but this node is not an authority, ignoring.
		return;
	}

	let Some(client) = blob_handle.client.get() else {
		log::error!(target: LOG_TARGET, "Client not yet registered");
		return;
	};

	let Some(network) = blob_handle.network.get() else {
		log::error!(target: LOG_TARGET, "Network not yet registered");
		return;
	};

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

	let validators = get_active_validators(&client, &announced_finalized_hash.encode());
	let nb_validators_per_blob = get_validator_per_blob(
		&client,
		&announced_finalized_hash.encode(),
		validators.len() as u32,
	);

	let maybe_metadata = match blob_handle
		.blob_store
		.get_blob_metadata(&blob_received.hash)
	{
		Ok(maybe_meta) => maybe_meta,
		Err(e) => {
			log::error!("Failed to check data from blob storage: {e}");
			return;
		},
	};

	// Get the existing blob or create a new one
	let mut blob_meta = maybe_metadata.unwrap_or_else(|| BlobMetadata {
		hash: blob_received.hash,
		size: blob_received.size,
		commitment: blob_received.commitment.clone(),
		is_notified: true,
		expires_at: 0,
		finalized_block_hash: Block::Hash::default(),
		finalized_block_number: 0,
		nb_validators_per_blob: 0,
	});

	// If we already had the blob metadata but incomplete (is notified == false) we fill the missing data.
	if !blob_meta.is_notified {
		blob_meta.size = blob_received.size;
		blob_meta.commitment = blob_received.commitment;
		blob_meta.is_notified = true;
	}

	// Here, no matter if it's a new blob or a duplicate, we set the new expiration time and update the ownership / block timestamp.
	blob_meta.finalized_block_hash = announced_finalized_hash;
	blob_meta.finalized_block_number = announced_finalized_number;
	blob_meta.expires_at = announced_finalized_number.saturating_add(BLOB_TTL);
	blob_meta.nb_validators_per_blob = nb_validators_per_blob;

	// Check signatures in case blob ownership is filled
	let mut ownerships_to_record: Vec<OwnershipEntry> = Vec::new();
	if let Some(ownership) = blob_received.ownership {
		let Some((expected_address, _)) = get_validator_id_from_key(
			&ownership.babe_key,
			client,
			&announced_finalized_hash.encode(),
		) else {
			log::error!("Could not get expected address from signer");
			return;
		};
		let expected_payload = build_signature_payload(
			blob_received.hash,
			[expected_address.encode(), b"stored".to_vec()].concat(),
		);
		let signer = ownership.babe_key.encode();
		match verify_signed_blob_data(
			BlobSignatureData {
				signer,
				signature: ownership.signature.clone(),
			},
			expected_payload.clone(),
		) {
			Ok(true) => {},
			Ok(false) => {
				log::error!("invalid signature");
				return;
			},
			Err(err) => {
				log::error!("verify error signature error {err}");
				return;
			},
		}
		ownerships_to_record.push(ownership);
	}

	let Some(keystore) = blob_handle.keystore.get() else {
		log::error!(target: LOG_TARGET, "Keystore not yet registered");
		return;
	};

	let (my_validator_id, babe_key) =
		match get_my_validator_id(&keystore, &client, &announced_finalized_hash.encode()) {
			Some(v) => v,
			None => {
				log::error!(target: LOG_TARGET, "No keys found while trying to get this node's id");
				return;
			},
		};

	let should_store_blob = match check_store_blob(
		blob_received.hash,
		&validators,
		&my_validator_id,
		&announced_finalized_hash.encode(),
		nb_validators_per_blob,
	) {
		Ok(s) => s,
		Err(e) => {
			log::error!(target: LOG_TARGET, "Could not check if I should store the blob: {e}");
			return;
		},
	};

	if should_store_blob {
		let my_peer_id = network.local_peer_id();
		let my_peer_id_base58 = my_peer_id.to_base58();

		let signature_payload = build_signature_payload(
			blob_received.hash,
			[my_validator_id.encode(), b"stored".to_vec()].concat(),
		);

		let signature = match sign_blob_data(blob_handle, signature_payload) {
			Ok(s) => s.signature,
			Err(e) => {
				log::error!(target: LOG_TARGET, "An error has occured while trying to sign data, exiting the function: {e}");
				return;
			},
		};

		let Ok(existing_ownership) = blob_handle
			.blob_store
			.get_blob_ownership(&blob_received.hash, &my_validator_id.encode())
		else {
			log::error!(
				target: LOG_TARGET,
				"Could not read the db for ownership for hash {}",
				blob_meta.hash,
			);
			return;
		};

		if existing_ownership.is_none() {
			// Get maybe already existing ownership so we can target those validators instead of the busy rpc
			let mut stored_ownerships = blob_handle
				.blob_store
				.get_blob_ownerships(&blob_received.hash)
				.unwrap_or(Vec::new());
			stored_ownerships.extend(ownerships_to_record.clone());
			
			let target_peer_id = if !stored_ownerships.is_empty() {
				let base_index = generate_base_index(
					blob_received.hash,
					&announced_finalized_hash.encode(),
					stored_ownerships.len(),
					Some(my_validator_id.encode()),
				)
				.unwrap_or(0);

				match stored_ownerships.get(base_index) {
					Some(o) => {
						let peer = PeerId::from_str(&o.encoded_peer_id).unwrap_or(original_peer_id);
						peer
					},
					None => original_peer_id,
				}
			} else {
				original_peer_id
			};

			let Some(blob_response) = send_blob_request(
				blob_received.hash,
				blob_handle,
				network,
				target_peer_id,
				original_peer_id,
			)
			.await
			else {
				log::error!(
					target: LOG_TARGET,
					"An error occured while trying to request a blob {}",
					blob_meta.hash,
				);
				return;
			};

			// Insert the blob in the store
			if let Err(e) = blob_handle.blob_data_store.insert_blob(&blob_response.blob) {
				log::error!(
					target: LOG_TARGET,
					"An error occured while trying to store blob {}: {}",
					blob_meta.hash,
					e
				);
				return;
			}
		}

		let ownership = OwnershipEntry {
			address: my_validator_id,
			babe_key,
			encoded_peer_id: my_peer_id_base58,
			signature,
		};
		ownerships_to_record.push(ownership.clone());

		send_blob_stored_notification(
			blob_received.hash,
			&blob_handle,
			ownership,
			announced_finalized_hash,
		)
		.await;
	}

	if let Err(e) = blob_handle.blob_store.insert_blob_metadata(&blob_meta) {
		log::error!(
			target: LOG_TARGET,
			"An error occured while trying to store blob metadata {}: {}",
			blob_meta.hash,
			e
		)
	}

	for o in ownerships_to_record {
		if let Err(e) = blob_handle
			.blob_store
			.insert_blob_ownership(&blob_received.hash, &o)
		{
			log::error!(
				target: LOG_TARGET,
				"An error occured while trying to store blob metadata {}: {}",
				blob_meta.hash,
				e
			)
		}
	}
	log::info!(
		"BLOB - handle_blob_received_notification - END - {:?} - {:?}",
		blob_received.hash,
		timer.elapsed()
	);
}

async fn send_blob_request<Block>(
	blob_hash: BlobHash,
	blob_handle: &BlobHandle<Block>,
	network: &Arc<NetworkService<Block, Block::Hash>>,
	target_peer_id: PeerId,
	original_peer_id: PeerId,
) -> Option<BlobResponse>
where
	Block: BlockT,
{
	let timer = std::time::Instant::now();
	log::info!(
		"BLOB - send_blob_request - START - {:?} - {:?}",
		blob_hash,
		timer.elapsed()
	);
	let signature_payload = build_signature_payload(blob_hash, b"request".to_vec());
	let signature_data = match sign_blob_data(blob_handle, signature_payload) {
		Ok(s) => s,
		Err(e) => {
			log::error!(target: LOG_TARGET, "An error has occured while trying to sign data, exiting the function: {e}");
			return None;
		},
	};

	let blob_request = BlobRequestEnum::BlobRequest(BlobRequest {
		hash: blob_hash,
		signature_data,
	});

	for attempt in 0..MAX_REQUEST_RETRIES {
		let target_peer = if attempt == 0 {
			target_peer_id
		} else {
			original_peer_id
		};
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
				match BlobResponseEnum::decode(&mut buf) {
					Ok(BlobResponseEnum::BlobResponse(blob_response)) => {
						log::info!(
							"BLOB - send_blob_request - END - {:?} - {:?}",
							blob_hash,
							timer.elapsed()
						);
						return Some(blob_response);
					},
					Ok(_other) => {
						log::error!(target: LOG_TARGET,
							"Invalid response in send blob request, expected BlobResponse");
						break;
					},
					Err(err) => {
						log::error!(target: LOG_TARGET,
							"Failed to decode Blob response ({} bytes): {:?}", data.len(), err);
						break;
					},
				}
			},
			Err(e) => {
				log::error!(target: LOG_TARGET,
					"An error has occured while trying to send blob request {blob_hash:?} (attempt {}/{}): {e}",
					attempt + 1,
					MAX_REQUEST_RETRIES
				);

				if attempt + 1 < MAX_REQUEST_RETRIES {
					let sleep_timer = tokio::time::Instant::now();
					let backoff_secs = 2;
					tokio::time::sleep(Duration::from_secs(backoff_secs)).await;
					log::info!(
						"Finished sleeping for next blob request {blob_hash:?}: sleep time: {:?}",
						sleep_timer.elapsed()
					);
					continue;
				}
			},
		}
	}

	log::info!(
		"BLOB - send_blob_request - END with errors - {:?} - {:?}",
		blob_hash,
		timer.elapsed()
	);
	None
}

pub fn handle_incoming_blob_request<Block: BlockT>(
	request: IncomingRequest,
	blob_data_store: &RocksdbBlobStore<Block>,
	network: &Arc<NetworkService<Block, Block::Hash>>,
) where
	Block: BlockT,
{
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
	match BlobRequestEnum::decode(&mut buf) {
		Ok(blob_request) => match blob_request {
			BlobRequestEnum::BlobRequest(blob_request) => {
				process_blob_request(blob_request, blob_data_store, response_tx);
			},
			BlobRequestEnum::BlobQueryRequest(blob_query_request) => {
				process_blob_query_request(blob_query_request, blob_data_store, response_tx);
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

fn process_blob_request<Block: BlockT>(
	blob_request: BlobRequest,
	blob_data_store: &RocksdbBlobStore<Block>,
	response_tx: oneshot::Sender<OutgoingResponse>,
) {
	let timer = std::time::Instant::now();
	log::info!(
		"BLOB - process_blob_request - START - {:?} - {:?}",
		blob_request.hash,
		timer.elapsed()
	);
	let expected_payload = build_signature_payload(blob_request.hash, b"request".to_vec());
	match verify_signed_blob_data(blob_request.signature_data.clone(), expected_payload) {
		Ok(valid) => {
			if !valid {
				log::error!(target: LOG_TARGET, "An error has occured in process_blob_request: Invalid signature");
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

	let blob = match blob_data_store.get_blob(&blob_request.hash) {
		Ok(b) => match b {
			Some(b) => b,
			None => {
				log::error!(target: LOG_TARGET, "Blob not found in store");
				let _ = response_tx.send(OutgoingResponse {
					result: Err(()),
					reputation_changes: Vec::default(),
					sent_feedback: None,
				});
				return;
			},
		},
		Err(e) => {
			log::error!(target: LOG_TARGET, "Could not get blob from store: {e}");
			let _ = response_tx.send(OutgoingResponse {
				result: Err(()),
				reputation_changes: Vec::default(),
				sent_feedback: None,
			});
			return;
		},
	};

	let blob_response = BlobResponseEnum::BlobResponse(BlobResponse {
		hash: blob.blob_hash,
		blob,
	});

	let res = OutgoingResponse {
		result: Ok(blob_response.encode()),
		reputation_changes: Vec::default(),
		sent_feedback: None,
	};

	if let Err(e) = response_tx.send(res) {
		log::error!(target: LOG_TARGET, "An error has occured in process_blob_request: {e:?}");
	}
	log::info!(
		"BLOB - process_blob_request - END - {:?} - {:?}",
		blob_request.hash,
		timer.elapsed()
	);
}

pub async fn send_blob_stored_notification<Block>(
	blob_hash: BlobHash,
	blob_handle: &BlobHandle<Block>,
	ownership_entry: OwnershipEntry,
	finalized_block_hash: Block::Hash,
) where
	Block: BlockT,
{
	let timer = std::time::Instant::now();
	log::info!(
		"BLOB - send_blob_stored_notification - START - {:?} - {:?}",
		blob_hash,
		timer.elapsed()
	);
	let blob_stored = BlobStored {
		hash: blob_hash,
		ownership_entry,
		finalized_block_hash,
	};

	let Some(gossip_cmd_sender) = blob_handle.gossip_cmd_sender.get() else {
		log::error!(target: LOG_TARGET, "Could not send gossip notification since gossip_cmd_sender was not initialized");
		return;
	};

	if let Err(e) = gossip_cmd_sender
		.send(BlobNotification::BlobStored(blob_stored))
		.await
	{
		log::error!(target: LOG_TARGET, "Could not send BlobStored notification: {e}")
	}
	log::info!(
		"BLOB - send_blob_stored_notification - END - {:?} - {:?}",
		blob_hash,
		timer.elapsed()
	);
}

async fn handle_blob_stored_notification<Block>(
	blob_stored: BlobStored<Block>,
	blob_handle: &BlobHandle<Block>,
) where
	Block: BlockT,
{
	let timer = std::time::Instant::now();
	log::info!(
		"BLOB - handle_blob_stored_notification - START - {:?} - {:?}",
		blob_stored.hash,
		timer.elapsed()
	);
	let is_authority = blob_handle.role.is_authority();
	if !is_authority {
		// Received a blob notification but this node is not an authority, ignoring
		return;
	}

	let Some(client) = blob_handle.client.get() else {
		log::error!(target: LOG_TARGET, "Client not yet registered");
		return;
	};

	let finalized_hash = blob_stored.finalized_block_hash;
	let Some((address, _)) = get_validator_id_from_key(
		&blob_stored.ownership_entry.babe_key,
		client,
		&finalized_hash.encode(),
	) else {
		log::error!("Could not find address associated to babe key");
		return;
	};
	let expected_payload = build_signature_payload(
		blob_stored.hash,
		[address.encode(), b"stored".to_vec()].concat(),
	);
	match verify_signed_blob_data(
		BlobSignatureData {
			signer: blob_stored.ownership_entry.babe_key.encode(),
			signature: blob_stored.ownership_entry.signature.clone(),
		},
		expected_payload,
	) {
		Ok(valid) => {
			if !valid {
				log::error!(target: LOG_TARGET, "An error has occured in handle_blob_stored_notification: Invalid signature");
				return;
			}
		},
		Err(e) => {
			log::error!(target: LOG_TARGET, "An error has occured while checking the signature: {e}");
			return;
		},
	}

	if let Err(e) = blob_handle
		.blob_store
		.insert_blob_ownership(&blob_stored.hash, &blob_stored.ownership_entry)
	{
		log::error!(
			target: LOG_TARGET,
			"An error has occured while trying to save blob ownership in the store: {e}"
		);
	}
	log::info!(
		"BLOB - handle_blob_stored_notification - END - {:?} - {:?}",
		blob_stored.hash,
		timer.elapsed()
	);
}

pub async fn send_blob_query_request<Block>(
	hash: BlobHash,
	target_peer: PeerId,
	network: &Arc<NetworkService<Block, Block::Hash>>,
) -> Result<Option<Blob>>
where
	Block: BlockT,
{
	let timer = std::time::Instant::now();
	log::info!(
		"BLOB - send_blob_query_request - START - {:?} - {:?}",
		hash,
		timer.elapsed()
	);
	let req = BlobRequestEnum::BlobQueryRequest(BlobQueryRequest { hash });

	let response = network
		.request(
			target_peer,
			BLOB_REQ_PROTO,
			req.encode(),
			None,
			IfDisconnected::TryConnect,
		)
		.await;

	match response {
		Ok((data, _proto)) => {
			let mut buf: &[u8] = &data;
			match BlobResponseEnum::decode(&mut buf) {
				Ok(response) => match response {
					BlobResponseEnum::BlobQueryResponse(blob_query_response) => {
						log::info!(
							"BLOB - send_blob_query_request - END - {:?} - {:?}",
							hash,
							timer.elapsed()
						);
						return Ok(blob_query_response);
					},
					_ => {
						return Err(anyhow!("Invalid response in send blob query request, expected BlobQueryResponse"));
					},
				},
				Err(err) => {
					return Err(anyhow!(
						"Failed to decode Blob response ({} bytes): {:?}",
						data.len(),
						err
					));
				},
			}
		},
		Err(e) => {
			return Err(anyhow!(
				"An error has occured while trying to send blob query request for a chunk: {e}"
			));
		},
	}
}

pub fn process_blob_query_request<Block: BlockT>(
	blob_query_request: BlobQueryRequest,
	blob_data_store: &RocksdbBlobStore<Block>,
	response_tx: oneshot::Sender<OutgoingResponse>,
) {
	let timer = std::time::Instant::now();
	log::info!(
		"BLOB - process_blob_query_request - START - {:?} - {:?}",
		blob_query_request.hash,
		timer.elapsed()
	);
	let maybe_blob = match blob_data_store.get_blob(&blob_query_request.hash) {
		Ok(s) => match s {
			None => None,
			Some(s) => Some(Blob {
				blob_hash: s.blob_hash,
				size: s.size,
				data: s.data.iter().cloned().take(5).collect(), // TODO Blob: Should we return all the data ? Seems heavy for the RPC
			}),
		},
		Err(e) => {
			log::error!("Could not get a blob for the requester RPC: {e}");
			let res = OutgoingResponse {
				result: Err(()),
				reputation_changes: Vec::default(),
				sent_feedback: None,
			};
			let _ = response_tx.send(res);
			return;
		},
	};

	let res = OutgoingResponse {
		result: Ok(BlobResponseEnum::BlobQueryResponse(maybe_blob).encode()),
		reputation_changes: Vec::default(),
		sent_feedback: None,
	};

	if let Err(e) = response_tx.send(res) {
		log::error!(target: LOG_TARGET, "An error has occured while trying to return a blob to requester: {e:?}")
	};
	log::info!(
		"BLOB - process_blob_query_request - END - {:?} - {:?}",
		blob_query_request.hash,
		timer.elapsed()
	);
}

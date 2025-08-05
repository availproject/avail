use crate::{
	p2p::BlobHandle,
	types::{
		Blob, BlobHash, BlobMetadata, BlobNotification, BlobQueryRequest, BlobReceived,
		BlobRequest, BlobRequestEnum, BlobResponse, BlobResponseEnum, BlobSignatureData,
		BlobStored, CellRequest, CellResponse, BLOB_REQ_PROTO,
	},
	utils::{
		build_signature_payload, check_blob_validity, check_store_blob, get_my_validator_id,
		get_validator_per_blob, sign_blob_data, verify_signed_blob_data,
	},
};
use anyhow::{anyhow, Result};
use avail_core::{BlockLengthColumns, BlockLengthRows};
use codec::{Decode, Encode};
use da_runtime::kate::{GDataProof, GProof, GRawScalar};
use futures::channel::oneshot;
use kate::{
	com::Cell,
	couscous::multiproof_params,
	gridgen::core::{AsBytes as _, EvaluationGrid as EGrid},
	Seed,
};
use kate_recovery::commons::ArkPublicParams;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use sc_authority_discovery::AuthorityDiscovery;
use sc_client_api::HeaderBackend;
use sc_network::{
	config::{IncomingRequest, OutgoingResponse},
	IfDisconnected, NetworkPeers, NetworkRequest, NetworkService, NetworkStateInfo, ObservedRole,
	PeerId,
};
use sp_authority_discovery::AuthorityId;
use sp_core::sr25519;
use sp_runtime::{traits::Block as BlockT, Perbill, SaturatedConversion};
use std::{num::NonZeroU16, str::FromStr, sync::Arc, time::Duration};
use store::{BlobStore, RocksdbBlobStore};

pub mod p2p;
pub mod rpc;
pub mod store;
pub mod types;
pub mod utils;

pub(crate) const LOG_TARGET: &str = "avail::blob";

/***** TODO Blob: Make CLI / Runtime args from this, must be compatible with rpc flags *****/
/// Maximum notification size, 128kb
const NOTIFICATION_MAX_SIZE: u64 = 1 * 1024 * 1024;
/// Maximum request size, 128kb
const REQUEST_MAX_SIZE: u64 = 128 * 1024;
/// Maximum response size, 64mb
const RESPONSE_MAX_SIZE: u64 = 64 * 1024 * 1024;
/// Maximum request time
const REQUEST_TIME_OUT: Duration = Duration::from_secs(5);
/// The maximum number of allowed concurrent request processing or notification processing
const CONCURRENT_REQUESTS: usize = 2048;

// Business logic
/// Maximum size of a blob, need to change the rpc request and response size to handle this
const MAX_BLOB_SIZE: u64 = 32 * 1024 * 1024;
/// Minimum amount of validator that needs to store a blob, if we have less than minimum, everyone stores it.
const MIN_BLOB_HOLDER_PERCENTAGE: Perbill = Perbill::from_percent(10);
/// We take the maximum between this and MIN_BLOB_HOLDER_PERCENTAGE
const MIN_BLOB_HOLDER_COUNT: u32 = 2;
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
const MAX_BLOB_RETRY_BEFORE_DISCARDING: u16 = 2;
/// The number of block for which a notification is considered valid
const NOTIFICATION_EXPIRATION_PERIOD: u64 = 180;
/// The number of retries the RPC is going to make before dropping a user blob request
const MAX_RPC_RETRIES: u8 = 3;
/// Number of cells required for 99.99% confidence
const CELL_COUNT: u32 = 14;

static PP: std::sync::OnceLock<ArkPublicParams> = std::sync::OnceLock::new();

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
	let is_authority = blob_handle.role.is_authority();
	if !is_authority {
		log::warn!("Received a blob notification but this node is not an authority, ignoring.");
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

	let validators = match client.authorities(finalized_hash).await {
		Ok(v) => v,
		Err(e) => {
			log::error!(target: LOG_TARGET, "Could not get validator from the runtime: {e}");
			return;
		},
	};
	let nb_validators_per_blob = get_validator_per_blob(validators.len() as u32);

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
	let expires_at = announced_finalized_number.saturating_add(BLOB_TTL);
	let mut blob_meta = maybe_metadata.unwrap_or_else(|| BlobMetadata {
		hash: blob_received.hash,
		size: blob_received.size,
		commitment: blob_received.commitment.clone(),
		ownership: Vec::new(),
		is_notified: true,
		expires_at: 0,
		finalized_block_hash: Block::Hash::default(),
		finalized_block_number: 0,
		nb_validators_per_blob: 0,
		is_validated: false,
		error_reason: None,
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
	blob_meta.expires_at = expires_at;
	blob_meta.nb_validators_per_blob = nb_validators_per_blob;
	blob_meta.merge_ownerships(blob_received.ownership);

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

		let signature_payload = build_signature_payload(blob_received.hash, b"stored".to_vec());

		let signature = match sign_blob_data(blob_handle, signature_payload) {
			Ok(s) => s.signature,
			Err(e) => {
				log::error!(target: LOG_TARGET, "An error has occured while trying to sign data, exiting the function: {e}");
				return;
			},
		};

		let is_new = blob_meta.insert_ownership(&my_validator_id, &my_peer_id_base58, signature);

		if is_new {
			send_blob_request(
				blob_received.hash,
				blob_handle,
				network,
				original_peer_id,
				my_validator_id.clone(),
			)
			.await;
		} else {
			send_blob_stored_notification(
				blob_received.hash,
				&blob_handle,
				my_validator_id.clone(),
			)
			.await;
		}

		check_blob_validity(
			network,
			keystore,
			&blob_handle.blob_store,
			&mut blob_meta,
			&announced_finalized_hash.encode(),
			my_validator_id,
		)
		.await;
	}

	match blob_handle.blob_store.insert_blob_metadata(&blob_meta) {
		Ok(()) => {},
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

async fn send_blob_request<Block>(
	blob_hash: BlobHash,
	blob_handle: &BlobHandle<Block>,
	network: &Arc<NetworkService<Block, Block::Hash>>,
	original_peer_id: PeerId,
	my_validator_id: AuthorityId,
) where
	Block: BlockT,
{
	let signature_payload = build_signature_payload(blob_hash, b"request".to_vec());
	let signature_data = match sign_blob_data(blob_handle, signature_payload) {
		Ok(s) => s,
		Err(e) => {
			log::error!(target: LOG_TARGET, "An error has occured while trying to sign data, exiting the function: {e}");
			return;
		},
	};

	let blob_request = BlobRequestEnum::BlobRequest(BlobRequest {
		hash: blob_hash,
		signature_data,
	});

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
			match BlobResponseEnum::decode(&mut buf) {
				Ok(response) => match response {
					BlobResponseEnum::BlobResponse(blob_response) => {
						process_blob_response(blob_response, &blob_handle, my_validator_id).await;
					},
					_ => {
						log::error!(
							target: LOG_TARGET,
							"Invalid response in send blob request, expected BlobResponse"
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
			log::error!(target: LOG_TARGET, "An error has occured while trying to send blob request: {e}");
		},
	}
}

pub fn handle_incoming_blob_request<Block: BlockT>(
	request: IncomingRequest,
	store: &RocksdbBlobStore<Block>,
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
				process_blob_request(blob_request, store, response_tx);
			},
			BlobRequestEnum::CellRequest(cell_request) => {
				process_cell_request(cell_request, store, response_tx);
			},
			BlobRequestEnum::BlobQueryRequest(blob_query_request) => {
				process_blob_query_request(blob_query_request, store, response_tx);
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
	store: &RocksdbBlobStore<Block>,
	response_tx: oneshot::Sender<OutgoingResponse>,
) {
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

	let blob = match store.get_blob(&blob_request.hash) {
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
}

async fn process_blob_response<Block>(
	blob_response: BlobResponse,
	blob_handle: &BlobHandle<Block>,
	my_validator_id: AuthorityId,
) where
	Block: BlockT,
{
	match blob_handle.blob_store.insert_blob(&blob_response.blob) {
		Ok(()) => {
			send_blob_stored_notification(blob_response.hash, blob_handle, my_validator_id).await;
		},
		Err(e) => {
			log::error!(target: LOG_TARGET, "An error occured while trying to store the blob: {e}");
		},
	};
}

async fn send_blob_stored_notification<Block>(
	blob_hash: BlobHash,
	blob_handle: &BlobHandle<Block>,
	my_validator_id: AuthorityId,
) where
	Block: BlockT,
{
	let Some(network) = blob_handle.network.get() else {
		log::error!(target: LOG_TARGET, "Network not yet registered, could not send blob stored notification");
		return;
	};

	let sig_payload = build_signature_payload(blob_hash, b"stored".to_vec());
	let signature = match sign_blob_data(blob_handle, sig_payload) {
		Ok(s) => s.signature,
		Err(e) => {
			log::error!(target: LOG_TARGET, "An error has occured while trying to sign data, exiting the function: {e}");
			return;
		},
	};

	let blob_stored = BlobStored {
		hash: blob_hash,
		address: my_validator_id,
		original_peer_id: network.local_peer_id().to_base58(),
		signature,
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
}

async fn handle_blob_stored_notification<Block>(
	blob_stored: BlobStored,
	blob_handle: &BlobHandle<Block>,
) where
	Block: BlockT,
{
	let is_authority = blob_handle.role.is_authority();
	if !is_authority {
		log::warn!("Received a blob notification but this node is not an authority, ignoring.");
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

	let Ok(maybe_meta) = blob_handle.blob_store.get_blob_metadata(&blob_stored.hash) else {
		log::error!(target: LOG_TARGET, "An error has occured while trying to get blob from the store");
		return;
	};

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

	let client_info = client.info();
	let finalized_hash = client_info.finalized_hash;

	let mut blob_metadata = maybe_meta.unwrap_or(BlobMetadata {
		hash: blob_stored.hash,
		size: 0,
		commitment: Vec::new(),
		ownership: Vec::new(),
		is_notified: false,
		finalized_block_hash: Block::Hash::default(),
		finalized_block_number: 0,
		nb_validators_per_blob: 0,
		is_validated: false,
		error_reason: None,
		expires_at: client_info
			.finalized_number
			.saturated_into::<u64>()
			.saturating_add(TEMP_BLOB_TTL),
	});
	if !blob_metadata.is_notified {
		log::warn!(target: LOG_TARGET, "A Blob Stored notification was received before Blob Received, creating empty blob metadata.");
	}

	let address: sr25519::Public = blob_stored.address.clone().into();
	let address_vec = address.0.to_vec();

	let expected_payload = build_signature_payload(blob_stored.hash, b"stored".to_vec());
	match verify_signed_blob_data(
		BlobSignatureData {
			signer: address_vec,
			signature: blob_stored.signature.clone(),
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

	blob_metadata.insert_ownership(
		&blob_stored.address,
		&blob_stored.original_peer_id,
		blob_stored.signature,
	);

	check_blob_validity(
		network,
		keystore,
		&blob_handle.blob_store,
		&mut blob_metadata,
		&finalized_hash.encode(),
		my_validator_id,
	)
	.await;

	if let Err(e) = blob_handle.blob_store.insert_blob_metadata(&blob_metadata) {
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
	let blob_request = BlobRequestEnum::CellRequest(cell_request);
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
				Ok(response) => match response {
					BlobResponseEnum::CellResponse(cell_response) => Ok(cell_response),
					_ => Err(anyhow!(
						"Invalid response in send cell request, expected CellResponse"
					)),
				},
				Err(err) => Err(anyhow!(
					"Failed to decode Blob response ({} bytes): {:?}",
					data.len(),
					err
				)),
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

pub fn process_cell_request<Block: BlockT>(
	cell_request: CellRequest,
	store: &RocksdbBlobStore<Block>,
	response_tx: oneshot::Sender<OutgoingResponse>,
) {
	let expected_payload: Vec<u8> =
		build_signature_payload(cell_request.hash, cell_request.cells.clone().encode());
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

	let cell_response = match process_cell_request_inner(cell_request, store) {
		Ok(cell_response) => cell_response,
		Err(e) => {
			log::error!(target: LOG_TARGET, "An error has occured while process the cell request: {e}");
			let _ = response_tx.send(OutgoingResponse {
				result: Err(()),
				reputation_changes: Vec::default(),
				sent_feedback: None,
			});
			return;
		},
	};

	// Return the data as a CellResponse
	let res = OutgoingResponse {
		result: Ok(BlobResponseEnum::CellResponse(cell_response.clone()).encode()),
		reputation_changes: Vec::default(),
		sent_feedback: None,
	};

	if let Err(e) = response_tx.send(res) {
		log::error!(target: LOG_TARGET, "An error has occured while trying to return cells to requester: {e:?}")
	};
}

pub fn process_cell_request_inner<Block: BlockT>(
	cell_request: CellRequest,
	store: &RocksdbBlobStore<Block>,
) -> Result<CellResponse> {
	let hash = cell_request.hash;
	// get blob from the store which is needed to compute the cell proofs
	let blob = match store.get_blob(&hash).ok().flatten() {
		Some(b) => b.data,
		None => {
			return Err(anyhow!("Failed to get blob; hash: {:?}", hash));
		},
	};

	// TODO Blob: Fetch the configured tx grid dimension
	let cols = 1024;
	let rows = 4096;
	let pp = PP.get_or_init(multiproof_params);
	let grid = EGrid::from_data(blob, cols, cols, rows, Seed::default())
		.expect("Grid construction from data works");
	let grid = EGrid::merge_with_padding(vec![grid])
		.expect("merging should work")
		.extend_columns(NonZeroU16::new(2).expect("2 > 0"))
		.expect("If dimensions are correct, extension should work");
	let poly = grid
		.make_polynomial_grid()
		.expect("polynomial grid construction works");

	let cell_proofs: Vec<GDataProof> = match cell_request
		.cells
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

			Ok((scalar, gproof))
		})
		.collect::<Result<Vec<_>, _>>()
	{
		Ok(proofs) => proofs,
		Err(e) => return Err(anyhow!("Proof generation failed: {e:?}")),
	};

	Ok(CellResponse { hash, cell_proofs })
}

pub async fn send_blob_query_request<Block>(
	hash: BlobHash,
	target_peer: PeerId,
	network: &Arc<NetworkService<Block, Block::Hash>>,
) -> Result<Option<Blob>>
where
	Block: BlockT,
{
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
	store: &RocksdbBlobStore<Block>,
	response_tx: oneshot::Sender<OutgoingResponse>,
) {
	let maybe_blob = match store.get_blob(&blob_query_request.hash) {
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
}

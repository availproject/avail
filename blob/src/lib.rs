use crate::p2p::NetworkHandle;
use codec::{Decode, Encode};
use once_cell::sync::OnceCell;
use sc_network::{
	config::{IncomingRequest, NonDefaultSetConfig, OutgoingResponse, RequestResponseConfig},
	IfDisconnected, NetworkRequest, NetworkService, PeerId,
};
use sp_runtime::traits::Block as BlockT;
use std::{sync::Arc, time::Duration};
use store::MockShardStore;
use types::{
	BlobNotification, BlobRequest, BlobResponse, Shard, ShardRequest, BLOB_NOTIF_PROTO,
	BLOB_REQ_PROTO,
};

pub mod p2p;
pub mod rpc;
pub mod store;
pub mod types;

pub const NOTIFICATION_MAX_SIZE: u64 = 1 * 1024;
pub const REQUEST_MAX_SIZE: u64 = 1 * 1024; // 1 kb
pub const RESPONSE_MAX_SIZE: u64 = 32 * 1024 * 1024; // 32 mb
pub const REQUEST_TIME_OUT: Duration = Duration::from_secs(5);
pub const MAX_BLOB_SIZE: u64 = 1024 * 1024 * 1024; // 1 gb
pub const SHARD_SIZE: u64 = 16 * 1024 * 1024; // 16mb

pub fn start_blob_service<Block>() -> (
	NonDefaultSetConfig,
	RequestResponseConfig,
	Arc<NetworkHandle<Block>>,
	Arc<MockShardStore>,
)
where
	Block: BlockT,
{
	// Initialize the shard store
	let shard_store = Arc::new(MockShardStore::default());

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
	));

	(blob_notif_cfg, blob_req_res_cfg, blob_handle, shard_store)
}

pub async fn decode_blob_notification<Block>(
	data: &[u8],
	network: &Arc<OnceCell<Arc<NetworkService<Block, <Block as BlockT>::Hash>>>>,
	peer: PeerId,
) where
	Block: BlockT,
{
	match BlobNotification::decode(&mut &data[..]) {
		Ok(notification) => {
			log::info!("Decoded Blob notification {:?}", notification);
			let net = network.get().expect("Network not yet registered");

			match notification {
				BlobNotification::Announce(blob_metadata) => {
					let shard_request = BlobRequest::ShardRequest(ShardRequest {
						hash: blob_metadata.hash,
						shard_id: 0,
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
								log::info!("Got A response : {response:?}")
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
}

pub async fn decode_blob_request(request: IncomingRequest) {
	let _peer = request.peer;
	let data = request.payload;
	let response_tx = request.pending_response;

	let mut buf: &[u8] = &data;
	match BlobRequest::decode(&mut buf) {
		Ok(blob_request) => match blob_request {
			BlobRequest::ShardRequest(shard_request) => {
				log::info!("Decoded Blob request {:?}", shard_request);
				let blob_response = BlobResponse::ShardResponse(Shard {
					hash: shard_request.hash,
					shard_id: shard_request.shard_id,
					data: Vec::default(),
				});
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
}

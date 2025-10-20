use std::{path::Path, sync::Arc, time::Duration};

use crate::{
	decode_blob_notification, handle_incoming_blob_request,
	store::{RocksdbBlobStore, StorageApiT},
	telemetry::TelemetryOperator,
	types::{BlobGossipValidator, BlobNotification, FullClient, BLOB_GOSSIP_PROTO, BLOB_REQ_PROTO},
	BLOB_EXPIRATION_CHECK_PERIOD, CONCURRENT_REQUESTS, LOG_TARGET, NOTIFICATION_MAX_SIZE,
	NOTIF_QUEUE_SIZE, REQUEST_MAX_SIZE, REQUEST_TIMEOUT_SECONDS, REQ_RES_QUEUE_SIZE,
	RESPONSE_MAX_SIZE,
};
use async_channel::Receiver;
use codec::Encode;
use futures::{future, FutureExt, StreamExt};
use sc_client_api::BlockchainEvents;
use sc_keystore::LocalKeystore;
use sc_network::{
	config::{IncomingRequest, NonDefaultSetConfig, RequestResponseConfig, Role},
	NetworkService, NotificationService,
};
use sc_network_gossip::GossipEngine;
use sc_network_sync::SyncingService;
use sc_service::SpawnTaskHandle;
use sp_runtime::{
	traits::{Block as BlockT, Hash as HashT, Header as HeaderT},
	SaturatedConversion,
};

pub fn get_blob_p2p_config() -> (
	RequestResponseConfig,
	async_channel::Receiver<IncomingRequest>,
	NonDefaultSetConfig,
	Box<dyn NotificationService>,
) {
	// Get blob Blob req/res protocol config
	let (blob_req_sender, blob_req_receiver) = async_channel::bounded(REQ_RES_QUEUE_SIZE as usize);
	let blob_req_res_cfg = RequestResponseConfig {
		name: BLOB_REQ_PROTO,
		fallback_names: vec![],
		max_request_size: REQUEST_MAX_SIZE,
		max_response_size: RESPONSE_MAX_SIZE,
		request_timeout: Duration::from_secs(REQUEST_TIMEOUT_SECONDS),
		inbound_queue: Some(blob_req_sender),
	};

	// Get blob gossip protocol config
	let (blob_gossip_cfg, blob_gossip_service) = NonDefaultSetConfig::new(
		BLOB_GOSSIP_PROTO,
		Vec::default(),
		NOTIFICATION_MAX_SIZE,
		None,
		Default::default(),
	);

	(
		blob_req_res_cfg,
		blob_req_receiver,
		blob_gossip_cfg,
		blob_gossip_service,
	)
}

#[derive(Clone)]
pub struct BlobHandle<Block>
where
	Block: BlockT,
{
	pub network: Arc<NetworkService<Block, Block::Hash>>,
	pub gossip_cmd_sender: async_channel::Sender<BlobNotification>,
	pub keystore: Arc<LocalKeystore>,
	pub client: Arc<FullClient>,
	pub blob_database: Arc<dyn StorageApiT>,
	pub role: Role,
	pub telemetry_operator: TelemetryOperator,
}

impl<Block> BlobHandle<Block>
where
	Block: BlockT,
{
	pub fn new(
		path: &Path,
		role: Role,
		blob_gossip_service: Box<dyn NotificationService>,
		req_receiver: async_channel::Receiver<IncomingRequest>,
		network: Arc<NetworkService<Block, Block::Hash>>,
		client: Arc<FullClient>,
		keystore: Arc<LocalKeystore>,
		sync_service: Arc<SyncingService<Block>>,
		telemetry_operator: TelemetryOperator,
		spawn_handle: SpawnTaskHandle,
	) -> Arc<Self> {
		// Initialize the Blob database
		let db_path = path.join("blob_database");
		let blob_database =
			Arc::new(RocksdbBlobStore::open(db_path).expect("opening RocksDB blob store failed"));

		// Init gossip sender / receiver
		let (gossip_cmd_sender, gossip_cmd_receiver) =
			async_channel::bounded::<BlobNotification>(NOTIF_QUEUE_SIZE as usize);

		let blob_handle = BlobHandle {
			network,
			keystore,
			client,
			gossip_cmd_sender,
			blob_database,
			role,
			telemetry_operator,
		};

		blob_handle.start_blob_req_res(spawn_handle.clone(), req_receiver);
		blob_handle.start_blob_gossip(
			spawn_handle.clone(),
			blob_gossip_service,
			sync_service,
			gossip_cmd_receiver,
		);
		blob_handle.start_blob_cleaning_service(spawn_handle);

		Arc::new(blob_handle)
	}

	fn start_blob_req_res(
		&self,
		spawn_handle: SpawnTaskHandle,
		req_receiver: async_channel::Receiver<IncomingRequest>,
	) {
		spawn_handle.spawn("request-listener", None, {
			let blob_database = self.blob_database.clone();
			let network = self.network.clone();
			async move {
				req_receiver
					.for_each_concurrent(CONCURRENT_REQUESTS, move |request| {
						let blob_database = blob_database.clone();
						let net = network.clone();
						tokio::task::spawn_blocking(move || {
							handle_incoming_blob_request(request, blob_database.as_ref(), &net);
						});
						future::ready(())
					})
					.await;
			}
		});
	}

	fn start_blob_gossip(
		&self,
		spawn_handle: SpawnTaskHandle,
		notif_service: Box<dyn NotificationService>,
		sync_service: Arc<SyncingService<Block>>,
		gossip_cmd_receiver: Receiver<BlobNotification>,
	) {
		let validator: Arc<BlobGossipValidator> = Arc::new(BlobGossipValidator::default());
		let mut gossip_engine = GossipEngine::<Block>::new(
			self.network.clone(),
			sync_service.clone(),
			notif_service,
			BLOB_GOSSIP_PROTO,
			validator,
			None,
		);

		let topic = <<Block::Header as HeaderT>::Hashing as HashT>::hash("blob_topic".as_bytes());
		let incoming_receiver = gossip_engine.messages_for(topic);

		spawn_handle.spawn("gossip-sender", None, async move {
			loop {
				futures::select! {
					() = (&mut gossip_engine).fuse() => break, // Important
					maybe_cmd = gossip_cmd_receiver.recv().fuse() => {
						match maybe_cmd {
							Ok(blob_notification) => {
								gossip_engine.gossip_message(topic, blob_notification.encode(), false)
							},
							_ => break,
						}
					}
				}
			}
		});

		spawn_handle.spawn("gossip-listener", None, {
			let blob_handle = self.clone();
			async move {
				incoming_receiver
					.for_each_concurrent(CONCURRENT_REQUESTS, |notification| {
						let blob_handle = blob_handle.clone();
						async move {
							if let Some(_notification_sender) = notification.sender {
								tokio::spawn({
									async move {
										decode_blob_notification(
											&notification.message,
											&blob_handle,
										)
										.await;
									}
								});
							}
						}
					})
					.await;
			}
		});
	}

	fn start_blob_cleaning_service(&self, spawn_handle: SpawnTaskHandle) {
		let blob_database = self.blob_database.clone();
		let client = self.client.clone();
		spawn_handle.spawn("blob-cleanup", None, async move {
			let mut block_sub = client.finality_notification_stream();

			while let Some(imported_block) = block_sub.next().await {
				let block_number = imported_block
					.header
					.number()
					.clone()
					.saturated_into::<u64>();
				if block_number % BLOB_EXPIRATION_CHECK_PERIOD == 0 {
					let blob_database = blob_database.clone();
					if let Err(e) = tokio::task::spawn_blocking(move || {
						match blob_database.clean_expired_blobs_info(block_number) {
							Ok((_hashes, _orphan_ownerships)) => Ok(()),
							Err(e) => Err(e),
						}
					})
					.await
					{
						log::warn!(target: LOG_TARGET, "cleanup join error: {e}");
					}
				}
			}
		});
	}
}

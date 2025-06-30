use std::{path::Path, sync::Arc};

use crate::{
	decode_blob_notification, handle_incoming_blob_request,
	store::RocksdbShardStore,
	types::{BlobGossipValidator, BlobNotification, FullClient, BLOB_GOSSIP_PROTO, BLOB_REQ_PROTO},
	LOG_TARGET, NOTIFICATION_MAX_SIZE, REQUEST_MAX_SIZE, REQUEST_TIME_OUT, RESPONSE_MAX_SIZE,
};
use codec::Encode;
use futures::{FutureExt, StreamExt};
use once_cell::sync::OnceCell;
use sc_keystore::LocalKeystore;
use sc_network::{
	config::{NonDefaultSetConfig, RequestResponseConfig, Role}, NetworkService, NotificationService
};
use sc_network_gossip::GossipEngine;
use sc_network_sync::SyncingService;
use sc_service::SpawnTaskHandle;
use sp_runtime::traits::Block as BlockT;
use tokio::sync::mpsc;

#[derive(Clone)]
pub struct BlobHandle<Block>
where
	Block: BlockT,
{
	pub network: Arc<OnceCell<Arc<NetworkService<Block, <Block as BlockT>::Hash>>>>,
	pub sync_service: Arc<OnceCell<Arc<SyncingService<Block>>>>,
	pub gossip_cmd_sender: Arc<OnceCell<mpsc::UnboundedSender<BlobNotification>>>,
	pub keystore: Arc<OnceCell<Arc<LocalKeystore>>>,
	pub client: Arc<OnceCell<Arc<FullClient>>>,
	pub shard_store: Arc<RocksdbShardStore>,
	pub role: Role,
}

impl<Block> BlobHandle<Block>
where
	Block: BlockT,
{
	pub fn new_blob_service(
		path: &Path,
		role: Role,
	) -> (
		Arc<Self>,
		RequestResponseConfig,
		NonDefaultSetConfig,
		Box<dyn NotificationService>,
	) {
		// Initialize the shard store
		let db_path = path.join("blob_store");
		let shard_store =
			Arc::new(RocksdbShardStore::open(db_path).expect("opening RocksDB blob store failed"));

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

		// Initialize the blob gossip protocol config
		let (blob_gossip_cfg, blob_gossip_service) = NonDefaultSetConfig::new(
			BLOB_GOSSIP_PROTO,
			Vec::default(),
			NOTIFICATION_MAX_SIZE,
			None,
			Default::default(),
		);

		let network = Arc::new(OnceCell::new());
		let sync_service = Arc::new(OnceCell::new());
		let keystore = Arc::new(OnceCell::new());
		let client = Arc::new(OnceCell::new());
		let gossip_cmd_sender = Arc::new(OnceCell::new());
		let shard_store_clone = shard_store.clone();

		tokio::spawn({
			async move {
				loop {
					futures::select! {
						// Request response handler
						maybe_req = request_receiver.recv().fuse() => {
							if let Ok(request) = maybe_req {
								handle_incoming_blob_request(request, &shard_store_clone);
							} else {
								break;
							}
						}


					}
				}
			}
		});

		let blob_handle = BlobHandle {
			network,
			keystore,
			client,
			sync_service,
			gossip_cmd_sender,
			shard_store,
			role,
		};

		log::info!(target: LOG_TARGET, "Blob service correctly initialized");
		(
			Arc::new(blob_handle),
			blob_req_res_cfg,
			blob_gossip_cfg,
			blob_gossip_service,
		)
	}

	pub fn start_blob_gossip(
		&self,
		spawn_handle: SpawnTaskHandle,
		notif_service: Box<dyn NotificationService>,
	) {
		log::info!(target: LOG_TARGET, "Blob gossip protocol initialized");
		let network = self.network.get().expect("Network should be registered");
		let sync_service = self
			.sync_service
			.get()
			.expect("Syncing service should be registered");
		let validator: Arc<BlobGossipValidator> = Arc::new(BlobGossipValidator);

		let mut gossip_engine = GossipEngine::<Block>::new(
			network.clone(),
			sync_service.clone(),
			notif_service,
			BLOB_GOSSIP_PROTO,
			validator,
			None,
		);

		let topic = Block::Hash::default();
		let mut incoming_receiver = gossip_engine.messages_for(topic);

		let (gossip_cmd_sender, mut gossip_cmd_receiver) =
			mpsc::unbounded_channel::<BlobNotification>();

		spawn_handle.spawn("gossip-sender", None, async move {
			loop {
				futures::select! {
					() = (&mut gossip_engine).fuse() => break,
					maybe_cmd = gossip_cmd_receiver.recv().fuse() => {
						match maybe_cmd {
							Some(blob_notification) => {
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
				while let Some(notification) = incoming_receiver.next().await {
					if let Some(_peer) = notification.sender {
						decode_blob_notification(
							&notification.message,
							&blob_handle
						)
						.await;
					};
				}
			}
		});

		self.gossip_cmd_sender
			.set(gossip_cmd_sender)
			.map_err(|_| "Setting gossip_cmd_sender called more than once")
			.expect("Registering gossip_cmd_sender cannot fail");
	}

	pub fn register_network_and_sync(
		&self,
		network: Arc<NetworkService<Block, <Block as BlockT>::Hash>>,
		sync_service: Arc<SyncingService<Block>>,
	) {
		log::info!(target: LOG_TARGET, "Registering network");
		self.network
			.set(network)
			.map_err(|_| "BlobHandle::register_network called more than once")
			.expect("Registering network cannot fail");

		self.sync_service
			.set(sync_service)
			.map_err(|_| "BlobHandle::register_sync_service called more than once")
			.expect("Registering sync_state cannot fail");
	}

	pub fn register_keystore(&self, keystore: Arc<LocalKeystore>) {
		log::info!(target: LOG_TARGET, "Registering keystore");
		self.keystore
			.set(keystore)
			.map_err(|_| "BlobHandle::register_keystore called more than once")
			.expect("Registering keystore cannot fail");
	}

	pub fn register_client(&self, client: Arc<FullClient>) {
		log::info!(target: LOG_TARGET, "Registering client");
		self.client
			.set(client)
			.map_err(|_| "BlobHandle::register_client called more than once")
			.expect("Registering client cannot fail");
	}
}

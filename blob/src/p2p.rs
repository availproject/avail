use std::{path::Path, sync::Arc};

use crate::{
	decode_blob_notification, handle_incoming_blob_request,
	store::{BlobStore, RocksdbBlobStore},
	types::{BlobGossipValidator, BlobNotification, FullClient, BLOB_GOSSIP_PROTO, BLOB_REQ_PROTO},
	BLOB_EXPIRATION_CHECK_PERIOD, CONCURRENT_REQUESTS, LOG_TARGET, NOTIFICATION_MAX_SIZE,
	REQUEST_MAX_SIZE, REQUEST_TIME_OUT, RESPONSE_MAX_SIZE,
};
use codec::Encode;
use futures::{FutureExt, StreamExt};
use once_cell::sync::OnceCell;
use sc_client_api::BlockchainEvents;
use sc_keystore::LocalKeystore;
use sc_network::{
	config::{IncomingRequest, NonDefaultSetConfig, RequestResponseConfig, Role},
	NetworkService, NotificationService,
};
use sc_network_gossip::GossipEngine;
use sc_network_sync::SyncingService;
use sc_service::SpawnTaskHandle;
use sp_runtime::traits::Block as BlockT;
use sp_runtime::traits::Header;
use sp_runtime::SaturatedConversion;

#[derive(Clone)]
pub struct BlobHandle<Block>
where
	Block: BlockT,
{
	pub network: Arc<OnceCell<Arc<NetworkService<Block, Block::Hash>>>>,
	pub sync_service: Arc<OnceCell<Arc<SyncingService<Block>>>>,
	pub gossip_cmd_sender: Arc<OnceCell<async_channel::Sender<BlobNotification<Block>>>>,
	pub keystore: Arc<OnceCell<Arc<LocalKeystore>>>,
	pub client: Arc<OnceCell<Arc<FullClient>>>,
	pub blob_store: Arc<RocksdbBlobStore<Block>>,
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
		async_channel::Receiver<IncomingRequest>,
		NonDefaultSetConfig,
		Box<dyn NotificationService>,
	) {
		// Initialize the Blob store
		let db_path = path.join("blob_store");
		let blob_store =
			Arc::new(RocksdbBlobStore::open(db_path).expect("opening RocksDB blob store failed"));

		// Initialize the blob Blob req/res protocol config
		let (blob_req_sender, blob_req_receiver) = async_channel::unbounded();
		let blob_req_res_cfg = RequestResponseConfig {
			name: BLOB_REQ_PROTO,
			fallback_names: vec![],
			max_request_size: REQUEST_MAX_SIZE,
			max_response_size: RESPONSE_MAX_SIZE,
			request_timeout: REQUEST_TIME_OUT,
			inbound_queue: Some(blob_req_sender),
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

		let blob_handle = BlobHandle {
			network,
			keystore,
			client,
			sync_service,
			gossip_cmd_sender,
			blob_store,
			role,
		};

		(
			Arc::new(blob_handle),
			blob_req_res_cfg,
			blob_req_receiver,
			blob_gossip_cfg,
			blob_gossip_service,
		)
	}

	pub fn start_blob_service(
		&self,
		spawn_handle: SpawnTaskHandle,
		req_receiver: async_channel::Receiver<IncomingRequest>,
		notif_service: Box<dyn NotificationService>,
		network: Arc<NetworkService<Block, Block::Hash>>,
		sync_service: Arc<SyncingService<Block>>,
		keystore: Arc<LocalKeystore>,
		client: Arc<FullClient>,
	) {
		self.register_keystore(keystore);
		self.register_client(client.clone());
		self.register_network_and_sync(network.clone(), sync_service.clone());
		self.start_blob_req_res(spawn_handle.clone(), req_receiver);
		self.start_blob_gossip(spawn_handle.clone(), notif_service);
		self.start_blob_cleaning_service(spawn_handle);
	}

	fn start_blob_req_res(
		&self,
		spawn_handle: SpawnTaskHandle,
		req_receiver: async_channel::Receiver<IncomingRequest>,
	) {
		let network_cell = self.network.clone();
		let network = network_cell
			.get()
			.expect("Network should be registered")
			.clone();
		spawn_handle.spawn("request-listener", None, {
			let blob_store_clone = self.blob_store.clone();
			let network_clone = network;
			async move {
				req_receiver
					.for_each_concurrent(CONCURRENT_REQUESTS, |request| {
						let store = blob_store_clone.clone();
						let network = network_clone.clone();
						async move {
							handle_incoming_blob_request(request, &store, &network);
						}
					})
					.await;
			}
		});
	}

	fn start_blob_gossip(
		&self,
		spawn_handle: SpawnTaskHandle,
		notif_service: Box<dyn NotificationService>,
	) {
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
		let incoming_receiver = gossip_engine.messages_for(topic);

		let (gossip_cmd_sender, gossip_cmd_receiver) =
			async_channel::unbounded::<BlobNotification<Block>>();

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
							if let Some(_peer) = notification.sender {
								decode_blob_notification(&notification.message, &blob_handle).await;
							}
						}
					})
					.await;
			}
		});

		self.gossip_cmd_sender
			.set(gossip_cmd_sender)
			.map_err(|_| "Setting gossip_cmd_sender called more than once")
			.expect("Registering gossip_cmd_sender cannot fail");
	}

	fn start_blob_cleaning_service(&self, spawn_handle: SpawnTaskHandle) {
		let Some(client) = self.client.get() else {
			log::error!(target: LOG_TARGET, "Client not yet registered");
			return;
		};

		let blob_store = self.blob_store.clone();
		let client = client.clone();
		spawn_handle.spawn("blob-cleanup", None, async move {
			let mut block_sub = client.finality_notification_stream();

			while let Some(imported_block) = block_sub.next().await {
				let block_number = imported_block
					.header
					.number()
					.clone()
					.saturated_into::<u64>();
				if block_number % BLOB_EXPIRATION_CHECK_PERIOD == 0 {
					let _ = blob_store.clean_expired_blobs(block_number);
				}
			}
		});
	}

	fn register_network_and_sync(
		&self,
		network: Arc<NetworkService<Block, Block::Hash>>,
		sync_service: Arc<SyncingService<Block>>,
	) {
		self.network
			.set(network)
			.map_err(|_| "BlobHandle::register_network called more than once")
			.expect("Registering network cannot fail");

		self.sync_service
			.set(sync_service)
			.map_err(|_| "BlobHandle::register_sync_service called more than once")
			.expect("Registering sync_state cannot fail");
	}

	fn register_keystore(&self, keystore: Arc<LocalKeystore>) {
		self.keystore
			.set(keystore)
			.map_err(|_| "BlobHandle::register_keystore called more than once")
			.expect("Registering keystore cannot fail");
	}

	fn register_client(&self, client: Arc<FullClient>) {
		self.client
			.set(client)
			.map_err(|_| "BlobHandle::register_client called more than once")
			.expect("Registering client cannot fail");
	}
}

use std::{sync::Arc, time::Duration};

use crate::{
	decode_blob_notification, handle_incoming_blob_request,
	slashing::check_missing_validators,
	store::StorageApiT,
	types::{BlobGossipValidator, BlobNotification, FullClient, BLOB_GOSSIP_PROTO, BLOB_REQ_PROTO},
	BLOB_EXPIRATION_CHECK_PERIOD, CONCURRENT_REQUESTS, LOG_TARGET, NOTIFICATION_MAX_SIZE,
	NOTIF_QUEUE_SIZE, REQUEST_MAX_SIZE, REQUEST_TIMEOUT_SECONDS, REQ_RES_QUEUE_SIZE,
	RESPONSE_MAX_SIZE,
};
use async_channel::Receiver;
use codec::Encode;
use core::marker::PhantomData;
use futures::{future, FutureExt, StreamExt};
use sc_client_api::BlockchainEvents;
use sc_keystore::LocalKeystore;
use sc_network::{
	config::{IncomingRequest, NonDefaultSetConfig, RequestResponseConfig, Role},
	NetworkService, NotificationService,
};
use sc_network::{service::traits::NetworkService as NetworkServiceT, NetworkBackend};
use sc_network_gossip::GossipEngine;
use sc_network_sync::SyncingService;
use sc_service::SpawnTaskHandle;
use sc_transaction_pool_api::TransactionPool;
use sp_runtime::{
	traits::{Block as BlockT, Hash as HashT, Header as HeaderT},
	SaturatedConversion,
};

pub fn get_blob_p2p_config<B: BlockT, N: NetworkBackend<B, <B as BlockT>::Hash>>(
	metrics: sc_network::service::NotificationMetrics,
	peer_store_handle: Arc<dyn sc_network::peer_store::PeerStoreProvider>,
) -> (
	N::RequestResponseProtocolConfig,
	async_channel::Receiver<IncomingRequest>,
	N::NotificationProtocolConfig,
	Box<dyn NotificationService>,
) {
	// Get blob Blob req/res protocol config
	let (blob_req_sender, blob_req_receiver) = async_channel::bounded(REQ_RES_QUEUE_SIZE as usize);
	let blob_req_res_cfg = N::request_response_config(
		BLOB_REQ_PROTO,
		Vec::new(),
		REQUEST_MAX_SIZE,
		RESPONSE_MAX_SIZE,
		Duration::from_secs(REQUEST_TIMEOUT_SECONDS),
		Some(blob_req_sender),
	);

	// Get blob gossip protocol config
	let (_peerset_cfg, blob_gossip_service) = NonDefaultSetConfig::new(
		BLOB_GOSSIP_PROTO,
		Vec::default(),
		NOTIFICATION_MAX_SIZE,
		None,
		Default::default(),
	);

	let (blob_gossip_cfg, _) = N::notification_config(
		BLOB_GOSSIP_PROTO,
		Vec::new(),
		NOTIFICATION_MAX_SIZE,
		None,
		sc_network::config::SetConfig {
			in_peers: 0,
			out_peers: 0,
			reserved_nodes: Vec::new(),
			non_reserved_mode: sc_network::config::NonReservedPeerMode::Deny,
		},
		metrics,
		peer_store_handle,
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
	pub network: Arc<dyn NetworkServiceT>,
	pub gossip_cmd_sender: async_channel::Sender<BlobNotification>,
	pub keystore: Arc<LocalKeystore>,
	pub client: Arc<FullClient>,
	pub blob_database: Arc<dyn StorageApiT>,
	pub role: Role,
	pub _marker: PhantomData<Block>,
}

impl<Block> BlobHandle<Block>
where
	Block: BlockT,
{
	pub fn new<Pool>(
		role: Role,
		blob_database: Arc<dyn StorageApiT>,
		blob_gossip_service: Box<dyn NotificationService>,
		req_receiver: async_channel::Receiver<IncomingRequest>,
		network: Arc<dyn NetworkServiceT>,
		client: Arc<FullClient>,
		keystore: Arc<LocalKeystore>,
		sync_service: Arc<SyncingService<Block>>,
		spawn_handle: SpawnTaskHandle,
		pool: Arc<Pool>,
	) -> Arc<Self>
	where
		Pool: TransactionPool<Block = Block> + 'static,
	{
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
			_marker: PhantomData,
		};

		blob_handle.start_blob_req_res(spawn_handle.clone(), req_receiver);
		blob_handle.start_blob_gossip(
			spawn_handle.clone(),
			blob_gossip_service,
			sync_service,
			gossip_cmd_receiver,
		);
		blob_handle.start_blob_cleaning_service(spawn_handle.clone());

		if blob_handle.role.is_authority() {
			blob_handle.start_missing_validators_listener(spawn_handle, pool);
		}

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
							handle_incoming_blob_request::<Block>(
								request,
								blob_database.as_ref(),
								&net,
							);
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

	fn start_missing_validators_listener<Pool>(
		&self,
		spawn_handle: SpawnTaskHandle,
		pool: Arc<Pool>,
	) where
		Block: BlockT,
		Pool: TransactionPool<Block = Block> + 'static,
	{
		let blob_database = self.blob_database.clone();
		let client = self.client.clone();
		let keystore = self.keystore.clone();
		let pool = pool.clone();

		spawn_handle.spawn("missing-validators-listener", None, async move {
			check_missing_validators(client, keystore, blob_database, pool).await;
		});
	}
}

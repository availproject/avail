use std::{sync::Arc, time::Duration};

use crate::{
	decode_blob_notification, handle_incoming_blob_request,
	slashing::check_missing_validators,
	store::StorageApiT,
	types::{
		BlobGossipValidator, BlobNotification, BlobOwnershipInfo, BlobOwnershipsRequest,
		BlobRequestEnum, BlobResponseEnum, FullClient, BLOB_GOSSIP_PROTO, BLOB_REQ_PROTO,
	},
	BLOB_EXPIRATION_CHECK_PERIOD, CONCURRENT_REQUESTS, LOG_TARGET, NOTIFICATION_MAX_SIZE,
	NOTIF_QUEUE_SIZE, REQUEST_MAX_SIZE, REQUEST_TIMEOUT_SECONDS, REQ_RES_QUEUE_SIZE,
	RESPONSE_MAX_SIZE,
};
use async_channel::Receiver;
use avail_core::{
	header::{extension::fri::FriHeader, HeaderExtension},
	traits::ExtendedHeader,
};
use codec::{Decode, Encode};
use futures::{future, FutureExt, StreamExt};
use sc_client_api::BlockchainEvents;
use sc_keystore::LocalKeystore;
use sc_network::{
	config::{IncomingRequest, NonDefaultSetConfig, RequestResponseConfig, Role},
	IfDisconnected, NetworkRequest, NetworkService, NotificationService, PeerId,
};
use sc_network_gossip::GossipEngine;
use sc_network_sync::SyncingService;
use sc_service::SpawnTaskHandle;
use sc_transaction_pool_api::TransactionPool;
use sp_core::H256;
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
	// temp: until we add light_client role
	pub is_light_node: bool,
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
		network: Arc<NetworkService<Block, Block::Hash>>,
		client: Arc<FullClient>,
		keystore: Arc<LocalKeystore>,
		sync_service: Arc<SyncingService<Block>>,
		spawn_handle: SpawnTaskHandle,
		pool: Arc<Pool>,
		is_light_node: bool,
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
			is_light_node,
		};

		blob_handle.start_blob_req_res(spawn_handle.clone(), req_receiver);
		blob_handle.start_blob_cleaning_service(spawn_handle.clone());

		if blob_handle.role.is_authority() {
			blob_handle.start_missing_validators_listener(spawn_handle.clone(), pool);
		}
		if is_light_node {
			blob_handle.start_blob_ownership_fetcher(spawn_handle);
		} else {
			// we dont need the blob_gossip service in light-node mode
			blob_handle.start_blob_gossip(
				spawn_handle.clone(),
				blob_gossip_service,
				sync_service,
				gossip_cmd_receiver,
			);
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

	fn start_blob_ownership_fetcher(&self, spawn_handle: SpawnTaskHandle) {
		let client = self.client.clone();
		let network = self.network.clone();
		let blob_db = self.blob_database.clone();

		spawn_handle.spawn("blob-ownership-fetcher", Some("light-node"), async move {
			let mut import_stream = client.import_notification_stream();

			while let Some(notification) = import_stream.next().await {
				let header = notification.header;

				if !header.extension().has_da_commitments() {
					continue;
				}

				let blobs = match header.extension() {
					HeaderExtension::Fri(FriHeader::V1(ext)) => &ext.blobs,
					_ => continue,
				};

				let missing: Vec<H256> = blobs
					.iter()
					.map(|b| b.blob_hash)
					.filter(|h| {
						blob_db
							.get_blob_ownerships(h)
							.map(|o| o.is_empty())
							.unwrap_or(true)
					})
					.collect();

				if missing.is_empty() {
					continue;
				}

				log::debug!(
					target: LOG_TARGET,
					"🔍 Block #{}: requesting ownership for {} blobs",
					header.number(),
					missing.len()
				);

				match request_blob_ownership_from_peers(
					network.clone(),
					missing.clone(),
					Duration::from_secs(3),
				)
				.await
				{
					Ok(ownerships) => {
						for info in ownerships {
							for entry in info.ownership {
								if let Err(e) =
									blob_db.insert_blob_ownership(&info.blob_hash, &entry)
								{
									log::warn!(
										target: LOG_TARGET,
										"Failed to store ownership for {:?}: {e}",
										info.blob_hash
									);
								}
							}
						}
					},
					Err(e) => {
						log::warn!(
							target: LOG_TARGET,
							"❌ Ownership fetch failed at block #{}: {e}",
							header.number()
						);
					},
				}
			}
		});
	}
}

/// Request blob ownership info for a set of blobs from connected peers.
pub async fn request_blob_ownership_from_peers<Block: BlockT>(
	network: Arc<NetworkService<Block, Block::Hash>>,
	blob_hashes: Vec<H256>,
	timeout: Duration,
) -> Result<Vec<BlobOwnershipInfo>, String> {
	if blob_hashes.is_empty() {
		return Ok(Vec::new());
	}

	let state = network
		.network_state()
		.await
		.map_err(|_| "Failed to fetch network state".to_string())?;

	let peers: Vec<PeerId> = state
		.connected_peers
		.keys()
		.filter_map(|k| k.parse::<PeerId>().ok())
		.collect();

	if peers.is_empty() {
		return Err("No connected peers available for blob ownership request".into());
	}

	log::debug!(
		target: LOG_TARGET,
		"📡 Requesting ownership for {} blobs from {} peers",
		blob_hashes.len(),
		peers.len()
	);

	let req =
		BlobRequestEnum::BlobOwnershipsRequest(BlobOwnershipsRequest { blob_hashes }).encode();

	for peer in peers {
		log::debug!(
			target: LOG_TARGET,
			"📤 Sending BlobOwnershipsRequest to peer {peer}"
		);

		let fut = network.request(
			peer.clone(),
			BLOB_REQ_PROTO,
			req.clone(),
			None,
			IfDisconnected::TryConnect,
		);

		let data = match tokio::time::timeout(timeout, fut).await {
			Ok(Ok((data, _))) => data,
			Ok(Err(e)) => {
				log::warn!(target: LOG_TARGET, "❌ Peer {peer} error: {e}");
				continue;
			},
			Err(_) => {
				log::warn!(target: LOG_TARGET, "⏱️ Peer {peer} timed out");
				continue;
			},
		};

		let mut buf: &[u8] = &data;
		match BlobResponseEnum::decode(&mut buf) {
			Ok(BlobResponseEnum::BlobOwnershipsResponse(resp)) => {
				log::info!(
					target: LOG_TARGET,
					"✅ Received ownership for {} blobs from peer {peer}",
					resp.blobs.len()
				);
				return Ok(resp.blobs);
			},
			Ok(other) => {
				log::warn!(
					target: LOG_TARGET,
					"⚠️ Unexpected response from {peer}: {other:?}"
				);
			},
			Err(e) => {
				log::warn!(
					target: LOG_TARGET,
					"❌ Failed to decode ownership response from {peer}: {e}"
				);
			},
		}
	}

	Err("All peers failed to provide blob ownership info".into())
}

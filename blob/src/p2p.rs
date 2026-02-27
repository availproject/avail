use std::{sync::Arc, time::Duration};

use crate::{
	decode_blob_notification, handle_incoming_blob_request,
	slashing::check_missing_validators,
	store::StorageApiT,
	types::{
		BlobGossipValidator, BlobHash, BlobNotification, BlobOwnershipInfo, BlobOwnershipsRequest,
		BlobRequestEnum, BlobResponseEnum, EvalClaimsMessage, FullClient, BLOB_GOSSIP_PROTO,
		BLOB_REQ_PROTO, BLOB_TOPIC, EVAL_CLAIMS_TOPIC,
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
use core::marker::PhantomData;
use futures::{future, FutureExt, StreamExt};
use parking_lot::Mutex;
use sc_client_api::BlockchainEvents;
use sc_keystore::LocalKeystore;
use sc_network::{
	config::{IncomingRequest, Role},
	IfDisconnected, NotificationService, PeerId,
};
use sc_network::{
	service::traits::NetworkService as NetworkServiceT, NetworkBackend, NetworkRequest,
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
use std::collections::HashMap;

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
	let (blob_gossip_cfg, blob_gossip_service) = N::notification_config(
		BLOB_GOSSIP_PROTO,
		Vec::new(),
		NOTIFICATION_MAX_SIZE,
		None,
		sc_network::config::SetConfig {
			// TODO: Wire these values to actual node config
			in_peers: 100,
			out_peers: 100,
			reserved_nodes: Vec::new(),
			non_reserved_mode: sc_network::config::NonReservedPeerMode::Accept,
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

/// Cache key for eval claims: (block_hash, blob_hash).
pub type EvalClaimsCache = Arc<Mutex<HashMap<(H256, BlobHash), EvalClaimsMessage>>>;

static GLOBAL_EVAL_SENDER: once_cell::sync::OnceCell<async_channel::Sender<EvalClaimsMessage>> =
	once_cell::sync::OnceCell::new();

pub fn set_global_eval_sender(sender: async_channel::Sender<EvalClaimsMessage>) {
	let _ = GLOBAL_EVAL_SENDER.set(sender);
}

pub fn global_eval_sender() -> Option<async_channel::Sender<EvalClaimsMessage>> {
	GLOBAL_EVAL_SENDER.get().cloned()
}

#[derive(Clone)]
pub struct BlobHandle<Block>
where
	Block: BlockT,
{
	pub network: Arc<dyn NetworkServiceT>,
	pub gossip_cmd_sender: async_channel::Sender<BlobNotification>,
	/// Sender for eval claims topic (full nodes only). When `Some`, publishing to eval_claims topic is enabled.
	pub eval_claims_cmd_sender: Option<async_channel::Sender<EvalClaimsMessage>>,
	pub eval_claims_cache: Option<EvalClaimsCache>,
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

		let (eval_claims_cmd_sender, eval_claims_cmd_receiver, eval_claims_cache) =
			if matches!(role, Role::LightClient) {
				(None, None, Some(Arc::new(Mutex::new(HashMap::new()))))
			} else {
				let (tx, rx) =
					async_channel::bounded::<EvalClaimsMessage>(NOTIF_QUEUE_SIZE as usize);
				set_global_eval_sender(tx.clone());
				(Some(tx), Some(rx), None)
			};

		let blob_handle = BlobHandle {
			network,
			keystore,
			client,
			gossip_cmd_sender,
			eval_claims_cmd_sender,
			eval_claims_cache,
			blob_database,
			role,
			_marker: PhantomData,
		};

		blob_handle.start_blob_req_res(spawn_handle.clone(), req_receiver);

		blob_handle.start_blob_cleaning_service(spawn_handle.clone());

		if blob_handle.role.is_authority() {
			blob_handle.start_missing_validators_listener(spawn_handle.clone(), pool);
		}
		if matches!(blob_handle.role, Role::LightClient) {
			blob_handle.start_blob_ownership_fetcher(spawn_handle.clone());
			blob_handle.start_eval_claims_listener(spawn_handle, blob_gossip_service, sync_service);
		} else {
			blob_handle.start_blob_gossip(
				spawn_handle.clone(),
				blob_gossip_service,
				sync_service,
				gossip_cmd_receiver,
				eval_claims_cmd_receiver.expect("full node has eval claims receiver"),
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
		eval_claims_cmd_receiver: Receiver<EvalClaimsMessage>,
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

		let blob_topic = <<Block::Header as HeaderT>::Hashing as HashT>::hash(BLOB_TOPIC);
		let eval_claims_topic =
			<<Block::Header as HeaderT>::Hashing as HashT>::hash(EVAL_CLAIMS_TOPIC);
		let incoming_receiver = gossip_engine.messages_for(blob_topic);
		let eval_claims_incoming = gossip_engine.messages_for(eval_claims_topic);

		spawn_handle.spawn("gossip-sender", None, async move {
			loop {
				futures::select! {
					() = (&mut gossip_engine).fuse() => break,
					maybe_cmd = gossip_cmd_receiver.recv().fuse() => {
						match maybe_cmd {
							Ok(blob_notification) => {
								gossip_engine.gossip_message(blob_topic, blob_notification.encode(), false)
							},
							_ => break,
						}
					}
					maybe_eval = eval_claims_cmd_receiver.recv().fuse() => {
						match maybe_eval {
							Ok(msg) => {
								gossip_engine.gossip_message(eval_claims_topic, msg.encode(), false)
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

		// Eval claims messages are consumed by light nodes via their listener; full nodes no-op
		spawn_handle.spawn("gossip-eval-claims-listener", None, {
			async move {
				eval_claims_incoming.for_each(|_| async {}).await;
			}
		});
	}

	pub fn get_eval_data_for_blob(
		&self,
		block_hash: H256,
		blob_hash: H256,
	) -> Option<EvalClaimsMessage> {
		self.eval_claims_cache
			.as_ref()
			.and_then(|c| c.lock().get(&(block_hash, blob_hash)).cloned())
	}

	/// Light node: subscribe to eval_claims topic and cache messages for verification against header.
	fn start_eval_claims_listener(
		&self,
		spawn_handle: SpawnTaskHandle,
		notif_service: Box<dyn NotificationService>,
		sync_service: Arc<SyncingService<Block>>,
	) {
		let cache = match &self.eval_claims_cache {
			Some(c) => c.clone(),
			None => return,
		};
		let validator: Arc<BlobGossipValidator> = Arc::new(BlobGossipValidator::default());
		let mut gossip_engine = GossipEngine::<Block>::new(
			self.network.clone(),
			sync_service,
			notif_service,
			BLOB_GOSSIP_PROTO,
			validator,
			None,
		);
		let eval_claims_topic =
			<<Block::Header as HeaderT>::Hashing as HashT>::hash(EVAL_CLAIMS_TOPIC);
		let incoming = gossip_engine.messages_for(eval_claims_topic);

		spawn_handle.spawn("eval-claims-gossip-engine", None, async move {
			(&mut gossip_engine).await;
		});

		spawn_handle.spawn("eval-claims-listener", Some("light-node"), async move {
			incoming
				.for_each_concurrent(CONCURRENT_REQUESTS, |notification| {
					let cache = cache.clone();
					async move {
						if notification.sender.is_some() {
							if let Ok(msg) =
								EvalClaimsMessage::decode(&mut &notification.message[..])
							{
								log::info!(
									target: LOG_TARGET,
									"✅ Received eval claims message for block {:?}, blob {:?}, app_id {:?}",
									msg.block_hash,
									msg.blob_hash,
									msg.app_id
								);
								cache.lock().insert((msg.block_hash, msg.blob_hash), msg);
							}
						}
					}
				})
				.await;
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

				let block_hash = header.hash();
				let infos = match blob_db.list_blob_infos_by_block(&block_hash) {
					Ok(infos) => infos,
					Err(e) => {
						log::warn!(
							target: LOG_TARGET,
							"⚠️ Failed to list BlobInfo entries for block {:?}: {e}",
							block_hash
						);
						continue;
					},
				};

				if infos.is_empty() {
					continue;
				}

				let missing: Vec<H256> = infos
					.iter()
					.map(|info| info.hash)
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

				match request_blob_ownership_from_peers::<Block>(
					&network.clone(),
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
	network: &Arc<dyn NetworkServiceT>,
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

use std::{collections::HashMap, sync::Arc};

use crate::{
	decode_blob_notification, decode_blob_request,
	store::RocksdbShardStore,
	types::{BlobNotification, FullClient},
};
use async_channel::Receiver;
use codec::Encode;
use futures::FutureExt;
use once_cell::sync::OnceCell;
use parking_lot::Mutex;
use sc_keystore::LocalKeystore;
use sc_network::{
	config::{IncomingRequest, Role},
	service::traits::NotificationEvent,
	NetworkPeers, NetworkService, NotificationService, ObservedRole, PeerId,
};
use sp_runtime::traits::Block as BlockT;
use tokio::sync::mpsc;

#[derive(Clone)]
pub struct NetworkHandle<Block>
where
	Block: BlockT,
{
	pub blob_notification_sender: mpsc::UnboundedSender<BlobNotification>,
	pub network: Arc<OnceCell<Arc<NetworkService<Block, <Block as BlockT>::Hash>>>>,
	pub keystore: Arc<OnceCell<Arc<LocalKeystore>>>,
	pub client: Arc<OnceCell<Arc<FullClient>>>,
}

impl<Block> NetworkHandle<Block>
where
	Block: BlockT,
{
	pub fn register_network(&self, network: Arc<NetworkService<Block, <Block as BlockT>::Hash>>) {
		self.network
			.set(network)
			.map_err(|_| "NetworkHandle::register_network called more than once")
			.expect("Registering network cannot fail");
	}

	pub fn register_keystore(&self, keystore: Arc<LocalKeystore>) {
		self.keystore
			.set(keystore)
			.map_err(|_| "NetworkHandle::register_keystore called more than once")
			.expect("Registering keystore cannot fail");
	}

	pub fn register_client(&self, client: Arc<FullClient>) {
		self.client
			.set(client)
			.map_err(|_| "NetworkHandle::register_client called more than once")
			.expect("Registering client cannot fail");
	}
}

pub fn spawn<Block>(
	store: Arc<RocksdbShardStore>,
	mut svc: Box<dyn NotificationService + Send>,
	request_receiver: Receiver<IncomingRequest>,
	role: Role,
) -> NetworkHandle<Block>
where
	Block: BlockT,
{
	let peers: Arc<Mutex<HashMap<PeerId, Option<ObservedRole>>>> =
		Arc::new(Mutex::new(HashMap::new()));
	let (blob_notification_sender, mut blob_notification_receiver) =
		mpsc::unbounded_channel::<BlobNotification>();
	let network = Arc::new(OnceCell::new());
	let keystore = Arc::new(OnceCell::new());
	let client = Arc::new(OnceCell::new());

	tokio::spawn({
		let network = network.clone();
		let keystore = keystore.clone();
		let client = client.clone();

		async move {
			loop {
				futures::select! {
					// Notification dispatcher
					maybe_cmd = blob_notification_receiver.recv().fuse() => {
						if let Some(annoucement) = maybe_cmd {
							let data = annoucement.encode();
							// Sending the notification only to authorities that need to handle it.
							let snapshot: Vec<PeerId> = peers.lock().iter().filter_map(|(peer_id, role_opt)| {
								match role_opt {
									Some(ObservedRole::Authority) => Some(peer_id.clone()),
									_ => None,
								}
							}).collect();
							for peer in snapshot {
								let _ = svc.send_async_notification(&peer, data.clone()).await.map_err(|e| {
									log::error!("Failed to send announcement notification to peer: {peer:?} - Error: {e:?}");
								});
							}
						} else {
							break;
						}
					}

					// Notification handler
					maybe_evt = svc.next_event().fuse() => {
						if let Some(evt) = maybe_evt {
							match evt {
								NotificationEvent::NotificationStreamOpened { peer, handshake, .. } => {
									let net: &Arc<NetworkService<Block, <Block as BlockT>::Hash>> = network.get().unwrap();
									let peer_role = net.peer_role(peer, handshake);

									if let Some(peer_role) = peer_role {
										peers.lock().insert(peer, Some(peer_role));
									}
								}
								NotificationEvent::NotificationStreamClosed { peer } => {
									peers.lock().remove(&peer);
								}
								NotificationEvent::NotificationReceived { peer, notification } => {
									decode_blob_notification(&notification, &network, peer, &store, &keystore, &role, &client).await;
								}
								NotificationEvent::ValidateInboundSubstream {..} => {
									// Nothing since we don't do handshake or peer validation for now
								}
							}
						} else {
							break;
						}
					},

					// Request response handler
					maybe_req = request_receiver.recv().fuse() => {
						if let Ok(request) = maybe_req {
							let peer = request.peer;
							let peer_role = peers.lock().get(&peer).cloned().unwrap_or(None);
							decode_blob_request(request, &store, &peer_role).await;
						} else {
							break;
						}
					}


				}
			}
		}
	});

	NetworkHandle {
		blob_notification_sender,
		network,
		keystore,
		client,
	}
}

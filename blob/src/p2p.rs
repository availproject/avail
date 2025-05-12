use std::{collections::HashSet, sync::Arc};

use crate::{
	decode_blob_notification, decode_blob_request, store::ShardStore, types::BlobNotification,
};
use async_channel::Receiver;
use codec::Encode;
use futures::FutureExt;
use once_cell::sync::OnceCell;
use parking_lot::Mutex;
use sc_network::{
	config::IncomingRequest, service::traits::NotificationEvent, NetworkService,
	NotificationService, PeerId,
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
}

impl<Block> NetworkHandle<Block>
where
	Block: BlockT,
{
	pub fn register_network(&self, network: Arc<NetworkService<Block, <Block as BlockT>::Hash>>) {
		self.network
			.set(network)
			.map_err(|_| "NetworkHandle::register_network called more than once")
			.unwrap();
	}
}

pub fn spawn<Block, S>(
	_store: Arc<S>,
	mut svc: Box<dyn NotificationService + Send>,
	request_receiver: Receiver<IncomingRequest>,
) -> NetworkHandle<Block>
where
	Block: BlockT,
	S: ShardStore + 'static,
{
	let peers = Arc::new(Mutex::new(HashSet::new()));
	let (blob_notification_sender, mut blob_notification_receiver) =
		mpsc::unbounded_channel::<BlobNotification>();
	let network = Arc::new(OnceCell::new());

	tokio::spawn({
		let network = network.clone();
		async move {
			loop {
				futures::select! {
					maybe_cmd = blob_notification_receiver.recv().fuse() => {
						if let Some(annoucement) = maybe_cmd {
							let data = annoucement.encode();
							let snapshot: Vec<PeerId> = peers.lock().iter().cloned().collect();
							for peer in snapshot {
								let _ = svc.send_async_notification(&peer, data.clone()).await.map_err(|e| {
									log::error!("Failed to send announcement notification to peer: {peer:?} - Error: {e:?}");
								});
							}
						} else {
							break;
						}
					}

					maybe_evt = svc.next_event().fuse() => {
						if let Some(evt) = maybe_evt {
							match evt {
								NotificationEvent::NotificationStreamOpened { peer, .. } => {
									peers.lock().insert(peer);
								}
								NotificationEvent::NotificationStreamClosed { peer } => {
									peers.lock().remove(&peer);
								}
								NotificationEvent::NotificationReceived { peer, notification } => {
									if peers.lock().contains(&peer) {
										decode_blob_notification(&notification, &network, peer).await;
									}
								}
								NotificationEvent::ValidateInboundSubstream {..} => {
									// Nothing since we don't do handshake or peer validation for now
								}
							}
						} else {
							break;
						}
					},

					maybe_req = request_receiver.recv().fuse() => {
						if let Ok(request) = maybe_req {
							decode_blob_request(request).await;
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
	}
}

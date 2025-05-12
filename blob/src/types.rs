use std::sync::Arc;

use codec::{Decode, Encode};
use sc_network::ProtocolName;
use sp_core::H256;

use crate::{p2p::NetworkHandle, store::MockShardStore};
use once_cell::sync::OnceCell;
use sp_runtime::traits::Block as BlockT;
use tokio::sync::mpsc;

pub type BlobHash = H256;

pub const BLOB_NOTIF_PROTO_STR: &str = "/avail/blob/notif/1";
pub const BLOB_NOTIF_PROTO: ProtocolName = ProtocolName::Static(BLOB_NOTIF_PROTO_STR);

pub const BLOB_REQ_PROTO_STR: &str = "/avail/blob/req/1";
pub const BLOB_REQ_PROTO: ProtocolName = ProtocolName::Static(BLOB_REQ_PROTO_STR);

#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
pub struct BlobMetadata {
	pub hash: BlobHash,
	pub size: u64,
	pub nb_shard: u16,
	// pub shard_holders: Vec<(PeerId, H256)> // Peer and signature
}

#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
pub struct Blob {
	pub metadata: BlobMetadata,
	pub data: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
pub struct Shard {
	pub hash: BlobHash,
	pub shard_id: u16,
	pub data: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
pub struct ShardRequest {
	pub hash: BlobHash,
	pub shard_id: u16,
}

#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
pub enum BlobNotification {
	Announce(BlobMetadata),
}

#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
pub enum BlobRequest {
	ShardRequest(ShardRequest),
}

#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
pub enum BlobResponse {
	ShardResponse(Shard),
}

#[derive(Clone)]
pub struct Deps<Block>
where
	Block: BlockT,
{
	pub shard_store: Arc<MockShardStore>,
	pub blob_handle: Arc<NetworkHandle<Block>>,
}
impl<Block> Default for Deps<Block>
where
	Block: BlockT,
{
	fn default() -> Self {
		let shard_store = Arc::new(MockShardStore::default());
		let (blob_notification_sender, _blob_notification_receiver) =
			mpsc::unbounded_channel::<BlobNotification>();
		let network = Arc::new(OnceCell::new());
		let blob_handle = Arc::new(NetworkHandle {
			blob_notification_sender,
			network,
		});
		Deps {
			shard_store,
			blob_handle,
		}
	}
}

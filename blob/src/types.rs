use std::sync::Arc;

use codec::{Decode, Encode};
use sc_keystore::LocalKeystore;
use sc_network::ProtocolName;
use sc_service::{Role, TFullClient};
use sp_core::H256;

use crate::{p2p::NetworkHandle, store::RocksdbShardStore};
use da_runtime::{apis::RuntimeApi, NodeBlock as Block};
use once_cell::sync::OnceCell;
use sc_executor::NativeElseWasmExecutor;
use sp_runtime::traits::Block as BlockT;
use tokio::sync::mpsc;

pub type BlobHash = H256;

pub const BLOB_NOTIF_PROTO_STR: &str = "/avail/blob/notif/1";
pub const BLOB_NOTIF_PROTO: ProtocolName = ProtocolName::Static(BLOB_NOTIF_PROTO_STR);

pub const BLOB_REQ_PROTO_STR: &str = "/avail/blob/req/1";
pub const BLOB_REQ_PROTO: ProtocolName = ProtocolName::Static(BLOB_REQ_PROTO_STR);

/// ExecutorDispatch and FullClient were put here cause we need it for blob service but we cannot have a circular dependency, clean later.
/// Maybe put in avail base later.

// Declare an instance of the native executor named `ExecutorDispatch`. Include the wasm binary as
// the equivalent wasm code.
pub struct ExecutorDispatch;

impl sc_executor::NativeExecutionDispatch for ExecutorDispatch {
	type ExtendHostFunctions = (
		frame_benchmarking::benchmarking::HostFunctions,
		frame_system::native::hosted_header_builder::hosted_header_builder::HostFunctions,
		avail_base::mem_tmp_storage::hosted_mem_tmp_storage::HostFunctions,
		da_runtime::kate::native::hosted_kate::HostFunctions,
	);

	fn dispatch(method: &str, data: &[u8]) -> Option<Vec<u8>> {
		da_runtime::apis::api::dispatch(method, data)
	}

	fn native_version() -> sc_executor::NativeVersion {
		da_runtime::native_version()
	}
}

/// The full client type definition.
pub type FullClient = TFullClient<Block, RuntimeApi, NativeElseWasmExecutor<ExecutorDispatch>>;

/// The metadata of a blob.
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
pub struct BlobMetadata {
	/// The Hash of the blob.
	pub hash: BlobHash,
	/// The size of the blob.
	pub size: u64,
	/// The number of shard for the blob.
	pub nb_shard: u16,
	/// The number of block for which we need to keep the metadata.
	/// Only for RPC nodes.
	pub block_ttl: u32,
}

/// Shard data
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
pub struct Shard {
	/// The hash of the blob this shard is associated to.
	pub hash: BlobHash,
	/// The index of the shard.
	pub shard_id: u16,
	/// The actual data of this shard (the part of the blob).
	pub data: Vec<u8>,
	/// The number of block for which we need to keep the shards.
	/// For validators this will be a long duration.
	/// For RPC nodes, it will be few minutes / hours
	pub block_ttl: u32,
}

/// Structure for a shard request.
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
pub struct ShardRequest {
	/// The hash of the blob.
	pub hash: BlobHash,
	/// The index of the shard.
	pub shard_ids: Vec<u16>,
}

/// Enum for different type of notification.
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
pub enum BlobNotification {
	Announce(BlobMetadata),
}

/// Enum for different type of request.
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
pub enum BlobRequest {
	ShardRequest(ShardRequest),
}

/// Enum for different type of response.
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
pub enum BlobResponse {
	ShardResponse(Vec<Shard>),
}

/// The RPC dependecies to enable blob service.
/// Default implementation is made for ease of use in different files.
#[derive(Clone)]
pub struct Deps<Block>
where
	Block: BlockT,
{
	pub shard_store: Arc<RocksdbShardStore>,
	pub blob_handle: Arc<NetworkHandle<Block>>,
	pub keystore: Option<Arc<LocalKeystore>>,
	pub role: Role,
}
impl<Block> Default for Deps<Block>
where
	Block: BlockT,
{
	fn default() -> Self {
		let shard_store = Arc::new(RocksdbShardStore::default());
		let (blob_notification_sender, _blob_notification_receiver) =
			mpsc::unbounded_channel::<BlobNotification>();
		let network = Arc::new(OnceCell::new());
		let keystore = Arc::new(OnceCell::new());
		let client = Arc::new(OnceCell::new());
		let blob_handle = Arc::new(NetworkHandle {
			blob_notification_sender,
			network,
			keystore,
			client,
		});
		Deps {
			shard_store,
			blob_handle,
			keystore: None,
			role: Role::Full,
		}
	}
}

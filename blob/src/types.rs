use std::sync::Arc;

use codec::{Decode, Encode};
use sc_network::{PeerId, ProtocolName};
use sc_service::{Role, TFullClient};
use sp_authority_discovery::AuthorityId;
use sp_core::H256;

use crate::{p2p::BlobHandle, store::RocksdbShardStore};
use da_runtime::{apis::RuntimeApi, NodeBlock as Block};
use once_cell::sync::OnceCell;
use sc_executor::NativeElseWasmExecutor;
use sc_network_gossip::{ValidationResult, Validator, ValidatorContext};
use sp_runtime::traits::Block as BlockT;
use sp_std::collections::btree_map::BTreeMap;

pub type BlobHash = H256;

pub const BLOB_REQ_PROTO_STR: &str = "/avail/blob/req/1";
pub const BLOB_REQ_PROTO: ProtocolName = ProtocolName::Static(BLOB_REQ_PROTO_STR);

pub const BLOB_GOSSIP_PROTO_STR: &str = "/avail/blob/gossip/1";
pub const BLOB_GOSSIP_PROTO: ProtocolName = ProtocolName::Static(BLOB_GOSSIP_PROTO_STR);

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

/// The network gossip validator for blob service
pub struct BlobGossipValidator;
impl<B: BlockT> Validator<B> for BlobGossipValidator {
	fn validate(
		&self,
		ctx: &mut dyn ValidatorContext<B>,
		_sender: &PeerId,
		data: &[u8],
	) -> ValidationResult<<B as BlockT>::Hash> {
		let topic = B::Hash::default();

		// Here we don't use directly ValidationResult::ProcessAndKeep(topic) cause we'll first process and start to ask shards to peers THEN we gossip the notification.
		// We prefer to first gossip the notification as the peers just want that piece of information
		ctx.broadcast_message(topic.clone(), data.to_vec(), false);

		ValidationResult::ProcessAndDiscard(topic)
	}
}

/// The RPC dependecies to enable blob service.
/// Default implementation is made for ease of use in different files.
#[derive(Clone)]
pub struct Deps<Block>
where
	Block: BlockT,
{
	pub blob_handle: Arc<BlobHandle<Block>>,
}

impl<Block> Default for Deps<Block>
where
	Block: BlockT,
{
	fn default() -> Self {
		let shard_store = Arc::new(RocksdbShardStore::default());
		let network = Arc::new(OnceCell::new());
		let keystore = Arc::new(OnceCell::new());
		let client = Arc::new(OnceCell::new());
		let sync_service = Arc::new(OnceCell::new());
		let gossip_cmd_sender = Arc::new(OnceCell::new());
		let role = Role::Full;
		let blob_handle = Arc::new(BlobHandle {
			network,
			keystore,
			client,
			sync_service,
			gossip_cmd_sender,
			role,
			shard_store,
		});
		Deps { blob_handle }
	}
}

/***** Structs that will go in the shard store *****/
/// The metadata of a blob and ownership data (who owns what shards)
/// This will be stored by everyone for now
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
pub struct BlobMetadata<Block: BlockT> {
	/// The Hash of the blob.
	pub hash: BlobHash,
	/// The size of the blob.
	pub size: u64,
	/// The number of shards for the blob.
	pub nb_shards: u16,
	/// The commitments of the blob.
	pub commitments: Vec<u8>,
	/// The ownership data of the blob: shard_id -> (validator address, base58 PeerId).
	pub ownership: BTreeMap<u16, Vec<(AuthorityId, String)>>,
	/// This field is used to determine wether we received the BlobReceived notification.
	/// In some cases, we can receive ShardReceived notification before BlobReceived notification.
	/// This is expected in P2P protocols, we use this field in case we record shard for a blob we don't have yet.
	/// In case we are not notified, we'll use a way shorter time to live.
	pub is_notified: bool,
	/// Block from which this blob is considered expired
	pub expires_at: u64,
	/// The finalized block hash for other nodes reference
	pub finalized_block_hash: Block::Hash,
	/// The finalized block number for other nodes reference
	pub finalized_block_number: u64,
}

impl<Block: BlockT> BlobMetadata<Block> {
	pub fn insert_in_ownership(
		&mut self,
		shard_id: &u16,
		authority_id: &AuthorityId,
		encoded_peer_id: &String,
	) -> bool {
		let mut already_stored = false;
		self.ownership
			.entry(*shard_id)
			.and_modify(|existing_ownership| {
				let new_entry = (authority_id.clone(), encoded_peer_id.clone());
				if !existing_ownership.contains(&new_entry) {
					existing_ownership.push(new_entry);
				} else {
					already_stored = true;
				}
			})
			.or_insert_with(|| vec![(authority_id.clone(), encoded_peer_id.clone())]);
		already_stored
	}

	pub fn merge_ownerships(&mut self, ownerhsip: BTreeMap<u16, Vec<(AuthorityId, String)>>) {
		for (shard_id, mut owners) in ownerhsip {
			let entry = self.ownership.entry(shard_id).or_default();
			entry.append(&mut owners);
			entry.sort_unstable();
			entry.dedup();
		}
	}
}

/// Shard object that will get store by each validator
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
pub struct Shard {
	/// The hash of the blob this shard is associated to.
	pub blob_hash: BlobHash,
	/// The index of the shard.
	pub shard_id: u16,
	/// The actual data of this shard (the part of the blob).
	pub data: Vec<u8>,
	/// The size of the shard
	pub size: u64,
}

/***** Structs used for notification / request / response *****/
/// Structure for the notification when a blob is received from the RPC
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
pub struct BlobReceived<Block: BlockT> {
	/// The hash of the blob
	pub hash: BlobHash,
	/// The size of the blob.
	pub size: u64,
	/// The number of shard for the blob.
	pub nb_shards: u16,
	/// The commitments of the blob
	pub commitments: Vec<u8>,
	/// The ownership data of the blob: shard_id -> (validator address, base58 PeerId)
	pub ownership: BTreeMap<u16, Vec<(AuthorityId, String)>>,
	/// The original encoded peerId that received the blob
	pub original_peer_id: String,
	/// The finalized block hash for other nodes reference
	pub finalized_block_hash: <Block as BlockT>::Hash,
	/// The finalized block number for other nodes reference
	pub finalized_block_number: u64,
}

/// Structure for the request when a shard is requested from a validator
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
pub struct ShardRequest {
	/// The hash of the blob.
	pub hash: BlobHash,
	/// The index of the shard.
	pub shard_ids: Vec<u16>,
}

/// Structure for the response after a shard is requested from a validator
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
pub struct ShardResponse {
	/// The hash of the blob.
	pub hash: BlobHash,
	/// The index of the shard.
	pub shards: Vec<Shard>,
}

/// Structure for the notification a validator sends after receiving a shard
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
pub struct ShardReceived {
	/// The hash of the blob.
	pub hash: BlobHash,
	/// The index of the shard.
	pub shard_ids: Vec<u16>,
	/// The validator address
	pub address: AuthorityId,
	/// The original encode peerId that received the blob
	pub original_peer_id: String,
}

/// Structure used in the Cell request
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
pub struct CellUnitRequest {
	/// The shard_id required
	pub shard_id: u16,
	/// The start index of the linear piece of data we want
	pub start: u64,
	/// The end index of the linear piece of data we want
	pub end: u64,
}

/// Structure for the request when cells are requested from a validator
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
pub struct CellRequest {
	/// The hash of the blob we required
	pub hash: BlobHash,
	/// The specific cell ranges we need
	pub cell_units: Vec<CellUnitRequest>,
}

/// Structure used in the Cell response
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
pub struct CellUnitResponse {
	/// The shard_id required
	pub shard_id: u16,
	/// The start index of the linear piece of data we want
	pub start: u64,
	/// The end index of the linear piece of data we want
	pub end: u64,
	/// The actual data corresponding to the request
	pub data: Vec<u8>,
	/// In case there's no data found, we'll have here the reason, None means success
	pub failed_reason: Option<String>,
}
/// Structure for the response after cells are requested from a validator
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
pub struct CellResponse {
	/// The hash of the blob we required
	pub hash: BlobHash,
	/// The specific cell ranges data
	pub cell_units_response: Vec<CellUnitResponse>,
}

/***** Enums used for notification / request / response *****/
/// Enum for different types of notifications.
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
pub enum BlobNotification<Block: BlockT> {
	BlobReceived(BlobReceived<Block>),
	ShardReceived(ShardReceived),
}

/// Enum for different types of requests.
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
pub enum BlobRequest {
	ShardRequest(ShardRequest),
	CellRequest(CellRequest),
}

/// Enum for different types of responses.
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
pub enum BlobResponse {
	ShardResponse(ShardResponse),
	CellResponse(CellResponse),
}

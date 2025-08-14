use crate::{p2p::BlobHandle, store::RocksdbBlobStore};
use codec::{Decode, Encode};
use da_runtime::{apis::RuntimeApi, NodeBlock as Block};
use once_cell::sync::OnceCell;
use sc_executor::NativeElseWasmExecutor;
use sc_network::{PeerId, ProtocolName};
use sc_network_gossip::{ValidationResult, Validator, ValidatorContext};
use sc_service::{Role, TFullClient};
use scale_info::TypeInfo;
use serde::{Deserialize, Serialize};
use sp_authority_discovery::AuthorityId;
use sp_core::H256;
use sp_runtime::traits::Block as BlockT;
use std::sync::Arc;

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
		da_control::extensions::native::hosted_commitment_builder::HostFunctions,
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

		// Here we don't use directly ValidationResult::ProcessAndKeep(topic) cause we'll first process and start to ask blobs to peers THEN we gossip the notification.
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
		let blob_store = Arc::new(RocksdbBlobStore::default());
		let blob_data_store = Arc::new(RocksdbBlobStore::default()); // TODO BLOB Actually use a different type of db for blob_data store and separate functions
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
			blob_store,
			blob_data_store,
		});
		Deps { blob_handle }
	}
}

/***** Structs that will go in the blob store *****/
/// The metadata of a blob and ownership data (who owns what blobs)
/// This will be stored by everyone for now
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode, Serialize, Deserialize)]
pub struct BlobMetadata<Block: BlockT> {
	/// The Hash of the blob.
	pub hash: BlobHash,
	/// The size of the blob.
	pub size: u64,
	/// The commitment of the blob.
	pub commitment: Vec<u8>,
	/// Store the number of validators per blob for this blob metadata
	pub nb_validators_per_blob: u32,
	/// This field is used to determine wether we received the BlobReceived notification.
	/// In some cases, we can receive BlobStored notification before BlobReceived notification.
	/// This is expected in P2P protocols, we use this field in case we record blob metadata for blobs we don't have yet.
	/// In case we are not notified, we'll use a way shorter time to live.
	pub is_notified: bool,
	/// Block from which this blob is considered expired
	pub expires_at: u64,
	/// The finalized block hash for other nodes reference
	pub finalized_block_hash: Block::Hash,
	/// The finalized block number for other nodes reference
	pub finalized_block_number: u64,
}

/// Blob object that will get store by each validator
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode, Serialize, Deserialize)]
pub struct Blob {
	/// The hash of the blob.
	pub blob_hash: BlobHash,
	/// The actual data of this blob.
	pub data: Vec<u8>,
	/// The size of the blob.
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
	/// The commitment of the blob
	pub commitment: Vec<u8>,
	/// The optional ownership entry of the sending rpc
	pub ownership: Option<OwnershipEntry>,
	/// The original encoded peerId that received the blob
	pub original_peer_id: String,
	/// The finalized block hash for other nodes reference
	pub finalized_block_hash: <Block as BlockT>::Hash,
	/// The finalized block number for other nodes reference
	pub finalized_block_number: u64,
}

/// Structure for the request when a blob is requested from a validator
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
pub struct BlobRequest {
	/// The hash of the blob.
	pub hash: BlobHash,
	/// The data the validator signs to prove he sent the blob request (blob_hash - "request")
	pub signature_data: BlobSignatureData,
}

/// Structure for the response after a blob is requested from a validator
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
pub struct BlobResponse {
	/// The hash of the blob.
	pub hash: BlobHash,
	/// The blob data.
	pub blob: Blob,
}

/// Structure for the notification a validator sends after receiving a blob
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
pub struct BlobStored {
	/// The hash of the blob.
	pub hash: BlobHash,
	/// The ownership entry for this validator
	pub ownership_entry: OwnershipEntry,
}

/// Structure for the signature that validator sends when sending notification / requests
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
pub struct BlobSignatureData {
	pub signer: Vec<u8>,
	pub signature: Vec<u8>,
}

/// Structure to hold data about a node having a blob
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode, TypeInfo, Serialize, Deserialize)]
pub struct OwnershipEntry {
	/// The address that owns the blob
	pub address: AuthorityId,
	/// The corresponding peer encoded
	pub encoded_peer_id: String,
	/// The signature of the holder (blob_hash - "stored")
	pub signature: Vec<u8>,
}

/// Helper structure used in the inherent to give a blob status
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode, TypeInfo)]
pub struct BlobTxSummary {
	/// The hash of the blob
	pub hash: BlobHash,
	/// The transaction index in the block
	pub tx_index: u32,
	/// Indicates if the blob was successfully uploaded to validators stores
	pub success: bool,
	/// In case of failure, this will contain the reason
	pub reason: Option<String>,
	/// The vector of ownership entries
	pub ownership: Vec<OwnershipEntry>,
}
impl BlobTxSummary {
	pub fn convert_to_primitives(
		input: Vec<BlobTxSummary>,
	) -> Vec<(
		H256,           // Blob hash
		u32,            // Tx Index
		bool,           // Success
		Option<String>, // Error reason
		Vec<(
			AuthorityId, // Validator address
			String,      // Encoded Peer id
			Vec<u8>,     // Signature
		)>,
	)> {
		input
			.into_iter()
			.map(|summary| {
				let ownership: Vec<(AuthorityId, String, Vec<u8>)> = summary
					.ownership
					.into_iter()
					.map(|entry| (entry.address, entry.encoded_peer_id, entry.signature))
					.collect();
				(
					summary.hash,
					summary.tx_index,
					summary.success,
					summary.reason,
					ownership,
				)
			})
			.collect()
	}
}

/// Structure for the request when a blob is requested from an RPC
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
pub struct BlobQueryRequest {
	/// The hash of the blob.
	pub hash: BlobHash,
}

/// Structure for the response when a blob is requested from an RPC
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
pub struct BlobQueryResponse {
	/// The blob.
	pub blob: Blob,
}

/***** Enums used for notification / request / response *****/
/// Enum for different types of notifications.
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
pub enum BlobNotification<Block: BlockT> {
	BlobReceived(BlobReceived<Block>),
	BlobStored(BlobStored),
}

/// Enum for different types of requests.
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
pub enum BlobRequestEnum {
	BlobRequest(BlobRequest),
	BlobQueryRequest(BlobQueryRequest),
}

/// Enum for different types of responses.
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
pub enum BlobResponseEnum {
	BlobResponse(BlobResponse),
	BlobQueryResponse(Option<Blob>),
}

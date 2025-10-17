use crate::{
	utils::{zstd_compress, zstd_decompress},
	LOG_TARGET,
};
use codec::{Decode, Encode};
use da_runtime::{apis::RuntimeApi, NodeBlock as Block};
use parking_lot::Mutex;
use sc_executor::NativeElseWasmExecutor;
use sc_network::{NetworkPeers, NetworkService, PeerId, ProtocolName, ReputationChange};
use sc_network_gossip::{MessageIntent, ValidationResult, Validator, ValidatorContext};
use sc_service::TFullClient;
use scale_info::TypeInfo;
use serde::{Deserialize, Serialize};
use sp_authority_discovery::AuthorityId;
use sp_core::{blake2_256, H256};
use sp_runtime::{
	traits::{Block as BlockT, Hash as HashT, HashingFor},
	AccountId32,
};
use std::{
	borrow::Cow,
	collections::HashMap,
	time::{Duration, Instant},
};

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
pub struct BlobGossipValidator {
	live: Mutex<HashMap<H256, Instant>>, // message hash -> expire_at
	ttl: Duration,                       // must be > PERIODIC_MAINTENANCE_INTERVAL
}
impl Default for BlobGossipValidator {
	fn default() -> Self {
		Self {
			ttl: Duration::from_millis(1500),
			live: Mutex::new(HashMap::new()),
		}
	}
}
impl<B: BlockT> Validator<B> for BlobGossipValidator {
	fn validate(
		&self,
		_ctx: &mut dyn ValidatorContext<B>,
		_sender: &PeerId,
		data: &[u8],
	) -> ValidationResult<<B as BlockT>::Hash> {
		// Some pre-checks that will avoid flooding the network with bad information
		// This means the peer won't get penalize "officially", but it keeps the network light
		let mut input = &data[..];
		match BlobNotification::decode(&mut input) {
			Ok(_) => { /* We can process it*/ },
			Err(_) => {
				return ValidationResult::Discard;
			},
		};

		let h = H256::from(blake2_256(data));
		self.live.lock().insert(h, Instant::now() + self.ttl);
		let topic: B::Hash = HashingFor::<B>::hash(b"blob_topic");

		ValidationResult::ProcessAndKeep(topic)
	}

	// Allow everything to be sent, EXCEPT periodic rebroadcasts.
	fn message_allowed<'a>(
		&'a self,
	) -> Box<dyn FnMut(&PeerId, MessageIntent, &B::Hash, &[u8]) -> bool + 'a> {
		Box::new(|_who, intent, _topic, _data| {
			// Initial/forced broadcasts are fine; periodic rebroadcasts are blocked.
			intent != MessageIntent::PeriodicRebroadcast
		})
	}

	// Expire at the next maintenance.
	fn message_expired<'a>(&'a self) -> Box<dyn FnMut(B::Hash, &[u8]) -> bool + 'a> {
		Box::new(move |_topic, msg| {
			let h = H256::from(blake2_256(msg));
			let now = Instant::now();
			let mut live = self.live.lock();
			match live.get(&h).copied() {
				Some(deadline) if deadline > now => false, // keep until first periodic happens
				_ => {
					live.remove(&h);
					true
				}, // expire after that
			}
		})
	}
}

#[derive(Debug, Clone, Copy)]
pub enum BlobReputationChange {
	MalformedRequest,
	MalformedResponse,
	InvalidSignature,
	InvalidRole,
}
impl BlobReputationChange {
	pub fn penalty(self) -> i32 {
		match self {
			BlobReputationChange::MalformedRequest => -200,
			BlobReputationChange::MalformedResponse => -200,
			BlobReputationChange::InvalidSignature => -1000,
			BlobReputationChange::InvalidRole => -1000,
		}
	}

	pub fn reason_str(self) -> &'static str {
		match self {
			BlobReputationChange::MalformedRequest => "blob-request-malformed",
			BlobReputationChange::MalformedResponse => "blob-response-malformed",
			BlobReputationChange::InvalidSignature => "blob-invalid-signature",
			BlobReputationChange::InvalidRole => "blob-invalid-role",
		}
	}

	pub fn reputation_change(self) -> ReputationChange {
		log::warn!("Issuing reputation change: {}", self.reason_str());
		ReputationChange::new(self.penalty(), self.reason_str())
	}

	pub fn no_change() -> Vec<ReputationChange> {
		Vec::default()
	}

	pub fn report<Block: BlockT>(
		self,
		network: &NetworkService<Block, Block::Hash>,
		peer: &PeerId,
	) {
		network.report_peer(peer.clone(), self.reputation_change());
	}
}

/***** Structs that will go in the blob store *****/
/// The metadata of a blob and ownership data (who owns what blobs)
/// This will be stored by everyone for now
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode, Serialize, Deserialize)]
pub struct BlobMetadata {
	/// The Hash of the blob.
	pub hash: BlobHash,
	/// The size of the blob.
	pub size: u64,
	/// The commitment of the blob.
	pub commitment: Vec<u8>,
	/// Store the number of validators per blob for this blob metadata
	pub nb_validators_per_blob: u32,
	/// Store the actual threshold number of validator for this blob to be considered valid, threshold<=nb_validators_per_blob
	pub nb_validators_per_blob_threshold: u32,
	/// This field is used to determine wether we received the BlobReceived notification.
	/// In some cases, we can receive BlobStored notification before BlobReceived notification.
	/// This is expected in P2P protocols, we use this field in case we record blob metadata for blobs we don't have yet.
	/// In case we are not notified, we'll use a way shorter time to live.
	pub is_notified: bool,
	/// Block from which this blob is considered expired
	pub expires_at: u64,
	/// The finalized block hash for other nodes reference
	pub finalized_block_hash: H256,
	/// The finalized block number for other nodes reference
	pub finalized_block_number: u64,
	/// The list of storing validators
	pub storing_validator_list: Vec<AccountId32>,
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
pub struct BlobReceived {
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
	pub finalized_block_hash: H256,
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
	/// The encoded and most likely compressed blob data,
	pub blob: CompressedBlob,
}

/// Structure for the notification a validator sends after receiving a blob
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
pub struct BlobStored {
	/// The hash of the blob.
	pub hash: BlobHash,
	/// The ownership entry for this validator
	pub ownership_entry: OwnershipEntry,
	/// The finalized block hash for other nodes reference
	pub finalized_block_hash: H256,
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
	pub address: AccountId32,
	/// The babe key of the validator
	pub babe_key: AuthorityId,
	/// The corresponding peer encoded
	pub encoded_peer_id: String,
	/// The signature of the holder (blob_hash - address(AccountId32) - "stored")
	pub signature: Vec<u8>,
}

/// Helper structure used in the inherent to give a blob status
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode, TypeInfo)]
pub struct BlobTxSummary {
	/// The hash of the blob
	pub hash: BlobHash,
	/// Finalized block hash checkpoint
	pub finalized_block_hash_checkpoint: H256,
	/// The transaction index in the block
	pub tx_index: u32,
	/// Indicates if the blob was successfully uploaded to validators stores
	pub success: bool,
	/// In case of failure, this will contain the reason
	pub reason: Option<String>,
	/// The list of missing validators
	pub missing_validators: Vec<AccountId32>,
	/// The vector of ownership entries
	pub ownership: Vec<OwnershipEntry>,
}
impl BlobTxSummary {
	pub fn convert_to_primitives(
		input: Vec<BlobTxSummary>,
	) -> Vec<(
		H256,             // Blob hash
		H256,             // Finalized block hash checkpoint
		u32,              // Tx Index
		bool,             // Success
		Option<String>,   // Error reason
		Vec<AccountId32>, // Missing validators list
		Vec<(
			AccountId32, // Validator address
			AuthorityId, // Babe key
			String,      // Encoded Peer id
			Vec<u8>,     // Signature
		)>,
	)> {
		input
			.into_iter()
			.map(|summary| {
				let ownership: Vec<(AccountId32, AuthorityId, String, Vec<u8>)> = summary
					.ownership
					.into_iter()
					.map(|entry| {
						(
							entry.address,
							entry.babe_key,
							entry.encoded_peer_id,
							entry.signature,
						)
					})
					.collect();

				(
					summary.hash,
					summary.finalized_block_hash_checkpoint,
					summary.tx_index,
					summary.success,
					summary.reason,
					summary.missing_validators,
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
pub enum BlobNotification {
	BlobReceived(BlobReceived),
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

#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
#[repr(u8)]
pub enum CompressedBlob {
	Nocompression(Vec<u8>) = 0,
	Zstd(Vec<u8>) = 1,
}

impl CompressedBlob {
	pub fn new_nocompression(blob: Vec<u8>) -> Self {
		Self::Nocompression(blob)
	}

	// Level 1, 3, 4, 8 and 9 are fine. Going above 9 is not recommended as
	// the compression benefits are not that great but the time it takes
	// to compress goes up sopmewhat exponentially
	//
	// Level 3 is ultra fast but still provides pretty good compression rate
	pub fn new_zstd_compressed(blob: &[u8], level: i32) -> Result<Self, std::io::Error> {
		let out = zstd_compress(blob, level)?;
		Ok(Self::Zstd(out))
	}

	pub fn new_zstd_compress_with_fallback(blob: &[u8]) -> CompressedBlob {
		match CompressedBlob::new_zstd_compressed(blob, 3) {
			Ok(zstd_compressed) => zstd_compressed,
			Err(_) => {
				log::warn!(target: LOG_TARGET, "🈵 Failed to compress data. Fallbacking to non-compression");
				CompressedBlob::new_nocompression(blob.to_vec())
			},
		}
	}

	pub fn data<'a>(&'a self) -> Result<Cow<'a, Vec<u8>>, std::io::Error> {
		match self {
			Self::Nocompression(b) => Ok(Cow::Borrowed(b)),
			Self::Zstd(c) => zstd_decompress(c).map(Cow::Owned),
		}
	}

	pub fn raw_data(&self) -> &[u8] {
		match self {
			Self::Nocompression(blob) => blob,
			Self::Zstd(c_blob) => c_blob,
		}
	}

	pub fn is_compressed(&self) -> bool {
		match self {
			Self::Nocompression(_) => false,
			Self::Zstd(_) => true,
		}
	}

	pub fn variant(&self) -> u8 {
		match self {
			Self::Nocompression(_) => 0,
			Self::Zstd(_) => 1,
		}
	}

	// TODO
	// Discriminant + Data = faster
	// Data + Discriminant = smaller memory footprint
	pub fn encode(&self) -> Vec<u8> {
		let mut encoded = vec![0u8; self.raw_data().len() + 1];
		encoded[0] = self.variant();
		encoded[1..].copy_from_slice(&self.raw_data());

		encoded
	}

	// TODO
	// Discriminant + Data = faster
	// Data + Discriminant = smaller memory footprint
	pub fn decode(mut data: Vec<u8>) -> Result<CompressedBlob, ()> {
		if data.len() < 1 {
			return Err(());
		}

		let variant = data[0];
		data.remove(0);
		let res = match variant {
			0 => Ok(Self::Nocompression(data)),
			1 => Ok(Self::Zstd(data)),
			_ => Err(()),
		};

		res
	}
}

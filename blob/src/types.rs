use crate::{p2p::BlobHandle, store::RocksdbBlobStore};
use codec::{Decode, Encode};
use da_runtime::{apis::RuntimeApi, NodeBlock as Block};
use once_cell::sync::OnceCell;
use parking_lot::Mutex;
use sc_executor::NativeElseWasmExecutor;
use sc_network::{PeerId, ProtocolName};
use sc_network_gossip::{MessageIntent, ValidationResult, Validator, ValidatorContext};
use sc_service::{Role, TFullClient};
use scale_info::TypeInfo;
use serde::{Deserialize, Serialize};
use sp_authority_discovery::AuthorityId;
use sp_core::{blake2_256, H256};
use sp_runtime::{
	traits::{Block as BlockT, Hash as HashT, HashingFor},
	AccountId32,
};
use std::{
	collections::HashMap,
	sync::Arc,
	time::{Duration, Instant},
};
use std::borrow::Cow;
use crate::LOG_TARGET;
use std::io::Write;
use base64::Engine;

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
		let blob_data_store = Arc::new(RocksdbBlobStore::default());
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
	/// The encoded blob data, must be decoded to Blob type.
	pub blob: CompressedBlob,
}

/// Structure for the notification a validator sends after receiving a blob
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
pub struct BlobStored<Block: BlockT> {
	/// The hash of the blob.
	pub hash: BlobHash,
	/// The ownership entry for this validator
	pub ownership_entry: OwnershipEntry,
	/// The finalized block hash for other nodes reference
	pub finalized_block_hash: Block::Hash,
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
	BlobStored(BlobStored<Block>),
	ClearBlob,
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
		let mut out = Vec::with_capacity(blob.len() / 3);
		let mut encoder = zstd::Encoder::new(&mut out, level)?;
		// Improves performance
		encoder.set_pledged_src_size(Some(blob.len() as u64))?;

		encoder.write_all(blob)?;
        encoder.finish()?;

		Ok(Self::Zstd(out))
	}

	pub fn new_zstd_compress_timed_with_fallback(blob: &[u8], hash: H256) -> CompressedBlob {
		// The code is quite simple but is clogged by a lot of misc stuff like logging and
		// measuing time.
		let blob_org_len = blob.len();
		const BLOB_SIZE_THRESHOLD_1: usize = 100_000; // 100kB
		if blob_org_len < BLOB_SIZE_THRESHOLD_1 {
			return CompressedBlob::new_nocompression(blob.to_vec());
		}

		let compression_level = 3;
		// const BLOB_SIZE_THRESHOLD_2: usize = 50_000_000; // 50MB
		// if blob_org_len > BLOB_SIZE_THRESHOLD_2 {
		// 	compression_level = 1
		// }

		let now = std::time::Instant::now();
		match CompressedBlob::new_zstd_compressed(blob, compression_level) {
			Ok(zstd_compressed) => {
				let duration = now.elapsed();
				let compression_size = zstd_compressed.raw_data().len();
				let ratio = blob_org_len as f32 / compression_size as f32;
				log::info!(target: LOG_TARGET, "🈵🈵🈵 Blob was compressed. Ratio: {}, Duration: {} ms, Blob Hash: {:?}, CL: {}", ratio, duration.as_millis(), hash, compression_level);
				zstd_compressed
			},
			Err(_) => {
				log::warn!(target: LOG_TARGET, "🈵🈵🈵 Failed to compress data. Fallbacking to non-compression");
				CompressedBlob::new_nocompression(blob.to_vec())
			},
		}
	}

	// If Nocompression variant, returns non-compressed data
	// If Zstd variant, decompresses the data and returns it
	// The decompresion could fail but most likely this will never be the case
	//
	// It's Cow becuase we don't want to clone data in case the data is already non-compressed
	pub fn data(&self) -> Result<Cow<Vec<u8>>, std::io::Error> {
		match self {
			Self::Nocompression(blob) => Ok(Cow::Borrowed(blob)),
			Self::Zstd(c_blob) => zstd::decode_all(c_blob.as_slice()).map(Cow::Owned)
		}
	}

	// If Nocompression variant, returns non-compressed data
	// If Zstd variant, returns compressed data
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
		let now = std::time::Instant::now();
		let mut encoded = vec![0u8; self.raw_data().len() + 1];
		encoded[0] = self.variant();
		encoded[1..].copy_from_slice(&self.raw_data());
		let duration = now.elapsed();
		log::info!(target: LOG_TARGET, "🈵🈵 Blob was encoded. Duration: {} ms", duration.as_millis());

		encoded
	}

		// TODO
	// Discriminant + Data = faster
	// Data + Discriminant = smaller memory footprint
	pub fn decode(mut data: Vec<u8>) -> Result<CompressedBlob, ()> {
		let now = std::time::Instant::now();
		if data.len() < 1 {
			return Err(())
		}

		let variant = data[0];
		data.remove(0);
		let res = match variant {
			0 => Ok(Self::Nocompression(data)),
			1 => Ok(Self::Zstd(data)),
			_ => Err(())
		};
		let duration = now.elapsed();
		log::info!(target: LOG_TARGET, "🈵🈵 Blob was decoded. Duration: {} ms", duration.as_millis());

		res
	}
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct B64Param(#[serde(deserialize_with = "deserialize_base64_to_vec")] pub Vec<u8>);

fn deserialize_base64_to_vec<'de, D>(d: D) -> Result<Vec<u8>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    struct B64Visitor;

    impl<'de> serde::de::Visitor<'de> for B64Visitor {
        type Value = Vec<u8>;

        fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.write_str("a base64-encoded string")
        }

        // This path borrows the incoming &str directly; no intermediate String allocation.
        fn visit_borrowed_str<E: serde::de::Error>(self, v: &'de str) -> Result<Self::Value, E> {
			let timer = std::time::Instant::now();
            let a = base64::engine::general_purpose::STANDARD
                .decode(v)
                .map_err(E::custom);

			log::info!("🈵 Base64 decoding duration: {} ms", timer.elapsed().as_millis());
			a
        }

        // Fallback if the JSON parser doesn’t give us a borrowed str.
        fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
			let timer = std::time::Instant::now();
            let a = base64::engine::general_purpose::STANDARD
                .decode(v)
                .map_err(E::custom);
			log::info!("🈵 Base64 decoding duration: {} ms", timer.elapsed().as_millis());
			a
        }
    }

    d.deserialize_str(B64Visitor)
}

use crate::{
	p2p::BlobHandle,
	store::ShardStore,
	types::{BlobMetadata, BlobNotification, BlobReceived, Deps, Shard},
	utils::{get_my_validator_id, get_nb_shards_from_blob_size, get_shards_to_store},
	BLOB_TTL, LOG_TARGET, MAX_BLOB_SIZE, MAX_TRANSACTION_VALIDITY, MIN_TRANSACTION_VALIDITY,
	SHARD_SIZE,
};
use anyhow::{anyhow, Result};
use codec::Decode;
use da_commitment::build_da_commitments::build_da_commitments;
use da_control::Call;
use da_runtime::{RuntimeCall, UncheckedExtrinsic};
use jsonrpsee::{
	core::{async_trait, RpcResult},
	proc_macros::rpc,
	types::error::ErrorObject,
};
use kate::Seed;
use sc_authority_discovery::AuthorityDiscovery;
use sc_client_api::{BlockBackend, HeaderBackend};
use sc_keystore::LocalKeystore;
use sc_network::NetworkStateInfo;
use sc_service::Role;
use sc_transaction_pool_api::TransactionPool;
use sp_api::ProvideRuntimeApi;
use sp_authority_discovery::AuthorityId;
use sp_core::{blake2_256, Bytes, H256};
use sp_runtime::transaction_validity::TransactionSource;
use sp_runtime::{traits::Block as BlockT, SaturatedConversion};
use sp_transaction_pool::runtime_api::TaggedTransactionQueue;
use std::{
	cmp,
	collections::BTreeMap,
	marker::{PhantomData, Sync},
	sync::Arc,
};

pub enum Error {
	BlobError,
}

impl From<Error> for i32 {
	fn from(e: Error) -> i32 {
		match e {
			Error::BlobError => 1,
		}
	}
}

macro_rules! internal_err {
    ($($arg:tt)*) => {{
        ErrorObject::owned(
            Error::BlobError.into(),
            format!($($arg)*),
            None::<()>
        )
    }}
}

#[rpc(client, server)]
pub trait BlobApi<Block>
where
	Block: BlockT,
{
	#[method(name = "blob_submitBlob")]
	async fn submit_blob(&self, metadata_signed_transaction: Bytes, blob: Bytes) -> RpcResult<()>;
}

pub struct BlobRpc<Client, Pool, Block: BlockT> {
	client: Arc<Client>,
	pool: Arc<Pool>,
	blob_handle: Arc<BlobHandle<Block>>,
	_block: PhantomData<Block>,
}

impl<Client, Pool, Block: BlockT> BlobRpc<Client, Pool, Block> {
	pub fn new(client: Arc<Client>, pool: Arc<Pool>, deps: Deps<Block>) -> Self {
		Self {
			client,
			pool,
			blob_handle: deps.blob_handle,
			_block: PhantomData,
		}
	}
}

#[async_trait]
impl<Client, Pool, Block> BlobApiServer<Block> for BlobRpc<Client, Pool, Block>
where
	Block: BlockT,
	Client: HeaderBackend<Block>
		+ ProvideRuntimeApi<Block>
		+ BlockBackend<Block>
		+ AuthorityDiscovery<Block>
		+ Send
		+ Sync
		+ 'static,
	Client::Api: TaggedTransactionQueue<Block>,
	Pool: TransactionPool<Block = Block> + 'static,
{
	async fn submit_blob(&self, metadata_signed_transaction: Bytes, blob: Bytes) -> RpcResult<()> {
		// --- 0. Quick checks -------------------------------------------------
		if blob.0.is_empty() {
			return Err(internal_err!("blob cannot be empty"));
		}

		if blob.0.len() as u64 > MAX_BLOB_SIZE {
			return Err(internal_err!("blob is too big"));
		}

		if metadata_signed_transaction.0.is_empty() {
			return Err(internal_err!("metadata tx cannot be empty"));
		}

		// --- 1. Decode the opaque extrinsic ---------------------------------
		let opaque_tx: Block::Extrinsic =
			codec::Decode::decode(&mut &metadata_signed_transaction[..])
				.map_err(|_| internal_err!("failed to decode metadata extrinsic"))?;

		// --- 2. Let the runtime validate it (signature, nonce, weight) ------
		let client_info = self.client.info();
		let best_hash = client_info.best_hash;

		let validity = self
			.client
			.runtime_api()
			.validate_transaction(
				best_hash,
				TransactionSource::External,
				opaque_tx.clone(),
				best_hash,
			)
			.map_err(|e| internal_err!("runtime validate_transaction error: {e}"))?;
		let Ok(validity) = validity else {
			return Err(internal_err!(
				"metadata extrinsic rejected by runtime: {validity:?}"
			));
		};

		// --- 3. Check also that transaction lifetime is above minimum tx lifetime so it does not expire. If validity is not correct, we reject the tx
		if validity.longevity < MIN_TRANSACTION_VALIDITY {
			return Err(internal_err!(
				"signed transaction does not live for enough time"
			));
		}

		if validity.longevity > MAX_TRANSACTION_VALIDITY {
			return Err(internal_err!("signed transaction lifetime is too long"));
		}

		// --- 4. Decode to concrete call to read the metadata, Check hash, commitment, ... of the blob compared to the submitted metadata ----------------
		let blob_hash = H256::from(blake2_256(&blob));
		let encoded_metadata_signed_transaction: UncheckedExtrinsic =
			Decode::decode(&mut &metadata_signed_transaction[..])
				.map_err(|_| internal_err!("failed to decode concrete metadata call"))?;

		let commitments: Vec<u8>;
		if let RuntimeCall::DataAvailability(Call::submit_blob_metadata {
			size,
			blob_hash: provided_blob_hash,
			commitments: provided_commitments,
		}) = encoded_metadata_signed_transaction.function
		{
			// Check size
			if size as usize != blob.0.len() {
				return Err(internal_err!(
					"submit data length ({}) != blob length ({})",
					size,
					blob.0.len()
				));
			}

			// Check blob_hash
			if provided_blob_hash != blob_hash {
				return Err(internal_err!("submitted blob: {provided_blob_hash:?} does not correspond to generated blob {blob_hash:?}"));
			}

			// Check commitments
			commitments = provided_commitments;
			let generated_commitment =
				build_da_commitments(blob.to_vec(), 1024, 4096, Seed::default())
					.map_err(|e| internal_err!("Build commitment error: {e:?}"))?;
			if commitments != generated_commitment {
				return Err(internal_err!("submitted blob commitments: {commitments:?} does not correspond to generated commitments {generated_commitment:?}"));
			}
		} else {
			return Err(internal_err!(
				"metadata extrinsic must be dataAvailability.submitBlobMetadata"
			));
		}

		// Get my own peer id data
		let net = self
			.blob_handle
			.network
			.get()
			.ok_or_else(|| internal_err!("Could not get network to get my peer id"))
			.map_err(|e| e)?;

		let my_peer_id = net.local_peer_id();
		let my_peer_id_base58 = my_peer_id.to_base58();

		// --- 5. Setup blob metadata and split the blob into shard
		// --- Check first in case we already received this exact blob before
		let finalized_block_number = client_info.finalized_number.saturated_into::<u64>();
		let finalized_block_hash = client_info.finalized_hash;

		let expiration = finalized_block_number.saturating_add(BLOB_TTL);
		let mut blob_metadata = self
			.blob_handle
			.shard_store
			.get_blob_metadata(&blob_hash)
			.map_err(|e| internal_err!("Failed to get data from blob storage: {e}"))?
			.unwrap_or_else(|| {
				let blob_len = blob.len();
				let nb_shards = get_nb_shards_from_blob_size(blob_len);

				BlobMetadata {
					hash: blob_hash,
					size: blob_len.saturated_into(),
					nb_shards,
					commitments,
					ownership: BTreeMap::new(),
					is_notified: true,
					expires_at: expiration,
					finalized_block_hash,
					finalized_block_number,
				}
			});

		if blob_metadata.expires_at < expiration {
			blob_metadata.expires_at = expiration;
		}

		// Get shards that we need to store in case we're a RPC and a validator
		// TODO Blob - RPC Will anyway store all shards for now, _shards_index_to_store should be used to check what shard we don't need to delete them after
		let (_shards_index_to_store, ownership) = get_shards_to_store_rpc::<Client, Block>(
			&self.blob_handle.role,
			&self.client,
			&self.blob_handle.keystore.get().cloned(),
			blob_metadata.clone(),
			finalized_block_hash,
			my_peer_id_base58.clone(),
		)
		.await
		.map_err(|e| internal_err!("could not get shards to store: {e}"))?;

		blob_metadata.merge_ownerships(ownership);

		let mut shards: Vec<Shard> = Vec::new();
		for shard_id in 0..blob_metadata.nb_shards {
			let start = shard_id as usize * (SHARD_SIZE as usize);
			let end = cmp::min(blob.len(), start + SHARD_SIZE as usize);
			let data = blob[start..end].to_vec();

			let shard = Shard {
				blob_hash,
				shard_id,
				size: data.len().saturated_into(),
				data,
			};

			shards.push(shard);
		}

		// --- 6. Store the blob in the store, temporarily except for shards that I need to keep -------------------
		self.blob_handle
			.shard_store
			.insert_blob_metadata(&blob_metadata.hash, &blob_metadata)
			.map_err(|e| internal_err!("failed to insert blob metadata into store: {e}"))?;
		self.blob_handle
			.shard_store
			.insert_shards(&shards)
			.map_err(|e| internal_err!("failed to insert blob shards into store: {e}"))?;

		// --- 7. Announce the blob to the network -------------------
		let blob_received_notification = BlobNotification::BlobReceived(BlobReceived {
			hash: blob_metadata.hash,
			size: blob_metadata.size,
			nb_shards: blob_metadata.nb_shards,
			commitments: blob_metadata.commitments,
			ownership: blob_metadata.ownership,
			original_peer_id: my_peer_id_base58,
			finalized_block_hash,
			finalized_block_number,
		});

		let Some(gossip_cmd_sender) = self.blob_handle.gossip_cmd_sender.get() else {
			return Err(internal_err!("gossip_cmd_sender was not initialized"));
		};
		gossip_cmd_sender
			.send(blob_received_notification)
			.await
			.map_err(|e| internal_err!("internal channel closed: {e}"))?;

		// --- 8. Push the clean extrinsic to the tx pool ---------------------
		// Get the best hash once more, to submit the tx
		let best_hash = self.client.info().best_hash;
		self.pool
			.submit_one(best_hash, TransactionSource::External, opaque_tx)
			.await
			.map_err(|e| internal_err!("tx-pool error: {e}"))?;

		log::info!(target:LOG_TARGET, "0 - Just submitted a TX to the pool after uploading the blob");

		Ok(())
	}
}

async fn get_shards_to_store_rpc<Client, Block>(
	role: &Role,
	client: &Client,
	keystore: &Option<Arc<LocalKeystore>>,
	blob_metadata: BlobMetadata<Block>,
	finalized_hash: Block::Hash,
	my_peer_id_encoded: String,
) -> Result<(Vec<u16>, BTreeMap<u16, Vec<(AuthorityId, String)>>)>
where
	Block: BlockT,
	Client: HeaderBackend<Block> + ProvideRuntimeApi<Block> + AuthorityDiscovery<Block>,
{
	let mut ownership = BTreeMap::new();

	if !role.is_authority() {
		log::info!(target: LOG_TARGET, "RPC node (me) is not an authority, so I don't have to store shards");
		return Ok((Vec::new(), ownership));
	}

	let validators = client
		.authorities(finalized_hash)
		.await
		.map_err(|e| anyhow!("failed to get validators from runtime: {e}"))?;

	let Some(keystore) = keystore else {
		return Err(anyhow!("failed to get keystore"));
	};

	let Some(my_validator_id) = get_my_validator_id(&keystore) else {
		return Ok((Vec::new(), ownership));
	};

	let shards_to_store = match get_shards_to_store(
		blob_metadata.hash,
		blob_metadata.nb_shards,
		&validators,
		&my_validator_id,
	) {
		Ok(s) => s,
		Err(e) => {
			return Err(anyhow!("failed to get shards from the store: {e}"));
		},
	};

	for shard_to_store in &shards_to_store {
		ownership.insert(
			*shard_to_store,
			vec![(my_validator_id.clone(), my_peer_id_encoded.clone())],
		);
	}

	Ok((shards_to_store, ownership))
}

use crate::{
	get_my_validator_id, get_shards_to_store,
	p2p::NetworkHandle,
	store::{RocksdbShardStore, ShardStore},
	types::{BlobMetadata, BlobNotification, Deps, Shard},
	BLOB_TTL, MAX_BLOB_SIZE, MIN_TRANSACTION_VALIDITY, SHARD_SIZE, TMP_BLOB_TTL,
};
use anyhow::{anyhow, Result};
use avail_base::build_da_commitments::build_da_commitments;
use codec::Decode;
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
use sc_service::Role;
use sc_transaction_pool_api::TransactionPool;
use sp_api::ProvideRuntimeApi;
use sp_core::{blake2_256, Bytes, H256};
use sp_runtime::transaction_validity::TransactionSource;
use sp_runtime::{traits::Block as BlockT, SaturatedConversion};
use sp_transaction_pool::runtime_api::TaggedTransactionQueue;
use std::{
	cmp,
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
	store: Arc<RocksdbShardStore>,
	network: Arc<NetworkHandle<Block>>,
	keystore: Option<Arc<LocalKeystore>>,
	role: Role,
	_block: PhantomData<Block>,
}

impl<Client, Pool, Block: BlockT> BlobRpc<Client, Pool, Block> {
	pub fn new(client: Arc<Client>, pool: Arc<Pool>, deps: Deps<Block>) -> Self {
		Self {
			client,
			pool,
			store: deps.shard_store,
			network: deps.blob_handle,
			keystore: deps.keystore,
			role: deps.role,
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
		let best_hash = self.client.info().best_hash;
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

		// --- 3. Todo, check also that transaction lifetime is above minimum tx lifetime so it does not expire. If validity is not correct, we reject the tx
		if validity.longevity < MIN_TRANSACTION_VALIDITY {
			return Err(internal_err!(
				"signed transaction does not live for enough time"
			));
		}

		// --- 4. Decode to concrete call to read the metadata, Check hash, commitment, ... of the blob compared to the submitted metadata ----------------
		let blob_hash = H256::from(blake2_256(&blob));
		let encoded_metadata_signed_transaction: UncheckedExtrinsic =
			Decode::decode(&mut &metadata_signed_transaction[..])
				.map_err(|_| internal_err!("failed to decode concrete metadata call"))?;

		if let RuntimeCall::DataAvailability(Call::submit_blob_metadata {
			size,
			blob_hash: provided_blob_hash,
			commitments,
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
			let generated_commitment =
				build_da_commitments(blob.to_vec(), 512, 512, Seed::default())
					.map_err(|e| internal_err!("Build commitment error: {e:?}"))?;
			if commitments != generated_commitment {
				return Err(internal_err!("submitted blob commitments: {commitments:?} does not correspond to generated commitments {generated_commitment:?}"));
			}

			println!("ALL CHECKS WENT FINE: {generated_commitment:?} VS {commitments:?}");
		} else {
			return Err(internal_err!(
				"metadata extrinsic must be dataAvailability.submitBlobMetadata"
			));
		}

		// --- 5. Setup blob metadata and split the blob into shard
		// If I am a validator, I also might need to store a shard, hence putting a bigger TTL
		let total_shards = ((blob.len() as u64 + SHARD_SIZE - 1) / SHARD_SIZE) as u16;
		let blob_metadata = BlobMetadata {
			hash: blob_hash,
			size: blob.len().saturated_into(),
			nb_shard: total_shards,
			block_ttl: TMP_BLOB_TTL,
		};

		let shards_index_to_store = get_shards_to_store_rpc::<Client, Block>(
			&self.role,
			&self.client,
			&self.keystore,
			blob_metadata.clone(),
			best_hash,
		)
		.await
		.map_err(|e| internal_err!("could not get shards to store: {e}"))?;

		let mut shards: Vec<Shard> = Vec::new();
		for shard_id in 0..total_shards {
			let start = shard_id as usize * (SHARD_SIZE as usize);
			let end = cmp::min(blob.len(), start + SHARD_SIZE as usize);
			let data = blob[start..end].to_vec();

			let shard = Shard {
				hash: blob_hash,
				shard_id,
				data,
				block_ttl: if shards_index_to_store.contains(&shard_id) {
					BLOB_TTL
				} else {
					TMP_BLOB_TTL
				},
			};

			shards.push(shard);
		}

		// --- 6. Store the blob in the store, temporarily except for shards that I need to keep -------------------
		self.store
			.insert_blob_metadata(&blob_metadata.hash, &blob_metadata)
			.map_err(|e| internal_err!("failed to insert blob metadata into store: {e}"))?;
		self.store
			.insert_shards(&shards)
			.map_err(|e| internal_err!("failed to insert blob shards into store: {e}"))?;

		// --- 7. Announce the blob to the network -------------------
		let announce = BlobNotification::Announce(blob_metadata);
		self.network
			.blob_notification_sender
			.send(announce)
			.map_err(|e| internal_err!("internal channel closed: {e}"))?;

		// --- 8. Check validity once more, if the tx has expired, we need to remove the blob once again
		// This should not happen, it would mean our minimum tx lifetime is too low
		let best_hash = self.client.info().best_hash;
		let validity = self
			.client
			.runtime_api()
			.validate_transaction(
				best_hash,
				TransactionSource::External,
				opaque_tx.clone(),
				best_hash,
			)
			.map_err(|e| internal_err!("second runtime validate_transaction error: {e}"))?;
		if validity.is_err() {
			return Err(internal_err!(
				"second validation failed: metadata extrinsic rejected by runtime: {validity:?}. Please check the `MIN_TRANSACTION_VALIDITY`."
			));
		}

		// --- 9. Push the clean extrinsic to the tx pool ---------------------
		self.pool
			.submit_one(best_hash, TransactionSource::External, opaque_tx)
			.await
			.map_err(|e| internal_err!("tx-pool error: {e}"))?;

		Ok(())
	}
}

async fn get_shards_to_store_rpc<Client, Block>(
	role: &Role,
	client: &Client,
	keystore: &Option<Arc<LocalKeystore>>,
	blob_metadata: BlobMetadata,
	best_hash: <Block as BlockT>::Hash,
) -> Result<Vec<u16>>
where
	Block: BlockT,
	Client: HeaderBackend<Block> + ProvideRuntimeApi<Block> + AuthorityDiscovery<Block>,
{
	if !role.is_authority() {
		return Ok(Vec::new());
	}

	let validators = client
		.authorities(best_hash)
		.await
		.map_err(|e| anyhow!("failed to get validators from runtime: {e}"))?;

	let Some(keystore) = keystore else {
		return Err(anyhow!("failed to get keystore"));
	};

	let Ok(maybe_val_id) = get_my_validator_id(&keystore) else {
		return Err(anyhow!("failed to get my validator id"));
	};

	let Some(my_validator_id) = maybe_val_id else {
		return Ok(Vec::new());
	};

	let shards_to_store = get_shards_to_store(blob_metadata, &validators, my_validator_id);

	shards_to_store
}

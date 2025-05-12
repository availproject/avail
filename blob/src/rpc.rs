use crate::{
	p2p::NetworkHandle,
	store::ShardStore,
	types::{BlobMetadata, BlobNotification},
	SHARD_SIZE,
};
use codec::Decode;
use da_control::Call;
use da_runtime::{RuntimeCall, UncheckedExtrinsic};
use jsonrpsee::{
	core::{async_trait, RpcResult},
	proc_macros::rpc,
	types::error::ErrorObject,
};
use sc_client_api::{BlockBackend, HeaderBackend};
use sc_transaction_pool_api::TransactionPool;
use sp_api::ProvideRuntimeApi;
use sp_core::{blake2_256, Bytes, H256};
use sp_runtime::transaction_validity::TransactionSource;
use sp_runtime::{traits::Block as BlockT, SaturatedConversion};
use sp_transaction_pool::runtime_api::TaggedTransactionQueue;
use std::{marker::PhantomData, marker::Sync, sync::Arc};

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

pub struct BlobRpc<Client, Pool, Block: BlockT, Store: ShardStore> {
	client: Arc<Client>,
	pool: Arc<Pool>,
	_store: Arc<Store>,
	network: Arc<NetworkHandle<Block>>,
	_block: PhantomData<Block>,
}

impl<Client, Pool, Block: BlockT, Store: ShardStore> BlobRpc<Client, Pool, Block, Store> {
	pub fn new(
		client: Arc<Client>,
		pool: Arc<Pool>,
		_store: Arc<Store>,
		network: Arc<NetworkHandle<Block>>,
	) -> Self {
		Self {
			client,
			pool,
			_store,
			network,
			_block: PhantomData,
		}
	}
}

#[async_trait]
impl<Client, Pool, Block, Store> BlobApiServer<Block> for BlobRpc<Client, Pool, Block, Store>
where
	Block: BlockT,
	Client: HeaderBackend<Block>
		+ ProvideRuntimeApi<Block>
		+ BlockBackend<Block>
		+ Send
		+ Sync
		+ 'static,
	Client::Api: TaggedTransactionQueue<Block>,
	Pool: TransactionPool<Block = Block> + 'static,
	Store: ShardStore + 'static,
{
	async fn submit_blob(&self, metadata_signed_transaction: Bytes, blob: Bytes) -> RpcResult<()> {
		// --- 0. Quick checks -------------------------------------------------
		if blob.0.is_empty() {
			return Err(internal_err!("blob cannot be empty"));
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

		if validity.is_err() {
			return Err(internal_err!(
				"metadata extrinsic rejected by runtime: {validity:?}"
			));
		}

		// --- 3. Decode to concrete call to read remark.len() ----------------
		let encoded_metadata_signed_transaction: UncheckedExtrinsic =
			Decode::decode(&mut &metadata_signed_transaction[..])
				.map_err(|_| internal_err!("failed to decode concrete metadata call"))?;

		if let RuntimeCall::DataAvailability(Call::submit_data { data }) =
			encoded_metadata_signed_transaction.function
		{
			if data.len() != blob.0.len() {
				// Change to check that the len in the metadata equals len of the blob
				// Just to show we can do some checks
				return Err(internal_err!(
					"submit data length ({}) != blob length ({})",
					data.len(),
					blob.0.len()
				));
			}
		} else {
			return Err(internal_err!(
				"metadata extrinsic must be dataAvailability.submitData"
			));
		}

		// --- 4. Do something with the blob -------------------

		// ------ A. Store temporarily the blob in the store ----------------
		// TODOOOOO

		// ------ B. Announce the blob to the network ----------------
		let blob_len: u64 = blob.len().saturated_into();
		let announce = BlobNotification::Announce(BlobMetadata {
			hash: H256::from(blake2_256(&blob)),
			size: blob_len,
			nb_shard: ((blob_len + SHARD_SIZE - 1) / SHARD_SIZE).saturated_into(),
		});
		self.network
			.blob_notification_sender
			.send(announce)
			.map_err(|e| internal_err!("internal channel closed: {e}"))?;

		// --- 5. Push the clean extrinsic to the tx pool ---------------------
		self.pool
			.submit_one(best_hash, TransactionSource::External, opaque_tx)
			.await
			.map_err(|e| internal_err!("tx-pool error: {e}"))?;

		Ok(())
	}
}

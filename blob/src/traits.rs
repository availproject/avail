use crate::BlobHandle;
use da_runtime::apis::BlobApi;
use da_runtime::AccountId;
use da_runtime::UncheckedExtrinsic;
use jsonrpsee::core::async_trait;
use sc_client_api::{BlockBackend, HeaderBackend, StateBackend};
use sc_network::NetworkStateInfo;
use sc_network::PeerId;
use sc_transaction_pool_api::TransactionPool;
use sp_api::ApiError;
use sp_api::ProvideRuntimeApi;
use sp_core::H256;
use sp_runtime::traits::{Block as BlockT, HashingFor, Header as HeaderT};
use sp_runtime::transaction_validity::TransactionSource;
use sp_runtime::transaction_validity::TransactionValidity;
use sp_transaction_pool::runtime_api::TaggedTransactionQueue;
use std::{
	marker::{PhantomData, Sync},
	sync::Arc,
};

#[async_trait]
pub trait ExternalitiesT: Send + Sync {
	fn client_info(&self) -> ClientInfo;
	fn get_blob_runtime_parameters(
		&self,
		block_hash: H256,
	) -> Result<da_control::BlobRuntimeParameters, ApiError>;

	fn validate_transaction(
		&self,
		at: H256,
		source: TransactionSource,
		uxt: UncheckedExtrinsic,
		block_hash: H256,
	) -> Result<TransactionValidity, ApiError>;

	async fn submit_one(
		&self,
		block_hash: H256,
		source: TransactionSource,
		uxt: UncheckedExtrinsic,
	) -> Result<H256, String>;

	fn get_active_validators(&self, block_hash: H256) -> Result<Vec<AccountId>, ApiError>;

	fn storage(&self, at: H256, key: &[u8]) -> Result<Option<Vec<u8>>, String>;

	fn local_peer_id(&self) -> Result<PeerId, ()>;
}

#[derive(Debug, Default, Clone)]
pub struct ClientInfo {
	pub best_hash: H256,
	pub best_height: u32,
	pub finalized_hash: H256,
	pub finalized_height: u32,
}

pub struct DummyExternalities;

#[async_trait]
impl ExternalitiesT for DummyExternalities {
	fn client_info(&self) -> ClientInfo {
		todo!()
	}

	fn get_blob_runtime_parameters(
		&self,
		_block_hash: H256,
	) -> Result<da_control::BlobRuntimeParameters, ApiError> {
		todo!()
	}

	fn validate_transaction(
		&self,
		_at: H256,
		_source: TransactionSource,
		_uxt: UncheckedExtrinsic,
		_block_hash: H256,
	) -> Result<TransactionValidity, ApiError> {
		todo!()
	}

	async fn submit_one(
		&self,
		_block_hash: H256,
		_source: TransactionSource,
		_uxt: UncheckedExtrinsic,
	) -> Result<H256, String> {
		todo!()
	}

	fn get_active_validators(&self, _block_hash: H256) -> Result<Vec<AccountId>, ApiError> {
		todo!()
	}

	fn storage(&self, _at: H256, _key: &[u8]) -> Result<Option<Vec<u8>>, String> {
		todo!()
	}

	fn local_peer_id(&self) -> Result<PeerId, ()> {
		todo!()
	}
}

pub struct RealExternalities<Client, Block, Backend, Pool>
where
	Block: BlockT,
{
	client: Arc<Client>,
	pool: Arc<Pool>,
	blob_handle: Arc<BlobHandle<Block>>,
	backend: Arc<Backend>,
	_block: PhantomData<Block>,
}

impl<Client, Block, Backend, Pool> RealExternalities<Client, Block, Backend, Pool>
where
	Block: BlockT,
	Client: HeaderBackend<Block>
		+ ProvideRuntimeApi<Block>
		+ BlockBackend<Block>
		+ Send
		+ Sync
		+ 'static,
	Client::Api: TaggedTransactionQueue<Block> + BlobApi<Block>,
	Pool: TransactionPool<Block = Block> + 'static,
	Backend: sc_client_api::Backend<Block> + Send + Sync + 'static,
	Backend::State: StateBackend<HashingFor<Block>>,
	H256: From<<Block as BlockT>::Hash>,
	<Block as BlockT>::Hash: From<H256>,
	u32: From<<<Block as BlockT>::Header as HeaderT>::Number>,
	<Block as BlockT>::Extrinsic: From<UncheckedExtrinsic>,
{
	pub fn new(
		client: Arc<Client>,
		pool: Arc<Pool>,
		blob_handle: Arc<BlobHandle<Block>>,
		backend: Arc<Backend>,
	) -> Self {
		Self {
			client,
			pool,
			blob_handle,
			backend,
			_block: PhantomData,
		}
	}
}

#[async_trait]
impl<Client, Block, Backend, Pool> ExternalitiesT
	for RealExternalities<Client, Block, Backend, Pool>
where
	Block: BlockT,
	Client: HeaderBackend<Block>
		+ ProvideRuntimeApi<Block>
		+ BlockBackend<Block>
		+ Send
		+ Sync
		+ 'static,
	Client::Api: TaggedTransactionQueue<Block> + BlobApi<Block>,
	Pool: TransactionPool<Block = Block> + 'static,
	Backend: sc_client_api::Backend<Block> + Send + Sync + 'static,
	Backend::State: StateBackend<HashingFor<Block>>,
	H256: From<<Block as BlockT>::Hash>,
	<Block as BlockT>::Hash: From<H256>,
	u32: From<<<Block as BlockT>::Header as HeaderT>::Number>,
	<Block as BlockT>::Extrinsic: From<UncheckedExtrinsic>,
	H256: From<<Pool as sc_transaction_pool_api::TransactionPool>::Hash>,
{
	fn client_info(&self) -> ClientInfo {
		let client_info = self.client.info();
		ClientInfo {
			best_hash: client_info.best_hash.into(),
			best_height: u32::from(client_info.best_number),
			finalized_hash: client_info.finalized_hash.into(),
			finalized_height: u32::from(client_info.finalized_number),
		}
	}

	fn get_blob_runtime_parameters(
		&self,
		block_hash: H256,
	) -> Result<da_control::BlobRuntimeParameters, ApiError> {
		self.client
			.runtime_api()
			.get_blob_runtime_parameters(block_hash.into())
	}

	fn validate_transaction(
		&self,
		at: H256,
		source: TransactionSource,
		uxt: UncheckedExtrinsic,
		block_hash: H256,
	) -> Result<TransactionValidity, ApiError> {
		self.client.runtime_api().validate_transaction(
			at.into(),
			source,
			uxt.into(),
			block_hash.into(),
		)
	}

	async fn submit_one(
		&self,
		block_hash: H256,
		source: TransactionSource,
		uxt: UncheckedExtrinsic,
	) -> Result<H256, String> {
		let hash = self
			.pool
			.submit_one(block_hash.into(), source, uxt.into())
			.await
			.map_err(|x| x.to_string())?;
		Ok(hash.into())
	}

	fn get_active_validators(&self, block_hash: H256) -> Result<Vec<AccountId>, ApiError> {
		self.client
			.runtime_api()
			.get_active_validators(block_hash.into())
	}

	fn storage(&self, at: H256, key: &[u8]) -> Result<Option<Vec<u8>>, String> {
		let state = self
			.backend
			.state_at(at.into())
			.map_err(|e| e.to_string())?;
		state.storage(key).map_err(|e| e.to_string())
	}

	fn local_peer_id(&self) -> Result<PeerId, ()> {
		let network = self.blob_handle.network.get().cloned();
		let Some(net) = network else {
			return Err(());
		};
		Ok(net.local_peer_id())
	}
}

use crate::{BlobHandle, BlobNotification, StorageApiT};
use da_runtime::{apis::BlobApi, AccountId, UncheckedExtrinsic};
use jsonrpsee::core::async_trait;
use sc_client_api::{BlockBackend, HeaderBackend, StateBackend};
use sc_keystore::LocalKeystore;
use sc_network::{NetworkStateInfo, PeerId};
use sc_service::Role;
use sc_transaction_pool_api::TransactionPool;
use sp_api::{ApiError, ProvideRuntimeApi};
use sp_core::{crypto::KeyTypeId, H256};
use sp_runtime::{
	traits::{Block as BlockT, HashingFor, Header as HeaderT},
	transaction_validity::{TransactionSource, TransactionValidity},
};
use sp_transaction_pool::runtime_api::TaggedTransactionQueue;
use std::{
	marker::{PhantomData, Sync},
	sync::Arc,
};

pub trait RuntimeApiT: Send + Sync {
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

	fn get_active_validators(&self, block_hash: H256) -> Result<Vec<AccountId>, ApiError>;

	fn get_validator_from_key(
		&self,
		at: H256,
		id: KeyTypeId,
		key_data: Vec<u8>,
	) -> Result<Option<AccountId>, ApiError>;
}

pub struct RuntimeClient<C, B>(Arc<C>, PhantomData<B>);

impl<C, B> RuntimeClient<C, B> {
	pub fn new(client: Arc<C>) -> Self {
		Self(client, PhantomData)
	}
}

impl<C, B> RuntimeApiT for RuntimeClient<C, B>
where
	B: BlockT,
	C: HeaderBackend<B> + ProvideRuntimeApi<B> + BlockBackend<B> + Send + Sync + 'static,
	C::Api: TaggedTransactionQueue<B> + BlobApi<B>,
	<B as BlockT>::Hash: From<H256>,
	<B as BlockT>::Extrinsic: From<UncheckedExtrinsic>,
{
	fn get_blob_runtime_parameters(
		&self,
		block_hash: H256,
	) -> Result<da_control::BlobRuntimeParameters, ApiError> {
		self.0
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
		self.0
			.runtime_api()
			.validate_transaction(at.into(), source, uxt.into(), block_hash.into())
	}

	fn get_active_validators(&self, block_hash: H256) -> Result<Vec<AccountId>, ApiError> {
		self.0
			.runtime_api()
			.get_active_validators(block_hash.into())
	}

	fn get_validator_from_key(
		&self,
		at: H256,
		id: KeyTypeId,
		key_data: Vec<u8>,
	) -> Result<Option<AccountId>, ApiError> {
		self.0
			.runtime_api()
			.get_validator_from_key(at.into(), id, key_data)
	}
}

pub trait BackendApiT: Send + Sync {
	fn storage(&self, at: H256, key: &[u8]) -> Result<Option<Vec<u8>>, String>;
}

pub struct BackendClient<C, B>(Arc<C>, PhantomData<B>);

impl<C, B> BackendClient<C, B> {
	pub fn new(client: Arc<C>) -> Self {
		Self(client, PhantomData)
	}
}

impl<C, B> BackendApiT for BackendClient<C, B>
where
	B: BlockT,
	<B as BlockT>::Hash: From<H256>,
	C: sc_client_api::Backend<B> + Send + Sync + 'static,
	C::State: StateBackend<HashingFor<B>>,
{
	fn storage(&self, at: H256, key: &[u8]) -> Result<Option<Vec<u8>>, String> {
		let state = self.0.state_at(at.into()).map_err(|e| e.to_string())?;
		state.storage(key).map_err(|e| e.to_string())
	}
}

#[async_trait]
pub trait TransactionPoolApiT: Send + Sync {
	async fn submit_one(
		&self,
		block_hash: H256,
		source: TransactionSource,
		uxt: UncheckedExtrinsic,
	) -> Result<H256, String>;
}
pub struct TransactionPoolClient<C, B>(Arc<C>, PhantomData<B>);

impl<C, B> TransactionPoolClient<C, B> {
	pub fn new(client: Arc<C>) -> Self {
		Self(client, PhantomData)
	}
}

#[async_trait]
impl<C, B> TransactionPoolApiT for TransactionPoolClient<C, B>
where
	B: BlockT,
	C: TransactionPool<Block = B>,
	<B as BlockT>::Hash: From<H256>,
	H256: From<<C as sc_transaction_pool_api::TransactionPool>::Hash>,
	<B as BlockT>::Extrinsic: From<UncheckedExtrinsic>,
{
	async fn submit_one(
		&self,
		block_hash: H256,
		source: TransactionSource,
		uxt: UncheckedExtrinsic,
	) -> Result<H256, String> {
		let hash = self
			.0
			.submit_one(block_hash.into(), source, uxt.into())
			.await
			.map_err(|x| x.to_string())?;
		Ok(hash.into())
	}
}

#[async_trait]
pub trait ExternalitiesT: Send + Sync {
	fn client_info(&self) -> ClientInfo;

	fn local_peer_id(&self) -> Result<PeerId, ()>;

	fn role(&self) -> Role;

	fn keystore(&self) -> std::option::Option<&Arc<LocalKeystore>>;

	fn gossip_cmd_sender(&self) -> std::option::Option<&async_channel::Sender<BlobNotification>>;

	fn blob_store(&self) -> Arc<dyn StorageApiT>;

	fn blob_data_store(&self) -> Arc<dyn StorageApiT>;
}

#[derive(Debug, Default, Clone)]
pub struct ClientInfo {
	pub best_hash: H256,
	pub best_height: u32,
	pub finalized_hash: H256,
	pub finalized_height: u32,
}

pub struct RealExternalities<Client, Block>
where
	Block: BlockT,
{
	client: Arc<Client>,
	blob_handle: Arc<BlobHandle<Block>>,
	_block: PhantomData<Block>,
}

impl<Client, Block> RealExternalities<Client, Block>
where
	Block: BlockT,
	Client: HeaderBackend<Block>
		+ ProvideRuntimeApi<Block>
		+ BlockBackend<Block>
		+ Send
		+ Sync
		+ 'static,
	Client::Api: TaggedTransactionQueue<Block> + BlobApi<Block>,
	H256: From<<Block as BlockT>::Hash>,
	<Block as BlockT>::Hash: From<H256>,
	u32: From<<<Block as BlockT>::Header as HeaderT>::Number>,
	<Block as BlockT>::Extrinsic: From<UncheckedExtrinsic>,
{
	pub fn new(client: Arc<Client>, blob_handle: Arc<BlobHandle<Block>>) -> Self {
		Self {
			client,
			blob_handle,
			_block: PhantomData,
		}
	}
}

#[async_trait]
impl<Client, Block> ExternalitiesT for RealExternalities<Client, Block>
where
	Block: BlockT,
	Client: HeaderBackend<Block>
		+ ProvideRuntimeApi<Block>
		+ BlockBackend<Block>
		+ Send
		+ Sync
		+ 'static,
	Client::Api: TaggedTransactionQueue<Block> + BlobApi<Block>,
	H256: From<<Block as BlockT>::Hash>,
	<Block as BlockT>::Hash: From<H256>,
	u32: From<<<Block as BlockT>::Header as HeaderT>::Number>,
	<Block as BlockT>::Extrinsic: From<UncheckedExtrinsic>,
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

	fn local_peer_id(&self) -> Result<PeerId, ()> {
		let network = self.blob_handle.network.get().cloned();
		let Some(net) = network else {
			return Err(());
		};
		Ok(net.local_peer_id())
	}

	fn role(&self) -> Role {
		self.blob_handle.role.clone()
	}

	fn keystore(&self) -> std::option::Option<&Arc<LocalKeystore>> {
		self.blob_handle.keystore.get()
	}

	fn gossip_cmd_sender(&self) -> std::option::Option<&async_channel::Sender<BlobNotification>> {
		self.blob_handle.gossip_cmd_sender.get()
	}

	fn blob_store(&self) -> Arc<dyn StorageApiT> {
		self.blob_handle.blob_store.clone()
	}

	fn blob_data_store(&self) -> Arc<dyn StorageApiT> {
		self.blob_handle.blob_data_store.clone()
	}
}

#[allow(dead_code)]
pub struct DummyExternalities;

#[async_trait]
impl ExternalitiesT for DummyExternalities {
	fn client_info(&self) -> ClientInfo {
		todo!()
	}

	fn local_peer_id(&self) -> Result<PeerId, ()> {
		todo!()
	}

	fn role(&self) -> Role {
		todo!()
	}

	fn keystore(&self) -> std::option::Option<&Arc<LocalKeystore>> {
		todo!()
	}

	fn gossip_cmd_sender(&self) -> std::option::Option<&async_channel::Sender<BlobNotification>> {
		todo!()
	}

	fn blob_store(&self) -> Arc<dyn StorageApiT> {
		todo!()
	}

	fn blob_data_store(&self) -> Arc<dyn StorageApiT> {
		todo!()
	}
}

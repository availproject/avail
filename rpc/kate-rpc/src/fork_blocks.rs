use jsonrpsee::{
	core::{async_trait, RpcResult},
	proc_macros::rpc,
	types::error::ErrorObject,
};
use sc_client_api::{Backend, BlockBackend};
use sp_blockchain::{Backend as BlockChainBackend, HeaderBackend};
use sp_runtime::traits::{Block as BlockT, Header};
use std::{marker::PhantomData, sync::Arc};

pub type HashOf<Block> = <Block as BlockT>::Hash;

#[rpc(client, server)]
pub trait Forks<Block>
where
	Block: BlockT,
{
	#[method(name = "getBlocksAtHeight")]
	async fn get_blocks_at_height(&self, height: u32) -> RpcResult<Vec<HashOf<Block>>>;

	#[method(name = "getChildrenOf")]
	async fn get_children_of(&self, hash: HashOf<Block>) -> RpcResult<Vec<HashOf<Block>>>;
}

pub struct ForkBlocks<BE: Backend<Block>, Client, Block: BlockT> {
	client: Arc<Client>,
	backend: Arc<BE>,
	_block: PhantomData<Block>,
}

impl<BE: Backend<Block>, Client, Block: BlockT> ForkBlocks<BE, Client, Block> {
	pub fn new(client: Arc<Client>, backend: Arc<BE>) -> Self {
		Self {
			client,
			backend,
			_block: PhantomData,
		}
	}
}

/// Error type for this RPC API.
pub enum Error {
	/// Generic fork-rpc error.
	ForkRpcError,
}

impl From<Error> for i32 {
	fn from(e: Error) -> i32 {
		match e {
			Error::ForkRpcError => 1,
		}
	}
}

macro_rules! internal_err {
	($($arg:tt)*) => {{
		ErrorObject::owned(
			Error::ForkRpcError.into(),
			format!($($arg)*),
			None::<()>
		)
	}}
}

#[async_trait]
impl<BE: Backend<Block>, Client, Block> ForksServer<Block> for ForkBlocks<BE, Client, Block>
where
	Block: BlockT,
	BE: Backend<Block> + 'static,
	Client: Send + Sync + 'static,
	Client: BlockBackend<Block> + HeaderBackend<Block>,
{
	async fn get_blocks_at_height(&self, height: u32) -> RpcResult<Vec<HashOf<Block>>> {
		// Fetch the block hash for the given block height
		let block_hash = self
			.client
			.block_hash(height.into())
			.map_err(|e| internal_err!("Error fetching block hash for height {height}: {e:?}"))?
			.ok_or_else(|| internal_err!("No block hash found for height {height}"))?;

		// Fetch the block using the retrieved block hash
		let block = self
			.client
			.block(block_hash)
			.map_err(|e| internal_err!("Error fetching block data for hash {block_hash}: {e:?}"))?
			.ok_or_else(|| internal_err!("Block not found for hash {block_hash}"))?;

		// Get children hashes of the parent block
		let parent_hash = *block.block.header().parent_hash();
		let child_hashes = self
			.backend
			.blockchain()
			.children(parent_hash)
			.map_err(|e| {
				internal_err!("Error fetching child hashes for parent {parent_hash}: {e}")
			})?;

		Ok(child_hashes)
	}

	async fn get_children_of(&self, hash: HashOf<Block>) -> RpcResult<Vec<HashOf<Block>>> {
		// Get children hashes of the given block
		let child_hashes = self
			.backend
			.blockchain()
			.children(hash)
			.map_err(|e| internal_err!("Error fetching child hashes for parent {hash}: {e}"))?;

		Ok(child_hashes)
	}
}

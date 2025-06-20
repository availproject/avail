use jsonrpsee::{
	core::{async_trait, RpcResult},
	proc_macros::rpc,
	types::error::ErrorObject,
};
use sc_client_api::BlockBackend;
use sp_runtime::traits::Block as BlockT;
use std::{marker::PhantomData, sync::Arc};

/// GRANDPA consensus engine_id
pub const GRANDPA_ENGINE_ID: [u8; 4] = *b"FRNK";

#[rpc(client, server)]
pub trait Grandpa<Block>
where
	Block: BlockT,
{
	#[method(name = "grandpa_blockJustification")]
	async fn block_justification(&self, block_number: u32) -> RpcResult<Option<Vec<u8>>>;
}

pub struct GrandpaJustifications<Client, Block: BlockT> {
	client: Arc<Client>,
	_block: PhantomData<Block>,
}

impl<Client, Block: BlockT> GrandpaJustifications<Client, Block> {
	pub fn new(client: Arc<Client>) -> Self {
		Self {
			client,
			_block: PhantomData,
		}
	}
}

/// Error type for this RPC API.
pub enum Error {
	/// Generic justification error.
	JustificationError,
}

impl From<Error> for i32 {
	fn from(e: Error) -> i32 {
		match e {
			Error::JustificationError => 1,
		}
	}
}

macro_rules! internal_err {
	($($arg:tt)*) => {{
		ErrorObject::owned(
			Error::JustificationError.into(),
			format!($($arg)*),
			None::<()>
		)
	}}
}

#[async_trait]
impl<Client, Block> GrandpaServer<Block> for GrandpaJustifications<Client, Block>
where
	Block: BlockT,
	Client: Send + Sync + 'static,
	Client: BlockBackend<Block>,
{
	/// Returns the GRANDPA justification for the given block number, if available.
	///
	/// # Parameters
	/// - `block_number`: The number of the block for which the justification is requested.
	///
	/// # Returns
	/// - `Ok(Some(Vec<u8>))`: The encoded justification bytes for the block.
	/// - `Ok(None)`: Indicates that no justification is available for the specified block.
	/// - `Err`: If there is an error retrieving the block hash or justifications from the backend.
	///
	/// # Notes
	/// This method checks whether a justification exists in the backend for the block.
	/// If the justification exists for the `GRANDPA_ENGINE_ID`, it is returned.
	/// Otherwise, `None` is returned, indicating no justification is present.
	async fn block_justification(&self, block_number: u32) -> RpcResult<Option<Vec<u8>>> {
		// Fetch the block hash
		let block_hash = self
			.client
			.block_hash(block_number.into())
			.map_err(|e| internal_err!("Failed to fetch block hash: {e:?}"))?
			.ok_or_else(|| internal_err!("Block hash not found for block #{block_number}"))?;

		// Fetch the justification for the block hash
		let justification = self
			.client
			.justifications(block_hash)
			.map_err(|e| internal_err!("Failed to fetch justifications: {e:?}"))?
			.and_then(|just| just.into_justification(GRANDPA_ENGINE_ID));

		Ok(justification)
	}
}

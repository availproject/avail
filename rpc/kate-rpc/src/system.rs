use frame_system_rpc_runtime_api::{system_events_api::fetch_events_v1, SystemEventsApi};
use jsonrpsee::{
	core::{async_trait, RpcResult},
	proc_macros::rpc,
	types::error::ErrorObject,
};
use sp_api::ProvideRuntimeApi;
use sp_core::H256;
use sp_runtime::traits::Block as BlockT;
use std::{marker::PhantomData, sync::Arc};

#[rpc(client, server)]
pub trait Api {
	#[method(name = "system_fetchEventsV1")]
	async fn fetch_events_v1(
		&self,
		params: fetch_events_v1::Params,
		at: H256,
	) -> RpcResult<fetch_events_v1::ApiResult>;
}

pub struct Rpc<C, Block>
where
	C: ProvideRuntimeApi<Block> + Send + Sync + 'static,
	C::Api: frame_system_rpc_runtime_api::SystemEventsApi<Block>,
	Block: BlockT,
	<Block as BlockT>::Hash: From<H256>,
{
	pub client: Arc<C>,
	_phantom: PhantomData<Block>,
}
impl<C, Block> Rpc<C, Block>
where
	C: ProvideRuntimeApi<Block> + Send + Sync + 'static,
	C::Api: frame_system_rpc_runtime_api::SystemEventsApi<Block>,
	Block: BlockT,
	<Block as BlockT>::Hash: From<H256>,
{
	pub fn new(client: Arc<C>) -> Self {
		Self {
			client,
			_phantom: PhantomData,
		}
	}
}

/// Error type for this RPC API.
pub enum Error {
	/// Generic runtime error.
	RuntimeApiError,
}

impl Error {
	pub fn runtime_api_error<'a>(msg: String) -> ErrorObject<'a> {
		ErrorObject::owned(Error::RuntimeApiError.into(), msg, None::<()>)
	}
}

impl From<Error> for i32 {
	fn from(e: Error) -> i32 {
		match e {
			Error::RuntimeApiError => 1,
		}
	}
}

#[async_trait]
impl<C, Block> ApiServer for Rpc<C, Block>
where
	C: ProvideRuntimeApi<Block> + Send + Sync + 'static,
	C::Api: frame_system_rpc_runtime_api::SystemEventsApi<Block>,
	Block: BlockT,
	<Block as BlockT>::Hash: From<H256>,
{
	async fn fetch_events_v1(
		&self,
		params: fetch_events_v1::Params,
		at: H256,
	) -> RpcResult<fetch_events_v1::ApiResult> {
		let runtime_api = self.client.runtime_api();
		let result = runtime_api
			.fetch_events_v1(at.into(), params)
			.map_err(|x| Error::runtime_api_error(x.to_string()))?;

		Ok(result)
	}
}

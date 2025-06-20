use codec::{decode_from_bytes, Encode};
use frame_system_rpc_runtime_api::system_events_api::fetch_events_v1;
use jsonrpsee::{
	core::{async_trait, RpcResult},
	proc_macros::rpc,
	types::error::ErrorObject,
};
use sc_service::RpcHandlers;
use sp_core::{bytes::from_hex, H256};

#[rpc(client, server)]
pub trait Api {
	#[method(name = "system_fetchEventsV1")]
	async fn fetch_events_v1(
		&self,
		params: fetch_events_v1::Params,
		at: H256,
	) -> RpcResult<fetch_events_v1::ApiResult>;
}

pub struct Rpc {
	pub handlers: RpcHandlers,
}
impl Rpc {
	pub fn new(handlers: RpcHandlers) -> Self {
		Self { handlers }
	}
}

fn internal_error<'a>(msg: String) -> ErrorObject<'a> {
	ErrorObject::owned(0, msg, None::<()>)
}

#[async_trait]
impl ApiServer for Rpc {
	async fn fetch_events_v1(
		&self,
		params: fetch_events_v1::Params,
		at: H256,
	) -> RpcResult<fetch_events_v1::ApiResult> {
		runtime_api_fetch_events_v1(&self.handlers, params, &at)
			.await
			.map_err(|x| internal_error(x))
	}
}

pub async fn runtime_api_fetch_events_v1(
	handlers: &RpcHandlers,
	params: fetch_events_v1::Params,
	at: &H256,
) -> Result<fetch_events_v1::ApiResult, String> {
	let query = format!(
		r#"{{
		"jsonrpc": "2.0",
		"method": "state_call",
		"params": ["SystemEventsApi_fetchEventsV1", "0x{}", "{:?}"],
		"id": 0
	}}"#,
		hex::encode(params.encode()),
		at
	);

	let (res, _) = handlers
		.rpc_query(&query)
		.await
		.map_err(|x| x.to_string())?;
	let json = serde_json::from_str::<serde_json::Value>(&res).map_err(|x| x.to_string())?;

	let Some(result_json) = json["result"].as_str() else {
		return Err(String::from("Failed to call event api"));
	};
	let result = from_hex(result_json).map_err(|x| x.to_string())?;
	let res: fetch_events_v1::ApiResult =
		decode_from_bytes::<fetch_events_v1::ApiResult>(result.into())
			.map_err(|x| x.to_string())?;

	Ok(res)
}

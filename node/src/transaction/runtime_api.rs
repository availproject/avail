use codec::{decode_from_bytes, Encode};
use frame_system_rpc_runtime_api::{SystemFetchEventsParams, SystemFetchEventsResult};
use sc_service::RpcHandlers;
use sp_core::{bytes::from_hex, H256};

pub(crate) async fn system_fetch_events(
	handlers: &RpcHandlers,
	params: SystemFetchEventsParams,
	block_hash: &H256,
) -> Option<SystemFetchEventsResult> {
	let query = format!(
		r#"{{
		"jsonrpc": "2.0",
		"method": "state_call",
		"params": ["SystemEventsApi_fetch_events", "0x{}", "{}"],
		"id": 0
	}}"#,
		hex::encode(params.encode()),
		std::format!("{:?}", block_hash)
	);

	let (res, _) = handlers.rpc_query(&query).await.ok()?;
	let json = serde_json::from_str::<serde_json::Value>(&res).ok()?;

	let result_json = json["result"].as_str()?;
	let result = from_hex(result_json).ok()?;
	let res: SystemFetchEventsResult =
		decode_from_bytes::<SystemFetchEventsResult>(result.into()).ok()?;

	Some(res)
}

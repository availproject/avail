use codec::{decode_from_bytes, Encode};
use frame_system_rpc_runtime_api::{SystemFetchEventsResult, TransactionSuccessStatus};
use sc_service::RpcHandlers;
use sp_core::{bytes::from_hex, H256};

pub(crate) async fn system_fetch_events(
	handlers: &RpcHandlers,
	tx_index: u32,
	enable_decoding: bool,
	block_hash: &H256,
) -> Option<SystemFetchEventsResult> {
	let query = format!(
		r#"{{
		"jsonrpc": "2.0",
		"method": "state_call",
		"params": ["SystemEventsApi_fetch_events", "0x{}{}", "{}"],
		"id": 0
	}}"#,
		hex::encode(vec![tx_index].encode()),
		if enable_decoding { "01" } else { "00" },
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

pub(crate) async fn system_fetch_transaction_success_status(
	handlers: &RpcHandlers,
	block_hash: &H256,
) -> Option<Vec<TransactionSuccessStatus>> {
	let query = format!(
		r#"{{
		"jsonrpc": "2.0",
		"method": "state_call",
		"params": ["SystemEventsApi_fetch_transaction_success_status", "0x", "{}"],
		"id": 0
	}}"#,
		std::format!("{:?}", block_hash)
	);

	let (res, _) = handlers.rpc_query(&query).await.ok()?;
	let json = serde_json::from_str::<serde_json::Value>(&res).ok()?;

	let result_json = json["result"].as_str()?;
	let result = from_hex(result_json).ok()?;
	let res = decode_from_bytes::<Vec<TransactionSuccessStatus>>(result.into()).ok()?;

	Some(res)
}

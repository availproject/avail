use std::time::Duration;

use avail_core::OpaqueExtrinsic;
use codec::{decode_from_bytes, Encode};
use da_runtime::UncheckedExtrinsic;
use frame_system_rpc_runtime_api::TransactionSuccessStatus;
use jsonrpsee::tokio;
use sc_service::RpcHandlers;
use sp_core::{bytes::from_hex, Blake2Hasher, Hasher, H256};

use super::{BlockDetails, TransactionState};

pub(crate) async fn wait_for_sync(handler: &RpcHandlers) {
	loop {
		match fetch_sync_status(handler).await {
			Some(true) => (),
			Some(false) => return,
			None => (),
		}

		tokio::time::sleep(Duration::from_secs(10)).await;
	}
}

pub(crate) async fn fetch_sync_status(handler: &RpcHandlers) -> Option<bool> {
	let query = r#"{
					"jsonrpc": "2.0",
					"method": "system_health",
					"params": [],
					"id": 0
				}"#;

	let res = handler.rpc_query(&query).await.ok()?;
	let json = serde_json::from_str::<serde_json::Value>(&res.0).ok()?;
	let result_json = json["result"].as_object()?;

	result_json["isSyncing"].as_bool()
}

pub(crate) async fn fetch_extrinsic_success_status(
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

pub(crate) async fn prepare_block(
	extrinsics: Vec<OpaqueExtrinsic>,
	block_hash: H256,
	block_height: u32,
	execution_status: Vec<TransactionSuccessStatus>,
	finalized: bool,
) -> BlockDetails {
	let mut txs: Vec<TransactionState> = Vec::with_capacity(extrinsics.len());
	for (i, ext) in extrinsics.iter().enumerate() {
		let unchecked_ext = match UncheckedExtrinsic::decode_no_vec_prefix(&mut ext.0.as_slice()) {
			Ok(x) => x,
			Err(err) => {
				println!("Failed to convert OpaqExt to Unchecked, {}", err);
				continue;
			},
		};

		let Some((pallet_index, call_index)) = read_pallet_call_index(&unchecked_ext) else {
			continue;
		};

		let tx_hash = Blake2Hasher::hash(&unchecked_ext.encode());

		let status = execution_status.iter().find(|x| x.tx_index == i as u32);
		let Some(status) = status else { continue };
		let info = TransactionState {
			tx_hash,
			tx_index: status.tx_index,
			tx_success: status.tx_success,
			pallet_index,
			call_index,
		};
		txs.push(info);
	}

	let block = BlockDetails {
		block_hash,
		block_height,
		finalized,
		transactions: txs,
	};

	block
}

pub(crate) fn read_pallet_call_index(ext: &UncheckedExtrinsic) -> Option<(u8, u8)> {
	let ext = ext.function.encode();
	if ext.len() < 2 {
		return None;
	}
	let pallet_index = ext[0];
	let call_index = ext[1];

	Some((pallet_index, call_index))
}

use std::time::Duration;

use super::{constants::NODE_SYNC_SLEEP_INTERNVAL, BlockDetails, TransactionState};
use crate::transaction_rpc_worker::read_pallet_call_index;
use avail_core::OpaqueExtrinsic;
use codec::Encode;
use da_runtime::UncheckedExtrinsic;
use frame_system_rpc_runtime_api::SystemFetchEventsResult;
use jsonrpsee::tokio;
use sc_service::RpcHandlers;
use sp_core::{Blake2Hasher, Hasher, H256};

pub(crate) async fn wait_for_sync(handler: &RpcHandlers) {
	loop {
		match fetch_sync_status(handler).await {
			Some(true) => (),
			Some(false) => return,
			None => (),
		}

		tokio::time::sleep(Duration::from_secs(NODE_SYNC_SLEEP_INTERNVAL)).await;
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

pub(crate) async fn prepare_block(
	extrinsics: Vec<OpaqueExtrinsic>,
	block_hash: H256,
	block_height: u32,
	events: SystemFetchEventsResult,
	finalized: bool,
) -> BlockDetails {
	let mut txs: Vec<TransactionState> = Vec::with_capacity(extrinsics.len());
	for (tx_index, ext) in extrinsics.iter().enumerate() {
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

		let tx_success = events.is_transaction_successful(tx_index as u32);
		let Some(tx_success) = tx_success else {
			continue;
		};
		let info = TransactionState {
			tx_hash,
			tx_index: tx_index as u32,
			tx_success,
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

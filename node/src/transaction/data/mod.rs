pub mod constants;

use crate::service::FullClient;
use codec::{decode_from_bytes, Encode};
use da_runtime::UncheckedExtrinsic;
use jsonrpsee::tokio;
use sc_service::RpcHandlers;
use sc_telemetry::log;
use sp_core::{bytes::from_hex, H256};
use sp_core::{Blake2Hasher, Hasher};
use sp_runtime::generic::BlockId;
use sp_runtime::traits::BlockIdTo;
use sp_runtime::MultiAddress;
use std::sync::Arc;
use std::time::Duration;

use transaction_rpc::{
	HashIndex, TransactionData, TransactionDataRPCParams, TransactionDataSigned, TransactionState,
	TxDataReceiver,
};

#[derive(Clone, Default)]
pub struct CliDeps {
	pub enabled: bool,
}

pub struct Worker {
	pub client: Arc<FullClient>,
	pub rpc_handlers: RpcHandlers,
	pub receiver: TxDataReceiver,
}

pub struct Deps {
	pub receiver: TxDataReceiver,
}

impl Worker {
	pub async fn run(mut self) {
		log::info!("🐖 Transaction Data Running");

		loop {
			if !self.receiver.is_empty() {
				while let Ok((params, oneshot)) = self.receiver.try_recv() {
					log::info!("🐖 Found something :)");
					let result = self.task(params).await;
					_ = oneshot.send(result);
					log::info!("🐖 Send Something :)");
				}
			}
			tokio::time::sleep(Duration::from_millis(constants::DATABASE_POOL_INTERVAL)).await;
		}
	}

	async fn task(&self, params: TransactionDataRPCParams) -> Result<TransactionData, String> {
		let block_hash = match params.block_id {
			HashIndex::Hash(v) => v,
			HashIndex::Index(block_height) => {
				let block_hash = self.client.to_hash(&BlockId::Number(block_height));
				let Some(block_hash) = block_hash.ok().flatten() else {
					return Err(std::format!(
						"No block hash found for block height: {}",
						block_height
					));
				};
				block_hash
			},
		};

		let Some((unchecked_ext, tx_hash, tx_index)) = self.get_extrinsic(block_hash, params.tx_id)
		else {
			return Err(String::from(
				"Failed to get block body or failed to find transaction",
			));
		};

		let Some((pallet_id, call_id)) = read_pallet_call_index(&unchecked_ext) else {
			return Err(String::from("Failed to read pallet and call id"));
		};

		let mut result = TransactionData::default();
		result.block_hash = block_hash;
		result.tx_index = tx_index;
		result.pallet_id = pallet_id;
		result.call_id = call_id;

		if let Some(sig) = &unchecked_ext.signature {
			let mut signed = TransactionDataSigned::default();
			if let MultiAddress::Id(id) = &sig.0 {
				signed.ss58_address = Some(std::format!("{}", id))
			};

			signed.nonce = sig.2 .5 .0;
			signed.app_id = sig.2 .8 .0 .0;
			match sig.2 .4 .0 {
				sp_runtime::generic::Era::Immortal => signed.mortality = None,
				sp_runtime::generic::Era::Mortal(x, y) => signed.mortality = Some((x, y)),
			};

			result.signed = Some(signed);
		}

		if params.fetch_call.unwrap_or(false) {
			result.call = Some(unchecked_ext.function.encode())
		}

		if params.fetch_events.unwrap_or(false) {
			result.events = fetch_events(&self.rpc_handlers, &block_hash, tx_index).await;
		}

		if params.fetch_state.unwrap_or(false) {
			result.states = fetch_state(&self.rpc_handlers, tx_hash).await
		}

		Ok(result)
	}

	fn get_extrinsic(
		&self,
		block_hash: H256,
		tx_id: HashIndex,
	) -> Option<(UncheckedExtrinsic, H256, u32)> {
		let block_body = self.client.body(block_hash).ok()??;

		match tx_id {
			HashIndex::Hash(target_hash) => {
				for (i, ext) in block_body.iter().enumerate() {
					let Ok(unchecked_ext) =
						UncheckedExtrinsic::decode_no_vec_prefix(&mut ext.0.as_slice())
					else {
						continue;
					};

					let tx_hash = Blake2Hasher::hash(&unchecked_ext.encode());
					if tx_hash == target_hash {
						return Some((unchecked_ext, target_hash, i as u32));
					}
				}
				None
			},
			HashIndex::Index(index) => {
				let tx = block_body.get(index as usize)?;
				let unchecked_ext =
					UncheckedExtrinsic::decode_no_vec_prefix(&mut tx.0.as_slice()).ok()?;
				let tx_hash = Blake2Hasher::hash(&unchecked_ext.encode());
				Some((unchecked_ext, tx_hash, index))
			},
		}
	}
}

fn read_pallet_call_index(ext: &UncheckedExtrinsic) -> Option<(u8, u8)> {
	let ext = ext.function.encode();
	if ext.len() < 2 {
		return None;
	}
	let pallet_index = ext[0];
	let call_index = ext[1];

	Some((pallet_index, call_index))
}

async fn fetch_events(
	handlers: &RpcHandlers,
	block_hash: &H256,
	tx_index: u32,
) -> Option<Vec<Vec<u8>>> {
	let query = format!(
		r#"{{
		"jsonrpc": "2.0",
		"method": "state_call",
		"params": ["SystemEventsApi_fetch_events", "0x{}", "{}"],
		"id": 0
	}}"#,
		hex::encode(Some(tx_index).encode()),
		std::format!("{:?}", block_hash)
	);

	let (res, _) = handlers.rpc_query(&query).await.ok()?;
	let json = serde_json::from_str::<serde_json::Value>(&res).ok()?;

	let result_json = json["result"].as_str()?;
	let result = from_hex(result_json).ok()?;
	let res = decode_from_bytes::<Vec<Vec<u8>>>(result.into()).ok()?;

	Some(res)
}

async fn fetch_state(handlers: &RpcHandlers, tx_hash: H256) -> Option<Vec<TransactionState>> {
	let query = format!(
		r#"{{
		"jsonrpc": "2.0",
		"method": "transaction_state",
		"params": ["{}"],
		"id": 0
	}}"#,
		std::format!("{:?}", tx_hash),
	);

	let (res, _) = handlers.rpc_query(&query).await.ok()?;
	let mut json = serde_json::from_str::<serde_json::Value>(&res).ok()?;
	let res = serde_json::from_value(json["result"].take()).ok()?;

	Some(res)
}

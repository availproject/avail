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
	HashIndex, TransactionData, TransactionDataRPCParams, TransactionDataSigned, TransactionDatas,
	TransactionState, TxDataReceiver,
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

	async fn task(&self, params: TransactionDataRPCParams) -> Result<TransactionDatas, String> {
		let (block_hash, block_height) = match params.block_id {
			HashIndex::Hash(block_hash) => {
				let block_height = self.client.to_number(&BlockId::Hash(block_hash.clone()));
				let Some(block_height) = block_height.ok().flatten() else {
					return Err(std::format!(
						"No block height found for block hash: {:?}",
						block_hash
					));
				};
				(block_hash, block_height)
			},
			HashIndex::Index(block_height) => {
				let block_hash = self.client.to_hash(&BlockId::Number(block_height));
				let Some(block_hash) = block_hash.ok().flatten() else {
					return Err(std::format!(
						"No block hash found for block height: {}",
						block_height
					));
				};
				(block_hash, block_height)
			},
		};

		let mut transactions = self.extrinsics(block_hash, &params)?;
		for ext in transactions.iter_mut() {
			if params.fetch_events.unwrap_or(false) {
				ext.events = fetch_events(&self.rpc_handlers, &block_hash, ext.tx_index).await;
			}

			if params.fetch_state.unwrap_or(false) {
				ext.states = fetch_state(&self.rpc_handlers, ext.tx_hash).await
			}
		}

		let result = TransactionDatas {
			block_hash,
			block_height,
			transactions,
		};

		Ok(result)
	}

	fn extrinsics(
		&self,
		block_hash: H256,
		params: &TransactionDataRPCParams,
	) -> Result<Vec<TransactionData>, String> {
		let filter = params.filter.clone().unwrap_or_default();

		let Some(block_body) = self.client.body(block_hash).ok().flatten() else {
			return Err(std::format!(
				"Failed to fetch block with block hash: {:?}",
				block_hash
			));
		};

		let mut extrinsics = Vec::new();
		for (i, ext) in block_body.iter().enumerate() {
			if let Some(HashIndex::Index(target_index)) = &filter.tx_id {
				if *target_index != i as u32 {
					continue;
				}
			};

			let unchecked_ext = UncheckedExtrinsic::decode_no_vec_prefix(&mut ext.0.as_slice());
			let Ok(unchecked_ext) = unchecked_ext else {
				return Err(std::format!(
					"Failed to fetch transaction. tx index: {}, block hash: {:?}",
					i,
					block_hash
				));
			};

			let Some((pallet_id, call_id)) = read_pallet_call_index(&unchecked_ext) else {
				return Err(std::format!(
					"Failed to read pallet and call id. Tx index: {}, block hash: {:?}",
					i,
					block_hash
				));
			};

			if filter.pallet_id.is_some_and(|x| x != pallet_id) {
				continue;
			};

			if filter.call_id.is_some_and(|x| x != call_id) {
				continue;
			};

			let requires_signed =
				filter.app_id.is_some() || filter.nonce.is_some() || filter.ss58_address.is_some();

			if unchecked_ext.signature.is_none() && requires_signed {
				continue;
			}

			let mut tx = TransactionData::default();
			tx.tx_index = i as u32;
			tx.pallet_id = pallet_id;
			tx.call_id = call_id;

			let mut signed = TransactionDataSigned::default();
			if let Some(sig) = &unchecked_ext.signature {
				if let MultiAddress::Id(id) = &sig.0 {
					signed.ss58_address = Some(std::format!("{}", id))
				};

				signed.nonce = sig.2 .5 .0;
				signed.app_id = sig.2 .8 .0 .0;
				match sig.2 .4 .0 {
					sp_runtime::generic::Era::Immortal => signed.mortality = None,
					sp_runtime::generic::Era::Mortal(x, y) => signed.mortality = Some((x, y)),
				};

				if filter.app_id.is_some_and(|x| x != signed.app_id) {
					continue;
				}

				if filter.nonce.is_some_and(|x| x != signed.nonce) {
					continue;
				}

				if filter.ss58_address.is_some() && filter.ss58_address != signed.ss58_address {
					continue;
				}

				tx.signed = Some(signed);
			}

			tx.tx_hash = Blake2Hasher::hash(&unchecked_ext.encode());
			if let Some(HashIndex::Hash(target_hash)) = &filter.tx_id {
				if tx.tx_hash != *target_hash {
					continue;
				}
			};

			if params.fetch_call.unwrap_or(false) {
				let encoded = hex::encode(unchecked_ext.function.encode());
				tx.call = Some(std::format!("0x{}", encoded))
			}

			extrinsics.push(tx);
		}

		Ok(extrinsics)
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
) -> Option<Vec<String>> {
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
	let res: Vec<Vec<u8>> = decode_from_bytes::<Vec<Vec<u8>>>(result.into()).ok()?;
	let res: Vec<String> = res
		.into_iter()
		.map(|x| std::format!("0x{}", hex::encode(x.encode())))
		.collect();

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
	let mut res: Vec<TransactionState> = serde_json::from_value(json["result"].take()).ok()?;

	while res.len() > 3 {
		res.pop();
	}

	Some(res)
}

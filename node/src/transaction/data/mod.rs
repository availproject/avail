pub mod constants;

use crate::service::FullClient;
use codec::{decode_from_bytes, Encode};
use da_runtime::UncheckedExtrinsic;
use frame_system_rpc_runtime_api::SystemFetchEventsResult;
use jsonrpsee::tokio;
use sc_service::RpcHandlers;
use sc_telemetry::log;
use sp_core::{bytes::from_hex, H256};
use sp_core::{Blake2Hasher, Hasher};
use sp_runtime::generic::BlockId;
use sp_runtime::traits::BlockIdTo;
use sp_runtime::{AccountId32, MultiAddress};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use transaction_rpc::{
	DataSubmittedEvent, DecodedEvents, HashIndex, TransactionData, TransactionDataRPCParams,
	TransactionDataSigned, TransactionDatas, TransactionState, TxDataReceiver,
};

#[derive(Clone, Default)]
pub struct CliDeps {
	pub enabled: bool,
}

pub struct Deps {
	pub receiver: TxDataReceiver,
}

type CachedEventValue = (
	Option<transaction_rpc::EncodedEvents>,
	Option<transaction_rpc::DecodedEvents>,
);
type CachedEventKey = (H256, u32, bool);

#[derive(Default)]
pub struct EventCache {
	map: HashMap<CachedEventKey, CachedEventValue>,
}
impl EventCache {
	pub fn get(&self, key: &CachedEventKey) -> Option<&CachedEventValue> {
		self.map.get(key)
	}

	pub fn insert(&mut self, key: CachedEventKey, value: CachedEventValue) {
		self.map.insert(key, value);
	}

	pub fn resize(&mut self) {
		if self.map.len() <= 1_000 {
			return;
		}

		self.map.clear();
	}
}

pub struct Worker {
	pub client: Arc<FullClient>,
	pub rpc_handlers: RpcHandlers,
	pub receiver: TxDataReceiver,
	pub event_cache: EventCache,
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

	async fn task(&mut self, params: TransactionDataRPCParams) -> Result<TransactionDatas, String> {
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
				let enable_decoding = params.decode_events.unwrap_or(false);
				self.fetch_events(&block_hash, enable_decoding, ext).await;
				self.event_cache.resize();
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

	async fn fetch_events(
		&mut self,
		block_hash: &H256,
		enable_decoding: bool,
		ext: &mut TransactionData,
	) {
		use codec::Decode;
		let cache = self
			.event_cache
			.get(&(*block_hash, ext.tx_index, enable_decoding));
		if let Some(cache) = cache {
			ext.encoded_events = cache.0.clone();
			ext.decoded_events = cache.1.clone();
			return;
		}

		let rpc_events = fetch_rpc_events(
			&self.rpc_handlers,
			&block_hash,
			ext.tx_index,
			enable_decoding,
		)
		.await;
		let Some(rpc_events) = rpc_events else { return };
		if rpc_events.error != 0 {
			return;
		}

		let encoded_events = rpc_events
			.encoded
			.into_iter()
			.find(|x| x.tx_index == ext.tx_index);
		let decoded_events = rpc_events
			.decoded
			.into_iter()
			.find(|x| x.tx_index == ext.tx_index);

		let encoded_events: Option<Vec<String>> = encoded_events.map(|x| {
			{
				x.value
					.iter()
					.map(|x| std::format!("0x{}", hex::encode(x.encode())))
			}
			.collect()
		});

		let decoded_events: Option<DecodedEvents> = decoded_events.map(|x| {
			let mut data_submitted = Vec::new();
			for ds in x.value.data_availability_data_submitted {
				let Ok(who) = AccountId32::decode(&mut ds.who.as_slice()) else {
					continue;
				};
				let Ok(data_hash) = H256::decode(&mut ds.data_hash.as_slice()) else {
					continue;
				};
				data_submitted.push(DataSubmittedEvent {
					who: std::format!("{}", who),
					data_hash: std::format!("{:?}", data_hash),
				});
			}

			DecodedEvents {
				system_extrinsic: x.value.system_extrinsic,
				sudo_sudid: x.value.sudo_sudid,
				sudo_sudo_as_done: x.value.sudo_sudo_as_done,
				multisig_executed: x.value.multisig_executed,
				proxy_executed: x.value.proxy_executed,
				data_availability_data_submitted: data_submitted,
			}
		});

		if encoded_events.is_none() && decoded_events.is_none() {
			return;
		}

		ext.encoded_events = encoded_events.clone();
		ext.decoded_events = decoded_events.clone();
		self.event_cache.insert(
			(*block_hash, ext.tx_index, enable_decoding),
			(encoded_events, decoded_events),
		);
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

async fn fetch_rpc_events(
	handlers: &RpcHandlers,
	block_hash: &H256,
	tx_index: u32,
	enable_decoding: bool,
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

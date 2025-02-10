use std::collections::VecDeque;
use std::sync::Arc;
use std::time::{Duration, Instant};

use avail_core::OpaqueExtrinsic;
use codec::{decode_from_bytes, Encode};
use da_runtime::UncheckedExtrinsic;
use frame_system_rpc_runtime_api::TransactionSuccessStatus;
use jsonrpsee::tokio;
use jsonrpsee::tokio::sync::mpsc::{Receiver, Sender};
use sc_service::RpcHandlers;
use sc_telemetry::log;
use serde::{Deserialize, Serialize};
use sp_core::bytes::to_hex;
use sp_core::{bytes::from_hex, Blake2Hasher, Hasher, H256};
use sp_runtime::generic::BlockId;
use sp_runtime::traits::BlockIdTo;
use system_rpc::TxStateReceiver as SearchReceiver;
use system_rpc::{OneShotTxStateSender, TransactionState as RPCTransactionState};

use crate::service::FullClient;

#[derive(Clone, Default)]
pub struct CliDeps {
	pub max_search_results: usize,
	pub max_stored_block_count: usize,
	pub block_channel_capacity: usize,
	pub search_channel_capacity: usize,
	pub enabled: bool,
}

pub struct Deps {
	pub block_receiver: Receiver<BlockDetails>,
	pub block_sender: Sender<BlockDetails>,
	pub search_receiver: SearchReceiver,
	pub cli: CliDeps,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockDetails {
	pub block_hash: H256,
	pub block_height: u32,
	pub finalized: bool,
	pub transactions: Vec<TransactionState>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionState {
	pub tx_hash: H256,
	pub tx_index: u32,
	pub tx_success: bool,
	pub pallet_index: u8,
	pub call_index: u8,
}

pub struct Database {
	included_blocks: VecDeque<BlockDetails>,
	finalized_blocks: VecDeque<BlockDetails>,
	block_receiver: Receiver<BlockDetails>,
	search_receiver: SearchReceiver,
	max_search_results: usize,
	max_stored_block_count: usize,
}

impl Database {
	pub fn new(
		block_receiver: Receiver<BlockDetails>,
		search_receiver: SearchReceiver,
		max_search_results: usize,
		max_stored_block_count: usize,
	) -> Self {
		Self {
			included_blocks: VecDeque::new(),
			finalized_blocks: VecDeque::new(),
			block_receiver,
			search_receiver,
			max_search_results,
			max_stored_block_count,
		}
	}

	pub async fn run(mut self) {
		log::info!("DB: Transaction State Running with following parameters: Max Search Result: {}, Max Stored Block Count: {}", self.max_search_results, self.max_stored_block_count);

		loop {
			let now = Instant::now();
			let mut work_done = false;

			if !self.block_receiver.is_empty() {
				while let Ok(new_block) = self.block_receiver.try_recv() {
					self.add_block(new_block);
					work_done = true;
				}
			}

			if !self.search_receiver.is_empty() {
				while let Ok(details) = self.search_receiver.try_recv() {
					println!("Searching TX");
					self.send_transaction_status(details);
					work_done = true;
				}
			}

			if work_done {
				log::info!("DB: Receiving and Sending took: {:.02?}", now.elapsed());
				log::info!(
					"DB: Inclusion Count: {}, Finalized Count: {}",
					self.included_blocks.len(),
					self.finalized_blocks.len()
				);
			}

			tokio::time::sleep(Duration::from_millis(2500)).await;
		}
	}

	fn search_transaction_status(
		&self,
		tx_hash: H256,
		array: &VecDeque<BlockDetails>,
		out: &mut Vec<RPCTransactionState>,
	) {
		for block in array.iter().rev() {
			for status in &block.transactions {
				if status.tx_hash != tx_hash {
					continue;
				}

				out.push(RPCTransactionState {
					block_hash: block.block_hash,
					block_height: block.block_height,
					tx_hash: status.tx_hash,
					tx_index: status.tx_index,
					tx_success: status.tx_success,
					pallet_index: status.pallet_index,
					call_index: status.call_index,
					is_finalized: block.finalized,
				});

				if out.len() >= self.max_search_results {
					return;
				}
			}
		}
	}

	fn send_transaction_status(&self, details: (H256, bool, OneShotTxStateSender)) {
		let (tx_hash, is_finalized, oneshot) = details;

		let mut result: Vec<RPCTransactionState> = Vec::new();
		if !is_finalized {
			self.search_transaction_status(tx_hash, &self.included_blocks, &mut result);
		}
		if result.len() < self.max_search_results {
			self.search_transaction_status(tx_hash, &self.finalized_blocks, &mut result);
		}

		_ = oneshot.send(result);
	}

	fn push_new_finalized_block(&mut self, new_block: BlockDetails, mut index: usize) {
		if self.finalized_blocks.len() >= self.max_stored_block_count {
			self.finalized_blocks.pop_front();

			if index > 0 {
				index -= 1
			}
		}

		self.finalized_blocks.insert(index, new_block);
	}

	fn add_finalized_block(&mut self, new_block: BlockDetails) {
		// Remove the same block height from included block vector
		while let Some(pos) = self
			.included_blocks
			.iter()
			.position(|b| b.block_height == new_block.block_height)
		{
			self.included_blocks.remove(pos);
		}

		if self
			.finalized_blocks
			.front()
			.is_some_and(|b| new_block.block_height < b.block_height)
		{
			self.push_new_finalized_block(new_block, 0);
			return;
		}

		if self
			.finalized_blocks
			.back()
			.is_some_and(|b| new_block.block_height >= b.block_height)
		{
			self.push_new_finalized_block(new_block, self.finalized_blocks.len());
			return;
		}

		for (i, elem) in self.finalized_blocks.iter().enumerate().rev() {
			if new_block.block_height >= elem.block_height {
				self.push_new_finalized_block(new_block, i + 1);
				return;
			}
		}

		self.push_new_finalized_block(new_block, 0);
	}

	fn add_block(&mut self, new_block: BlockDetails) {
		match new_block.finalized {
			true => self.add_finalized_block(new_block),
			false => self.add_included_block(new_block),
		}
	}

	fn add_included_block(&mut self, new_block: BlockDetails) {
		if self.included_blocks.len() >= self.max_stored_block_count {
			self.included_blocks.pop_front();
		}

		self.included_blocks.push_back(new_block);
	}
}

pub struct InclusionWorker {
	pub rpc_handlers: RpcHandlers,
	pub client: Arc<FullClient>,
	pub sender: Sender<BlockDetails>,
}

impl InclusionWorker {
	pub async fn run(self) {
		log::info!("WKI: Transaction State Inclusion Worker up and running.");
		wait_for_sync(&self.rpc_handlers).await;
		log::info!("WKI: Sync Done");

		let mut current_block_hash = H256::default();
		loop {
			let block = self.fetch_next_block(&current_block_hash).await;
			let block = build_block_details(block.0, block.1, block.2, block.3, false).await;
			current_block_hash = block.block_hash.clone();

			log::info!(
				"WKI: New Block Fetched. Hash: {:?}, Height: {}",
				block.block_hash,
				block.block_height
			);

			let ok = self.sender.send(block).await;
			if ok.is_err() {
				return;
			}
		}
	}

	async fn fetch_next_block(
		&self,
		current_block_hash: &H256,
	) -> (
		Vec<OpaqueExtrinsic>,
		H256,
		u32,
		Vec<TransactionSuccessStatus>,
	) {
		loop {
			let chain_info = self.client.chain_info();
			let (block_hash, block_height) = (chain_info.best_hash, chain_info.best_number);

			if (*current_block_hash).eq(&block_hash) {
				tokio::time::sleep(Duration::from_millis(1000)).await;
				continue;
			}

			log::info!(
				"WKI: Fetching block. Hash: {:?}, Height: {}",
				block_hash,
				block_hash
			);

			let Some(states) = fetch_execution_states(&self.rpc_handlers, &block_hash).await else {
				log::info!("WKI: Failed to fetch execution states");
				tokio::time::sleep(Duration::from_millis(2500)).await;
				continue;
			};

			let Ok(Some(extrinsics)) = self.client.body(block_hash) else {
				log::info!("WKI: Failed to fetch block body");
				tokio::time::sleep(Duration::from_millis(2500)).await;
				continue;
			};

			return (extrinsics, block_hash, block_height, states);
		}
	}
}

pub struct FinalizedWorker {
	pub rpc_handlers: RpcHandlers,
	pub client: Arc<FullClient>,
	pub sender: Sender<BlockDetails>,
}

impl FinalizedWorker {
	pub async fn run(self) {
		log::info!("WKF: Transaction State Finalization Worker up and running.");
		wait_for_sync(&self.rpc_handlers).await;
		log::info!("WKF: Sync Done");
		let mut height = self.index_old_blocks().await;
		log::info!("WKF: Old Block indexing Done");

		loop {
			let block = self.fetch_next_block(&mut height).await;
			let block = build_block_details(block.0, block.1, height, block.2, true).await;
			log::info!(
				"WKF: New Block Fetched: Hash: {:?}, Height: {}",
				block.block_hash,
				block.block_height
			);

			let ok = self.sender.send(block).await;
			if ok.is_err() {
				return;
			}

			log::info!("WKF: Successfully indexed block. Height: {}", height);

			height += 1;
		}
	}

	async fn index_old_blocks(&self) -> u32 {
		let chain_info = self.client.chain_info();
		log::info!(
			"WKF: Indexing old blocks. Latest finalized block height: {}",
			chain_info.finalized_number
		);
		if chain_info.finalized_number == 0 {
			return chain_info.finalized_number;
		}

		let mut height = chain_info.finalized_number - 1;
		loop {
			log::info!("WKF: Indexing old block. Height: {}", height);
			// If we cannot fetch header, block details, or transaction states then we bail out.
			//
			// This most likely means that the pruning strategy removed the header and/or block body
			// or the new runtime API is not there so there isn't much that we can do.
			let Some(block) = self.fetch_block(height).await else {
				log::info!("WKF: Failed to index old block. Height: {}", height);
				break;
			};

			let block = build_block_details(block.0, block.1, height, block.2, true).await;

			// Failure would mean that the other end of the channel is closed which means that we should bail out.
			let ok = self.sender.send(block).await;
			if ok.is_err() {
				break;
			}

			log::info!("WKF: Successfully indexed old block. Height: {}", height);

			if height == 0 {
				break;
			}

			height -= 1;
		}

		chain_info.finalized_number
	}

	async fn fetch_block(
		&self,
		block_height: u32,
	) -> Option<(Vec<OpaqueExtrinsic>, H256, Vec<TransactionSuccessStatus>)> {
		let block_hash = self.client.to_hash(&BlockId::Number(block_height));

		// If Err then bail out.
		// If None then bail out as there is no header available.
		let Ok(Some(block_hash)) = block_hash else {
			log::info!("WKF: Failed to get block hash from block height");
			return None;
		};

		// If Err then bail out.
		// If None then bail out as there is no block to be found.
		let Ok(Some(extrinsics)) = self.client.body(block_hash) else {
			log::info!("WKF: Failed to fetch block body");
			return None;
		};

		// If we cannot fetch the transaction execution statutes (success or failure) then we bail out.
		//
		// This most likely means that our new Runtime API is not available so there isn't much that we can do.
		let Some(states) = fetch_execution_states(&self.rpc_handlers, &block_hash).await else {
			log::info!("WKF: Failed to fetch execution states");
			return None;
		};

		return Some((extrinsics, block_hash, states));
	}

	async fn fetch_next_block(
		&self,
		height: &mut u32,
	) -> (Vec<OpaqueExtrinsic>, H256, Vec<TransactionSuccessStatus>) {
		loop {
			let chain_info = self.client.chain_info();
			if *height > chain_info.finalized_number {
				tokio::time::sleep(Duration::from_millis(1000)).await;
				continue;
			}

			log::info!("WKF: Indexing block height: {}", *height);
			let block_hash = self.client.to_hash(&BlockId::Number(*height));
			let Ok(Some(block_hash)) = block_hash else {
				log::info!("WKF: Failed to get block hash from block height");
				*height = *height + 1;
				continue;
			};

			let Ok(Some(extrinsics)) = self.client.body(block_hash) else {
				log::info!("WKF: Failed to fetch block body");
				*height = *height + 1;
				continue;
			};

			let Some(states) = fetch_execution_states(&self.rpc_handlers, &block_hash).await else {
				log::info!("WKF: Failed to fetch execution states");
				*height = *height + 1;
				continue;
			};

			return (extrinsics, block_hash, states);
		}
	}
}

async fn wait_for_sync(handler: &RpcHandlers) {
	loop {
		match fetch_sync_status(handler).await {
			Some(true) => (),
			Some(false) => return,
			None => (),
		}

		tokio::time::sleep(Duration::from_secs(10)).await;
	}
}

async fn fetch_sync_status(handler: &RpcHandlers) -> Option<bool> {
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

async fn fetch_execution_states(
	handlers: &RpcHandlers,
	block_hash: &H256,
) -> Option<Vec<TransactionSuccessStatus>> {
	let query = format!(
		r#"{{
		"jsonrpc": "2.0",
		"method": "state_call",
		"params": ["SystemEventsApi_fetch_events", "0x", "{}"],
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

async fn build_block_details(
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

		println!("Hex Tx Hash: {}", to_hex(ext.as_ref(), false));
		let tx_hash = Blake2Hasher::hash(ext.as_ref());
		println!(
			"Height: {}, Tx Index: {}, Tx Hash: {:?}",
			block_height, i, tx_hash
		);

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

fn read_pallet_call_index(ext: &UncheckedExtrinsic) -> Option<(u8, u8)> {
	let ext = ext.function.encode();
	if ext.len() < 2 {
		return None;
	}
	let pallet_index = ext[0];
	let call_index = ext[1];

	Some((pallet_index, call_index))
}

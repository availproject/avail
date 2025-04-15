use super::{constants, BlockDetails, CliDeps, Deps, Sender, TransactionState};
use crate::{
	service::FullClient,
	workers::{chain_api, macros::profile, read_pallet_call_index, Timer},
};
use avail_core::OpaqueExtrinsic;
use codec::Encode;
use da_runtime::UncheckedExtrinsic;
use frame_system_rpc_runtime_api::{SystemFetchEventsParams, SystemFetchEventsResult};
use jsonrpsee::tokio::{self, sync::Notify};
use sc_service::RpcHandlers;
use sc_telemetry::log;
use sp_core::{Blake2Hasher, Hasher, H256};
use sp_runtime::{generic::BlockId, traits::BlockIdTo};
use std::{
	sync::Arc,
	time::{Duration, Instant},
};
use tokio::time::sleep;

pub struct Worker {
	client: Arc<FullClient>,
	handlers: RpcHandlers,
	sender: Sender,
	notifier: Arc<Notify>,
	cli: CliDeps,
}

impl Worker {
	pub fn new(client: Arc<FullClient>, handlers: RpcHandlers, deps: &Deps) -> Self {
		Self {
			client,
			handlers,
			sender: deps.block_sender.clone(),
			notifier: deps.notifier.clone(),
			cli: deps.cli.clone(),
		}
	}

	pub async fn run(self, finalization: bool) {
		wait_for_sync(&self.handlers).await;
		wait_for_events_api_to_be_ready(&self.client, &self.handlers).await;

		match finalization {
			true => self.run_finalization().await,
			false => self.run_inclusion().await,
		};
	}

	async fn run_inclusion(self) {
		let mut logger = Logger::new("Inclusion Worker".into(), self.cli.logging_interval);

		let mut current_block_hash = H256::default();
		loop {
			let (block_height, block_hash) =
				wait_for_new_best_block(&self.client, current_block_hash).await;
			current_block_hash = block_hash;
			let now = Instant::now();

			let block = fetch_block_body(&self.client, block_height).await;
			let Ok((opaques, block_hash)) = block else {
				continue;
			};
			let Ok(events) = fetch_events(&self.handlers, &block_hash).await else {
				return;
			};
			let block = prepare_block(opaques, block_hash, block_height, events, true);

			logger.increment_block_fetch(now.elapsed());

			if let Err(e) = self.sender.send(block).await {
				logger.log_error(e.to_string());
				return;
			}

			self.notifier.notify_one();

			logger.log_stats();
			current_block_hash = block_hash;
		}
	}

	pub async fn run_finalization(self) {
		let mut logger = Logger::new("Finalization Worker".into(), self.cli.logging_interval);
		let (duration, mut next_block_height) = profile!(self.index_old_blocks(&logger).await);
		logger.log_index_old_blocks_time(duration);

		loop {
			wait_for_new_finalized_block(&self.client, next_block_height).await;

			let now = Instant::now();
			let Ok((opaques, block_hash)) = fetch_block_body(&self.client, next_block_height).await
			else {
				next_block_height += 1;
				continue;
			};
			let Ok(events) = fetch_events(&self.handlers, &block_hash).await else {
				return;
			};
			let block = prepare_block(opaques, block_hash, next_block_height, events, true);

			logger.increment_block_fetch(now.elapsed());

			if let Err(e) = self.sender.send(block).await {
				logger.log_error(e.to_string());
				return;
			}

			self.notifier.notify_one();

			logger.log_stats();
			next_block_height += 1;
		}
	}

	// Returns the next block height that needs to be fetched
	async fn index_old_blocks(&self, logger: &Logger) -> u32 {
		let finalized_height = self.client.chain_info().finalized_number as u32;
		if finalized_height == 0 || self.cli.max_stored_block_count == 0 {
			return finalized_height;
		}

		// We can index only up to the maximum amount of blocks that we are allowed to store in the database
		let mut limit = self.cli.max_stored_block_count;
		let mut height = finalized_height;
		let mut index_count = 0u32;

		while height != 0 && limit != 0 {
			limit -= 1;
			height -= 1;

			// If we cannot fetch header, block details, or transaction states then we bail out.
			//
			// This most likely means that the pruning strategy removed the header and/or block body
			// or the new runtime API is not there so there isn't much that we can do.
			let block = fetch_block_body(&self.client, height).await;
			let Ok((opaques, block_hash)) = block else {
				break;
			};
			let Ok(events) = fetch_events(&self.handlers, &block_hash).await else {
				break;
			};

			let block = prepare_block(opaques, block_hash, height, events, true);

			// Failure would mean that the other end of the channel is closed which means that we should bail out.
			if self.sender.send(block).await.is_err() {
				break;
			}

			index_count += 1;
		}

		logger.log(std::format!("Indexed {} old blocks.", index_count));
		finalized_height
	}
}

async fn wait_for_sync(handler: &RpcHandlers) {
	loop {
		if chain_api::system_fetch_sync_status(handler).await == Some(false) {
			break;
		}

		sleep(Duration::from_secs(constants::NODE_SYNC_SLEEP_INTERVAL)).await;
	}
}

async fn wait_for_events_api_to_be_ready(client: &Arc<FullClient>, handlers: &RpcHandlers) {
	let block_hash = client.chain_info().finalized_hash;
	let params = SystemFetchEventsParams::default();
	loop {
		if chain_api::system_fetch_events(handlers, params.clone(), &block_hash)
			.await
			.is_some()
		{
			break;
		}

		sleep(Duration::from_secs(constants::NODE_SYNC_SLEEP_INTERVAL)).await;
	}
}

async fn wait_for_new_best_block(
	client: &Arc<FullClient>,
	current_block_hash: H256,
) -> (u32, H256) {
	loop {
		let chain_info = client.chain_info();
		let (block_hash, block_height) = (chain_info.best_hash, chain_info.best_number);

		if current_block_hash.eq(&block_hash) {
			sleep(Duration::from_millis(constants::WORKER_SLEEP_ON_FETCH)).await;
			continue;
		}

		return (block_height, block_hash);
	}
}

async fn wait_for_new_finalized_block(client: &Arc<FullClient>, height: u32) {
	loop {
		let chain_info = client.chain_info();
		if height > chain_info.finalized_number {
			sleep(Duration::from_millis(constants::WORKER_SLEEP_ON_FETCH)).await;
			continue;
		}
	}
}

fn prepare_block(
	opaques: Vec<OpaqueExtrinsic>,
	block_hash: H256,
	block_height: u32,
	events: SystemFetchEventsResult,
	finalized: bool,
) -> BlockDetails {
	let mut txs: Vec<TransactionState> = Vec::with_capacity(opaques.len());
	for (tx_index, ext) in opaques.iter().enumerate() {
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

async fn fetch_block_body(
	client: &Arc<FullClient>,
	block_height: u32,
) -> Result<(Vec<OpaqueExtrinsic>, H256), FetchBlockError> {
	let block_hash = client.to_hash(&BlockId::Number(block_height));

	// If Err then bail out.
	// If None then bail out as there is no header available.
	let Ok(Some(block_hash)) = block_hash else {
		return Err(FetchBlockError::BlockHashNotFound);
	};

	// If Err then bail out.
	// If None then bail out as there is no block to be found.
	let Ok(Some(opaques)) = client.body(block_hash) else {
		return Err(FetchBlockError::BlockBodyNotFound);
	};

	return Ok((opaques, block_hash));
}

async fn fetch_events(
	handlers: &RpcHandlers,
	block_hash: &H256,
) -> Result<SystemFetchEventsResult, FetchEventsError> {
	use frame_system_rpc_runtime_api::events::event_id::system;

	let mut params = SystemFetchEventsParams::default();
	params.filter_events = Some(vec![
		(system::PALLET_ID, system::EXTRINSIC_SUCCESS),
		(system::PALLET_ID, system::EXTRINSIC_FAILED),
	]);
	let Some(events) = chain_api::system_fetch_events(handlers, params, block_hash).await else {
		return Err(FetchEventsError::FailedToRetrieveEvents);
	};

	if events.error != 0 {
		return Err(FetchEventsError::RetrievedEventsErrored);
	}

	return Ok(events);
}

#[derive(Clone, Copy)]
pub enum FetchBlockError {
	BlockHashNotFound,
	BlockBodyNotFound,
}

#[derive(Clone, Copy)]
pub enum FetchEventsError {
	FailedToRetrieveEvents,
	RetrievedEventsErrored,
}

pub(crate) struct Logger {
	count: u32,
	total_duration: Duration,
	timer: Timer,
	name: String,
}

impl Logger {
	pub fn new(name: String, logging_interval: u64) -> Self {
		Self {
			count: 0,
			total_duration: Duration::default(),
			timer: Timer::new(logging_interval),
			name,
		}
	}

	pub fn increment_block_fetch(&mut self, value: Duration) {
		self.total_duration += value;
		self.count += 1;
	}

	pub fn log_stats(&mut self) {
		if !self.timer.expired() {
			return;
		}

		let message = std::format!(
			"Total Duration: {} ms, Block Count: {}",
			self.total_duration.as_millis(),
			self.count
		);
		self.log(message);

		self.count = 0;
		self.total_duration = Duration::default();
		self.timer.restart();
	}

	pub fn log_index_old_blocks_time(&self, duration: Duration) {
		log::info!(
			"👾 {}: Index old blocks duration: {:.02?}",
			self.name,
			duration
		);
	}

	pub fn log_error(&self, message: String) {
		log::warn!("👾 {}: {}", self.name, message);
	}

	pub fn log(&self, message: String) {
		log::info!("👾 {}: {}", self.name, message);
	}
}

use super::super::runtime_api;
use super::worker;
use super::worker_logger::Logger;
use super::BlockDetails;
use crate::service::FullClient;
use crate::transaction::macros::profile;
use avail_core::OpaqueExtrinsic;
use frame_system_rpc_runtime_api::{
	events::event_id::system, SystemFetchEventsParams, SystemFetchEventsResult,
};
use jsonrpsee::tokio;
use jsonrpsee::tokio::sync::mpsc::Sender;
use sc_service::RpcHandlers;
use sp_core::H256;
use sp_runtime::generic::BlockId;
use sp_runtime::traits::BlockIdTo;
use std::sync::Arc;
use std::time::{Duration, Instant};

const SLEEP_ON_FETCH: u64 = 1000; // ms
const SLEEP_ON_ERROR: u64 = 2500; // ms

pub struct FinalizedWorker {
	pub rpc_handlers: RpcHandlers,
	pub client: Arc<FullClient>,
	pub sender: Sender<BlockDetails>,
	pub max_stored_block_count: usize,
	pub logger: Logger,
}

impl FinalizedWorker {
	pub async fn run(mut self) {
		// Do nothing if we are not allowed to store any blocks.
		if self.max_stored_block_count == 0 {
			self.logger
				.log("Max Stored Block Count is equal to 0. Worker won't run.".into());
			return;
		}

		let (duration, _) = profile!(worker::wait_for_sync(&self.rpc_handlers).await);
		self.logger.log_sync_time(duration);

		let (duration, mut next_block_height) = profile!(self.index_old_blocks().await);
		self.logger.log_index_old_blocks_time(duration);

		loop {
			let (extrinsics, block_hash, block_height, states) =
				self.fetch_next_block(next_block_height).await;
			let block =
				worker::prepare_block(extrinsics, block_hash, block_height, states, true).await;

			if let Err(e) = self.sender.send(block).await {
				self.logger.log_error(e.to_string());
				return;
			}

			self.logger.log_stats();
			next_block_height = block_height + 1;
		}
	}

	// Returns the next block height that needs to be fetched
	async fn index_old_blocks(&self) -> u32 {
		let finalized_height = self.client.chain_info().finalized_number as u32;
		if finalized_height == 0 || self.max_stored_block_count == 0 {
			return finalized_height;
		}

		// We can index only up to the maximum amount of blocks that we are allowed to store in the database
		let mut limit = self.max_stored_block_count;
		let mut height = finalized_height;
		let mut index_count = 0u32;

		while height != 0 && limit != 0 {
			limit -= 1;
			height -= 1;

			// If we cannot fetch header, block details, or transaction states then we bail out.
			//
			// This most likely means that the pruning strategy removed the header and/or block body
			// or the new runtime API is not there so there isn't much that we can do.
			let Some((extrinsics, block_hash, states)) = self.fetch_block(height).await else {
				break;
			};

			let block = worker::prepare_block(extrinsics, block_hash, height, states, true).await;

			// Failure would mean that the other end of the channel is closed which means that we should bail out.
			if self.sender.send(block).await.is_err() {
				break;
			}

			index_count += 1;
		}

		self.logger
			.log(std::format!("Indexed {} old blocks.", index_count));
		finalized_height
	}

	async fn fetch_block(
		&self,
		block_height: u32,
	) -> Option<(Vec<OpaqueExtrinsic>, H256, SystemFetchEventsResult)> {
		let block_hash = self.client.to_hash(&BlockId::Number(block_height));

		// If Err then bail out.
		// If None then bail out as there is no header available.
		let Ok(Some(block_hash)) = block_hash else {
			return None;
		};

		// If Err then bail out.
		// If None then bail out as there is no block to be found.
		let Ok(Some(extrinsics)) = self.client.body(block_hash) else {
			return None;
		};

		// TODO
		let params = SystemFetchEventsParams::default();
		let Some(events) =
			runtime_api::system_fetch_events(&self.rpc_handlers, params, &block_hash).await
		else {
			return None;
		};

		if events.error != 0 {
			return None;
		}

		return Some((extrinsics, block_hash, events));
	}

	async fn fetch_next_block(
		&mut self,
		mut height: u32,
	) -> (Vec<OpaqueExtrinsic>, H256, u32, SystemFetchEventsResult) {
		loop {
			let chain_info = self.client.chain_info();
			if height > chain_info.finalized_number {
				tokio::time::sleep(Duration::from_millis(SLEEP_ON_FETCH)).await;
				continue;
			}

			let now = Instant::now();

			let block_hash = self.client.to_hash(&BlockId::Number(height));
			let Ok(Some(block_hash)) = block_hash else {
				self.logger.log_error(std::format!(
					"Failed to get block hash for block number: {}",
					height
				));
				tokio::time::sleep(Duration::from_millis(SLEEP_ON_ERROR)).await;
				continue;
			};

			let Ok(Some(extrinsics)) = self.client.body(block_hash) else {
				self.logger.log_error(std::format!(
					"Failed to get the body for block hash: {:?}",
					block_hash
				));
				tokio::time::sleep(Duration::from_millis(SLEEP_ON_ERROR)).await;
				continue;
			};

			let mut params = SystemFetchEventsParams::default();
			params.filter_events = Some(vec![
				(system::PALLET_ID, system::EXTRINSIC_SUCCESS),
				(system::PALLET_ID, system::EXTRINSIC_FAILED),
			]);
			let Some(events) =
				runtime_api::system_fetch_events(&self.rpc_handlers, params, &block_hash).await
			else {
				tokio::time::sleep(Duration::from_millis(SLEEP_ON_ERROR)).await;
				height = height + 1;
				continue;
			};

			if events.error != 0 {
				tokio::time::sleep(Duration::from_millis(SLEEP_ON_ERROR)).await;
				height = height + 1;
				continue;
			}

			self.logger.add_block_fetch(now.elapsed());

			return (extrinsics, block_hash, height, events);
		}
	}
}

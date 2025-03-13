use super::macros::profile;
use super::worker;
use super::worker_logger::Logger;
use super::BlockDetails;
use crate::service::FullClient;
use avail_core::OpaqueExtrinsic;
use frame_system_rpc_runtime_api::TransactionSuccessStatus;
use jsonrpsee::tokio;
use jsonrpsee::tokio::sync::mpsc::Sender;
use sc_service::RpcHandlers;
use sp_core::H256;
use sp_runtime::generic::BlockId;
use sp_runtime::traits::BlockIdTo;
use std::sync::Arc;
use std::time::{Duration, Instant};
pub struct FinalizedWorker {
	pub rpc_handlers: RpcHandlers,
	pub client: Arc<FullClient>,
	pub sender: Sender<BlockDetails>,
	pub max_stored_block_count: usize,
	pub logger: Logger,
}

impl FinalizedWorker {
	pub async fn run(mut self) {
		let (duration, _) = profile!(worker::wait_for_sync(&self.rpc_handlers).await);
		self.logger.log_sync_time(duration);

		let (duration, mut height) = profile!(self.index_old_blocks().await);
		self.logger.log_index_old_blocks_time(duration);

		loop {
			let block = self.fetch_next_block(&mut height).await;
			let block = worker::prepare_block(block.0, block.1, height, block.2, true).await;

			if let Err(e) = self.sender.send(block).await {
				self.logger.log_error(e.to_string());
				return;
			}

			self.logger.log();
			height += 1;
		}
	}

	async fn index_old_blocks(&self) -> u32 {
		let chain_info = self.client.chain_info();
		if chain_info.finalized_number == 0 {
			return chain_info.finalized_number;
		}

		let mut max_block_count = self.max_stored_block_count;
		let mut height = chain_info.finalized_number - 1;
		loop {
			// If we cannot fetch header, block details, or transaction states then we bail out.
			//
			// This most likely means that the pruning strategy removed the header and/or block body
			// or the new runtime API is not there so there isn't much that we can do.
			let Some(block) = self.fetch_block(height).await else {
				break;
			};

			let block = worker::prepare_block(block.0, block.1, height, block.2, true).await;

			// Failure would mean that the other end of the channel is closed which means that we should bail out.
			let ok = self.sender.send(block).await;
			if ok.is_err() {
				break;
			}

			if height == 0 || max_block_count == 0 {
				break;
			}

			max_block_count -= 1;
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
			return None;
		};

		// If Err then bail out.
		// If None then bail out as there is no block to be found.
		let Ok(Some(extrinsics)) = self.client.body(block_hash) else {
			return None;
		};

		// If we cannot fetch the transaction execution statutes (success or failure) then we bail out.
		//
		// This most likely means that our new Runtime API is not available so there isn't much that we can do.
		let Some(states) =
			worker::fetch_extrinsic_success_status(&self.rpc_handlers, &block_hash).await
		else {
			return None;
		};

		return Some((extrinsics, block_hash, states));
	}

	async fn fetch_next_block(
		&mut self,
		height: &mut u32,
	) -> (Vec<OpaqueExtrinsic>, H256, Vec<TransactionSuccessStatus>) {
		loop {
			let chain_info = self.client.chain_info();
			if *height > chain_info.finalized_number {
				tokio::time::sleep(Duration::from_millis(1000)).await;
				continue;
			}

			let now = Instant::now();

			let block_hash = self.client.to_hash(&BlockId::Number(*height));
			let Ok(Some(block_hash)) = block_hash else {
				*height = *height + 1;
				continue;
			};

			let Ok(Some(extrinsics)) = self.client.body(block_hash) else {
				*height = *height + 1;
				continue;
			};

			let Some(states) =
				worker::fetch_extrinsic_success_status(&self.rpc_handlers, &block_hash).await
			else {
				*height = *height + 1;
				continue;
			};

			self.logger.add_block_fetch(now.elapsed());

			return (extrinsics, block_hash, states);
		}
	}
}

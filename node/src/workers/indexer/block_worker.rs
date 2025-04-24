use super::*;
use crate::workers::{chain_api, common::NodeContext, macros::profile};
use jsonrpsee::tokio::{self, sync::Notify};
use sc_telemetry::log;
use sp_core::H256;
use std::{sync::Arc, time::Duration};
use tokio::time::sleep;

pub struct BlockWorker {
	ctx: NodeContext,
	sender: Sender,
	notifier: Arc<Notify>,
	cli: CliDeps,
	name: String,
}

impl BlockWorker {
	pub fn new(context: NodeContext, deps: &Deps, name: String) -> Self {
		Self {
			ctx: context,
			sender: deps.block_sender.clone(),
			notifier: deps.notifier.clone(),
			cli: deps.cli.clone(),
			name,
		}
	}

	pub async fn run(self, index_finalized_blocks: bool) {
		if self.wait_for_sync().await.is_err() {
			self.log("Failed to wait for sync");
			return;
		}

		match index_finalized_blocks {
			true => self.index_finalized_blocks().await,
			false => self.index_new_blocks().await,
		};
	}

	async fn index_new_blocks(self) {
		let mut current_block_hash = H256::default();
		loop {
			let block_id = self.wait_for_new_best_block(current_block_hash).await;
			current_block_hash = block_id.hash;

			let Some(opaques) = self.ctx.block_body_hash(block_id.hash) else {
				continue;
			};
			let block = BlockDetails::from_opaques(opaques, block_id, false);

			if let Err(e) = self.sender.send(block).await {
				self.log(&e.to_string());
				return;
			}

			self.notifier.notify_one();
		}
	}

	pub async fn index_finalized_blocks(self) {
		let (duration, (mut block_height, count)) = profile!(self.index_old_blocks().await);
		let message = std::format!(
			"Indexing old blocks took: {} ms. Count: {}",
			duration.as_millis(),
			count
		);
		self.log(&message);

		loop {
			self.wait_for_new_finalized_block(block_height).await;
			let Some((opaques, block_id)) = self.ctx.block_body(block_height) else {
				block_height += 1;
				continue;
			};
			let block = BlockDetails::from_opaques(opaques, block_id, true);

			if let Err(e) = self.sender.send(block).await {
				self.log(&e.to_string());
				return;
			}

			self.notifier.notify_one();
			block_height += 1;
		}
	}

	// Returns the next block height that needs to be fetched
	async fn index_old_blocks(&self) -> (u32, usize) {
		let finalized_height = self.ctx.client.chain_info().finalized_number;
		if finalized_height == 0 {
			return (finalized_height, 0);
		}

		// We can index only up to the maximum amount of blocks that we are allowed to store in the database
		let mut limit = self.cli.block_pruning;
		let mut height = finalized_height;

		while limit != 0 {
			// If we cannot fetch block body then we bail out.
			let Some((opaques, block_id)) = self.ctx.block_body(height) else {
				break;
			};
			let block = BlockDetails::from_opaques(opaques, block_id, true);

			// Failure would mean that the other end of the channel is closed which means that we should bail out.
			if self.sender.send(block).await.is_err() {
				break;
			}

			self.notifier.notify_one();
			if height == 0 {
				break;
			}

			limit -= 1;
			height -= 1;
		}

		(finalized_height + 1, self.cli.block_pruning - limit)
	}

	async fn wait_for_sync(&self) -> Result<(), ()> {
		loop {
			let status = chain_api::system_fetch_sync_status(&self.ctx.handlers).await;
			match status {
				Some(true) => (),
				Some(false) => return Ok(()),
				None => return Err(()),
			}

			sleep(Duration::from_secs(NODE_SYNC_SLEEP_INTERVAL)).await;
		}
	}

	async fn wait_for_new_best_block(&self, current_block_hash: H256) -> BlockIdentifier {
		loop {
			let chain_info = self.ctx.client.chain_info();
			let (block_hash, block_height) = (chain_info.best_hash, chain_info.best_number);
			if current_block_hash.eq(&block_hash) {
				sleep(Duration::from_millis(WORKER_SLEEP_ON_FETCH)).await;
				continue;
			}

			return BlockIdentifier::from((block_hash, block_height));
		}
	}

	async fn wait_for_new_finalized_block(&self, height: u32) {
		loop {
			let chain_info = self.ctx.client.chain_info();
			if height > chain_info.finalized_number {
				sleep(Duration::from_millis(WORKER_SLEEP_ON_FETCH)).await;
				continue;
			}

			break;
		}
	}

	fn log(&self, message: &str) {
		log::info!("👾 {}: {}", self.name, message);
	}
}

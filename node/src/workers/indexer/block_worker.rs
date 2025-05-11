use super::*;
use crate::workers::{common::NodeContext, macros::profile};
use jsonrpsee::tokio::sync::Notify;
use sc_telemetry::log;
use sp_core::H256;
use std::sync::Arc;

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
		if self.ctx.wait_for_sync().await.is_err() {
			self.log("Failed to wait for sync");
			return;
		}

		match index_finalized_blocks {
			true => self.index_finalized_blocks().await,
			false => self.index_best_blocks().await,
		};
	}

	async fn index_best_blocks(self) {
		let mut current_block_hash = H256::default();
		loop {
			let block_id = self.ctx.wait_for_new_best_block(current_block_hash).await;
			current_block_hash = block_id.hash;

			let Some(opaques) = self.ctx.block_body_hash(block_id.hash) else {
				continue;
			};
			let block = BlockDetails::from_opaques(opaques, block_id.height, false, true);

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
			self.ctx.wait_for_new_finalized_block(block_height).await;
			let Some((opaques, block_id)) = self.ctx.block_body(block_height) else {
				block_height += 1;
				continue;
			};
			let block = BlockDetails::from_opaques(opaques, block_id.height, true, true);

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
			let block = BlockDetails::from_opaques(opaques, block_id.height, true, false);

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

	fn log(&self, message: &str) {
		log::info!("👾 {}: {}", self.name, message);
	}
}

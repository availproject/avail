use super::{constants, BlockDetails, CliDeps, Deps, Sender};
use crate::{
	service::FullClient,
	workers::{chain_api, macros::profile},
};
use avail_core::OpaqueExtrinsic;
use jsonrpsee::tokio::{self, sync::Notify};
use sc_service::RpcHandlers;
use sc_telemetry::log;
use sp_core::H256;
use sp_runtime::{generic::BlockId, traits::BlockIdTo};
use std::{sync::Arc, time::Duration};
use tokio::time::sleep;

pub struct BlockWorker {
	client: Arc<FullClient>,
	handlers: RpcHandlers,
	sender: Sender,
	notifier: Arc<Notify>,
	cli: CliDeps,
	name: String,
}

impl BlockWorker {
	pub fn new(client: Arc<FullClient>, handlers: RpcHandlers, deps: &Deps, name: String) -> Self {
		Self {
			client,
			handlers,
			sender: deps.block_sender.clone(),
			notifier: deps.notifier.clone(),
			cli: deps.cli.clone(),
			name,
		}
	}

	pub async fn run(self, index_finalized_blocks: bool) {
		if self.wait_for_sync().await.is_err() {
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
			let (block_height, best_block_hash) =
				self.wait_for_new_best_block(current_block_hash).await;

			let Ok((opaques, block_hash)) = self.fetch_block_body(block_height) else {
				current_block_hash = best_block_hash;
				continue;
			};
			let block = BlockDetails::from_opaques(opaques, block_hash, block_height, false);

			if let Err(e) = self.sender.send(block).await {
				self.log(e.to_string());
				return;
			}

			self.notifier.notify_one();
			current_block_hash = block_hash;
		}
	}

	pub async fn index_finalized_blocks(self) {
		let (duration, (mut block_height, count)) = profile!(self.index_old_blocks().await);
		let message = std::format!(
			"Indexing old blocks took: {} ms. Count: {}",
			duration.as_millis(),
			count
		);
		self.log(message);

		loop {
			self.wait_for_new_finalized_block(block_height).await;
			let Ok((opaques, block_hash)) = self.fetch_block_body(block_height) else {
				block_height += 1;
				continue;
			};

			let block = BlockDetails::from_opaques(opaques, block_hash, block_height, true);
			if let Err(e) = self.sender.send(block).await {
				self.log(e.to_string());
				return;
			}

			self.notifier.notify_one();
			block_height += 1;
		}
	}

	// Returns the next block height that needs to be fetched
	async fn index_old_blocks(&self) -> (u32, usize) {
		let finalized_height = self.client.chain_info().finalized_number;
		if finalized_height == 0 {
			return (finalized_height, 0);
		}

		// We can index only up to the maximum amount of blocks that we are allowed to store in the database
		let mut limit = self.cli.block_pruning;
		let mut height = finalized_height;

		while limit != 0 {
			// If we cannot fetch header, block details, or transaction states then we bail out.
			//
			// This most likely means that the pruning strategy removed the header and/or block body
			// or the new runtime API is not there so there isn't much that we can do.
			let Ok((opaques, block_hash)) = self.fetch_block_body(height) else {
				break;
			};

			let block = BlockDetails::from_opaques(opaques, block_hash, height, true);

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
			let status = chain_api::system_fetch_sync_status(&self.handlers).await;
			match status {
				Some(true) => (),
				Some(false) => return Ok(()),
				None => return Err(()),
			}

			sleep(Duration::from_secs(constants::NODE_SYNC_SLEEP_INTERVAL)).await;
		}
	}

	async fn wait_for_new_best_block(&self, current_block_hash: H256) -> (u32, H256) {
		loop {
			let chain_info = self.client.chain_info();
			let (block_hash, block_height) = (chain_info.best_hash, chain_info.best_number);
			if current_block_hash.eq(&block_hash) {
				sleep(Duration::from_millis(constants::WORKER_SLEEP_ON_FETCH)).await;
				continue;
			}

			return (block_height, block_hash);
		}
	}

	async fn wait_for_new_finalized_block(&self, height: u32) {
		loop {
			let chain_info = self.client.chain_info();
			if height > chain_info.finalized_number {
				sleep(Duration::from_millis(constants::WORKER_SLEEP_ON_FETCH)).await;
				continue;
			}

			break;
		}
	}

	fn fetch_block_body(&self, block_height: u32) -> Result<(Vec<OpaqueExtrinsic>, H256), ()> {
		let block_hash = self.client.to_hash(&BlockId::Number(block_height));

		// If Err or None then bail out as there is no header available.
		let Ok(Some(block_hash)) = block_hash else {
			return Err(());
		};

		// If Err or None then bail out as there is no block to be found.
		let Ok(Some(opaques)) = self.client.body(block_hash) else {
			return Err(());
		};

		Ok((opaques, block_hash))
	}

	fn log(&self, message: String) {
		log::info!("👾 {}: {}", self.name, message);
	}
}

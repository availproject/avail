use sc_telemetry::log;
use sp_core::H256;
use std::collections::VecDeque;
use transaction_rpc::TransactionState as RPCTransactionState;

use super::database::{Config, DatabaseLike};
use super::BlockDetails;

pub struct Database {
	included_blocks: VecDeque<BlockDetails>,
	finalized_blocks: VecDeque<BlockDetails>,
	config: Config,
}

impl Database {
	pub fn new(config: Config) -> Self {
		Self {
			included_blocks: VecDeque::new(),
			finalized_blocks: VecDeque::new(),
			config,
		}
	}

	fn search_transaction_state(
		&self,
		tx_hash: &H256,
		array: &VecDeque<BlockDetails>,
		out: &mut Vec<RPCTransactionState>,
	) {
		for block in array.iter().rev() {
			for status in &block.transactions {
				if status.tx_hash != *tx_hash {
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

				if out.len() >= self.config.max_search_results {
					return;
				}
			}
		}
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

		// If higher height push it to the back
		if self
			.finalized_blocks
			.back()
			.is_some_and(|b| new_block.block_height >= b.block_height)
		{
			self.finalized_blocks
				.insert(self.finalized_blocks.len(), new_block);
			return;
		}

		// If lower height push it to the front
		if self
			.finalized_blocks
			.front()
			.is_some_and(|b| new_block.block_height <= b.block_height)
		{
			self.finalized_blocks.insert(0, new_block);
			return;
		}

		// If somewhere in between push it there.
		//
		// It's unlikely that this code will be executed.
		// During the sync phase new blocks are pushed to the front and during normal
		// operations blocks are push to the back.
		for (i, elem) in self.finalized_blocks.iter().enumerate().rev() {
			if new_block.block_height >= elem.block_height {
				self.finalized_blocks.insert(i + 1, new_block);
				return;
			}
		}

		// If no block is present or if we didn't find a position for it, push it to the front.
		self.finalized_blocks.insert(0, new_block);
	}
}

impl DatabaseLike for Database {
	fn add_block(&mut self, block: BlockDetails) {
		match block.finalized {
			true => self.add_finalized_block(block),
			false => self.included_blocks.push_back(block),
		}
	}

	fn find_transaction_state(
		&self,
		tx_hash: &H256,
		is_finalized: bool,
	) -> Vec<RPCTransactionState> {
		let mut result: Vec<RPCTransactionState> = Vec::new();
		if !is_finalized {
			self.search_transaction_state(tx_hash, &self.included_blocks, &mut result);
		}
		if result.len() < self.config.max_search_results {
			self.search_transaction_state(tx_hash, &self.finalized_blocks, &mut result);
		}
		result
	}

	fn resize(&mut self) {
		let old_len_inc = self.included_blocks.len();
		let old_len_fin = self.finalized_blocks.len();
		let old_cap_inc = self.included_blocks.capacity();
		let old_cap_fin = self.finalized_blocks.capacity();

		if self.included_blocks.capacity() > self.included_blocks.len() * 2 {
			self.included_blocks.shrink_to_fit();
		}

		if self.finalized_blocks.capacity() > self.finalized_blocks.len() * 2 {
			self.finalized_blocks.shrink_to_fit();
		}

		let new_len_inc = self.included_blocks.len();
		let new_len_fin = self.finalized_blocks.len();
		let new_cap_inc = self.included_blocks.capacity();
		let new_cap_fin = self.finalized_blocks.capacity();

		log::info!(
			"👾 I Old Cap: {}, I New Cap: {}, I Old Len: {}, I New Len: {}",
			old_cap_inc,
			new_cap_inc,
			old_len_inc,
			new_len_inc,
		);

		log::info!(
			"👾 F Old Cap: {}, F New Cap: {}, F Old Len: {}, F New Len: {}",
			old_cap_fin,
			new_cap_fin,
			old_len_fin,
			new_len_fin,
		);
	}

	fn config(&self) -> &Config {
		&self.config
	}

	fn variant(&self) -> &str {
		"Vector Database"
	}

	fn new(config: Config) -> Self {
		Self::new(config)
	}

	fn log(&self) {
		log::info!("👾 Database: Inclusion Vec Len: {}, Inclusion Vec Cap: {}, Finalized Vec Len: {}, Finalized Vec Cap: {}", self.included_blocks.len(), self.included_blocks.capacity(), self.finalized_blocks.len(), self.finalized_blocks.capacity());
	}
}

use sc_telemetry::log;
use sp_core::H256;
use std::collections::HashMap;
use transaction_rpc::TransactionState as RPCTransactionState;

use super::database::{Config, DatabaseLike};
use super::{BlockDetails, TransactionState};

#[derive(Debug, Clone)]
struct BlockData {
	pub block_hash: H256,
	pub block_height: u32,
}

#[derive(Debug, Clone)]
struct TransactionData {
	pub tx_index: u32,
	pub tx_success: bool,
	pub pallet_index: u8,
	pub call_index: u8,
	pub block_index: u32,
}

pub struct Database {
	block_map: HashMap<u32, BlockData>,
	block_map_counter: u32,
	included_tx: Map,
	finalized_tx: Map,
	config: Config,
}

impl Database {
	pub fn new(config: Config) -> Self {
		Self {
			block_map: HashMap::new(),
			block_map_counter: 0,
			included_tx: Map::default(),
			finalized_tx: Map::default(),
			config,
		}
	}

	fn add_transaction(&mut self, state: TransactionState, block_index: u32, is_finalized: bool) {
		if is_finalized {
			self.finalized_tx
				.add_transaction(state, block_index, self.config.max_search_results);
		} else {
			self.included_tx
				.add_transaction(state, block_index, self.config.max_search_results);
		};
	}

	fn get_block_index(&self, block_hash: &H256, block_height: u32) -> Option<u32> {
		for (key, value) in self.block_map.iter() {
			if value.block_hash == *block_hash || value.block_height == block_height {
				return Some(*key);
			}
		}

		return None;
	}

	fn get_or_create_block_index(&mut self, block_hash: &H256, block_height: u32) -> u32 {
		if let Some(key) = self.get_block_index(block_hash, block_height) {
			return key;
		}

		let key = self.block_map_counter;
		let value = BlockData {
			block_hash: *block_hash,
			block_height,
		};

		self.block_map.insert(key, value);
		self.block_map_counter += 1;

		key
	}
}

impl DatabaseLike for Database {
	fn add_block(&mut self, new_block: BlockDetails) {
		let block_index: u32 =
			self.get_or_create_block_index(&new_block.block_hash, new_block.block_height);

		// Clean up Included Tx
		if new_block.finalized {
			self.included_tx.remove_block_index(block_index);
		}

		// Add new transaction
		for new_tx in new_block.transactions {
			self.add_transaction(new_tx, block_index, new_block.finalized);
		}
	}

	fn find_transaction_state(
		&self,
		tx_hash: &H256,
		is_finalized: bool,
	) -> Vec<RPCTransactionState> {
		let mut result: Vec<RPCTransactionState> = Vec::new();
		if !is_finalized {
			self.included_tx.search_transaction_state(
				tx_hash,
				&self.block_map,
				self.config.max_search_results,
				false,
				&mut result,
			);
		}

		self.finalized_tx.search_transaction_state(
			tx_hash,
			&self.block_map,
			self.config.max_search_results,
			true,
			&mut result,
		);

		result
	}

	fn resize(&mut self) {
		let old_len_inc = self.included_tx.len();
		let old_len_fin = self.finalized_tx.len();
		let old_cap_inc = self.included_tx.capacity();
		let old_cap_fin = self.finalized_tx.capacity();

		while self.block_map.len() > self.config.max_stored_block_count {
			let entry = self
				.block_map
				.iter()
				.min_by(|x, y| x.1.block_height.cmp(&y.1.block_height));
			let Some(entry) = entry else {
				return;
			};
			let block_index = *entry.0;

			self.block_map.remove(&block_index);
			self.included_tx.remove_block_index(block_index);
			self.finalized_tx.remove_block_index(block_index);
		}

		if self.block_map.capacity() > self.block_map.len() * 2 {
			self.block_map.shrink_to_fit();
		}

		self.included_tx.shrink_to_fit();
		self.finalized_tx.shrink_to_fit();

		let new_len_inc = self.included_tx.len();
		let new_len_fin = self.finalized_tx.len();
		let new_cap_inc = self.included_tx.capacity();
		let new_cap_fin = self.finalized_tx.capacity();

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
		"Map Database"
	}

	fn new(config: Config) -> Self {
		Self::new(config)
	}

	fn log(&self) {
		log::info!("👾 Block Map Counter: {}, Inclusion Map Len: {}, Inclusion Map Cap: {}, Finalized Map Len: {}, Finalized Map Cap: {}", self.block_map_counter, self.included_tx.len(), self.included_tx.capacity(), self.finalized_tx.len(), self.finalized_tx.capacity());
	}
}

#[derive(Debug, Clone, Default)]
struct Map {
	single: HashMap<H256, TransactionData>,
	multi: HashMap<H256, Vec<TransactionData>>,
}

impl Map {
	fn search_transaction_state(
		&self,
		tx_hash: &H256,
		block_map: &HashMap<u32, BlockData>,
		max_count: usize,
		finalized: bool,
		out: &mut Vec<RPCTransactionState>,
	) {
		if out.len() >= max_count {
			return;
		}

		if let Some(data) = self.single.get(tx_hash) {
			if let Some(block) = block_map.get(&data.block_index) {
				out.push(RPCTransactionState {
					block_hash: block.block_hash,
					block_height: block.block_height,
					tx_hash: tx_hash.clone(),
					tx_index: data.tx_index,
					tx_success: data.tx_success,
					pallet_index: data.pallet_index,
					call_index: data.call_index,
					is_finalized: finalized,
				});
			};
		}

		if let Some(entry) = self.multi.get(tx_hash) {
			for data in entry.iter() {
				if out.len() >= max_count {
					break;
				}

				let Some(block) = block_map.get(&data.block_index) else {
					continue;
				};

				out.push(RPCTransactionState {
					block_hash: block.block_hash,
					block_height: block.block_height,
					tx_hash: tx_hash.clone(),
					tx_index: data.tx_index,
					tx_success: data.tx_success,
					pallet_index: data.pallet_index,
					call_index: data.call_index,
					is_finalized: finalized,
				});
			}
		}
	}

	fn add_transaction(&mut self, state: TransactionState, block_index: u32, max_length: usize) {
		let v = TransactionData {
			tx_index: state.tx_index,
			tx_success: state.tx_success,
			pallet_index: state.pallet_index,
			call_index: state.call_index,
			block_index,
		};

		if let Some(entry) = self.multi.get_mut(&state.tx_hash) {
			if entry.len() >= max_length {
				entry.pop();
			}

			entry.insert(0, v);
			return;
		}

		if let Some(entry) = self.single.remove(&state.tx_hash) {
			self.multi.insert(state.tx_hash.clone(), vec![entry, v]);
			return;
		}

		self.single.insert(state.tx_hash.clone(), v);
	}

	fn remove_block_index(&mut self, block_index: u32) {
		self.single
			.retain(|_key, value| value.block_index != block_index);

		self.multi.retain(|_key, value| {
			value.retain(|v| v.block_index != block_index);

			!value.is_empty()
		});
	}

	fn len(&self) -> usize {
		self.single.len() + self.multi.len()
	}

	fn capacity(&self) -> usize {
		self.single.capacity() + self.multi.capacity()
	}

	fn shrink_to_fit(&mut self) {
		if self.single.capacity() > self.single.len() * 2 {
			self.single.shrink_to_fit();
		}

		if self.multi.capacity() > self.multi.len() * 2 {
			self.multi.shrink_to_fit();
		}
	}
}

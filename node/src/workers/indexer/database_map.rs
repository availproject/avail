use sp_core::H256;
use std::collections::HashMap;
use transaction_rpc::transaction_overview;

use super::{BlockDetails, CliDeps, TransactionState};

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
	cli: CliDeps,
}

impl Database {
	pub fn new(cli: CliDeps) -> Self {
		Self {
			block_map: HashMap::new(),
			block_map_counter: 0,
			included_tx: Map::default(),
			finalized_tx: Map::default(),
			cli,
		}
	}

	fn add_transaction(
		&mut self,
		state: TransactionState,
		block_index: u32,
		is_finalized: bool,
		block_height: u32,
	) {
		if is_finalized {
			self.finalized_tx.add_transaction(
				state,
				block_index,
				self.cli.max_search_results,
				block_height,
				&self.block_map,
			);
		} else {
			self.included_tx.add_transaction(
				state,
				block_index,
				self.cli.max_search_results,
				block_height,
				&self.block_map,
			);
		};
	}

	fn get_block_index(&self, block_hash: &H256, block_height: u32) -> Option<u32> {
		for (key, value) in self.block_map.iter() {
			if value.block_hash == *block_hash && value.block_height == block_height {
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

	pub fn add_block(&mut self, new_block: BlockDetails) {
		let block_index: u32 =
			self.get_or_create_block_index(&new_block.block_hash, new_block.block_height);

		// Clean up Included Tx
		if new_block.finalized {
			self.included_tx.filter_up_to(block_index, &self.block_map);
		}

		// Add new transaction
		for new_tx in new_block.transactions {
			self.add_transaction(
				new_tx,
				block_index,
				new_block.finalized,
				new_block.block_height,
			);
		}
	}

	pub fn find_overview(
		&self,
		tx_hash: &H256,
		is_finalized: bool,
	) -> Vec<transaction_overview::Response> {
		let mut result: Vec<transaction_overview::Response> = Vec::new();
		if !is_finalized {
			self.included_tx.search_overview(
				tx_hash,
				&self.block_map,
				self.cli.max_search_results,
				false,
				&mut result,
			);
		}

		self.finalized_tx.search_overview(
			tx_hash,
			&self.block_map,
			self.cli.max_search_results,
			true,
			&mut result,
		);

		result
	}

	pub fn resize(&mut self) {
		if self.cli.max_stored_block_count >= self.block_map.len() {
			return;
		}

		while self.block_map.len() > self.cli.max_stored_block_count {
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
	}

	pub fn current_state(&self) -> String {
		std::format!("Block Map Counter: {}, Block Map Size: {}/{}, Inclusion Map Size: {}/{}, Finalized Map Size: {}/{}", self.block_map_counter, self.block_map.len(), self.block_map.capacity(), self.included_tx.len(), self.included_tx.capacity(), self.finalized_tx.len(), self.finalized_tx.capacity())
	}
}

#[derive(Debug, Clone, Default)]
struct Map {
	single: HashMap<H256, TransactionData>,
	multi: HashMap<H256, Vec<TransactionData>>,
}

impl Map {
	fn search_overview(
		&self,
		tx_hash: &H256,
		block_map: &HashMap<u32, BlockData>,
		max_count: usize,
		finalized: bool,
		out: &mut Vec<transaction_overview::Response>,
	) {
		if out.len() >= max_count {
			return;
		}

		if let Some(data) = self.single.get(tx_hash) {
			if let Some(block) = block_map.get(&data.block_index) {
				out.push(transaction_overview::Response {
					block_finalized: finalized,
					block_hash: block.block_hash,
					block_height: block.block_height,
					tx_hash: tx_hash.clone(),
					tx_index: data.tx_index,
					tx_success: data.tx_success,
					pallet_index: data.pallet_index,
					call_index: data.call_index,
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

				out.push(transaction_overview::Response {
					block_finalized: finalized,
					block_hash: block.block_hash,
					block_height: block.block_height,
					tx_hash: tx_hash.clone(),
					tx_index: data.tx_index,
					tx_success: data.tx_success,
					pallet_index: data.pallet_index,
					call_index: data.call_index,
				});
			}
		}
	}

	fn add_transaction(
		&mut self,
		state: TransactionState,
		block_index: u32,
		max_length: usize,
		block_height: u32,
		block_map: &HashMap<u32, BlockData>,
	) {
		let v = TransactionData {
			tx_index: state.tx_index,
			tx_success: state.tx_success,
			pallet_index: state.pallet_index,
			call_index: state.call_index,
			block_index,
		};

		if let Some(entry) = self.multi.get_mut(&state.tx_hash) {
			if entry.len() < max_length {
				entry.insert(0, v);
				entry.sort_by(|x, y| {
					let xh = block_map
						.get(&x.block_index)
						.map(|x| x.block_height)
						.unwrap_or_default();
					let yh = block_map
						.get(&y.block_index)
						.map(|x| x.block_height)
						.unwrap_or_default();
					yh.cmp(&xh)
				});
				return;
			}

			let highest_height = entry
				.first()
				.map(|x| block_map.get(&x.block_index).map(|y| y.block_height))
				.flatten()
				.unwrap_or_default();
			if block_height > highest_height {
				entry.insert(0, v);
				entry.pop();
				return;
			}

			let lowest_height = entry
				.last()
				.map(|x| block_map.get(&x.block_index).map(|y| y.block_height))
				.flatten()
				.unwrap_or_default();
			if block_height < lowest_height {
				return;
			}

			entry.insert(0, v);
			entry.sort_by(|x, y| {
				let xh = block_map
					.get(&x.block_index)
					.map(|x| x.block_height)
					.unwrap_or_default();
				let yh = block_map
					.get(&y.block_index)
					.map(|x| x.block_height)
					.unwrap_or_default();
				yh.cmp(&xh)
			});
			entry.pop();
			return;
		}

		if let Some(entry) = self.single.remove(&state.tx_hash) {
			let mut value = vec![entry, v];
			value.sort_by(|x, y| {
				let xh = block_map
					.get(&x.block_index)
					.map(|x| x.block_height)
					.unwrap_or_default();
				let yh = block_map
					.get(&y.block_index)
					.map(|x| x.block_height)
					.unwrap_or_default();
				yh.cmp(&xh)
			});
			self.multi.insert(state.tx_hash.clone(), value);
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

	// Removes all blocks up to and including `block_index` height.
	fn filter_up_to(&mut self, block_index: u32, block_map: &HashMap<u32, BlockData>) {
		let Some(block_data) = block_map.get(&block_index) else {
			return;
		};

		let target_height = block_data.block_height;
		self.single.retain(|_x, tx_data| {
			let Some(tx_block_data) = block_map.get(&tx_data.block_index) else {
				return false;
			};

			if target_height >= tx_block_data.block_height {
				return false;
			}

			true
		});

		self.multi.retain(|_x, entries| {
			entries.retain_mut(|tx_data| {
				let Some(tx_block_data) = block_map.get(&tx_data.block_index) else {
					return false;
				};

				if target_height >= tx_block_data.block_height {
					return false;
				}

				true
			});

			!entries.is_empty()
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

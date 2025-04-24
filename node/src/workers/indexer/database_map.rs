use super::{BlockDetails, CliDeps, TransactionDetails};
use sp_core::H256;
use std::{cmp::Ordering, collections::HashMap};
use transaction_rpc::{transaction_overview, BlockIdentifier};

type BlockMap = HashMap<u32, BlockIdentifier>;

pub struct Database {
	block_map: BlockMap,
	block_map_counter: u32,
	included_tx: Map,
	finalized_tx: Map,
	result_length: usize,
}

impl Database {
	pub fn new(cli: &CliDeps) -> Self {
		Self {
			block_map: HashMap::new(),
			block_map_counter: 0,
			included_tx: Map::default(),
			finalized_tx: Map::default(),
			result_length: cli.result_length,
		}
	}

	fn get_or_create_block_index(&mut self, block_id: BlockIdentifier) -> u32 {
		if let Some(item) = self.block_map.iter().find(|x| *x.1 == block_id) {
			return *item.0;
		}

		let key = self.block_map_counter;
		self.block_map.insert(key, block_id);
		self.block_map_counter += 1;

		key
	}

	pub fn add_block(&mut self, new_block: BlockDetails) {
		let block_id = BlockIdentifier::from((new_block.block_hash, new_block.block_height));
		let block_index: u32 = self.get_or_create_block_index(block_id);

		// Clean up Included Tx
		if new_block.finalized {
			self.included_tx
				.filter_up_to(block_id.height, &self.block_map);
		}

		// Add new transaction
		let map = match new_block.finalized {
			true => &mut self.finalized_tx,
			false => &mut self.included_tx,
		};

		map.add_transactions(
			new_block.transactions,
			block_index,
			block_id.height,
			self.result_length,
			&self.block_map,
		);
	}

	pub fn find_overview(
		&self,
		tx_hash: H256,
		is_finalized: bool,
	) -> Vec<transaction_overview::Response> {
		use transaction_overview::Response;
		let mut response: Vec<Response> = Vec::new();

		let to_response =
			|block_finalized: bool, x: (BlockIdentifier, TransactionData)| -> Response {
				Response {
					block_id: x.0,
					block_finalized,
					tx_hash,
					tx_index: x.1.index,
					dispatch_index: x.1.dispatch_index,
					events: None,
				}
			};

		if !is_finalized {
			let result = self.included_tx.find_entries(tx_hash, &self.block_map);
			let result: Vec<Response> = result.into_iter().map(|x| to_response(false, x)).collect();
			response.extend(result);
		}

		let result = self.finalized_tx.find_entries(tx_hash, &self.block_map);
		let result: Vec<Response> = result.into_iter().map(|x| to_response(true, x)).collect();
		response.extend(result);

		response
	}

	pub fn block_count(&self) -> usize {
		self.block_map.len()
	}

	pub fn resize(&mut self, max_size: usize) {
		if self.block_map.len() <= max_size {
			return;
		}
		let diff = max_size - self.block_map.len();

		let mut to_remove: Vec<(u32, u32)> =
			self.block_map.iter().map(|x| (*x.0, x.1.height)).collect();
		to_remove.sort_by(|x, y| x.1.cmp(&y.1));
		to_remove.truncate(diff);

		while let Some((block_index, ..)) = to_remove.pop() {
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
		std::format!(
			"Block Map Size: {}/{}/{}, Inclusion Map Size: {}/{}, Finalized Map Size: {}/{}",
			self.block_map_counter,
			self.block_map.len(),
			self.block_map.capacity(),
			self.included_tx.len(),
			self.included_tx.capacity(),
			self.finalized_tx.len(),
			self.finalized_tx.capacity()
		)
	}
}

#[derive(Debug, Clone)]
struct TransactionData {
	pub index: u32,
	pub dispatch_index: (u8, u8),
	pub block_index: u32,
}

impl TransactionData {
	pub fn from_details(details: &TransactionDetails, block_index: u32) -> Self {
		Self {
			index: details.index,
			dispatch_index: details.dispatch_index,
			block_index,
		}
	}
}

#[derive(Debug, Clone, Default)]
struct Map {
	single: HashMap<H256, TransactionData>,
	multi: HashMap<H256, Vec<TransactionData>>,
}

impl Map {
	fn find_entries(
		&self,
		tx_hash: H256,
		block_map: &BlockMap,
	) -> Vec<(BlockIdentifier, TransactionData)> {
		let mut result = Vec::new();
		if let Some(data) = self.single.get(&tx_hash) {
			if let Some(block_id) = block_map.get(&data.block_index).copied() {
				result.push((block_id, data.clone()));
			};
		}

		if let Some(entry) = self.multi.get(&tx_hash) {
			for data in entry.iter() {
				let Some(block_id) = block_map.get(&data.block_index).copied() else {
					continue;
				};
				result.push((block_id, data.clone()));
			}
		}

		result
	}

	fn add_transactions(
		&mut self,
		details: Vec<TransactionDetails>,
		block_index: u32,
		block_height: u32,
		result_length: usize,
		block_map: &BlockMap,
	) {
		let sort = |x: &TransactionData, y: &TransactionData| -> Ordering {
			let xh = block_map.get(&x.block_index).map(|x| x.height);
			let xh = xh.unwrap_or_default();
			let yh = block_map.get(&y.block_index).map(|x| x.height);
			let yh = yh.unwrap_or_default();
			yh.cmp(&xh)
		};

		for details in details {
			let v = TransactionData::from_details(&details, block_index);

			if let Some(entry) = self.multi.get_mut(&details.hash) {
				let lowest_height = entry
					.last()
					.and_then(|x| block_map.get(&x.block_index).map(|y| y.height))
					.unwrap_or_default();
				if block_height < lowest_height && entry.len() >= result_length {
					continue;
				}

				entry.push(v);
				entry.sort_by(sort);
				while entry.len() >= result_length {
					entry.pop();
				}

				continue;
			}

			if let Some(entry) = self.single.remove(&details.hash) {
				let mut value = vec![entry, v];
				value.sort_by(sort);
				self.multi.insert(details.hash, value);
				continue;
			}

			self.single.insert(details.hash, v);
		}
	}

	fn remove_block_index(&mut self, block_index: u32) {
		self.single
			.retain(|_key, value| value.block_index != block_index);

		self.multi.retain(|_key, value| {
			value.retain(|v| v.block_index != block_index);
			!value.is_empty()
		});
	}

	// Removes all blocks up to and including `min_block_height` height.
	fn filter_up_to(&mut self, min_block_height: u32, block_map: &BlockMap) {
		let retain = |tx_data: &mut TransactionData| {
			let Some(block_id) = block_map.get(&tx_data.block_index) else {
				return false;
			};
			block_id.height > min_block_height
		};

		self.single.retain(|_x, tx_data| retain(tx_data));
		self.multi.retain(|_x, entries| {
			entries.retain_mut(retain);
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

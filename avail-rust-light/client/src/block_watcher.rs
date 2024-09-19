use crate::rpc;
use jsonrpsee_http_client::HttpClient as JRPSHttpClient;
use sdk_core::{crypto::blake2_256, types::H256};
use std::{
	collections::BTreeMap,
	sync::{Arc, Mutex},
	time::Duration,
};

#[derive(Debug, Clone, Default)]
pub struct Database {
	pub value: BTreeMap<H256, bool>,
	pub last_best_block: Option<H256>,
	pub last_finalized_block: Option<H256>,
}

const SLEEP_DURATION: u64 = 1;

#[derive(Debug, Clone)]
pub struct BlockWatcher {
	client: Arc<JRPSHttpClient>,
	db: Arc<Mutex<Database>>,
}
impl BlockWatcher {
	pub fn new(client: Arc<JRPSHttpClient>) -> Self {
		Self {
			client,
			db: Arc::new(Mutex::new(Database::default())),
		}
	}

	pub async fn run(&mut self) {
		/* 		loop {
			self.best_block().await;
			self.finalized_block().await;
			tokio::time::sleep(Duration::from_secs(SLEEP_DURATION)).await;
		} */
	}

	pub async fn best_block(&mut self) {
		let block_hash = rpc::fetch_best_block_hash(&self.client).await.unwrap();
		{
			let db = self.db.lock().unwrap();
			if Some(block_hash) == db.last_best_block {
				return;
			}
		}

		let block = rpc::fetch_block(&self.client, Some(block_hash))
			.await
			.unwrap();

		let mut db = self.db.lock().unwrap();
		for hex_tx in block.block.extrinsics {
			let hex_tx = &hex_tx[2..];
			let encoded_tx: Vec<u8> = hex::decode(hex_tx).unwrap();
			let tx_hash = H256(blake2_256(&encoded_tx));
			db.value.insert(tx_hash, false);
		}
		db.last_best_block = Some(block_hash);
	}

	pub async fn finalized_block(&mut self) {
		let block_hash = rpc::fetch_finalized_block_hash(&self.client).await.unwrap();
		{
			let db = self.db.lock().unwrap();
			if Some(block_hash) == db.last_finalized_block {
				return;
			}
		}

		let block = rpc::fetch_block(&self.client, Some(block_hash))
			.await
			.unwrap();

		let mut db = self.db.lock().unwrap();
		for hex_tx in block.block.extrinsics {
			let hex_tx = &hex_tx[2..];
			let encoded_tx: Vec<u8> = hex::decode(hex_tx).unwrap();
			let tx_hash = H256(blake2_256(&encoded_tx));
			db.value.insert(tx_hash, true);
		}
		db.last_best_block = Some(block_hash);
	}

	pub async fn wait_block_inclusion(&self, tx_hash: &H256) -> bool {
		let mut total_wait_time = 0;

		while total_wait_time < 120 {
			{
				let db = self.db.lock().unwrap();
				if db.value.contains_key(tx_hash) {
					return true;
				}
			}
			tokio::time::sleep(Duration::from_secs(SLEEP_DURATION)).await;
			total_wait_time += 1;
		}

		false
	}

	pub async fn wait_finalization_inclusion(&self, tx_hash: &H256) -> bool {
		let mut total_wait_time = 0;

		while total_wait_time < 120 {
			{
				let db = self.db.lock().unwrap();
				if db.value.get(tx_hash).cloned() == Some(true) {
					return true;
				}
			}
			tokio::time::sleep(Duration::from_secs(SLEEP_DURATION)).await;
			total_wait_time += 1;
		}

		false
	}
}

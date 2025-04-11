use super::super::runtime_api;
use super::worker_logger::Logger;
use super::{worker, Sender};
use crate::service::FullClient;
use crate::transaction_rpc_worker::macros::profile;
use avail_core::OpaqueExtrinsic;
use frame_system_rpc_runtime_api::{
	events::event_id::system, SystemFetchEventsParams, SystemFetchEventsResult,
};
use jsonrpsee::tokio::{sync::Notify, time::sleep};
use sc_service::RpcHandlers;
use sp_core::H256;
use std::sync::Arc;
use std::time::{Duration, Instant};

const SLEEP_ON_ERROR: u64 = 2500; // ms
const SLEEP_ON_FETCH: u64 = 1000; // ms

pub struct IncludedWorker {
	pub rpc_handlers: RpcHandlers,
	pub client: Arc<FullClient>,
	pub sender: Sender,
	pub logger: Logger,
	pub notifier: Arc<Notify>,
}

impl IncludedWorker {
	pub async fn run(mut self) {
		let (duration, _) = profile!(worker::wait_for_sync(&self.rpc_handlers).await);
		self.logger.log_sync_time(duration);

		let mut block_hash = H256::default();
		loop {
			let block = self.fetch_next_block(&mut block_hash).await;
			let block = worker::prepare_block(block.0, block.1, block.2, block.3, false).await;

			if let Err(e) = self.sender.send(block).await {
				self.logger.log_error(e.to_string());
				return;
			}

			self.notifier.notify_one();

			self.logger.log_stats();
		}
	}

	async fn fetch_next_block(
		&mut self,
		current_block_hash: &mut H256,
	) -> (Vec<OpaqueExtrinsic>, H256, u32, SystemFetchEventsResult) {
		loop {
			let chain_info = self.client.chain_info();
			let (block_hash, block_height) = (chain_info.best_hash, chain_info.best_number);

			if (*current_block_hash).eq(&block_hash) {
				sleep(Duration::from_millis(SLEEP_ON_FETCH)).await;
				continue;
			}

			let now = Instant::now();
			let mut params = SystemFetchEventsParams::default();
			params.filter_events = Some(vec![
				(system::PALLET_ID, system::EXTRINSIC_SUCCESS),
				(system::PALLET_ID, system::EXTRINSIC_FAILED),
			]);
			let Some(events) =
				runtime_api::system_fetch_events(&self.rpc_handlers, params, &block_hash).await
			else {
				sleep(Duration::from_millis(SLEEP_ON_ERROR)).await;
				continue;
			};

			if events.error != 0 {
				sleep(Duration::from_millis(SLEEP_ON_ERROR)).await;
				continue;
			}

			let Ok(Some(extrinsics)) = self.client.body(block_hash) else {
				sleep(Duration::from_millis(SLEEP_ON_ERROR)).await;
				continue;
			};

			self.logger.add_block_fetch(now.elapsed());

			*current_block_hash = block_hash.clone();
			return (extrinsics, block_hash, block_height, events);
		}
	}
}

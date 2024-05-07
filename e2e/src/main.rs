fn main() {}

#[cfg(test)]
mod tests {
	use std::sync::{atomic::AtomicU64, OnceLock};

	use anyhow::{anyhow, Result};
	use async_std::{
		sync::{Condvar, Mutex},
		task::block_on,
	};
	use avail_core::currency::Balance;
	use avail_subxt::{api, AccountId, AvailClient, AvailConfig};
	use subxt::{tx::Signer, OnlineClient};

	/// Returns an Avail Client on local connection.
	pub async fn local_connection() -> Result<AvailClient> {
		let ws = String::from("ws://127.0.0.1:9944");
		AvailClient::new(ws)
			.await
			.map_err(|e| anyhow!("Client cannot be connected: {e:?}"))
	}

	/// Returns the free balance of `signer`
	pub async fn free_balance_of<S>(
		client: &OnlineClient<AvailConfig>,
		signer: &S,
	) -> Result<Balance>
	where
		S: Signer<AvailConfig>,
	{
		let acc: AccountId = signer.account_id();
		let query = api::storage().system().account(acc.clone());
		let acc_info = client
			.storage()
			.at_latest()
			.await?
			.fetch(&query)
			.await?
			.ok_or_else(|| anyhow!("Missing account({acc:?}) info"))?;

		Ok(acc_info.data.free)
	}

	pub static ALICE_NONCE: AtomicU64 = AtomicU64::new(0);

	#[derive(Default)]
	struct ConcurrentController {
		running_tasks: Mutex<usize>,
		cvar: Condvar,
	}

	struct ConcurrentGuard<'a> {
		controller: &'a ConcurrentController,
	}

	impl<'a> ConcurrentGuard<'a> {
		pub async fn new(controller: &'a ConcurrentController) -> Self {
			controller.inc_running_tasks().await;
			Self { controller }
		}
	}

	impl<'a> Drop for ConcurrentGuard<'a> {
		fn drop(&mut self) {
			let controller = self.controller;
			block_on(async { controller.dec_running_tasks().await });
		}
	}

	impl ConcurrentController {
		pub async fn inc_running_tasks(&self) {
			let mut count = self.running_tasks.lock().await;
			*count += 1;
		}

		pub async fn dec_running_tasks(&self) {
			let mut count = self.running_tasks.lock().await;
			*count -= 1;
			self.cvar.notify_one();
		}

		pub async fn allow_concurrency(&self) -> ConcurrentGuard {
			ConcurrentGuard::new(self).await
		}

		pub async fn no_concurrency(&self) -> ConcurrentGuard {
			let mut count = self.running_tasks.lock().await;
			while *count > 1 {
				count = self.cvar.wait(count).await;
			}
			ConcurrentGuard::new(self).await
		}
	}

	fn concurrent_controller() -> &'static ConcurrentController {
		static CTC: OnceLock<ConcurrentController> = OnceLock::new();
		CTC.get_or_init(ConcurrentController::default)
	}

	mod accounts_from_mnemonics;
	mod create_app_key;
	mod download_digest_items;
	mod headers;
	mod max_block_submit;
	mod query_proof;
	mod rpc_queries;
	mod submit_block_length_proposal;
	mod submit_data;
	mod vector_send_msg;
}

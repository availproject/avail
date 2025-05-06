mod max_block_submit;

use avail_rust::prelude::*;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), ClientError> {
	max_block_submit::run().await?;

	Ok(())
}

pub async fn wait_for_new_block(sdk: &SDK) -> Result<(), ClientError> {
	let current_block = sdk.client.best_block_number().await?;
	loop {
		let new_block = sdk.client.best_block_number().await?;
		if current_block != new_block {
			break Ok(());
		}

		tokio::time::sleep(Duration::from_secs(1)).await;
	}
}
/*
#[cfg(test)]
mod tests {

	use avail_core::currency::Balance;
	use avail_subxt::{api, AccountId, AvailClient, AvailConfig};
	use sp_core::H256;
	use subxt::{blocks::BlockRef, tx::Signer, OnlineClient};
	use subxt_signer::sr25519::dev;

	use anyhow::{anyhow, Result};
	use std::sync::{atomic::AtomicU64, Arc, OnceLock};
	use tokio::sync::{OnceCell, OwnedSemaphorePermit, Semaphore};
	use tracing::trace;

	/// Returns an Avail Client on local connection.
	pub async fn local_connection() -> Result<AvailClient> {
		let ws = String::from("ws://127.0.0.1:9944");
		AvailClient::new(ws)
			.await
			.map_err(|e| anyhow!("Client cannot be connected: {e:?}"))
	}

	/// Returns the free balance of `signer` at given block.
	/// If `maybe_block == None` then the latest one will be used.
	pub async fn free_balance_of<S>(
		client: &OnlineClient<AvailConfig>,
		signer: &S,
		maybe_block: Option<BlockRef<H256>>,
	) -> Result<Balance>
	where
		S: Signer<AvailConfig>,
	{
		let acc: AccountId = signer.account_id();
		let query = api::storage().system().account(acc.clone());

		let storage = match maybe_block {
			Some(block) => client.storage().at(block),
			None => client.storage().at_latest().await?,
		};
		let acc_info = storage
			.fetch(&query)
			.await?
			.ok_or_else(|| anyhow!("Missing account({acc:?}) info"))?;

		Ok(acc_info.data.free)
	}

	pub const MAX_PERMITS: usize = 16;

	/// It allows the test to run tests concurrently.
	///
	/// It requires that user manages the alice's nonce manually (using `fn alice_nonce()` helper
	/// for instance).
	/// The CTC allows up to `MAX_PERMITS` concurrent tasks .
	pub async fn allow_concurrency(name: &str) -> OwnedSemaphorePermit {
		let ctc = concurrent_controller();
		let available_permits = ctc.available_permits();
		let permit = ctc.clone().acquire_owned().await.unwrap();
		trace!(target: "CTC", "Acquired single permit on `{name}`, it was {available_permits} available permits");
		permit
	}

	/// It forces the test to run alone in the CTC.
	///
	/// It is used when you want to check your test in the block context, in the sense that no
	/// other tests are going to send their transactions, producing side effect on the results.
	/// An example of this use case are any `rpc_query` test, where you want to control the exact
	/// data you submit during the block, in order to check the `data_proof` or `query_proof` in
	/// that block.
	pub async fn no_concurrency(name: &str) -> OwnedSemaphorePermit {
		let ctc = concurrent_controller();
		let available_permits = ctc.available_permits();
		let permit = ctc
			.clone()
			.acquire_many_owned(MAX_PERMITS as u32)
			.await
			.unwrap();
		trace!(target: "CTC", "Acquired all permits on `{name}`, it was {available_permits} available permits");

		permit
	}

	/// Global Concurrency controller.
	fn concurrent_controller() -> Arc<Semaphore> {
		static CTC: OnceLock<Arc<Semaphore>> = OnceLock::new();
		Arc::clone(CTC.get_or_init(|| Arc::new(Semaphore::const_new(MAX_PERMITS))))
	}

	/// It initializes the alice's nonce just once, making a request to local client.
	/// From that point, the nonce should be increased locally to track parallel requests.
	///
	/// It facilitates the re-execution of test using the same local node (without restart) and
	/// it also helps to track the nonce when several tests are running concurrently.
	pub async fn alice_nonce() -> &'static AtomicU64 {
		static ALICE_NONCE: OnceCell<AtomicU64> = OnceCell::const_new();
		ALICE_NONCE
			.get_or_init(|| async {
				trace!(target: "CTC", "Initializing Alice Nonce");
				let client = local_connection().await.unwrap();
				let alice = dev::alice();
				let nonce = avail_subxt::tx::nonce(&client, &alice).await.unwrap();
				trace!(target: "CTC", "Initialized Alice Nonce with {nonce}");
				AtomicU64::new(nonce)
			})
			.await
	}

	mod accounts_from_mnemonics;
	mod create_app_key;
	mod download_digest_items;
	mod headers;
	mod max_block_submit;
	mod max_send_message;
	mod query_proof;
	mod retrieve_data_hash;
	mod retrieve_data_subscription;
	mod rpc_queries;
	mod submit_block_length_proposal;
	mod submit_data;
	mod vector_send_msg;
}
 */

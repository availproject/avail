
fn main() {}

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

	/// Returns the free balance of `signer`
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

	pub async fn allow_concurrency(name: &str) -> OwnedSemaphorePermit {
		let ctc = concurrent_controller();
		let permit = ctc.clone().acquire_owned().await.unwrap();
		let available_permits = ctc.available_permits();
		trace!(target: "CTC", "Acquired single permit on `{name}`, available {available_permits}");
		permit
	}

	pub async fn no_concurrency(name: &str) -> OwnedSemaphorePermit {
		let ctc = concurrent_controller();
		tokio::task::yield_now().await;
		let permit = ctc
			.clone()
			.acquire_many_owned(MAX_PERMITS as u32)
			.await
			.unwrap();
		let available_permits = ctc.available_permits();
		trace!(target: "CTC", "Acquired all permits on `{name}`, available {available_permits}");

		permit
	}

	fn concurrent_controller() -> Arc<Semaphore> {
		static CTC: OnceLock<Arc<Semaphore>> = OnceLock::new();
		Arc::clone(CTC.get_or_init(|| Arc::new(Semaphore::const_new(MAX_PERMITS))))
	}

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
	mod query_proof;
	mod rpc_queries;
	// mod submit_block_length_proposal;
	mod submit_data;
	mod vector_send_msg;
}

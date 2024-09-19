use crate::{block_watcher::BlockWatcher, error::ClientError, rpc};

use super::params::{Extra, Mortality, Nonce};
use jsonrpsee_http_client::HttpClient as JRPSHttpClient;
use parity_scale_codec::Compact;
use sdk_core::{
	crypto::{AccountId, Signature},
	types::{
		self, avail::RuntimeVersion, Additional, Call, Era, OpaqueTransaction,
		UnsignedEncodedPayload, UnsignedPayload, H256,
	},
};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Client {
	pub client: Arc<JRPSHttpClient>,
}

impl Client {
	pub fn new(endpoint: &str) -> Result<Self, ClientError> {
		let client = JRPSHttpClient::builder().build(endpoint);
		let client = client.map_err(|e| ClientError::Jsonrpsee(e))?;

		Ok(Self {
			client: Arc::new(client),
		})
	}

	pub async fn build_payload(
		&self,
		call: Call,
		account_id: AccountId,
		extra: Extra,
	) -> Result<UnsignedEncodedPayload, ClientError> {
		let (nonce, mortality, tip, app_id) = extra.deconstruct();

		let app_id = Compact(app_id.unwrap_or(0u32));
		let tip = Compact(tip.unwrap_or(0u128));
		let nonce = self.check_nonce(nonce, &account_id).await?;
		let (mortality, fork_hash) = self.check_mortality(mortality).await?;

		let extra = types::Extra {
			mortality,
			nonce,
			tip,
			app_id,
		};

		let RuntimeVersion {
			spec_version,
			transaction_version,
			..
		} = rpc::state_get_runtime_version(&self.client).await?;
		let genesis_hash = rpc::chain_spec_v1_genesis_hash(&self.client).await?;

		let additional =
			Additional::new(spec_version, transaction_version, genesis_hash, fork_hash);

		Ok(UnsignedPayload::new(call, extra, additional).encode())
	}

	pub fn sign(
		&self,
		payload: &UnsignedEncodedPayload,
		account_id: AccountId,
		signature: Signature,
	) -> OpaqueTransaction {
		OpaqueTransaction::new(&payload.extra, &payload.call, account_id, signature)
	}

	pub async fn submit_transaction(
		&self,
		transaction: OpaqueTransaction,
	) -> Result<H256, ClientError> {
		rpc::author_submit_extrinsic(&self.client, transaction).await
	}

	async fn check_nonce(
		&self,
		nonce: Option<Nonce>,
		account_id: &AccountId,
	) -> Result<Compact<u32>, ClientError> {
		let nonce = match nonce {
			Some(Nonce::BestBlockAndTxPool) | None => {
				rpc::system_account_next_index(&self.client, &account_id).await?
			},
			Some(Nonce::BestBlock) => {
				let block_hash = rpc::fetch_best_block_hash(&self.client).await?;
				rpc::account_nonce_api_account_nonce(&self.client, &account_id, block_hash).await?
			},
			Some(Nonce::FinalizedBlock) => {
				let block_hash = rpc::fetch_finalized_block_hash(&self.client).await?;
				rpc::account_nonce_api_account_nonce(&self.client, &account_id, block_hash).await?
			},
			Some(Nonce::Custom(n)) => n,
		};

		Ok(Compact(nonce))
	}

	async fn check_mortality(
		&self,
		mortality: Option<Mortality>,
	) -> Result<(Era, H256), ClientError> {
		let (era, fork_hash) = match mortality {
			Some(x) => match x {
				Mortality::Period(period) => {
					let hash = rpc::fetch_best_block_hash(&self.client).await?;
					let header = rpc::fetch_block_header(&self.client, Some(hash)).await?;
					let number = header.number;
					(Era::mortal(period, number as u64), hash)
				},
				Mortality::Custom((period, best_number, block_hash)) => {
					(Era::mortal(period, best_number as u64), block_hash)
				},
			},
			None => {
				let hash = rpc::fetch_best_block_hash(&self.client).await?;
				let header = rpc::fetch_block_header(&self.client, Some(hash)).await?;
				let number = header.number;
				(Era::mortal(32, number as u64), hash)
			},
		};

		Ok((era, fork_hash))
	}
}

pub struct SubmittedTransaction {
	pub tx_hash: H256,
	block_watcher: BlockWatcher,
}

impl SubmittedTransaction {
	pub fn new(tx_hash: H256, block_watcher: BlockWatcher) -> Self {
		Self {
			tx_hash,
			block_watcher,
		}
	}

	pub async fn wait_for_block_inclusion(&self) {
		self.block_watcher.wait_block_inclusion(&self.tx_hash).await;
	}

	pub async fn wait_for_block_finalization(&self) {
		self.block_watcher
			.wait_finalization_inclusion(&self.tx_hash)
			.await;
	}
}

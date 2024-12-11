use subxt::backend::rpc::reconnecting_rpc_client::RpcClient;

use crate::error::ClientError;
use crate::rpcs::{account_next_index, get_block_hash, get_header};
use crate::Block;
use crate::{AOnlineClient, AccountId, AvailExtrinsicParamsBuilder, BlockHash};

use super::Params;

#[derive(Debug, Clone, Copy)]
pub struct Options {
	app_id: Option<u32>,
	mortality: Option<Mortality>,
	nonce: Option<Nonce>,
	tip: Option<u128>,
}

impl Options {
	pub fn new() -> Self {
		Self {
			app_id: None,
			mortality: None,
			nonce: None,
			tip: None,
		}
	}

	pub fn app_id(mut self, value: u32) -> Self {
		self.app_id = Some(value);
		self
	}

	pub fn mortality(mut self, value: Mortality) -> Self {
		self.mortality = Some(value);
		self
	}

	pub fn nonce(mut self, value: Nonce) -> Self {
		self.nonce = Some(value);
		self
	}

	pub fn tip(mut self, value: u128) -> Self {
		self.tip = Some(value);
		self
	}

	pub async fn build(
		self,
		online_client: &AOnlineClient,
		rpc_client: &RpcClient,
		account: &AccountId,
	) -> Result<PopulatedOptions, ClientError> {
		let app_id = self.app_id.unwrap_or_default();
		let tip = self.tip.unwrap_or_default();
		let nonce = self.nonce.unwrap_or(Nonce::FinalizedBlock);
		let nonce = parse_nonce(online_client, rpc_client, nonce, account).await?;
		let mortality = self.mortality.unwrap_or_else(|| Mortality {
			period: 32,
			block_hash: None,
		});

		Ok(PopulatedOptions {
			app_id,
			mortality,
			nonce,
			tip,
		})
	}
}

impl Default for Options {
	fn default() -> Self {
		Self::new()
	}
}

#[derive(Debug, Clone, Copy)]
pub struct PopulatedOptions {
	pub app_id: u32,
	pub mortality: Mortality,
	pub nonce: u64,
	pub tip: u128,
}

impl PopulatedOptions {
	pub async fn build(self, client: &RpcClient) -> Result<Params, ClientError> {
		let mut builder = AvailExtrinsicParamsBuilder::new();
		builder = builder.app_id(self.app_id);
		builder = builder.tip(self.tip);
		builder = builder.nonce(self.nonce);

		let header = get_header(client, self.mortality.block_hash).await?;
		builder = builder.mortal(&header, self.mortality.period);

		Ok(builder.build())
	}
}

#[derive(Debug, Clone, Copy)]
pub struct Mortality {
	pub period: u64,
	pub block_hash: Option<BlockHash>,
}
impl Mortality {
	pub fn new(period: u64, block_hash: Option<BlockHash>) -> Self {
		Self { period, block_hash }
	}
}

#[derive(Debug, Clone, Copy)]
pub enum Nonce {
	BestBlock,
	FinalizedBlock,
	BestBlockAndTxPool,
	Custom(u32),
}

pub async fn parse_nonce(
	online_client: &AOnlineClient,
	rpc_client: &RpcClient,
	nonce: Nonce,
	account: &AccountId,
) -> Result<u64, ClientError> {
	let nonce = match nonce {
		Nonce::BestBlock => {
			let hash = get_block_hash(rpc_client, None).await?;
			let block = online_client.blocks().at(hash).await?;
			block.account_nonce(&account).await?
		},
		Nonce::FinalizedBlock => {
			let hash = Block::fetch_finalized_block_hash(rpc_client).await?;
			let block = online_client.blocks().at(hash).await?;
			block.account_nonce(&account).await?
		},
		Nonce::BestBlockAndTxPool => {
			account_next_index(rpc_client, account.to_string()).await? as u64
		},
		Nonce::Custom(x) => x as u64,
	};

	Ok(nonce)
}

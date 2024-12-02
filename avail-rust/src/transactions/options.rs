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

	pub async fn parse(
		self,
		online_client: &AOnlineClient,
		rpc_client: &RpcClient,
		account: &AccountId,
	) -> Result<Params, ClientError> {
		if self.app_id.is_none()
			&& self.mortality.is_none()
			&& self.nonce.is_none()
			&& self.tip.is_none()
		{
			parse_options(online_client, rpc_client, account, None).await
		} else {
			parse_options(online_client, rpc_client, account, Some(self)).await
		}
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

pub async fn parse_options(
	online_client: &AOnlineClient,
	rpc_client: &RpcClient,
	account: &AccountId,
	options: Option<Options>,
) -> Result<Params, ClientError> {
	let Some(options) = options else {
		return Ok(AvailExtrinsicParamsBuilder::new().build());
	};

	let mut builder = AvailExtrinsicParamsBuilder::new();
	builder = builder.app_id(options.app_id.unwrap_or_default());
	builder = builder.tip(options.tip.unwrap_or_default());

	let mortality = options.mortality.unwrap_or_else(|| Mortality {
		period: 32,
		block_hash: None,
	});
	let header = get_header(rpc_client, mortality.block_hash).await?;
	builder = builder.mortal(&header, mortality.period);

	if let Some(nonce) = options.nonce {
		builder = match nonce {
			Nonce::BestBlock => {
				let hash = get_block_hash(rpc_client, None).await?;
				let block = online_client.blocks().at(hash).await?;
				let nonce = block.account_nonce(&account).await?;
				builder.nonce(nonce)
			},
			Nonce::FinalizedBlock => {
				let hash = Block::fetch_finalized_block_hash(rpc_client).await?;
				let block = online_client.blocks().at(hash).await?;
				let nonce = block.account_nonce(&account).await?;
				builder.nonce(nonce)
			},
			Nonce::BestBlockAndTxPool => {
				let nonce = account_next_index(rpc_client, account.to_string()).await?;
				builder.nonce(nonce as u64)
			},
			Nonce::Custom(x) => builder.nonce(x as u64),
		};
	}

	Ok(builder.build())
}

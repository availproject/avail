use subxt::backend::rpc::RpcClient;

use crate::rpcs::{account_next_index, get_block_hash, get_finalized_head, get_header};
use crate::{ABlocksClient, AccountId, AvailExtrinsicParamsBuilder, BlockHash};

use super::Params;

#[derive(Debug, Clone, Copy)]
pub struct Options {
	pub app_id: Option<u32>,
	pub mortality: Option<Mortality>,
	pub nonce: Option<Nonce>,
	pub tip: Option<u128>,
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

pub async fn from_options_to_params(
	rpc_client: &RpcClient,
	blocks_client: &ABlocksClient,
	account: &AccountId,
	options: Option<Options>,
) -> Result<Params, String> {
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
	let header = get_header(rpc_client, mortality.block_hash);
	let header = header.await.map_err(|e| e.to_string())?;
	builder = builder.mortal(&header, mortality.period);

	if let Some(nonce) = options.nonce {
		builder = match nonce {
			Nonce::BestBlock => {
				let hash = get_block_hash(rpc_client, None).await.unwrap();
				let block = blocks_client.at(hash).await.map_err(|e| e.to_string())?;
				let nonce = block
					.account_nonce(&account)
					.await
					.map_err(|e| e.to_string())?;
				builder.nonce(nonce)
			},
			Nonce::FinalizedBlock => {
				let hash = get_finalized_head(rpc_client).await.unwrap();
				let block = blocks_client.at(hash).await.map_err(|e| e.to_string())?;
				let nonce = block
					.account_nonce(&account)
					.await
					.map_err(|e| e.to_string())?;
				builder.nonce(nonce)
			},
			Nonce::BestBlockAndTxPool => {
				let nonce = account_next_index(rpc_client, account.to_string())
					.await
					.map_err(|e| e.to_string())?;
				builder.nonce(nonce as u64)
			},
			Nonce::Custom(x) => builder.nonce(x as u64),
		};
	}

	Ok(builder.build())
}

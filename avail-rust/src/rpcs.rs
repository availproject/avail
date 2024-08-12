use avail_core::data_proof::ProofResponse;
use subxt::rpc_params;

use crate::avail::runtime_types::frame_system::limits::BlockLength;
use crate::subxt::backend::legacy::rpc_methods::Bytes;
use crate::subxt::backend::rpc::RpcClient;
use crate::{AvailBlockDetailsRPC, AvailHeader, BlockHash, BlockNumber, Cell, GDataProof, GRow};

pub struct Rpc {
	pub kate: Kate,
	pub author: Author,
	pub chain: Chain,
}

impl Rpc {
	pub async fn new(endpoint: &str) -> Result<Self, Box<dyn std::error::Error>> {
		let client = RpcClient::from_insecure_url(endpoint).await?;
		let kate = Kate::new(client.clone());
		let author = Author::new(client.clone());
		let chain = Chain::new(client.clone());

		Ok(Self {
			kate,
			author,
			chain,
		})
	}
}

pub struct Chain {
	client: RpcClient,
}

impl Chain {
	pub fn new(client: RpcClient) -> Self {
		Self { client }
	}

	pub async fn get_block(
		&self,
		hash: Option<BlockHash>,
	) -> Result<AvailBlockDetailsRPC, subxt::Error> {
		let value: AvailBlockDetailsRPC = self
			.client
			.request("chain_getBlock", rpc_params![hash])
			.await?;

		Ok(value)
	}

	pub async fn get_finalized_head(&self) -> Result<BlockHash, subxt::Error> {
		let value: BlockHash = self
			.client
			.request("chain_getFinalizedHead", rpc_params![])
			.await?;

		Ok(value)
	}

	pub async fn get_block_hash(
		&self,
		block_number: Option<BlockNumber>,
	) -> Result<BlockHash, subxt::Error> {
		let value: BlockHash = self
			.client
			.request("chain_getBlockHash", rpc_params![block_number])
			.await?;
		Ok(value)
	}

	pub async fn get_header(&self, hash: Option<BlockHash>) -> Result<AvailHeader, subxt::Error> {
		let value: AvailHeader = self
			.client
			.request("chain_getHeader", rpc_params![hash])
			.await?;
		Ok(value)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::str::FromStr;

	#[tokio::test]
	async fn testing_function() {
		let sdk = crate::sdk::SDK::new("ws://127.0.0.1:9944").await.unwrap();
		let h = BlockHash::from_str(
			"0x845f7c5ab3b6cc0639548feb1c9c8919eef3bae1c2f24bfc6d1dc894c5b23ee9",
		);
		match sdk.rpc.chain.get_block(h.ok()).await {
			Ok(a) => {
				dbg!(a);
			},
			Err(a) => {
				dbg!(a);
			},
		};
	}
}

pub struct Author {
	client: RpcClient,
}

impl Author {
	pub fn new(client: RpcClient) -> Self {
		Self { client }
	}

	pub async fn rotate_keys(&self) -> Result<Vec<u8>, subxt::Error> {
		let bytes: Bytes = self
			.client
			.request("author_rotateKeys", rpc_params![])
			.await?;
		Ok(bytes.0)
	}
}

pub struct Kate {
	client: RpcClient,
}

impl Kate {
	pub fn new(client: RpcClient) -> Self {
		Self { client }
	}

	pub async fn block_length(&self, at: Option<BlockHash>) -> Result<BlockLength, subxt::Error> {
		let result: BlockLength = self
			.client
			.request("kate_blockLength", rpc_params![at])
			.await?;
		Ok(result)
	}

	pub async fn query_data_proof(
		&self,
		transaction_index: u32,
		at: Option<BlockHash>,
	) -> Result<ProofResponse, subxt::Error> {
		let result: ProofResponse = self
			.client
			.request("kate_queryDataProof", rpc_params![transaction_index, at])
			.await?;
		Ok(result)
	}

	pub async fn query_proof(
		&self,
		cells: Vec<Cell>,
		at: Option<BlockHash>,
	) -> Result<Vec<GDataProof>, subxt::Error> {
		let result: Vec<GDataProof> = self
			.client
			.request("kate_queryProof", rpc_params![cells, at])
			.await?;
		Ok(result)
	}

	pub async fn query_rows(
		&self,
		rows: Vec<u32>,
		at: Option<BlockHash>,
	) -> Result<Vec<GRow>, subxt::Error> {
		let result: Vec<GRow> = self
			.client
			.request("kate_queryRows", rpc_params![rows, at])
			.await?;
		Ok(result)
	}
}

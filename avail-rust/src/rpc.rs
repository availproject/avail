use avail_core::data_proof::ProofResponse;
use subxt::rpc_params;
use subxt::utils::H256;

use crate::avail::runtime_types::frame_system::limits::BlockLength;
use crate::subxt::backend::legacy::rpc_methods::Bytes;
use crate::{subxt::backend::rpc::RpcClient, Api};
use crate::{Cell, GDataProof, GRow};

pub struct Rpc {
	pub kate: Kate,
	pub author: Author,
}

impl Rpc {
	pub async fn new(api: Api, endpoint: &str) -> Result<Self, Box<dyn std::error::Error>> {
		let client = RpcClient::from_insecure_url(endpoint).await?;
		let kate = Kate::new(client.clone());
		let author = Author::new(client.clone());

		Ok(Self { kate, author })
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

	pub async fn block_length(&self, at: Option<H256>) -> Result<BlockLength, subxt::Error> {
		let result: BlockLength = self
			.client
			.request("kate_blockLength", rpc_params![at])
			.await?;
		Ok(result)
	}

	pub async fn query_data_proof(
		&self,
		transaction_index: u32,
		at: Option<H256>,
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
		at: Option<H256>,
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
		at: Option<H256>,
	) -> Result<Vec<GRow>, subxt::Error> {
		let result: Vec<GRow> = self
			.client
			.request("kate_queryRows", rpc_params![rows, at])
			.await?;
		Ok(result)
	}
}

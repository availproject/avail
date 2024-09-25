use avail_core::data_proof::ProofResponse;
use subxt::backend::legacy::LegacyRpcMethods;

use crate::avail::runtime_types::frame_system::limits::BlockLength;
use crate::from_substrate::{FeeDetails, NodeRole, PeerInfo, RuntimeDispatchInfo, SyncState};
use crate::{
	AvailBlockDetailsRPC, AvailConfig, AvailHeader, BlockHash, BlockNumber, Cell, GDataProof, GRow,
};
use subxt::backend::legacy::rpc_methods::{Bytes, SystemHealth};
use subxt::backend::rpc::RpcClient;
use subxt::rpc_params;

/// Arbitrary properties defined in chain spec as a JSON object
pub type Properties = serde_json::map::Map<String, serde_json::Value>;

#[derive(Clone)]
pub struct Rpc {
	pub client: RpcClient,
	pub legacy_methods: LegacyRpcMethods<AvailConfig>,
	pub kate: Kate,
	pub author: Author,
	pub chain: Chain,
	pub system: System,
	pub payment: Payment,
}

impl Rpc {
	pub async fn new(endpoint: &str, secure: bool) -> Result<Self, Box<dyn std::error::Error>> {
		let client: RpcClient = match secure {
			true => RpcClient::from_url(endpoint).await?,
			false => RpcClient::from_insecure_url(endpoint).await?,
		};
		let legacy_methods = LegacyRpcMethods::new(client.clone());
		let kate = Kate::new(client.clone());
		let author = Author::new(client.clone());
		let chain: Chain = Chain::new(client.clone());
		let system = System::new(client.clone());
		let payment = Payment::new(client.clone());

		Ok(Self {
			client,
			legacy_methods,
			kate,
			author,
			chain,
			system,
			payment,
		})
	}
}

#[derive(Clone)]
pub struct Payment {
	client: RpcClient,
}

impl Payment {
	pub fn new(client: RpcClient) -> Self {
		Self { client }
	}

	pub async fn query_fee_details(
		&self,
		extrinsic: Bytes,
		at: Option<BlockHash>,
	) -> Result<FeeDetails, subxt::Error> {
		let value: FeeDetails = self
			.client
			.request("payment_queryFeeDetails", rpc_params![extrinsic, at])
			.await?;
		Ok(value)
	}

	pub async fn query_info(
		&self,
		extrinsic: Bytes,
		at: Option<BlockHash>,
	) -> Result<RuntimeDispatchInfo, subxt::Error> {
		let value: RuntimeDispatchInfo = self
			.client
			.request("payment_queryInfo", rpc_params![extrinsic, at])
			.await?;
		Ok(value)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::Keypair;
	use crate::SecretUri;
	use std::str::FromStr;

	#[tokio::test]
	async fn testing_function() {
		let secret_uri = SecretUri::from_str("//Alice").unwrap();
		let account = Keypair::from_uri(&secret_uri).unwrap();

		let sdk = crate::sdk::SDK::new("ws://127.0.0.1:9944").await.unwrap();
		let keys = sdk.rpc.author.rotate_keys().await.unwrap();

		let keys = sdk.util.deconstruct_session_keys(keys).unwrap();
		let b = sdk
			.tx
			.session
			.set_keys(keys, crate::WaitFor::BlockFinalization, &account, None)
			.await
			.unwrap();

		/* 		let h = BlockHash::from_str(
			"0x90c6d99b3fb9d608a5bee9eb59bb107d5fd11b0aa398f3b1132503c15db40551",
		)
		.unwrap();
		let block = sdk.rpc.chain.get_block(Some(h)).await.unwrap();
		//dbg!(&block.block.extrinsics);
		let info = sdk
			.rpc
			.payment
			.query_info(block.block.extrinsics[1].clone(), Some(h))
			.await;

		match info {
			Ok(a) => {
				dbg!(&a);
			},
			Err(a) => {
				dbg!(a);
				panic!();
			},
		}; */
	}
}

#[derive(Clone)]
pub struct System {
	client: RpcClient,
}

impl System {
	pub fn new(client: RpcClient) -> Self {
		Self { client }
	}

	pub async fn account_next_index(&self, account: String) -> Result<u32, subxt::Error> {
		let value: u32 = self
			.client
			.request("system_accountNextIndex", rpc_params![account])
			.await?;
		Ok(value)
	}

	pub async fn chain(&self) -> Result<String, subxt::Error> {
		let value: String = self.client.request("system_chain", rpc_params![]).await?;
		Ok(value)
	}

	pub async fn chain_type(&self) -> Result<String, subxt::Error> {
		let value: String = self
			.client
			.request("system_chainType", rpc_params![])
			.await?;
		Ok(value)
	}

	pub async fn health(&self) -> Result<SystemHealth, subxt::Error> {
		let value: SystemHealth = self.client.request("system_health", rpc_params![]).await?;
		Ok(value)
	}

	pub async fn local_listen_addresses(&self) -> Result<Vec<String>, subxt::Error> {
		let value: Vec<String> = self
			.client
			.request("system_localListenAddresses", rpc_params![])
			.await?;
		Ok(value)
	}

	pub async fn local_peer_id(&self) -> Result<String, subxt::Error> {
		let value: String = self
			.client
			.request("system_localPeerId", rpc_params![])
			.await?;
		Ok(value)
	}

	pub async fn name(&self) -> Result<String, subxt::Error> {
		let value: String = self.client.request("system_name", rpc_params![]).await?;
		Ok(value)
	}

	pub async fn node_roles(&self) -> Result<Vec<NodeRole>, subxt::Error> {
		let value: Vec<NodeRole> = self
			.client
			.request("system_nodeRoles", rpc_params![])
			.await?;
		Ok(value)
	}

	pub async fn peers(&self) -> Result<Vec<PeerInfo>, subxt::Error> {
		let value: Vec<PeerInfo> = self.client.request("system_peers", rpc_params![]).await?;
		Ok(value)
	}

	pub async fn properties(&self) -> Result<Properties, subxt::Error> {
		let value: Properties = self
			.client
			.request("system_properties", rpc_params![])
			.await?;
		Ok(value)
	}

	pub async fn sync_state(&self) -> Result<SyncState, subxt::Error> {
		let value: SyncState = self
			.client
			.request("system_syncState", rpc_params![])
			.await?;
		Ok(value)
	}

	pub async fn version(&self) -> Result<String, subxt::Error> {
		let value: String = self.client.request("system_version", rpc_params![]).await?;
		Ok(value)
	}
}

#[derive(Clone)]
pub struct Chain {
	client: RpcClient,
}

impl Chain {
	pub fn new(client: RpcClient) -> Self {
		Self { client }
	}

	pub async fn get_block(
		&self,
		at: Option<BlockHash>,
	) -> Result<AvailBlockDetailsRPC, subxt::Error> {
		let value: AvailBlockDetailsRPC = self
			.client
			.request("chain_getBlock", rpc_params![at])
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

	pub async fn get_finalized_head(&self) -> Result<BlockHash, subxt::Error> {
		let value: BlockHash = self
			.client
			.request("chain_getFinalizedHead", rpc_params![])
			.await?;
		Ok(value)
	}

	pub async fn get_header(&self, at: Option<BlockHash>) -> Result<AvailHeader, subxt::Error> {
		let value: AvailHeader = self
			.client
			.request("chain_getHeader", rpc_params![at])
			.await?;
		Ok(value)
	}
}

#[derive(Clone)]
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

#[derive(Clone)]
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

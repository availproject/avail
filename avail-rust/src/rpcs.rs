use avail_core::data_proof::ProofResponse;
use subxt::backend::legacy::LegacyRpcMethods;

use crate::avail::runtime_types::frame_system::limits::BlockLength;
use crate::from_substrate::{FeeDetails, NodeRole, PeerInfo, RuntimeDispatchInfo, SyncState};
use crate::{
	ABlockDetailsRPC, AvailConfig, AvailHeader, BlockHash, BlockNumber, Cell, GDataProof, GRow,
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
	pub async fn new(client: RpcClient) -> Self {
		let legacy_methods = LegacyRpcMethods::new(client.clone());
		let kate = Kate::new(client.clone());
		let author = Author::new(client.clone());
		let chain: Chain = Chain::new(client.clone());
		let system = System::new(client.clone());
		let payment = Payment::new(client.clone());

		Self {
			client,
			legacy_methods,
			kate,
			author,
			chain,
			system,
			payment,
		}
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
		query_fee_details(&self.client, extrinsic, at).await
	}

	pub async fn query_info(
		&self,
		extrinsic: Bytes,
		at: Option<BlockHash>,
	) -> Result<RuntimeDispatchInfo, subxt::Error> {
		query_info(&self.client, extrinsic, at).await
	}
}

pub async fn query_fee_details(
	client: &RpcClient,
	extrinsic: Bytes,
	at: Option<BlockHash>,
) -> Result<FeeDetails, subxt::Error> {
	let value: FeeDetails = client
		.request("payment_queryFeeDetails", rpc_params![extrinsic, at])
		.await?;
	Ok(value)
}

pub async fn query_info(
	client: &RpcClient,
	extrinsic: Bytes,
	at: Option<BlockHash>,
) -> Result<RuntimeDispatchInfo, subxt::Error> {
	let value: RuntimeDispatchInfo = client
		.request("payment_queryInfo", rpc_params![extrinsic, at])
		.await?;
	Ok(value)
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
		account_next_index(&self.client, account).await
	}

	pub async fn chain(&self) -> Result<String, subxt::Error> {
		chain(&self.client).await
	}

	pub async fn chain_type(&self) -> Result<String, subxt::Error> {
		chain_type(&self.client).await
	}

	pub async fn health(&self) -> Result<SystemHealth, subxt::Error> {
		health(&self.client).await
	}

	pub async fn local_listen_addresses(&self) -> Result<Vec<String>, subxt::Error> {
		local_listen_addresses(&self.client).await
	}

	pub async fn local_peer_id(&self) -> Result<String, subxt::Error> {
		local_peer_id(&self.client).await
	}

	pub async fn name(&self) -> Result<String, subxt::Error> {
		name(&self.client).await
	}

	pub async fn node_roles(&self) -> Result<Vec<NodeRole>, subxt::Error> {
		node_roles(&self.client).await
	}

	pub async fn peers(&self) -> Result<Vec<PeerInfo>, subxt::Error> {
		peers(&self.client).await
	}

	pub async fn properties(&self) -> Result<Properties, subxt::Error> {
		properties(&self.client).await
	}

	pub async fn sync_state(&self) -> Result<SyncState, subxt::Error> {
		sync_state(&self.client).await
	}

	pub async fn version(&self) -> Result<String, subxt::Error> {
		version(&self.client).await
	}
}

pub async fn account_next_index(client: &RpcClient, account: String) -> Result<u32, subxt::Error> {
	let value: u32 = client
		.request("system_accountNextIndex", rpc_params![account])
		.await?;
	Ok(value)
}

pub async fn chain(client: &RpcClient) -> Result<String, subxt::Error> {
	let value: String = client.request("system_chain", rpc_params![]).await?;
	Ok(value)
}

pub async fn chain_type(client: &RpcClient) -> Result<String, subxt::Error> {
	let value: String = client.request("system_chainType", rpc_params![]).await?;
	Ok(value)
}

pub async fn health(client: &RpcClient) -> Result<SystemHealth, subxt::Error> {
	let value: SystemHealth = client.request("system_health", rpc_params![]).await?;
	Ok(value)
}

pub async fn local_listen_addresses(client: &RpcClient) -> Result<Vec<String>, subxt::Error> {
	let value: Vec<String> = client
		.request("system_localListenAddresses", rpc_params![])
		.await?;
	Ok(value)
}

pub async fn local_peer_id(client: &RpcClient) -> Result<String, subxt::Error> {
	let value: String = client.request("system_localPeerId", rpc_params![]).await?;
	Ok(value)
}

pub async fn name(client: &RpcClient) -> Result<String, subxt::Error> {
	let value: String = client.request("system_name", rpc_params![]).await?;
	Ok(value)
}

pub async fn node_roles(client: &RpcClient) -> Result<Vec<NodeRole>, subxt::Error> {
	let value: Vec<NodeRole> = client.request("system_nodeRoles", rpc_params![]).await?;
	Ok(value)
}

pub async fn peers(client: &RpcClient) -> Result<Vec<PeerInfo>, subxt::Error> {
	let value: Vec<PeerInfo> = client.request("system_peers", rpc_params![]).await?;
	Ok(value)
}

pub async fn properties(client: &RpcClient) -> Result<Properties, subxt::Error> {
	let value: Properties = client.request("system_properties", rpc_params![]).await?;
	Ok(value)
}

pub async fn sync_state(client: &RpcClient) -> Result<SyncState, subxt::Error> {
	let value: SyncState = client.request("system_syncState", rpc_params![]).await?;
	Ok(value)
}

pub async fn version(client: &RpcClient) -> Result<String, subxt::Error> {
	let value: String = client.request("system_version", rpc_params![]).await?;
	Ok(value)
}

#[derive(Clone)]
pub struct Chain {
	client: RpcClient,
}

impl Chain {
	pub fn new(client: RpcClient) -> Self {
		Self { client }
	}

	pub async fn get_block(&self, at: Option<BlockHash>) -> Result<ABlockDetailsRPC, subxt::Error> {
		get_block(&self.client, at).await
	}

	pub async fn get_block_hash(
		&self,
		block_number: Option<BlockNumber>,
	) -> Result<BlockHash, subxt::Error> {
		get_block_hash(&self.client, block_number).await
	}

	pub async fn get_finalized_head(&self) -> Result<BlockHash, subxt::Error> {
		get_finalized_head(&self.client).await
	}

	pub async fn get_header(&self, at: Option<BlockHash>) -> Result<AvailHeader, subxt::Error> {
		get_header(&self.client, at).await
	}
}

pub async fn get_block(
	client: &RpcClient,
	at: Option<BlockHash>,
) -> Result<ABlockDetailsRPC, subxt::Error> {
	let value: ABlockDetailsRPC = client.request("chain_getBlock", rpc_params![at]).await?;
	Ok(value)
}

pub async fn get_best_block(client: &RpcClient) -> Result<ABlockDetailsRPC, subxt::Error> {
	get_block(client, None).await
}

pub async fn get_finalized_block(client: &RpcClient) -> Result<ABlockDetailsRPC, subxt::Error> {
	let hash = get_finalized_head(client).await?;
	get_block(client, Some(hash)).await
}

pub async fn get_block_hash(
	client: &RpcClient,
	block_number: Option<BlockNumber>,
) -> Result<BlockHash, subxt::Error> {
	let value: BlockHash = client
		.request("chain_getBlockHash", rpc_params![block_number])
		.await?;
	Ok(value)
}

pub async fn get_finalized_head(client: &RpcClient) -> Result<BlockHash, subxt::Error> {
	let value: BlockHash = client
		.request("chain_getFinalizedHead", rpc_params![])
		.await?;
	Ok(value)
}

pub async fn get_header(
	client: &RpcClient,
	at: Option<BlockHash>,
) -> Result<AvailHeader, subxt::Error> {
	let value: AvailHeader = client.request("chain_getHeader", rpc_params![at]).await?;
	Ok(value)
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
		rotate_keys(&self.client).await
	}
}

pub async fn rotate_keys(client: &RpcClient) -> Result<Vec<u8>, subxt::Error> {
	let bytes: Bytes = client.request("author_rotateKeys", rpc_params![]).await?;
	Ok(bytes.0)
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
		block_length(&self.client, at).await
	}

	pub async fn query_data_proof(
		&self,
		transaction_index: u32,
		at: Option<BlockHash>,
	) -> Result<ProofResponse, subxt::Error> {
		query_data_proof(&self.client, transaction_index, at).await
	}

	pub async fn query_proof(
		&self,
		cells: Vec<Cell>,
		at: Option<BlockHash>,
	) -> Result<Vec<GDataProof>, subxt::Error> {
		query_proof(&self.client, cells, at).await
	}

	pub async fn query_rows(
		&self,
		rows: Vec<u32>,
		at: Option<BlockHash>,
	) -> Result<Vec<GRow>, subxt::Error> {
		query_rows(&self.client, rows, at).await
	}
}

pub async fn block_length(
	client: &RpcClient,
	at: Option<BlockHash>,
) -> Result<BlockLength, subxt::Error> {
	let result: BlockLength = client.request("kate_blockLength", rpc_params![at]).await?;
	Ok(result)
}

pub async fn query_data_proof(
	client: &RpcClient,
	transaction_index: u32,
	at: Option<BlockHash>,
) -> Result<ProofResponse, subxt::Error> {
	let result: ProofResponse = client
		.request("kate_queryDataProof", rpc_params![transaction_index, at])
		.await?;
	Ok(result)
}

pub async fn query_proof(
	client: &RpcClient,
	cells: Vec<Cell>,
	at: Option<BlockHash>,
) -> Result<Vec<GDataProof>, subxt::Error> {
	let result: Vec<GDataProof> = client
		.request("kate_queryProof", rpc_params![cells, at])
		.await?;
	Ok(result)
}

pub async fn query_rows(
	client: &RpcClient,
	rows: Vec<u32>,
	at: Option<BlockHash>,
) -> Result<Vec<GRow>, subxt::Error> {
	let result: Vec<GRow> = client
		.request("kate_queryRows", rpc_params![rows, at])
		.await?;
	Ok(result)
}

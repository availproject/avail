use avail_core::data_proof::ProofResponse;

use crate::avail::runtime_types::frame_system::limits::BlockLength;
use crate::error::ClientError;
use crate::from_substrate::{FeeDetails, NodeRole, PeerInfo, RuntimeDispatchInfo, SyncState};
use crate::{ABlockDetailsRPC, AvailHeader, BlockHash, BlockNumber, Cell, GDataProof, GRow};
use subxt::backend::legacy::rpc_methods::{Bytes, SystemHealth};
use subxt::backend::rpc::reconnecting_rpc_client::RpcClient;
use subxt::rpc_params;

/// Arbitrary properties defined in chain spec as a JSON object
pub type Properties = serde_json::map::Map<String, serde_json::Value>;

#[derive(Clone)]
pub struct Rpc {
	pub client: RpcClient,
	pub kate: Kate,
	pub author: Author,
	pub chain: Chain,
	pub system: System,
	pub payment: Payment,
}

impl Rpc {
	pub async fn new(client: RpcClient) -> Self {
		let kate = Kate::new(client.clone());
		let author = Author::new(client.clone());
		let chain: Chain = Chain::new(client.clone());
		let system = System::new(client.clone());
		let payment = Payment::new(client.clone());

		Self {
			client,
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
	) -> Result<FeeDetails, ClientError> {
		query_fee_details(&self.client, extrinsic, at).await
	}

	pub async fn query_info(
		&self,
		extrinsic: Bytes,
		at: Option<BlockHash>,
	) -> Result<RuntimeDispatchInfo, ClientError> {
		query_info(&self.client, extrinsic, at).await
	}
}

pub async fn query_fee_details(
	client: &RpcClient,
	extrinsic: Bytes,
	at: Option<BlockHash>,
) -> Result<FeeDetails, ClientError> {
	let params = rpc_params![extrinsic, at].build();
	let value = client
		.request("payment_queryFeeDetails".into(), params)
		.await?;
	Ok(serde_json::from_str(value.get())?)
}

pub async fn query_info(
	client: &RpcClient,
	extrinsic: Bytes,
	at: Option<BlockHash>,
) -> Result<RuntimeDispatchInfo, ClientError> {
	let params = rpc_params![extrinsic, at].build();
	let value = client.request("payment_queryInfo".into(), params).await?;
	Ok(serde_json::from_str(value.get())?)
}

#[derive(Clone)]
pub struct System {
	client: RpcClient,
}

impl System {
	pub fn new(client: RpcClient) -> Self {
		Self { client }
	}

	pub async fn account_next_index(&self, account: String) -> Result<u32, ClientError> {
		account_next_index(&self.client, account).await
	}

	pub async fn chain(&self) -> Result<String, ClientError> {
		chain(&self.client).await
	}

	pub async fn chain_type(&self) -> Result<String, ClientError> {
		chain_type(&self.client).await
	}

	pub async fn health(&self) -> Result<SystemHealth, ClientError> {
		health(&self.client).await
	}

	pub async fn local_listen_addresses(&self) -> Result<Vec<String>, ClientError> {
		local_listen_addresses(&self.client).await
	}

	pub async fn local_peer_id(&self) -> Result<String, ClientError> {
		local_peer_id(&self.client).await
	}

	pub async fn name(&self) -> Result<String, ClientError> {
		name(&self.client).await
	}

	pub async fn node_roles(&self) -> Result<Vec<NodeRole>, ClientError> {
		node_roles(&self.client).await
	}

	pub async fn peers(&self) -> Result<Vec<PeerInfo>, ClientError> {
		peers(&self.client).await
	}

	pub async fn properties(&self) -> Result<Properties, ClientError> {
		properties(&self.client).await
	}

	pub async fn sync_state(&self) -> Result<SyncState, ClientError> {
		sync_state(&self.client).await
	}

	pub async fn version(&self) -> Result<String, ClientError> {
		version(&self.client).await
	}
}

pub async fn account_next_index(client: &RpcClient, account: String) -> Result<u32, ClientError> {
	let params = rpc_params![account].build();
	let value = client
		.request("system_accountNextIndex".into(), params)
		.await?;
	Ok(serde_json::from_str(value.get())?)
}

pub async fn chain(client: &RpcClient) -> Result<String, ClientError> {
	let params = rpc_params![].build();
	let value = client.request("system_chain".into(), params).await?;
	Ok(serde_json::from_str(value.get())?)
}

pub async fn chain_type(client: &RpcClient) -> Result<String, ClientError> {
	let params = rpc_params![].build();
	let value = client.request("system_chainType".into(), params).await?;
	Ok(serde_json::from_str(value.get())?)
}

pub async fn health(client: &RpcClient) -> Result<SystemHealth, ClientError> {
	let params = rpc_params![].build();
	let value = client.request("system_health".into(), params).await?;
	Ok(serde_json::from_str(value.get())?)
}

pub async fn local_listen_addresses(client: &RpcClient) -> Result<Vec<String>, ClientError> {
	let params = rpc_params![].build();
	let value = client
		.request("system_localListenAddresses".into(), params)
		.await?;
	Ok(serde_json::from_str(value.get())?)
}

pub async fn local_peer_id(client: &RpcClient) -> Result<String, ClientError> {
	let params = rpc_params![].build();
	let value = client.request("system_localPeerId".into(), params).await?;
	Ok(serde_json::from_str(value.get())?)
}

pub async fn name(client: &RpcClient) -> Result<String, ClientError> {
	let params = rpc_params![].build();
	let value = client.request("system_name".into(), params).await?;
	Ok(serde_json::from_str(value.get())?)
}

pub async fn node_roles(client: &RpcClient) -> Result<Vec<NodeRole>, ClientError> {
	let params = rpc_params![].build();
	let value = client.request("system_nodeRoles".into(), params).await?;
	Ok(serde_json::from_str(value.get())?)
}

pub async fn peers(client: &RpcClient) -> Result<Vec<PeerInfo>, ClientError> {
	let params = rpc_params![].build();
	let value = client.request("system_peers".into(), params).await?;
	Ok(serde_json::from_str(value.get())?)
}

pub async fn properties(client: &RpcClient) -> Result<Properties, ClientError> {
	let params = rpc_params![].build();
	let value = client.request("system_properties".into(), params).await?;
	Ok(serde_json::from_str(value.get())?)
}

pub async fn sync_state(client: &RpcClient) -> Result<SyncState, ClientError> {
	let params = rpc_params![].build();
	let value = client.request("system_syncState".into(), params).await?;
	Ok(serde_json::from_str(value.get())?)
}

pub async fn version(client: &RpcClient) -> Result<String, ClientError> {
	let params = rpc_params![].build();
	let value = client.request("system_version".into(), params).await?;
	Ok(serde_json::from_str(value.get())?)
}

#[derive(Clone)]
pub struct Chain {
	client: RpcClient,
}

impl Chain {
	pub fn new(client: RpcClient) -> Self {
		Self { client }
	}

	pub async fn get_block(&self, at: Option<BlockHash>) -> Result<ABlockDetailsRPC, ClientError> {
		get_block(&self.client, at).await
	}

	pub async fn get_block_hash(
		&self,
		block_number: Option<BlockNumber>,
	) -> Result<BlockHash, ClientError> {
		get_block_hash(&self.client, block_number).await
	}

	pub async fn get_finalized_head(&self) -> Result<BlockHash, ClientError> {
		get_finalized_head(&self.client).await
	}

	pub async fn get_header(&self, at: Option<BlockHash>) -> Result<AvailHeader, ClientError> {
		get_header(&self.client, at).await
	}
}

pub async fn get_block(
	client: &RpcClient,
	at: Option<BlockHash>,
) -> Result<ABlockDetailsRPC, ClientError> {
	let params = rpc_params![at].build();
	let value = client.request("chain_getBlock".into(), params).await?;
	Ok(serde_json::from_str(value.get())?)
}

pub async fn get_best_block(client: &RpcClient) -> Result<ABlockDetailsRPC, ClientError> {
	get_block(client, None).await
}

pub async fn get_finalized_block(client: &RpcClient) -> Result<ABlockDetailsRPC, ClientError> {
	let hash = get_finalized_head(client).await?;
	get_block(client, Some(hash)).await
}

pub async fn get_block_hash(
	client: &RpcClient,
	block_number: Option<BlockNumber>,
) -> Result<BlockHash, ClientError> {
	let params = rpc_params![block_number].build();
	let value = client.request("chain_getBlockHash".into(), params).await?;
	Ok(serde_json::from_str(value.get())?)
}

pub async fn get_finalized_head(client: &RpcClient) -> Result<BlockHash, ClientError> {
	let params = rpc_params![].build();
	let value = client
		.request("chain_getFinalizedHead".into(), params)
		.await?;
	Ok(serde_json::from_str(value.get())?)
}

pub async fn get_header(
	client: &RpcClient,
	at: Option<BlockHash>,
) -> Result<AvailHeader, ClientError> {
	let params = rpc_params![at].build();
	let value = client.request("chain_getHeader".into(), params).await?;
	Ok(serde_json::from_str(value.get())?)
}

#[derive(Clone)]
pub struct Author {
	client: RpcClient,
}

impl Author {
	pub fn new(client: RpcClient) -> Self {
		Self { client }
	}

	pub async fn rotate_keys(&self) -> Result<Vec<u8>, ClientError> {
		rotate_keys(&self.client).await
	}
}

pub async fn rotate_keys(client: &RpcClient) -> Result<Vec<u8>, ClientError> {
	let params = rpc_params![].build();
	let value = client.request("author_rotateKeys".into(), params).await?;
	let value: Bytes = serde_json::from_str(value.get())?;
	Ok(value.0)
}

#[derive(Clone)]
pub struct Kate {
	client: RpcClient,
}

impl Kate {
	pub fn new(client: RpcClient) -> Self {
		Self { client }
	}

	pub async fn block_length(&self, at: Option<BlockHash>) -> Result<BlockLength, ClientError> {
		block_length(&self.client, at).await
	}

	pub async fn query_data_proof(
		&self,
		transaction_index: u32,
		at: Option<BlockHash>,
	) -> Result<ProofResponse, ClientError> {
		query_data_proof(&self.client, transaction_index, at).await
	}

	pub async fn query_proof(
		&self,
		cells: Vec<Cell>,
		at: Option<BlockHash>,
	) -> Result<Vec<GDataProof>, ClientError> {
		query_proof(&self.client, cells, at).await
	}

	pub async fn query_rows(
		&self,
		rows: Vec<u32>,
		at: Option<BlockHash>,
	) -> Result<Vec<GRow>, ClientError> {
		query_rows(&self.client, rows, at).await
	}
}

pub async fn block_length(
	client: &RpcClient,
	at: Option<BlockHash>,
) -> Result<BlockLength, ClientError> {
	let params = rpc_params![at].build();
	let value = client.request("kate_blockLength".into(), params).await?;
	Ok(serde_json::from_str(value.get())?)
}

pub async fn query_data_proof(
	client: &RpcClient,
	transaction_index: u32,
	at: Option<BlockHash>,
) -> Result<ProofResponse, ClientError> {
	let params = rpc_params![transaction_index, at].build();
	let value = client.request("kate_queryDataProof".into(), params).await?;
	Ok(serde_json::from_str(value.get())?)
}

pub async fn query_proof(
	client: &RpcClient,
	cells: Vec<Cell>,
	at: Option<BlockHash>,
) -> Result<Vec<GDataProof>, ClientError> {
	let params = rpc_params![cells, at].build();
	let value = client.request("kate_queryProof".into(), params).await?;
	Ok(serde_json::from_str(value.get())?)
}

pub async fn query_rows(
	client: &RpcClient,
	rows: Vec<u32>,
	at: Option<BlockHash>,
) -> Result<Vec<GRow>, ClientError> {
	let params = rpc_params![rows, at].build();
	let value = client.request("kate_queryRows".into(), params).await?;
	Ok(serde_json::from_str(value.get())?)
}

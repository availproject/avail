use avail_core::data_proof::ProofResponse;

use jsonrpsee::proc_macros::rpc;
use serde::Deserialize;
use sp_core::{H256, U256};

use crate::{
	api::runtime_types::frame_system::limits::BlockLength,
	avail::{Cells, Rows},
	primitives::GDataProof,
	AppId,
};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Health {
	pub is_syncing: bool,
	pub peers: u32,
	pub should_have_peers: bool,
}

#[rpc(client, namespace = "system")]
pub trait Rpc {
	#[method(name = "health")]
	async fn health(&self) -> RpcResult<Health>;
}

#[rpc(client, namespace = "kate")]
pub trait KateRpc {
	#[method(name = "queryRows")]
	async fn query_rows(&self, rows: Rows, block: H256) -> RpcResult<Vec<Vec<U256>>>;

	#[method(name = "queryProof")]
	async fn query_proof(&self, cells: Cells, block: H256) -> RpcResult<Vec<GDataProof>>;

	#[method(name = "queryAppData")]
	async fn query_app_data(&self, app_id: AppId, block: H256)
		-> RpcResult<Vec<Option<Vec<U256>>>>;

	#[method(name = "blockLength")]
	async fn query_block_length(&self, block: H256) -> RpcResult<BlockLength>;

	#[method(name = "queryDataProof")]
	async fn query_data_proof(&self, transaction_index: u32, at: H256) -> RpcResult<ProofResponse>;
}

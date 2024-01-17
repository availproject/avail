use crate::{
	api::runtime_types::frame_system::limits::BlockLength,
	avail::{Cells, Rows},
	AppId,
};
use avail_core::data_proof_v2::ProofResponse;
use avail_core::DataProof;

use jsonrpsee::{core::Error, proc_macros::rpc};
use serde::Deserialize;
use sp_core::H256;

pub type Result<T> = std::result::Result<T, Error>;

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
	async fn health(&self) -> Result<Health>;
}

#[rpc(client, namespace = "kate")]
pub trait KateRpc {
	#[method(name = "queryRows")]
	async fn query_rows(&self, rows: Rows, block: H256) -> Result<Vec<Vec<u8>>>;

	#[method(name = "queryProof")]
	async fn query_proof(&self, cells: Cells, block: H256) -> Result<Vec<u8>>;

	#[method(name = "queryAppData")]
	async fn query_app_data(&self, app_id: AppId, block: H256) -> Result<Vec<Option<Vec<u8>>>>;

	#[method(name = "blockLength")]
	async fn query_block_length(&self, block: H256) -> Result<BlockLength>;

	#[method(name = "queryDataProof")]
	async fn query_data_proof(&self, transaction_index: u32, block: H256) -> Result<DataProof>;

	#[method(name = "queryDataProofV2")]
	async fn query_data_proof_v2(
		&self,
		transaction_index: u32,
		block: H256,
	) -> Result<ProofResponse>;
}

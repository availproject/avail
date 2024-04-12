use crate::{
	api::runtime_types::frame_system::limits::BlockLength,
	avail::{Cells, GDataProof, GRow, Rows},
};
use avail_core::data_proof::ProofResponse;

use derive_more::From;
use jsonrpsee::proc_macros::rpc;
use serde::{Deserialize, Serialize};
use sp_core::H256;

#[rpc(client, namespace = "kate")]
pub trait KateRpc {
	#[method(name = "queryRows")]
	async fn query_rows(&self, rows: Rows, block: H256) -> RpcResult<Vec<GRow>>;

	#[method(name = "queryProof")]
	async fn query_proof(&self, cells: Cells, block: H256) -> RpcResult<Vec<GDataProof>>;

	#[method(name = "blockLength")]
	async fn query_block_length(&self, block: H256) -> RpcResult<BlockLength>;

	#[method(name = "queryDataProof")]
	async fn query_data_proof(&self, transaction_index: u32, at: H256) -> RpcResult<ProofResponse>;
}

#[derive(Debug, From, Clone, Copy, Serialize, Deserialize)]
#[serde(try_from = "Vec<u8>", into = "Vec<u8>")]
pub struct GProof(pub [u8; 48]);

impl From<GProof> for Vec<u8> {
	fn from(proof: GProof) -> Self {
		proof.0.to_vec()
	}
}

impl TryFrom<Vec<u8>> for GProof {
	type Error = u32;
	fn try_from(data: Vec<u8>) -> Result<Self, Self::Error> {
		if data.len() != 48 {
			return Err(data.len() as u32);
		};

		let mut proof = [0u8; 48];
		proof.copy_from_slice(&data);
		Ok(GProof(proof))
	}
}

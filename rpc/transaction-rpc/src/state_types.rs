use jsonrpsee::tokio::sync::{
	mpsc::{Receiver, Sender},
	oneshot,
};
use serde::{Deserialize, Serialize};
use sp_core::H256;

pub type TxStateChannelResponse = oneshot::Sender<Vec<RPCResult>>;
pub type TxStateChannel = (H256, bool, TxStateChannelResponse);
pub type TxStateReceiver = Receiver<TxStateChannel>;
pub type TxStateSender = Sender<TxStateChannel>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RPCResult {
	pub block_hash: H256,
	pub block_height: u32,
	pub tx_hash: H256,
	pub tx_index: u32,
	pub tx_success: bool,
	pub pallet_index: u8,
	pub call_index: u8,
	pub is_finalized: bool,
}

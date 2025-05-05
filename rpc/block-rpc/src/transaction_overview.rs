use crate::common::{DispatchIndex, TransactionLocation};

use super::{common, BlockId};
use jsonrpsee::tokio::sync::{mpsc, oneshot};
use serde::{Deserialize, Serialize};
use sp_core::H256;

pub type ChannelResponse = oneshot::Sender<Vec<Response>>;
pub type Channel = (Params, ChannelResponse);
pub type Receiver = mpsc::Receiver<Channel>;
pub type Sender = mpsc::Sender<Channel>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response {
	pub block_id: BlockId,
	pub block_finalized: bool,
	pub tx_location: TransactionLocation,
	pub dispatch_index: DispatchIndex,
	pub events: Option<common::events::Events>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseDebug {
	pub value: Vec<Response>,
	pub debug_execution_time: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Params {
	pub tx_hash: H256,
	#[serde(default)]
	pub finalized: bool,
	#[serde(default)]
	pub fetch_events: bool,
	#[serde(default)]
	pub enable_event_decoding: bool,
}

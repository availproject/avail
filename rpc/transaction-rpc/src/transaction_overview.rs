use super::common;
use jsonrpsee::tokio::sync::{mpsc, oneshot};
use serde::{Deserialize, Serialize};
use sp_core::H256;

pub type ChannelResponse = oneshot::Sender<Vec<Response>>;
pub type Channel = (RPCParams, ChannelResponse);
pub type Receiver = mpsc::Receiver<Channel>;
pub type Sender = mpsc::Sender<Channel>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response {
	pub block_finalized: bool,
	pub block_hash: H256,
	pub block_height: u32,
	pub tx_hash: H256,
	pub tx_index: u32,
	pub pallet_index: u8,
	pub call_index: u8,
	pub events: Option<common::events::Events>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseDebug {
	pub value: Vec<Response>,
	pub debug_execution_time: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RPCParams {
	pub tx_hash: H256,
	#[serde(default)]
	pub finalized: bool,
	#[serde(default)]
	pub fetch_events: bool,
	#[serde(default)]
	pub enable_event_decoding: bool,
}

use crate::{BlockState, HashIndex};
use jsonrpsee::tokio::sync::{mpsc, oneshot};
use serde::{Deserialize, Serialize};
use sp_core::H256;

pub type ChannelResponse = oneshot::Sender<Result<RPCResult, String>>;
pub type Channel = (RPCParams, ChannelResponse);
pub type Receiver = mpsc::Receiver<Channel>;
pub type Sender = mpsc::Sender<Channel>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RPCParams {
	pub block_id: HashIndex,
	#[serde(default)]
	pub fetch_calls: bool,
	#[serde(default)]
	pub fetch_events: bool,
	pub call_filter: Option<CallFilter>,
	#[serde(default)]
	pub event_filter: EventFilter,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CallFilter {
	#[serde(default)]
	pub tx: CallFilterTxKind,
	#[serde(default)]
	pub signature: CallFilterSignature,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CallFilterTxKind {
	All,
	Some(Vec<HashIndex>),
	Pallet(Vec<u8>),
	PalletCall(Vec<(u8, u8)>),
	HasEvent(Vec<(u8, u8)>),
}

impl Default for CallFilterTxKind {
	fn default() -> Self {
		Self::All
	}
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CallFilterSignature {
	pub ss58_address: Option<String>,
	pub app_id: Option<u32>,
	pub nonce: Option<u32>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct RPCResult {
	pub block_hash: H256,
	pub block_height: u32,
	pub block_state: BlockState,
	pub calls: Option<Vec<CallData>>,
	pub events: Option<Vec<EventData>>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct RPCResultDebug {
	pub value: RPCResult,
	pub debug_execution_time: u64,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CallData {
	// (pallet id, call id)
	pub id: (u8, u8),
	pub tx_id: u32,
	pub tx_hash: H256,
	pub data: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct EventData {
	// (pallet id, event id)
	pub id: (u8, u8),
	pub phase: Phase,
	pub data: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct EventFilter {
	#[serde(default)]
	pub by_transaction: EventTransactionFilterKind,
	#[serde(default)]
	pub by_id: EventIdFilterKind,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventTransactionFilterKind {
	All,
	Some(Vec<HashIndex>),
	OnlyConsensus,
}

impl Default for EventTransactionFilterKind {
	fn default() -> Self {
		Self::All
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventIdFilterKind {
	All,
	Pallet(u8),
	Combination(Vec<(u8, u8)>),
}

impl Default for EventIdFilterKind {
	fn default() -> Self {
		Self::All
	}
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Phase {
	/// Applying an extrinsic.
	ApplyExtrinsic(u32),
	/// Finalizing the block.
	Finalization,
	/// Initializing the block.
	Initialization,
}

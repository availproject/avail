use crate::{BlockState, HashIndex};
use serde::{Deserialize, Serialize};
use sp_core::H256;

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
	pub tx: CallTransactionFilterKind,
	#[serde(default)]
	pub id: CallIdFilterKind,
	pub ss58_address: Option<String>,
	pub app_id: Option<u32>,
	pub nonce: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CallTransactionFilterKind {
	All,
	Some(Vec<HashIndex>),
}

impl Default for CallTransactionFilterKind {
	fn default() -> Self {
		Self::All
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CallIdFilterKind {
	All,
	Pallet(u8),
	Combination(Vec<(u8, u8)>),
}

impl Default for CallIdFilterKind {
	fn default() -> Self {
		Self::All
	}
}

#[derive(Clone, Serialize, Deserialize)]
pub struct RPCResult {
	pub block_hash: H256,
	pub block_height: u32,
	pub block_state: BlockState,
	pub calls: Option<Vec<Data>>,
	pub events: Option<Vec<Data>>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Data {
	// (pallet id, call/event id)
	pub id: (u8, u8),
	pub tx_id: u32,
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

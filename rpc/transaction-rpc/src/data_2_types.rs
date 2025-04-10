use serde::{Deserialize, Serialize};
use sp_core::H256;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HashIndex {
	Hash(H256),
	Index(u32),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RPCParams {
	pub block_id: HashIndex,
	#[serde(default)]
	pub fetch_calls: bool,
	#[serde(default)]
	pub fetch_events: bool,
	pub call_filter: Option<CallFilter>,
	pub event_filter: Option<EventFilter>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CallFilter {
	pub tx_id: Option<HashIndex>,
	pub pallet_id: Option<u8>,
	pub call_id: Option<u8>,
	pub ss58_address: Option<String>,
	pub app_id: Option<u32>,
	pub nonce: Option<u32>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct EventFilter {
	pub tx_id: Option<HashIndex>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct RPCResult {
	pub block_hash: H256,
	pub block_height: u32,
	pub calls: Option<Vec<Data>>,
	pub events: Option<Vec<Data>>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Data {
	// (pallet id, call/event id)
	id: (u8, u8),
	tx_id: u32,
	data: String,
}

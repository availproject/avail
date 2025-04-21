use crate::{BlockState, HashIndex};
use jsonrpsee::tokio::sync::{mpsc, oneshot};
use serde::{Deserialize, Serialize};
use sp_core::H256;

pub type ChannelResponse = oneshot::Sender<Result<Response, String>>;
pub type Channel = (RPCParams, ChannelResponse);
pub type Receiver = mpsc::Receiver<Channel>;
pub type Sender = mpsc::Sender<Channel>;

pub use filter::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RPCParams {
	pub block_id: HashIndex,
	#[serde(default)]
	pub fetch_calls: bool,
	#[serde(default)]
	pub fetch_events: bool,
	#[serde(default)]
	pub call_filter: CallFilter,
	#[serde(default)]
	pub event_filter: EventFilter,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Response {
	pub block_hash: H256,
	pub block_height: u32,
	pub block_state: BlockState,
	pub calls: Option<Vec<CallData>>,
	pub events: Option<Vec<EventData>>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ResponseDebug {
	pub value: Response,
	pub debug_execution_time: u64,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CallData {
	// (pallet id, call id)
	pub id: (u8, u8),
	pub tx_index: u32,
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

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Phase {
	/// Applying an extrinsic.
	ApplyExtrinsic(u32),
	/// Finalizing the block.
	Finalization,
	/// Initializing the block.
	Initialization,
}

pub mod filter {
	pub use super::*;

	#[derive(Debug, Default, Clone, Serialize, Deserialize)]
	pub struct CallFilter {
		#[serde(default)]
		pub transaction: TransactionFilterOptions,
		#[serde(default)]
		pub signature: SignatureFilterOptions,
	}

	#[derive(Debug, Clone, Serialize, Deserialize)]
	pub enum TransactionFilterOptions {
		All,
		TxHash(Vec<H256>),
		TxIndex(Vec<u32>),
		Pallet(Vec<u8>),
		PalletCall(Vec<(u8, u8)>),
	}

	impl Default for TransactionFilterOptions {
		fn default() -> Self {
			Self::All
		}
	}
	/*
	impl TransactionFilterOptions {
		pub fn filter_out_pallet(&self, value: u8) -> bool {
			match self {
				TransactionFilterOptions::Pallet(items) => !items.contains(&value),
				_ => false,
			}
		}
	} */

	#[derive(Debug, Default, Clone, Serialize, Deserialize)]
	pub struct SignatureFilterOptions {
		pub ss58_address: Option<String>,
		pub app_id: Option<u32>,
		pub nonce: Option<u32>,
	}

	#[derive(Debug, Default, Clone, Serialize, Deserialize)]
	pub struct EventFilter {
		#[serde(default)]
		pub phase: PhaseFilterOptions,
		#[serde(default)]
		pub pallet: PalletFilterOptions,
	}

	#[derive(Debug, Clone, Serialize, Deserialize)]
	pub enum PhaseFilterOptions {
		All,
		TxIndex(Vec<u32>),
		OnlyConsensus,
	}

	impl Default for PhaseFilterOptions {
		fn default() -> Self {
			Self::All
		}
	}

	#[derive(Debug, Clone, Serialize, Deserialize)]
	pub enum PalletFilterOptions {
		All,
		Pallet(u8),
		Combination(Vec<(u8, u8)>),
	}

	impl Default for PalletFilterOptions {
		fn default() -> Self {
			Self::All
		}
	}
}

use jsonrpsee::tokio::sync::{mpsc, oneshot};
use serde::{Deserialize, Serialize};
use sp_core::H256;

use crate::{BlockState, HashIndex};
pub type ChannelResponse = oneshot::Sender<Result<Response, String>>;
pub type Channel = (RPCParams, ChannelResponse);
pub type Receiver = mpsc::Receiver<Channel>;
pub type Sender = mpsc::Sender<Channel>;

pub use filter::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RPCParams {
	pub block_id: HashIndex,
	#[serde(default)]
	pub extension: RPCParamsExtension,
	#[serde(default)]
	pub filter: Filter,
}

#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize)]
pub struct RPCParamsExtension {
	#[serde(default)]
	pub enable_call_decoding: bool,
	#[serde(default)]
	pub fetch_events: bool,
	#[serde(default)]
	pub enable_event_decoding: bool,
}

pub type Events = Vec<Event>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response {
	pub block_hash: H256,
	pub block_height: u32,
	pub block_state: BlockState,
	pub transactions: Vec<TransactionData>,
	pub consensus_events: Option<Events>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseDebug {
	pub value: Response,
	pub debug_execution_time: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionData {
	pub tx_hash: H256,
	pub tx_index: u32,
	pub pallet_id: u8,
	pub call_id: u8,
	pub signed: Option<TransactionDataSigned>,
	pub decoded: Option<u8>,
	pub events: Option<Events>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TransactionDataSigned {
	pub ss58_address: Option<String>,
	pub nonce: u32,
	pub app_id: u32,
	pub mortality: Option<(u64, u64)>, // None means the tx is Immortal
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
	pub index: u32,
	pub pallet_id: u8,
	pub event_id: u8,
	pub decoded: Option<DecodedEventData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncodedEvent {
	pub index: u32,
	pub pallet_id: u8,
	pub event_id: u8,
	// First N bytes of every encoded event is CompactU32.
	pub data: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecodedEvent {
	pub index: u32,
	pub pallet_id: u8,
	pub event_id: u8,
	pub data: DecodedEventData,
}

impl DecodedEvent {
	pub fn new(index: u32, pallet_id: u8, event_id: u8, data: DecodedEventData) -> Self {
		Self {
			index,
			pallet_id,
			event_id,
			data,
		}
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DecodedEventData {
	Unknown,
	SystemExtrinsicSuccess,
	SystemExtrinsicFailed,
	SudoSudid(bool),
	SudoSudoAsDone(bool),
	MultisigMultisigExecuted(bool),
	ProxyProxyExecuted(bool),
	DataAvailabilityDataSubmitted(DataSubmittedEvent),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataSubmittedEvent {
	pub who: String,
	pub data_hash: String,
}

pub mod filter {
	pub use super::*;

	#[derive(Debug, Default, Clone, Serialize, Deserialize)]
	pub struct Filter {
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
		HasEvent(Vec<(u8, u8)>),
	}

	impl Default for TransactionFilterOptions {
		fn default() -> Self {
			Self::All
		}
	}

	#[derive(Debug, Default, Clone, Serialize, Deserialize)]
	pub struct SignatureFilterOptions {
		pub ss58_address: Option<String>,
		pub app_id: Option<u32>,
		pub nonce: Option<u32>,
	}
}

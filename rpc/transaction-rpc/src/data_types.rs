use super::state_types;
use jsonrpsee::tokio::sync::{
	mpsc::{Receiver, Sender},
	oneshot,
};
use serde::{Deserialize, Serialize};
use sp_core::H256;

pub type TxDataChannelResponse = oneshot::Sender<Result<RPCResult, String>>;
pub type TxDataChannel = (RPCParams, TxDataChannelResponse);
pub type TxDataReceiver = Receiver<TxDataChannel>;
pub type TxDataSender = Sender<TxDataChannel>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HashIndex {
	Hash(H256),
	Index(u32),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RPCParams {
	pub block_id: HashIndex,
	pub extension: Option<RPCParamsExtension>,
	pub filter: Option<Filter>,
}

#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize)]
pub struct RPCParamsExtension {
	pub fetch_call: Option<bool>,
	pub enable_call_encoding: Option<bool>,
	pub enable_call_decoding: Option<bool>,
	pub fetch_events: Option<bool>,
	pub enable_event_encoding: Option<bool>,
	pub enable_event_decoding: Option<bool>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Filter {
	pub tx_id: Option<HashIndex>,
	pub pallet_id: Option<u8>,
	pub call_id: Option<u8>,
	pub ss58_address: Option<String>,
	pub app_id: Option<u32>,
	pub nonce: Option<u32>,
}

pub type EncodedCall = String;
pub type DecodedCall = String;
pub type EncodedEvents = Vec<EncodedEvent>;
pub type DecodedEvents = Vec<DecodedEvent>;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct RPCResult {
	pub block_hash: H256,
	pub block_height: u32,
	pub transactions: Vec<TransactionData>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TransactionData {
	pub tx_hash: H256,
	pub tx_index: u32,
	pub pallet_id: u8,
	pub call_id: u8,
	pub signed: Option<TransactionDataSigned>,
	pub extension: TransactionDataExtension,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TransactionDataExtension {
	pub encoded_call: Option<EncodedCall>,
	pub decoded_call: Option<DecodedCall>,
	// First N bytes of every encoded event is CompactU32 (number of bytes).
	pub encoded_events: Option<EncodedEvents>,
	pub decoded_events: Option<DecodedEvents>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TransactionDataSigned {
	pub ss58_address: Option<String>,
	pub nonce: u32,
	pub app_id: u32,
	pub mortality: Option<(u64, u64)>, // None means the tx is Immortal
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

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct DataSubmittedEvent {
	pub who: String,
	pub data_hash: String,
}

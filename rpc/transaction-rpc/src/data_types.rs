use jsonrpsee::tokio::sync::{
	mpsc::{Receiver, Sender},
	oneshot,
};
use serde::{Deserialize, Serialize};
use sp_core::H256;

pub type TxDataChannelResponse = oneshot::Sender<Result<TransactionDatas, String>>;
pub type TxDataChannel = (TransactionDataRPCParams, TxDataChannelResponse);
pub type TxDataReceiver = Receiver<TxDataChannel>;
pub type TxDataSender = Sender<TxDataChannel>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HashIndex {
	Hash(H256),
	Index(u32),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionDataRPCParams {
	pub block_id: HashIndex,
	pub fetch_call: Option<bool>,
	pub fetch_events: Option<bool>,
	pub fetch_state: Option<bool>,
	pub decode_events: Option<bool>,
	pub filter: Option<TransactionDataFilter>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TransactionDataFilter {
	pub tx_id: Option<HashIndex>,
	pub pallet_id: Option<u8>,
	pub call_id: Option<u8>,
	pub ss58_address: Option<String>,
	pub app_id: Option<u32>,
	pub nonce: Option<u32>,
}

pub type EncodedCall = String;
pub type EncodedEvents = Vec<String>;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TransactionDatas {
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
	pub call: Option<EncodedCall>,
	pub encoded_events: Option<EncodedEvents>,
	pub decoded_events: Option<DecodedEvents>,
	pub states: Option<Vec<super::state_types::TransactionState>>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct DecodedEvents {
	pub system_extrinsic: Option<bool>,
	pub sudo_sudid: Vec<bool>,
	pub sudo_sudo_as_done: Vec<bool>,
	pub multisig_executed: Vec<bool>,
	pub proxy_executed: Vec<bool>,
	pub data_availability_data_submitted: Vec<DataSubmittedEvent>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct DataSubmittedEvent {
	pub who: String,
	pub data_hash: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TransactionDataSigned {
	pub ss58_address: Option<String>,
	pub nonce: u32,
	pub app_id: u32,
	pub mortality: Option<(u64, u64)>, // None means the tx is Immortal
}

use async_trait::async_trait;
use jsonrpsee::{
	core::RpcResult,
	proc_macros::rpc,
	tokio::sync::{
		mpsc::{Receiver, Sender},
		oneshot,
	},
	types::ErrorObject,
};
use serde::{Deserialize, Serialize};
use sp_core::H256;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionState {
	pub block_hash: H256,
	pub block_height: u32,
	pub tx_hash: H256,
	pub tx_index: u32,
	pub tx_success: bool,
	pub pallet_index: u8,
	pub call_index: u8,
	pub is_finalized: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HashIndex {
	Hash(H256),
	Index(u32),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionDataRPCParams {
	pub block_id: HashIndex,
	pub tx_id: HashIndex,
	pub fetch_call: Option<bool>,
	pub fetch_events: Option<bool>,
	pub fetch_state: Option<bool>,
}

pub type EncodedCall = Vec<u8>;
pub type EncodedEvents = Vec<Vec<u8>>;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TransactionData {
	pub block_hash: H256,
	pub tx_index: u32,
	pub pallet_id: u8,
	pub call_id: u8,
	pub signed: Option<TransactionDataSigned>,
	pub call: Option<EncodedCall>,
	pub events: Option<EncodedEvents>,
	pub states: Option<Vec<TransactionState>>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TransactionDataSigned {
	pub ss58_address: Option<String>,
	pub nonce: u32,
	pub app_id: u32,
	pub mortality: Option<(u64, u64)>, // None means the tx is Immortal
}

#[derive(Clone, Copy, Default, Serialize, Deserialize)]
pub struct EnabledServices {
	pub tx_state: bool,
	pub tx_data: bool,
}

#[derive(Clone, Default)]
pub struct Deps {
	pub tx_state_sender: Option<TxStateSender>,
	pub tx_data_sender: Option<TxDataSender>,
}

pub type OneShotTxStateSender = oneshot::Sender<Vec<TransactionState>>;
pub type TxStateReceiver = Receiver<(H256, bool, OneShotTxStateSender)>;
pub type TxStateSender = Sender<(H256, bool, OneShotTxStateSender)>;
pub type TxStateChannel = (H256, bool, OneShotTxStateSender);

pub type OneShotTxDataSender = oneshot::Sender<Result<TransactionData, String>>;
pub type TxDataReceiver = Receiver<(TransactionDataRPCParams, OneShotTxDataSender)>;
pub type TxDataSender = Sender<(TransactionDataRPCParams, OneShotTxDataSender)>;
pub type TxDataChannel = (TransactionDataRPCParams, OneShotTxDataSender);

#[rpc(client, server)]
pub trait TransactionState {
	#[method(name = "transaction_state")]
	async fn transaction_state(
		&self,
		tx_hash: H256,
		is_finalized: Option<bool>,
	) -> RpcResult<Vec<TransactionState>>;

	#[method(name = "transaction_data")]
	async fn transaction_data(
		&self,
		params: TransactionDataRPCParams,
	) -> RpcResult<TransactionData>;

	#[method(name = "transaction_enabled_services")]
	async fn transaction_enabled_services(&self) -> RpcResult<EnabledServices>;
}

pub struct System {
	tx_state_sender: Option<TxStateSender>,
	tx_data_sender: Option<TxDataSender>,
}

impl System {
	pub fn new(deps: Deps) -> Self {
		Self {
			tx_state_sender: deps.tx_state_sender,
			tx_data_sender: deps.tx_data_sender,
		}
	}
}

#[async_trait]
impl TransactionStateServer for System {
	async fn transaction_state(
		&self,
		tx_hash: H256,
		finalized: Option<bool>,
	) -> RpcResult<Vec<TransactionState>> {
		let Some(sender) = self.tx_state_sender.as_ref() else {
			return Err(internal_error(String::from(
				"Transaction State RPC service disabled",
			)));
		};

		let (response_tx, response_rx) = oneshot::channel();

		let finalized = finalized.unwrap_or(false);
		let res = sender.send((tx_hash, finalized, response_tx)).await;
		if let Err(e) = res {
			return Err(internal_error(e.to_string()));
		}

		match response_rx.await {
			Ok(x) => Ok(x),
			Err(e) => Err(internal_error(e.to_string())),
		}
	}

	async fn transaction_data(
		&self,
		params: TransactionDataRPCParams,
	) -> RpcResult<TransactionData> {
		let Some(sender) = self.tx_data_sender.as_ref() else {
			return Err(internal_error(String::from(
				"Transaction Data RPC service disabled",
			)));
		};

		let (response_tx, response_rx) = oneshot::channel();

		let res = sender.send((params, response_tx)).await;
		if let Err(e) = res {
			return Err(internal_error(e.to_string()));
		}

		let res = match response_rx.await {
			Ok(x) => x,
			Err(e) => return Err(internal_error(e.to_string())),
		};

		match res {
			Ok(x) => Ok(x),
			Err(e) => return Err(internal_error(e)),
		}
	}

	async fn transaction_enabled_services(&self) -> RpcResult<EnabledServices> {
		Ok(EnabledServices {
			tx_state: self.tx_state_sender.is_some(),
			tx_data: self.tx_data_sender.is_some(),
		})
	}
}

fn internal_error<'a>(msg: String) -> ErrorObject<'a> {
	ErrorObject::owned(0, msg, None::<()>)
}

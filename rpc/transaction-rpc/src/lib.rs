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

#[derive(Clone)]
pub struct Deps {
	pub sender: TxStateSender,
}

pub type OneShotTxStateSender = oneshot::Sender<Vec<TransactionState>>;
pub type TxStateReceiver = Receiver<(H256, bool, OneShotTxStateSender)>;
pub type TxStateSender = Sender<(H256, bool, OneShotTxStateSender)>;
pub type TxStateChannel = (H256, bool, OneShotTxStateSender);

#[rpc(client, server)]
pub trait TransactionState {
	#[method(name = "transaction_state")]
	async fn transaction_state(
		&self,
		tx_hash: H256,
		is_finalized: Option<bool>,
	) -> RpcResult<Vec<TransactionState>>;
}

pub struct System {
	sender: TxStateSender,
}

impl System {
	pub fn new(deps: Deps) -> Self {
		Self {
			sender: deps.sender,
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
		let (response_tx, response_rx) = oneshot::channel();

		let finalized = finalized.unwrap_or(false);
		let res = self.sender.send((tx_hash, finalized, response_tx)).await;
		if let Err(e) = res {
			return Err(internal_error(e.to_string()));
		}

		match response_rx.await {
			Ok(x) => Ok(x),
			Err(e) => Err(internal_error(e.to_string())),
		}
	}
}

fn internal_error<'a>(msg: String) -> ErrorObject<'a> {
	ErrorObject::owned(0, msg, None::<()>)
}

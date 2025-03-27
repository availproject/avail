pub mod data_types;
pub mod state_types;

use async_trait::async_trait;
use data_types::{TransactionDataRPCParams, TransactionDatas, TxDataSender};
use jsonrpsee::{core::RpcResult, proc_macros::rpc, tokio::sync::oneshot, types::ErrorObject};
use serde::{Deserialize, Serialize};
use sp_core::H256;
use state_types::TxStateSender;

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

#[rpc(client, server)]
pub trait TransactionApi {
	#[method(name = "transaction_state")]
	async fn transaction_state(
		&self,
		tx_hash: H256,
		is_finalized: Option<bool>,
	) -> RpcResult<Vec<state_types::TransactionState>>;

	#[method(name = "transaction_data")]
	async fn transaction_data(
		&self,
		params: TransactionDataRPCParams,
	) -> RpcResult<TransactionDatas>;

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
impl TransactionApiServer for System {
	async fn transaction_state(
		&self,
		tx_hash: H256,
		finalized: Option<bool>,
	) -> RpcResult<Vec<state_types::TransactionState>> {
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
	) -> RpcResult<TransactionDatas> {
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

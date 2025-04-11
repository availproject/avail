pub mod block_data_types;
pub mod block_overview_types;
pub mod state_types;
use std::sync::Arc;

use async_trait::async_trait;
use block_overview_types::TxDataSender;
use jsonrpsee::{
	core::RpcResult,
	proc_macros::rpc,
	tokio::sync::{oneshot, Notify},
	types::ErrorObject,
};
use serde::{Deserialize, Serialize};
use sp_core::H256;
use state_types::TxStateSender;

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct EnabledServices {
	pub tx_state: bool,
	pub block_overview: bool,
	pub block_data: bool,
}

#[derive(Clone, Default)]
pub struct Deps {
	pub tx_state_sender: Option<TxStateSender>,
	pub tx_state_notifier: Option<Arc<Notify>>,
	pub block_overview_sender: Option<TxDataSender>,
	pub block_data_sender: Option<()>,
	pub block_notifier: Option<Arc<Notify>>,
}

#[rpc(client, server)]
pub trait TransactionApi {
	#[method(name = "transaction_state")]
	async fn transaction_state(
		&self,
		tx_hash: H256,
		is_finalized: Option<bool>,
	) -> RpcResult<state_types::RPCResultDebug>;

	#[method(name = "block_overview")]
	async fn block_overview(
		&self,
		params: block_overview_types::RPCParams,
	) -> RpcResult<block_overview_types::RPCResultDebug>;

	#[method(name = "block_data")]
	async fn block_data(&self, params: block_overview_types::RPCParams) -> RpcResult<()>;

	#[method(name = "transaction_enabled_services")]
	async fn transaction_enabled_services(&self) -> RpcResult<EnabledServices>;
}

pub struct System {
	tx_state_sender: Option<TxStateSender>,
	tx_state_notifier: Option<Arc<Notify>>,
	block_overview_sender: Option<TxDataSender>,
	block_data_sender: Option<()>,
	block_notifier: Option<Arc<Notify>>,
}

impl System {
	pub fn new(deps: Deps) -> Self {
		Self {
			tx_state_sender: deps.tx_state_sender,
			tx_state_notifier: deps.tx_state_notifier,
			block_overview_sender: deps.block_overview_sender,
			block_data_sender: deps.block_data_sender,
			block_notifier: deps.block_notifier,
		}
	}
}

#[async_trait]
impl TransactionApiServer for System {
	async fn transaction_state(
		&self,
		tx_hash: H256,
		finalized: Option<bool>,
	) -> RpcResult<state_types::RPCResultDebug> {
		let now = std::time::Instant::now();
		let Some(sender) = self.tx_state_sender.as_ref() else {
			return Err(internal_error(String::from(
				"Transaction State RPC service disabled",
			)));
		};

		let Some(notifier) = self.tx_state_notifier.as_ref() else {
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

		notifier.notify_one();

		let response = response_rx.await;
		let elapsed = now.elapsed();

		match response {
			Ok(x) => {
				let r = state_types::RPCResultDebug {
					value: x,
					debug_execution_time: elapsed.as_millis() as u64,
				};

				Ok(r)
			},
			Err(e) => Err(internal_error(e.to_string())),
		}
	}

	async fn block_overview(
		&self,
		params: block_overview_types::RPCParams,
	) -> RpcResult<block_overview_types::RPCResultDebug> {
		let now = std::time::Instant::now();
		let Some(sender) = self.block_overview_sender.as_ref() else {
			return Err(internal_error(String::from(
				"Block Overview RPC service disabled",
			)));
		};

		let Some(notifier) = self.block_notifier.as_ref() else {
			return Err(internal_error(String::from(
				"Block Overview RPC service disabled",
			)));
		};

		let (response_tx, response_rx) = oneshot::channel();

		let res = sender.send((params, response_tx)).await;
		if let Err(e) = res {
			return Err(internal_error(e.to_string()));
		}

		notifier.notify_one();

		let res = match response_rx.await {
			Ok(x) => x,
			Err(e) => return Err(internal_error(e.to_string())),
		};

		let elapsed = now.elapsed();

		match res {
			Ok(x) => {
				let r = block_overview_types::RPCResultDebug {
					value: x,
					debug_execution_time: elapsed.as_millis() as u64,
				};
				Ok(r)
			},
			Err(e) => return Err(internal_error(e)),
		}
	}

	async fn block_data(&self, params: block_overview_types::RPCParams) -> RpcResult<()> {
		Ok(())
	}

	async fn transaction_enabled_services(&self) -> RpcResult<EnabledServices> {
		Ok(EnabledServices {
			tx_state: self.tx_state_sender.is_some(),
			block_overview: self.block_overview_sender.is_some(),
			block_data: self.block_data_sender.is_some(),
		})
	}
}

fn internal_error<'a>(msg: String) -> ErrorObject<'a> {
	ErrorObject::owned(0, msg, None::<()>)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HashIndex {
	Hash(H256),
	Index(u32),
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum BlockState {
	Included,
	Finalized,
	Discarded,
}

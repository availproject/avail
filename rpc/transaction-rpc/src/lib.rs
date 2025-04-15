pub mod block_data;
pub mod block_overview;
pub mod transaction_overview;
use std::sync::Arc;

use async_trait::async_trait;
use jsonrpsee::{
	core::RpcResult,
	proc_macros::rpc,
	tokio::sync::{oneshot, Notify},
	types::ErrorObject,
};
use serde::{Deserialize, Serialize};
use sp_core::H256;

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct EnabledServices {
	pub transaction_overview: bool,
	pub block_overview: bool,
	pub block_data: bool,
}

#[derive(Clone, Default)]
pub struct Deps {
	pub transaction_overview_sender: Option<transaction_overview::Sender>,
	pub transaction_overview_notifier: Option<Arc<Notify>>,
	pub block_overview_sender: Option<block_overview::Sender>,
	pub block_data_sender: Option<block_data::Sender>,
	pub block_notifier: Option<Arc<Notify>>,
}

#[rpc(client, server)]
pub trait Api {
	#[method(name = "transaction_overview")]
	async fn transaction_overview(
		&self,
		tx_hash: H256,
		is_finalized: Option<bool>,
	) -> RpcResult<transaction_overview::ResponseDebug>;

	#[method(name = "block_overview")]
	async fn block_overview(
		&self,
		params: block_overview::RPCParams,
	) -> RpcResult<block_overview::ResponseDebug>;

	#[method(name = "block_data")]
	async fn block_data(
		&self,
		params: block_data::RPCParams,
	) -> RpcResult<block_data::ResponseDebug>;

	#[method(name = "block_service_enabled")]
	async fn block_service_enabled(&self) -> RpcResult<EnabledServices>;
}

pub struct RPC {
	transaction_overview_sender: Option<transaction_overview::Sender>,
	transaction_overview_notifier: Option<Arc<Notify>>,
	block_overview_sender: Option<block_overview::Sender>,
	block_data_sender: Option<block_data::Sender>,
	block_notifier: Option<Arc<Notify>>,
}

impl RPC {
	pub fn new(deps: Deps) -> Self {
		Self {
			transaction_overview_sender: deps.transaction_overview_sender,
			transaction_overview_notifier: deps.transaction_overview_notifier,
			block_overview_sender: deps.block_overview_sender,
			block_data_sender: deps.block_data_sender,
			block_notifier: deps.block_notifier,
		}
	}
}

#[async_trait]
impl ApiServer for RPC {
	async fn transaction_overview(
		&self,
		tx_hash: H256,
		finalized: Option<bool>,
	) -> RpcResult<transaction_overview::ResponseDebug> {
		let now = std::time::Instant::now();
		let Some(sender) = self.transaction_overview_sender.as_ref() else {
			return Err(internal_error(String::from(
				"Transaction Overview RPC service disabled",
			)));
		};

		let Some(notifier) = self.transaction_overview_notifier.as_ref() else {
			return Err(internal_error(String::from(
				"Transaction Overview RPC service disabled",
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
				let r = transaction_overview::ResponseDebug {
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
		params: block_overview::RPCParams,
	) -> RpcResult<block_overview::ResponseDebug> {
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
				let r = block_overview::ResponseDebug {
					value: x,
					debug_execution_time: elapsed.as_millis() as u64,
				};
				Ok(r)
			},
			Err(e) => return Err(internal_error(e)),
		}
	}

	async fn block_data(
		&self,
		params: block_data::RPCParams,
	) -> RpcResult<block_data::ResponseDebug> {
		let now = std::time::Instant::now();
		let Some(sender) = self.block_data_sender.as_ref() else {
			return Err(internal_error(String::from(
				"Block Data RPC service disabled",
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
				let r = block_data::ResponseDebug {
					value: x,
					debug_execution_time: elapsed.as_millis() as u64,
				};
				Ok(r)
			},
			Err(e) => return Err(internal_error(e)),
		}
	}

	async fn block_service_enabled(&self) -> RpcResult<EnabledServices> {
		Ok(EnabledServices {
			transaction_overview: self.transaction_overview_sender.is_some(),
			block_overview: self.block_overview_sender.is_some(),
			block_data: self.block_data_sender.is_some(),
		})
	}
}

fn internal_error<'a>(msg: String) -> ErrorObject<'a> {
	ErrorObject::owned(0, msg, None::<()>)
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
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

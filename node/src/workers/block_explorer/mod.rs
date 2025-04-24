mod cache;
mod worker;

pub use worker::Worker;

use jsonrpsee::tokio::sync::Notify;
use std::sync::Arc;
use transaction_rpc::{block_data, block_overview};

pub(crate) const RPC_CHANNEL_LIMIT: usize = 20_000;

#[derive(Clone, Default)]
pub struct CliDeps {
	pub enabled: bool,
}

pub struct Deps {
	pub overview_receiver: block_overview::Receiver,
	pub data_receiver: block_data::Receiver,
	pub notifier: Arc<Notify>,
}

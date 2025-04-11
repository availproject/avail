use std::sync::Arc;

use jsonrpsee::tokio::sync::Notify;
use transaction_rpc::{block_data, block_overview};

#[derive(Clone, Default)]
pub struct CliDeps {
	pub enabled: bool,
}

pub struct Deps {
	pub overview_receiver: block_overview::Receiver,
	pub data_receiver: block_data::Receiver,
	pub notifier: Arc<Notify>,
}

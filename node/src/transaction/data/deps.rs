use std::sync::Arc;

use jsonrpsee::tokio::sync::Notify;
use transaction_rpc::data_types::TxDataReceiver;

#[derive(Clone, Default)]
pub struct CliDeps {
	pub enabled: bool,
}

pub struct Deps {
	pub receiver: TxDataReceiver,
	pub notifier: Arc<Notify>,
}

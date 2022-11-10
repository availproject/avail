use jsonrpsee::{core::Error, proc_macros::rpc};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Health {
	pub is_syncing: bool,
	pub peers: u32,
	pub should_have_peers: bool,
}

#[rpc(client, namespace = "system")]
pub trait Rpc {
	#[method(name = "health")]
	async fn health(&self) -> Result<Health, Error>;
}

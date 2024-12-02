use crate::avail::runtime_types::da_runtime::primitives::SessionKeys;
use crate::{avail, AOnlineClient};

use super::Transaction;
use subxt::backend::rpc::reconnecting_rpc_client::RpcClient;

pub type SetKeysCall = avail::session::calls::types::SetKeys;

#[derive(Clone)]
pub struct Session {
	online_client: AOnlineClient,
	rpc_client: RpcClient,
}

impl Session {
	pub fn new(online_client: AOnlineClient, rpc_client: RpcClient) -> Self {
		Self {
			online_client,
			rpc_client,
		}
	}

	pub fn set_keys(&self, keys: SessionKeys) -> Transaction<SetKeysCall> {
		let payload = avail::tx().session().set_keys(keys, vec![]);
		Transaction::new(self.online_client.clone(), self.rpc_client.clone(), payload)
	}
}

use crate::AvailConfig;

use core::ops::Deref;
use subxt::{
	backend::{legacy::LegacyRpcMethods, rpc::RpcClient},
	utils::validate_url_is_secure,
	Error, OnlineClient,
};

#[derive(Debug)]
pub struct AvailClient {
	online: OnlineClient<AvailConfig>,
	rpc: RpcClient,
}

impl AvailClient {
	pub async fn new<U: AsRef<str>>(ws_uri: U) -> Result<Self, Error> {
		validate_url_is_secure(ws_uri.as_ref())?;
		let rpc = RpcClient::from_url(ws_uri).await?;
		let online = OnlineClient::<AvailConfig>::from_rpc_client(rpc.clone()).await?;

		Ok(AvailClient { online, rpc })
	}

	pub async fn new_insecure<U: AsRef<str>>(ws_uri: U) -> Result<Self, Error> {
		let rpc = RpcClient::from_insecure_url(ws_uri).await?;
		let online = OnlineClient::<AvailConfig>::from_rpc_client(rpc.clone()).await?;

		Ok(AvailClient { online, rpc })
	}

	pub fn legacy_rpc(&self) -> LegacyRpcMethods<AvailConfig> {
		LegacyRpcMethods::<AvailConfig>::new(self.rpc.clone())
	}

	pub fn rpc(&self) -> &RpcClient {
		&self.rpc
	}

	pub fn online(&self) -> &OnlineClient<AvailConfig> {
		&self.online
	}
}

impl Deref for AvailClient {
	type Target = OnlineClient<AvailConfig>;
	fn deref(&self) -> &Self::Target {
		&self.online
	}
}

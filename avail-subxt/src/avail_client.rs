use crate::AvailConfig;

use core::ops::Deref;
pub use jsonrpsee::core::client::Client as RpcMethods;
use subxt::{
	backend::{legacy::LegacyRpcMethods, rpc::RpcClient},
	utils::validate_url_is_secure,
	Error, OnlineClient,
};

#[derive(Debug)]
pub struct AvailClient {
	rpc_methods: RpcMethods,
	online: OnlineClient<AvailConfig>,
	rpc: RpcClient,
}

impl AvailClient {
	pub async fn new<U: AsRef<str>>(ws_uri: U) -> Result<Self, Error> {
		validate_url_is_secure(ws_uri.as_ref())?;
		let rpc_methods = jsonrpsee_helpers::client(ws_uri.as_ref())
			.await
			.map_err(|e| Error::Other(format!("Client cannot be created: {e:?}")))?;

		let rpc = RpcClient::from_url(ws_uri).await?;
		let online = OnlineClient::<AvailConfig>::from_rpc_client(rpc.clone()).await?;

		Ok(AvailClient {
			rpc_methods,
			online,
			rpc,
		})
	}

	pub async fn new_insecure<U: AsRef<str>>(ws_uri: U) -> Result<Self, Error> {
		let rpc_methods = jsonrpsee_helpers::client(ws_uri.as_ref())
			.await
			.map_err(|e| Error::Other(format!("Client cannot be created: {e:?}")))?;

		let rpc = RpcClient::from_insecure_url(ws_uri).await?;
		let online = OnlineClient::<AvailConfig>::from_rpc_client(rpc.clone()).await?;

		Ok(AvailClient {
			rpc_methods,
			online,
			rpc,
		})
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

	pub fn rpc_methods(&self) -> &RpcMethods {
		&self.rpc_methods
	}
}

impl Deref for AvailClient {
	type Target = OnlineClient<AvailConfig>;
	fn deref(&self) -> &Self::Target {
		&self.online
	}
}

// #[cfg(feature = "native")]
mod jsonrpsee_helpers {
	pub use jsonrpsee::{
		client_transport::ws::{self, EitherStream, Url, WsTransportClientBuilder},
		core::client::{Client, Error},
	};
	use tokio_util::compat::Compat;

	pub type Sender = ws::Sender<Compat<EitherStream>>;
	pub type Receiver = ws::Receiver<Compat<EitherStream>>;

	/// Build WS RPC client from URL
	pub async fn client(url: &str) -> Result<Client, Error> {
		let (sender, receiver) = ws_transport(url).await?;
		Ok(Client::builder()
			.max_buffer_capacity_per_subscription(4096)
			.build_with_tokio(sender, receiver))
	}

	async fn ws_transport(url: &str) -> Result<(Sender, Receiver), Error> {
		let url = Url::parse(url).map_err(|e| Error::Transport(e.into()))?;
		WsTransportClientBuilder::default()
			.build(url)
			.await
			.map_err(|e| Error::Transport(e.into()))
	}
}

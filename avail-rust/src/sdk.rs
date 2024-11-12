use std::str::FromStr;

use subxt::backend::rpc::RpcClient;
use subxt_signer::{sr25519::Keypair, SecretUri};

use crate::{rpcs::Rpc, transactions::Transactions, AOnlineClient};

#[derive(Clone)]
pub struct SDK {
	pub online_client: AOnlineClient,
	pub rpc_client: RpcClient,
	pub tx: Transactions,
	pub rpc: Rpc,
}

impl SDK {
	pub async fn new(endpoint: &str) -> Result<Self, Box<dyn std::error::Error>> {
		Self::new_inner(endpoint, true).await
	}

	pub async fn new_insecure(endpoint: &str) -> Result<Self, Box<dyn std::error::Error>> {
		Self::new_inner(endpoint, false).await
	}

	async fn new_inner(endpoint: &str, secure: bool) -> Result<Self, Box<dyn std::error::Error>> {
		let (online_client, rpc_client) = initialize_api(endpoint, secure).await?;

		let rpc = Rpc::new(rpc_client.clone()).await?;
		let tx = Transactions::new(online_client.clone(), rpc_client.clone());

		Ok(SDK {
			online_client,
			rpc_client,
			tx,
			rpc,
		})
	}

	fn alice() -> Result<Keypair, String> {
		let secret_uri = SecretUri::from_str("//Alice").map_err(|e| e.to_string())?;
		Keypair::from_uri(&secret_uri).map_err(|e| e.to_string())
	}
}

pub async fn initialize_api(
	endpoint: &str,
	secure: bool,
) -> Result<(AOnlineClient, RpcClient), Box<dyn std::error::Error>> {
	let rpc_client: RpcClient = match secure {
		true => RpcClient::from_url(endpoint).await?,
		false => RpcClient::from_insecure_url(endpoint).await?,
	};

	let rpc = Rpc::new(rpc_client.clone()).await?;
	// Cloning RpcClient is cheaper and doesn't create a new WS connection
	let api = AOnlineClient::from_rpc_client(rpc.client.clone()).await?;

	Ok((api, rpc_client))
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum WaitFor {
	BlockInclusion,
	BlockFinalization,
}

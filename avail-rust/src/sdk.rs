use std::{str::FromStr, time::Duration};

use subxt::backend::rpc::reconnecting_rpc_client::{ExponentialBackoff, RpcClient};
use subxt_signer::{sr25519::Keypair, SecretUri};

use crate::{error::ClientError, rpcs::Rpc, transactions::Transactions, AOnlineClient};

#[derive(Clone)]
pub struct SDK {
	pub online_client: AOnlineClient,
	pub rpc_client: RpcClient,
	pub tx: Transactions,
	pub rpc: Rpc,
}

impl SDK {
	pub async fn new(endpoint: &str) -> Result<Self, ClientError> {
		env_logger::builder().init();
		let (online_client, rpc_client) = initialize_api(endpoint).await?;

		let rpc = Rpc::new(rpc_client.clone()).await;
		let tx = Transactions::new(online_client.clone(), rpc_client.clone());

		Ok(SDK {
			online_client,
			rpc_client,
			tx,
			rpc,
		})
	}

	pub async fn new_custom(
		online_client: AOnlineClient,
		rpc_client: RpcClient,
		enable_logging: bool,
	) -> Result<Self, ClientError> {
		if enable_logging {
			env_logger::builder().init();
		}

		let rpc = Rpc::new(rpc_client.clone()).await;
		let tx = Transactions::new(online_client.clone(), rpc_client.clone());

		Ok(SDK {
			online_client,
			rpc_client,
			tx,
			rpc,
		})
	}

	fn enable_logging() {
		env_logger::builder().init();
	}

	pub fn alice() -> Result<Keypair, ClientError> {
		let secret_uri = SecretUri::from_str("//Alice")?;
		Ok(Keypair::from_uri(&secret_uri)?)
	}

	pub fn bob() -> Result<Keypair, ClientError> {
		let secret_uri = SecretUri::from_str("//Bob")?;
		Ok(Keypair::from_uri(&secret_uri)?)
	}

	pub fn charlie() -> Result<Keypair, ClientError> {
		let secret_uri = SecretUri::from_str("//Charlie")?;
		Ok(Keypair::from_uri(&secret_uri)?)
	}

	pub fn eve() -> Result<Keypair, ClientError> {
		let secret_uri = SecretUri::from_str("//Eve")?;
		Ok(Keypair::from_uri(&secret_uri)?)
	}

	pub fn one_avail() -> u128 {
		1_000_000_000_000_000_000u128
	}

	pub fn local_endpoint() -> &'static str {
		"ws://127.0.0.1:9944"
	}

	pub fn turing_endpoint() -> &'static str {
		"wss://turing-rpc.avail.so/ws"
	}

	pub fn mainnet_endpoint() -> &'static str {
		"wss://mainnet-rpc.avail.so/ws"
	}
}

pub async fn initialize_api(endpoint: &str) -> Result<(AOnlineClient, RpcClient), ClientError> {
	let rpc_client = RpcClient::builder()
		.retry_policy(
			ExponentialBackoff::from_millis(1000)
				.max_delay(Duration::from_secs(3))
				.take(3),
		)
		.build(endpoint)
		.await
		.map_err(|e| e.to_string())?;

	// Cloning RpcClient is cheaper and doesn't create a new WS connection
	let api = AOnlineClient::from_rpc_client(rpc_client.clone()).await?;

	Ok((api, rpc_client))
}

pub async fn initialize_api_custom(rpc_client: RpcClient) -> Result<AOnlineClient, ClientError> {
	// Cloning RpcClient is cheaper and doesn't create a new WS connection
	let online_client = AOnlineClient::from_rpc_client(rpc_client.clone()).await?;

	Ok(online_client)
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum WaitFor {
	BlockInclusion,
	BlockFinalization,
}

impl WaitFor {
	pub fn to_str(&self) -> &'static str {
		match self {
			WaitFor::BlockInclusion => "Block Inclusion",
			WaitFor::BlockFinalization => "Block Finalization",
		}
	}
}

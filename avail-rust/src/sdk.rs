use crate::{rpcs::Rpc, transactions::Transactions, utils::Util, Api};

#[derive(Clone)]
pub struct SDK {
	pub api: Api,
	pub tx: Transactions,
	pub util: Util,
	pub rpc: Rpc,
}

impl SDK {
	pub async fn new(endpoint: &str) -> Result<Self, Box<dyn std::error::Error>> {
		let rpc = Rpc::new(endpoint, true).await?;
		// Cloning RpcClient is cheaper and doesn't create a new WS connection.
		let api = Api::from_rpc_client(rpc.client.clone()).await?;

		Ok(SDK {
			tx: Transactions::new(api.clone(), rpc.clone()),
			util: Util::new(api.clone()),
			rpc,
			rpc,
			api,
		})
	}

	pub async fn new_insecure(endpoint: &str) -> Result<Self, Box<dyn std::error::Error>> {
		let rpc = Rpc::new(endpoint, false).await?;
		let api = Api::from_rpc_client(rpc.client.clone()).await?;

		Ok(SDK {
			tx: Transactions::new(api.clone(), rpc.clone()),
			util: Util::new(api.clone()),
			rpc,
			rpc,
			api,
		})
	}
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum WaitFor {
	BlockInclusion,
	BlockFinalization,
}

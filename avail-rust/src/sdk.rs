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
		let api = Api::from_url(endpoint).await?;
		let rpc = Rpc::new(endpoint, true).await?;

		Ok(SDK {
			tx: Transactions::new(api.clone(), rpc.clone()),
			util: Util::new(api.clone()),
			rpc,
			api,
		})
	}

	pub async fn new_insecure(endpoint: &str) -> Result<Self, Box<dyn std::error::Error>> {
		let api = Api::from_insecure_url(endpoint).await?;
		let rpc = Rpc::new(endpoint, false).await?;

		Ok(SDK {
			tx: Transactions::new(api.clone(), rpc.clone()),
			util: Util::new(api.clone()),
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

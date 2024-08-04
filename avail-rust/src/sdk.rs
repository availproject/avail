use crate::{rpcs::Rpc, transactions::Transactions, utils::Util, Api};

pub struct SDK {
	pub api: Api,
	pub tx: Transactions,
	pub util: Util,
	pub rpc: Rpc,
}

impl SDK {
	pub async fn new(endpoint: &str) -> Result<Self, Box<dyn std::error::Error>> {
		let api = Api::from_url(endpoint).await?;
		let tx = Transactions::new(api.clone());
		let util = Util::new(api.clone());
		let rpc = Rpc::new(endpoint).await?;

		Ok(SDK { api, tx, util, rpc })
	}
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum WaitFor {
	BlockInclusion,
	BlockFinalization,
}

use crate::{transactions::Transactions, utils::Util, Api};

pub struct SDK {
	pub api: Api,
	pub tx: Transactions,
	pub util: Util,
}

impl SDK {
	pub async fn new(endpoint: &str) -> Result<Self, Box<dyn std::error::Error>> {
		let api = Api::from_url(endpoint).await?;
		let tx = Transactions::new(api.clone());
		let util = Util::new(api.clone());

		Ok(SDK { api, tx, util })
	}
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum WaitFor {
	BlockInclusion,
	BlockFinalization,
}

use crate::{transactions::Transactions, Api};

pub struct SDK {
	pub api: Api,
	pub tx: Transactions,
}

impl SDK {
	pub async fn new(endpoint: &str) -> Result<Self, Box<dyn std::error::Error>> {
		let api = Api::from_url(endpoint).await?;
		let tx = Transactions::new(api.clone());

		Ok(SDK { api, tx })
	}
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum WaitFor {
	BlockInclusion,
	BlockFinalization,
}

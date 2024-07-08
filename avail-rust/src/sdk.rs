use subxt::blocks::BlocksClient;
use subxt::OnlineClient;

use crate::config::AvailConfig;
use crate::transactions::Transactions;

pub type Api = OnlineClient<AvailConfig>;
pub type AvailBlocksClient = BlocksClient<AvailConfig, Api>;

pub struct SDK {
	pub tx: Transactions,
}

impl SDK {
	pub async fn new(endpoint: &str) -> Result<Self, Box<dyn std::error::Error>> {
		let api = OnlineClient::<AvailConfig>::from_url(endpoint).await?;
		let tx = Transactions::new(api.clone());

		Ok(SDK { tx })
	}
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum WaitFor {
	BlockInclusion,
	BlockFinalization,
}

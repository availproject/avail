mod balances;
mod da;
mod nom_pools;
mod options;
mod session;
mod staking;

pub use balances::*;
pub use da::*;
pub use nom_pools::*;
pub use options::{Mortality, Nonce, Options};
pub use session::*;
pub use staking::*;

use super::{
	rpcs::Rpc, utils_raw::progress_transaction, Api, AvailBlocksClient, AvailConfig, BlockHash,
	TransactionInBlock, WaitFor,
};
use sp_core::H256;
use subxt::{blocks::ExtrinsicEvents, tx::TxProgress};

pub type Params =
	<<AvailConfig as subxt::Config>::ExtrinsicParams as subxt::config::ExtrinsicParams<
		AvailConfig,
	>>::Params;

#[derive(Clone)]
pub struct Transactions {
	pub balances: Balances,
	pub staking: Staking,
	pub data_availability: DataAvailability,
	pub session: Session,
	pub nomination_pools: NominationPools,
}

impl Transactions {
	pub fn new(api: Api, rpc_client: Rpc) -> Self {
		let tx = api.tx();
		let blocks = api.blocks();

		Self {
			balances: Balances::new(tx.clone(), rpc_client.clone(), blocks.clone()),
			staking: Staking::new(tx.clone(), rpc_client.clone(), blocks.clone()),
			data_availability: DataAvailability::new(
				tx.clone(),
				rpc_client.clone(),
				blocks.clone(),
			),
			session: Session::new(tx.clone(), rpc_client.clone(), blocks.clone()),
			nomination_pools: NominationPools::new(tx.clone(), rpc_client.clone(), blocks.clone()),
		}
	}
}

async fn block_and_tx_hash(
	tx_in_block: &TransactionInBlock,
	events: &ExtrinsicEvents<AvailConfig>,
	blocks: &AvailBlocksClient,
) -> Result<(H256, u32, H256, u32), String> {
	let tx_hash = tx_in_block.extrinsic_hash();
	let tx_index = events.extrinsic_index();
	let block_hash = tx_in_block.block_hash();
	let block_number = get_block_number(blocks, block_hash).await?;

	Ok((block_hash, block_number, tx_hash, tx_index))
}

async fn get_block_number(
	blocks: &AvailBlocksClient,
	block_hash: BlockHash,
) -> Result<u32, String> {
	let block_number = blocks
		.at(block_hash)
		.await
		.map_err(|e| e.to_string())?
		.number();

	Ok(block_number)
}

async fn progress_transaction_ex(
	maybe_tx_progress: Result<TxProgress<AvailConfig, Api>, subxt::Error>,
	wait_for: WaitFor,
	blocks: &AvailBlocksClient,
) -> Result<(ExtrinsicEvents<AvailConfig>, (H256, u32, H256, u32)), String> {
	let transaction = progress_transaction(maybe_tx_progress, wait_for).await;
	let tx_in_block = match transaction {
		Ok(tx_in_block) => tx_in_block,
		Err(message) => return Err(message),
	};

	let events = match tx_in_block.wait_for_success().await {
		Ok(e) => e,
		Err(error) => return Err(error.to_string()),
	};

	let data = block_and_tx_hash(&tx_in_block, &events, blocks).await?;

	Ok((events, data))
}

#[cfg(test)]
mod tests {
	use super::*;

	#[tokio::test]
	async fn testing_function() {
		let sdk = crate::sdk::SDK::new("ws://127.0.0.1:9944").await.unwrap();
		let h = BlockHash::from_str(
			"0x6c5ebed687ed008b76028072fe1ad0a06ecf3c00dd9067aa049ea14e180702f8",
		)
		.unwrap();
		match sdk.rpc.kate.query_rows(vec![0], Some(h)).await {
			Ok(a) => {
				dbg!(a);
			},
			Err(a) => {
				dbg!(a);
			},
		};
	}
}

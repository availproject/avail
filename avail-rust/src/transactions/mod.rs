mod balances;
mod da;
mod nom_pools;
mod options;
mod session;
mod staking;

pub use balances::*;
pub use da::*;
pub use nom_pools::*;
use options::from_options_to_params;
pub use options::{Mortality, Nonce, Options};
pub use session::*;
pub use staking::*;
use subxt_signer::sr25519::Keypair;

use crate::{
	rpcs::Rpc, utils_raw::progress_transaction, ABlocksClient, AOnlineClient, ATxClient,
	AvailConfig, TransactionInBlock, WaitFor, H256,
};
use subxt::{
	backend::rpc::RpcClient,
	blocks::{ExtrinsicEvents, StaticExtrinsic},
	ext::scale_encode::EncodeAsFields,
	tx::{DefaultPayload, TxProgress},
};

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
	pub fn new(api: AOnlineClient, rpc_client: Rpc) -> Self {
		let tx = api.tx();
		let blocks = api.blocks();

		Self {
			balances: Balances::new(tx.clone(), rpc_client.client.clone(), blocks.clone()),
			staking: Staking::new(tx.clone(), rpc_client.client.clone(), blocks.clone()),
			data_availability: DataAvailability::new(
				tx.clone(),
				rpc_client.client.clone(),
				blocks.clone(),
			),
			session: Session::new(tx.clone(), rpc_client.client.clone(), blocks.clone()),
			nomination_pools: NominationPools::new(
				tx.clone(),
				rpc_client.client.clone(),
				blocks.clone(),
			),
		}
	}
}

async fn block_and_tx_hash(
	tx_in_block: &TransactionInBlock,
	events: &ExtrinsicEvents<AvailConfig>,
	blocks: &ABlocksClient,
) -> Result<(H256, u32, H256, u32), String> {
	let tx_hash = tx_in_block.extrinsic_hash();
	let tx_index = events.extrinsic_index();
	let block_hash = tx_in_block.block_hash();
	let block_number = get_block_number(blocks, block_hash).await?;

	Ok((block_hash, block_number, tx_hash, tx_index))
}

async fn get_block_number(blocks: &ABlocksClient, block_hash: H256) -> Result<u32, String> {
	let block_number = blocks
		.at(block_hash)
		.await
		.map_err(|e| e.to_string())?
		.number();

	Ok(block_number)
}

async fn sign_and_submit_and_progress_transaction<T>(
	tx_client: &ATxClient,
	blocks_client: &ABlocksClient,
	rpc_client: &RpcClient,
	account: &Keypair,
	call: DefaultPayload<T>,
	wait_for: WaitFor,
	options: Option<Options>,
) -> Result<TxResultDetails, String>
where
	T: StaticExtrinsic + EncodeAsFields,
{
	let account_id = account.public_key().to_account_id();
	let params = from_options_to_params(rpc_client, blocks_client, &account_id, options).await?;

	let maybe_tx_progress = tx_client
		.sign_and_submit_then_watch(&call, account, params)
		.await;
	let details = progress_transaction_ex(blocks_client, maybe_tx_progress, wait_for).await?;

	Ok(details)
}

async fn progress_transaction_ex(
	client: &ABlocksClient,
	maybe_tx_progress: Result<TxProgress<AvailConfig, AOnlineClient>, subxt::Error>,
	wait_for: WaitFor,
) -> Result<TxResultDetails, String> {
	let transaction = progress_transaction(maybe_tx_progress, wait_for).await;
	let tx_in_block = match transaction {
		Ok(tx_in_block) => tx_in_block,
		Err(message) => return Err(message),
	};

	let events = match tx_in_block.wait_for_success().await {
		Ok(e) => e,
		Err(error) => return Err(error.to_string()),
	};

	let (block_hash, block_number, tx_hash, tx_index) =
		block_and_tx_hash(&tx_in_block, &events, client).await?;
	let result = TxResultDetails::new(
		tx_in_block,
		events,
		tx_hash,
		tx_index,
		block_hash,
		block_number,
	);

	Ok(result)
}

#[derive(Debug)]
pub struct TxResultDetails {
	pub tx_result: TransactionInBlock,
	pub events: ExtrinsicEvents<AvailConfig>,
	pub tx_hash: H256,
	pub tx_index: u32,
	pub block_hash: H256,
	pub block_number: u32,
}

impl TxResultDetails {
	pub fn new(
		tx_result: TransactionInBlock,
		events: ExtrinsicEvents<AvailConfig>,
		tx_hash: H256,
		tx_index: u32,
		block_hash: H256,
		block_number: u32,
	) -> Self {
		Self {
			tx_result,
			events,
			tx_hash,
			tx_index,
			block_hash,
			block_number,
		}
	}

	pub async fn fetch_block(
		&self,
		block_client: &ABlocksClient,
	) -> Result<crate::block::Block, subxt::Error> {
		crate::block::Block::new(block_client, self.block_hash).await
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[tokio::test]
	async fn testing_function() {
		let sdk = crate::sdk::SDK::new("ws://127.0.0.1:9944").await.unwrap();
		let h =
			H256::from_str("0x6c5ebed687ed008b76028072fe1ad0a06ecf3c00dd9067aa049ea14e180702f8")
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

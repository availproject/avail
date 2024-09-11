use crate::avail::runtime_types::da_runtime::primitives::SessionKeys;
use crate::rpcs::Rpc;
use crate::sdk::WaitFor;
use crate::utils_raw::fetch_transaction;
use crate::{avail, AvailBlocksClient, AvailConfig, BlockHash, TxApi};

use subxt::blocks::ExtrinsicEvents;
use subxt_signer::sr25519::Keypair;

use super::options::from_options_to_params;
use super::{options::Options, progress_transaction_ex};

use avail::session::calls::types as SessionCalls;

#[derive(Debug)]
pub struct SetKeysTxSuccess {
	pub events: ExtrinsicEvents<AvailConfig>,
	pub tx_data: SessionCalls::SetKeys,
	pub tx_hash: BlockHash,
	pub tx_index: u32,
	pub block_hash: BlockHash,
	pub block_number: u32,
}

#[derive(Clone)]
pub struct Session {
	api: TxApi,
	rpc_client: Rpc,
	blocks: AvailBlocksClient,
}

impl Session {
	pub fn new(api: TxApi, rpc_client: Rpc, blocks: AvailBlocksClient) -> Self {
		Self {
			api,
			rpc_client,
			blocks,
		}
	}

	pub async fn set_keys(
		&self,
		keys: SessionKeys,
		wait_for: WaitFor,
		account: &Keypair,
		options: Option<Options>,
	) -> Result<SetKeysTxSuccess, String> {
		let account_id = account.public_key().to_account_id();
		let params =
			from_options_to_params(options, &self.rpc_client, account_id, &self.blocks).await?;
		let call = avail::tx().session().set_keys(keys, vec![]);

		let maybe_tx_progress = self
			.api
			.sign_and_submit_then_watch(&call, account, params)
			.await;

		let (events, data) =
			progress_transaction_ex(maybe_tx_progress, wait_for, &self.blocks).await?;
		let (block_hash, block_number, tx_hash, tx_index) = data;

		let tx_data = tx_data_session_set_keys(block_hash, tx_hash, &self.blocks).await?;

		Ok(SetKeysTxSuccess {
			events,
			tx_data,
			tx_hash,
			tx_index,
			block_hash,
			block_number,
		})
	}
}

pub async fn tx_data_session_set_keys(
	block_hash: BlockHash,
	tx_hash: BlockHash,
	blocks: &AvailBlocksClient,
) -> Result<SessionCalls::SetKeys, String> {
	let transaction = fetch_transaction::<SessionCalls::SetKeys>(block_hash, tx_hash, blocks).await;
	let transaction = transaction.map_err(|err| err.to_string())?;
	Ok(transaction.value)
}

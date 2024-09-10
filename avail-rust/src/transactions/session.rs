use crate::avail::runtime_types::da_runtime::primitives::SessionKeys;
use crate::sdk::WaitFor;
use crate::{avail, transaction_data, AvailBlocksClient, AvailConfig, BlockHash, TxApi};

use subxt::blocks::ExtrinsicEvents;
use subxt_signer::sr25519::Keypair;

use super::{progress_transaction_ex, Params};

#[derive(Debug)]
pub struct SetKeysTxSuccess {
	pub events: ExtrinsicEvents<AvailConfig>,
	pub tx_data: transaction_data::session::SetKeys,
	pub tx_hash: BlockHash,
	pub tx_index: u32,
	pub block_hash: BlockHash,
	pub block_number: u32,
}

#[derive(Clone)]
pub struct Session {
	api: TxApi,
	blocks: AvailBlocksClient,
}

impl Session {
	pub fn new(api: TxApi, blocks: AvailBlocksClient) -> Self {
		Self { api, blocks }
	}

	pub async fn set_keys(
		&self,
		keys: SessionKeys,
		wait_for: WaitFor,
		account: &Keypair,
		options: Option<Params>,
	) -> Result<SetKeysTxSuccess, String> {
		let params = options.unwrap_or_default();
		let call = avail::tx().session().set_keys(keys, vec![]);

		let maybe_tx_progress = self
			.api
			.sign_and_submit_then_watch(&call, account, params)
			.await;

		let (events, data) =
			progress_transaction_ex(maybe_tx_progress, wait_for, &self.blocks).await?;
		let (block_hash, block_number, tx_hash, tx_index) = data;

		let tx_data =
			transaction_data::session::SetKeys::new(block_hash, tx_hash, &self.blocks).await?;

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

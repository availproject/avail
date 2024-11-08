use crate::avail::runtime_types::da_runtime::primitives::SessionKeys;
use crate::sdk::WaitFor;
use crate::utils_raw::fetch_transaction;
use crate::{avail, ABlocksClient, ATxClient, H256};

use subxt::backend::rpc::RpcClient;
use subxt_signer::sr25519::Keypair;

use super::{options::Options, sign_and_submit_and_progress_transaction, TxResultDetails};

use avail::session::calls::types as SessionCalls;

#[derive(Debug)]
pub struct SetKeysTxSuccess {
	pub data: SessionCalls::SetKeys,
	pub details: TxResultDetails,
}

#[derive(Clone)]
pub struct Session {
	tx_client: ATxClient,
	blocks_client: ABlocksClient,
	rpc_client: RpcClient,
}

impl Session {
	pub fn new(tx_client: ATxClient, rpc_client: RpcClient, blocks_client: ABlocksClient) -> Self {
		Self {
			tx_client,
			blocks_client,
			rpc_client,
		}
	}

	pub async fn set_keys(
		&self,
		keys: SessionKeys,
		wait_for: WaitFor,
		account: &Keypair,
		options: Option<Options>,
	) -> Result<SetKeysTxSuccess, String> {
		let call = avail::tx().session().set_keys(keys, vec![]);
		let details = sign_and_submit_and_progress_transaction(
			&self.tx_client,
			&self.blocks_client,
			&self.rpc_client,
			account,
			call,
			wait_for,
			options,
		)
		.await?;

		let data =
			tx_data_session_set_keys(&self.blocks_client, details.block_hash, details.tx_hash)
				.await?;

		Ok(SetKeysTxSuccess { data, details })
	}
}

pub async fn tx_data_session_set_keys(
	client: &ABlocksClient,
	block_hash: H256,
	tx_hash: H256,
) -> Result<SessionCalls::SetKeys, String> {
	let transaction = fetch_transaction::<SessionCalls::SetKeys>(client, block_hash, tx_hash).await;
	let transaction = transaction.map_err(|err| err.to_string())?;
	Ok(transaction.value)
}

use crate::avail::runtime_types::da_runtime::primitives::SessionKeys;
use crate::sdk::WaitFor;
use crate::{avail, AOnlineClient};

use subxt::backend::rpc::RpcClient;
use subxt_signer::sr25519::Keypair;

use super::{options::Options, progress_and_parse_transaction, TransactionDetails};

use avail::session::calls::types as SessionCalls;

#[derive(Debug)]
pub struct SetKeysTxSuccess {
	pub data: SessionCalls::SetKeys,
	pub details: TransactionDetails,
}

#[derive(Clone)]
pub struct Session {
	online_client: AOnlineClient,
	rpc_client: RpcClient,
}

impl Session {
	pub fn new(online_client: AOnlineClient, rpc_client: RpcClient) -> Self {
		Self {
			online_client,
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
		let details = progress_and_parse_transaction(
			&self.online_client,
			&self.rpc_client,
			account,
			call,
			wait_for,
			options,
		)
		.await?;

		let block = details.fetch_block(&self.online_client).await;
		let block = block.map_err(|e| e.to_string())?;
		let data = block.transaction_by_index_static::<SessionCalls::SetKeys>(details.tx_index);
		let data = data
			.ok_or(String::from("Failed to find transaction data"))?
			.value;

		Ok(SetKeysTxSuccess { data, details })
	}
}

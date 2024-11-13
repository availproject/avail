use crate::avail::runtime_types::da_runtime::primitives::SessionKeys;
use crate::sdk::WaitFor;
use crate::{avail, AOnlineClient};

use subxt::backend::rpc::RpcClient;
use subxt_signer::sr25519::Keypair;

use super::{find_data_or_return_error, TransactionFailed};
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
	) -> Result<SetKeysTxSuccess, TransactionFailed> {
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

		let data = find_data_or_return_error::<SessionCalls::SetKeys>(
			&self.online_client,
			"Failed to find Session::SetKeys data",
			&details,
		)
		.await?;

		Ok(SetKeysTxSuccess { data, details })
	}
}

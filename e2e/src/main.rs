fn main() {}

#[cfg(test)]
mod tests {
	use anyhow::{anyhow, Result};
	use avail_core::currency::Balance;
	use avail_subxt::{api, AccountId, AvailClient, AvailConfig};
	use subxt::{tx::Signer, OnlineClient};

	/// Returns an Avail Client on local connection.
	pub async fn local_connection() -> Result<AvailClient> {
		let ws = String::from("ws://127.0.0.1:9944");
		AvailClient::new(ws)
			.await
			.map_err(|e| anyhow!("Client cannot be connected: {e:?}"))
	}

	/// Returns the free balance of `signer`
	pub async fn free_balance_of<S>(
		client: &OnlineClient<AvailConfig>,
		signer: &S,
	) -> Result<Balance>
	where
		S: Signer<AvailConfig>,
	{
		let acc: AccountId = signer.account_id();
		let query = api::storage().system().account(acc.clone());
		let acc_info = client
			.storage()
			.at_latest()
			.await?
			.fetch(&query)
			.await?
			.ok_or_else(|| anyhow!("Missing account({acc:?}) info"))?;

		Ok(acc_info.data.free)
	}

	mod accounts_from_mnemonics;
	mod create_app_key;
	mod download_digest_items;
	mod headers;
	mod max_block_submit;
	mod query_proof;
	mod rpc_queries;
	mod submit_block_length_proposal;
	mod submit_data;
	mod vector_send_msg;
}

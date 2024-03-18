use crate::{api, avail::TxProgress, tx, AppId, AvailConfig, BoundedVec};
use subxt::{tx::Signer as SignerT, Error, OnlineClient};

/// It submits `data` using the `app_id` as application ID.
/// It does not wait for the transaction to be included in a block.
pub async fn submit_data<S, ID, D>(
	client: &OnlineClient<AvailConfig>,
	signer: &S,
	data: D,
	app_id: ID,
) -> Result<TxProgress, Error>
where
	ID: Into<AppId>,
	D: Into<Vec<u8>>,
	S: SignerT<AvailConfig>,
{
	let data = BoundedVec(data.into());
	let call = api::tx().data_availability().submit_data(data);
	tx::send(client, &call, signer, app_id).await
}

pub async fn submit_data_with_nonce<S, ID, D>(
	client: &OnlineClient<AvailConfig>,
	signer: &S,
	data: D,
	app_id: ID,
	nonce: u64,
) -> Result<TxProgress, Error>
where
	ID: Into<AppId>,
	D: Into<Vec<u8>>,
	S: SignerT<AvailConfig>,
{
	let data = BoundedVec(data.into());
	let call = api::tx().data_availability().submit_data(data);
	tx::send_with_nonce(client, &call, signer, app_id, nonce).await
}

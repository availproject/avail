use crate::{
	avail::{TxInBlock, TxProgress, TxStatus},
	primitives::new_params_from_app_id,
	AccountId, AppId, AvailConfig,
};

use subxt::{
	tx::{Signer as SignerT, TxPayload},
	Error, OnlineClient,
};

pub async fn in_finalized(progress: TxProgress) -> Result<TxInBlock, Error> {
	progress.wait_for_finalized().await
}

pub async fn send_then_finalized<C, S, A>(
	client: &OnlineClient<AvailConfig>,
	call: &C,
	signer: &S,
	app_id: A,
) -> Result<TxInBlock, Error>
where
	C: TxPayload,
	S: SignerT<AvailConfig>,
	A: Into<AppId>,
{
	in_finalized(send(client, call, signer, app_id).await?).await
}

pub async fn send_then_in_block<C, S, A>(
	client: &OnlineClient<AvailConfig>,
	call: &C,
	signer: &S,
	app_id: A,
) -> Result<TxInBlock, Error>
where
	C: TxPayload,
	S: SignerT<AvailConfig>,
	A: Into<AppId>,
{
	let progress = send(client, call, signer, app_id).await?;
	then_in_block(progress).await
}

pub async fn then_in_block(mut progress: TxProgress) -> Result<TxInBlock, Error> {
	let in_block = loop {
		let status = progress.next().await.transpose()?;
		if let Some(TxStatus::InBestBlock(in_block)) = status {
			break in_block;
		}
	};
	Ok(in_block)
}

pub async fn then_in_finalized_block(mut progress: TxProgress) -> Result<TxInBlock, Error> {
	let in_block = loop {
		let status = progress.next().await.transpose()?;
		if let Some(TxStatus::InFinalizedBlock(in_block)) = status {
			break in_block;
		}
	};
	Ok(in_block)
}

pub async fn send<C, S, A>(
	client: &OnlineClient<AvailConfig>,
	call: &C,
	signer: &S,
	app_id: A,
) -> Result<TxProgress, Error>
where
	C: TxPayload,
	S: SignerT<AvailConfig>,
	A: Into<AppId>,
{
	let params = new_params_from_app_id(app_id.into());
	client
		.tx()
		.sign_and_submit_then_watch(call, signer, params)
		.await
}

pub async fn send_with_nonce<C, S, A>(
	client: &OnlineClient<AvailConfig>,
	call: &C,
	signer: &S,
	app_id: A,
	nonce: u64,
) -> Result<TxProgress, Error>
where
	C: TxPayload,
	S: SignerT<AvailConfig>,
	A: Into<AppId>,
{
	let params = new_params_from_app_id(app_id.into());
	client
		.tx()
		.create_signed_with_nonce(call, signer, nonce, params)?
		.submit_and_watch()
		.await
}

pub async fn nonce<S>(client: &OnlineClient<AvailConfig>, signer: &S) -> Result<u64, Error>
where
	S: SignerT<AvailConfig>,
{
	let acc_id: AccountId = signer.account_id();
	client.tx().account_nonce(&acc_id).await
}

use super::{alice_nonce, allow_concurrency, local_connection};

use avail_core::AppId;
use avail_subxt::{
	api,
	api::{
		data_availability::calls::types::SubmitData,
		runtime_types::bounded_collections::bounded_vec::BoundedVec,
	},
	primitives::CheckAppId,
	tx,
};
use subxt_signer::sr25519::dev;

use anyhow::Result;
use std::sync::atomic::Ordering::Relaxed;
use test_log::test;
use tracing::trace;

/// This example demonstrates submitting data in a block, and retrieving the data and the AppId from the block hash.
#[test(tokio::test)]
async fn test() -> Result<()> {
	let _cg = allow_concurrency("retrieve_data_hash").await;
	let client = local_connection().await?;

	// Send the data and get the block hash
	let alice = dev::alice();
	let call = api::tx()
		.data_availability()
		.submit_data(BoundedVec(b"test_data".to_vec()));

	let nonce = alice_nonce().await.fetch_add(1, Relaxed);
	let tx_progress = tx::send_with_nonce(&client, &call, &alice, AppId(1), nonce).await?;
	let block_hash = tx::then_in_block(tx_progress).await?.block_hash();

	// Retrieve the data from the block hash
	let block = client.blocks().at(block_hash).await?;
	let extrinsics = block.extrinsics().await?;
	let da_submissions = extrinsics.find::<SubmitData>();
	let mut found = false;
	for da_submission in da_submissions {
		let da_submission = da_submission?;
		found = true;

		let submitted_data = String::from_utf8(da_submission.value.data.0)?;

		let app_id = da_submission
			.details
			.signed_extensions()
			.unwrap()
			.find::<CheckAppId>()
			.unwrap()
			.unwrap();

		trace!(
			"DA submission found in block: {} \nSubmitted Data: {:?} \nApp Id: {:?}",
			block_hash,
			submitted_data,
			app_id
		);
	}

	if !found {
		trace!("No DA submission found in block: {}", block_hash);
	}

	Ok(())
}

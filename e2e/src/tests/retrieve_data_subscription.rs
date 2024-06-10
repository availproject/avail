use super::local_connection;

use avail_subxt::{api::data_availability::calls::types::SubmitData, primitives::CheckAppId};

use anyhow::Result;
use test_log::test;
use tracing::trace;

/// This example demonstrates how to listen to blocks (we'll listen to 5 blocks) and list data submissions and their AppIds.
/// By default, no data submission will be made, to see something displayed here, you need to send data blobs with your local node.
#[test(tokio::test)]
async fn test() -> Result<()> {
	let client = local_connection().await?;

	let total_blocks = 5;
	let mut block_count = 0;
	let mut blocks_sub = client.blocks().subscribe_finalized().await?;

	while let Some(block) = blocks_sub.next().await {
		let block = block?;
		let block_hash = block.hash();
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

		block_count += 1;
		if block_count >= total_blocks {
			break;
		}
	}

	Ok(())
}

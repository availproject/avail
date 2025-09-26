mod infinity_da_automatic_test;
mod infinity_da_test;
mod max_block_submit;

use std::time::Duration;

use avail_rust::prelude::*;
use avail_rust_core::rpc::Error as RpcError;

#[tokio::main]
async fn main() -> Result<(), RpcError> {
	// Uncomment the following two lines for automatic tests when a node is launched
	// max_block_submit::run().await?;
	// infinity_da_automatic_test::run().await?;

	// This is a manual test
	infinity_da_test::run().await?;

	Ok(())
}

pub async fn wait_for_new_block(client: &Client) -> Result<(), RpcError> {
	let current_block = client.best().block_height().await?;
	loop {
		let new_block = client.best().block_height().await?;
		if current_block != new_block {
			break Ok(());
		}

		tokio::time::sleep(Duration::from_secs(1)).await;
	}
}

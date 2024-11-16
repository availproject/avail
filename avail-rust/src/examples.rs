#[cfg(test)]
mod tests {
	use super::*;
	use crate::{account::Account, avail, block::Block, rpcs::get_best_block};

	#[tokio::test]
	async fn testing_function() {
		let sdk = crate::sdk::SDK::new("ws://127.0.0.1:9944").await.unwrap();

		let block = Block::new_best_block(&sdk.online_client, &sdk.rpc_client)
			.await
			.unwrap();
		let address = avail::storage().data_availability().next_app_id();
		let storage_value = block.storage_fetch_or_default(&address).await.unwrap();

		dbg!(storage_value);
	}
}

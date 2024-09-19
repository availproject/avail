//use crate::params::*;
use sdk_client::{
	block_watcher::BlockWatcher,
	core::{
		crypto::{Keypair, SecretUri},
		types::avail,
	},
	rpc,
};
use sdk_client::{http::Client, params::Extra};
use std::str::FromStr;

pub fn main() {
	let rt = tokio::runtime::Builder::new_current_thread()
		.enable_all()
		.build()
		.unwrap();

	let client = Client::new("http://127.0.0.1:9944").unwrap();
	let block_watcher = BlockWatcher::new(client.client.clone());

	let task_1 = rt.spawn(run_client(client.clone(), block_watcher.clone()));
	let _ = rt.spawn(run_watcher(block_watcher.clone()));

	rt.block_on(async move {
		task_1.await.unwrap();
	});
}

async fn run_client(client: Client, watcher: BlockWatcher) {
	let secret_uri = SecretUri::from_str("//Alice").unwrap();
	let account = Keypair::from_uri(&secret_uri).unwrap();
	let account_id = account.account_id();

	/* 	let a = rpc::fetch_block2(&client.client, None).await.unwrap();
	dbg!(a); */

	let a = rpc::state_query_storage_events(&client.client, None)
		.await
		.unwrap();
	dbg!(a);

	/* 		let a = sdk_client::rpc::fetch_block(
		&client.0,
		Some(
			H256::from_hex_string(
				"0x3fb2cacead655e3ae5dbb79a077e0b0ec57032a7151c60430f2cba962e396a8f",
			)
			.unwrap(),
		),
	)
	.await
	.unwrap(); */

	/* 	let data = String::from("aabbcc");
	let call = avail::DataAvailabilityCalls::submit_data(data.as_bytes().to_vec());
	let extra = Extra::new();
	let payload = client.build_payload(call, account_id, extra).await.unwrap();

	let signature = payload.sign(&account);
	let tx = client.sign(&payload, account_id.clone(), signature);
	let tx_hash = client.submit_transaction(tx).await.unwrap();
	dbg!(tx_hash.to_hex_string());

	watcher.wait_block_inclusion(&tx_hash).await;
	dbg!("DOne"); */
}

async fn run_watcher(mut watcher: BlockWatcher) {
	watcher.run().await;
}

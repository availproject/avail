//use crate::params::*;
use sdk_client::core::{
	crypto::{Keypair, SecretUri},
	types::avail,
};
use sdk_client::{http::Client, params::Extra};
use sdk_core::types::H256;
use std::str::FromStr;

pub fn main() {
	let rt = tokio::runtime::Builder::new_current_thread()
		.enable_all()
		.build()
		.unwrap();

	rt.block_on(async {
		let secret_uri = SecretUri::from_str("//Alice").unwrap();
		let account = Keypair::from_uri(&secret_uri).unwrap();
		let account_id = account.account_id();
		let client = Client::new("http://127.0.0.1:9944").unwrap();

		let a = sdk_client::rpc::fetch_block_header(
			&client.0,
			Some(
				H256::from_hex_string(
					"0x38c790ceaa28ef7889e870c381ba1b021403cd427c4e05cb6e6c2c4b7167d06b",
				)
				.unwrap(),
			),
		)
		.await
		.unwrap();

		println!("{}", a.digest.to_human_readable());
		dbg!(a.extension);

		/* 		let data = String::from("aabbcc");
		let call = avail::DataAvailabilityCalls::submit_data(data.as_bytes().to_vec());
		let extra = Extra::new();
		let payload = client.build_payload(call, account_id, extra).await.unwrap();

		let signature = payload.sign(&account);
		let tx = client.sign(&payload, account_id.clone(), signature);
		let tx_hash = client.submit_transaction(tx).await.unwrap();
		dbg!(tx_hash.to_hex_string()); */
	});
}

//use crate::params::*;
use sdk_client::core::{
	crypto::{Keypair, SecretUri},
	types::avail,
};
use sdk_client::{http::Client, params::Extra};
use std::str::FromStr;

pub fn main() {
	let rt = tokio::runtime::Builder::new_current_thread()
		.enable_all()
		.build()
		.unwrap();

	rt.block_on(async {
		let a = String::from("aabbcc");
		let call = avail::DataAvailabilityCalls::create_application_key(a.as_bytes().to_vec());

		let secret_uri = SecretUri::from_str("//Alice").unwrap();
		let account = Keypair::from_uri(&secret_uri).unwrap();
		let account_id = account.account_id();
		let client = Client::new("http://127.0.0.1:9944").unwrap();

		let a = Extra::new();
		let payload = client.build_payload(call, account_id, a).await.unwrap();
		let signature = payload.sign(&account);
		let tx = client.sign(&payload, account_id.clone(), signature);
		dbg!(tx.get_hash().to_hex_string());
		let a = client.submit_transaction(tx).await.unwrap();
		dbg!(a.to_hex_string());
	});
}

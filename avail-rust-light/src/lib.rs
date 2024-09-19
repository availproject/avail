mod client;
mod core;
mod test {
	use super::*;

	#[test]
	pub fn something() {
		use crate::core::{Keypair, SecretUri};
		use client::http::Client;
		use client::params::*;
		use std::str::FromStr;

		let rt = tokio::runtime::Builder::new_current_thread()
			.enable_all()
			.build()
			.unwrap();

		rt.block_on(async {
			let a = String::from("aabbcc");
			let call = core::types::avail::calls::data_availability::create_application_key(
				a.as_bytes().to_vec(),
			);

			let secret_uri = SecretUri::from_str("//Alice").unwrap();
			let account = Keypair::from_uri(&secret_uri).unwrap();
			let account_id = account.public_key().to_account_id();
			let client = Client::new("http://127.0.0.1:9944").unwrap();

			let a = Extra::new();
			let payload = client.build_payload(call, account_id, a).await.unwrap();
			let signature = payload.sign(&account);
			let tx = client.sign(&payload, account.public_key(), signature);
			dbg!(tx.get_hash().to_hex_string());
			let a = client.rpc_author_submit_extrinsic(tx.data).await.unwrap();
			dbg!(a.to_hex_string());
		});
	}
}

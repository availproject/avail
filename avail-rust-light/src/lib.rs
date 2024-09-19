mod client;
mod crypto;
mod interface;
mod rpc;
mod transaction;

use jsonrpsee_core::{traits::ToRpcParams, JsonRawValue};
use parity_scale_codec::Encode;
use serde::{Deserialize, Serialize};

pub fn decode_hex(s: &str) -> Result<Vec<u8>, ()> {
	if s.len() % 2 != 0 {
		return Err(());
	}

	let result: Result<Vec<u8>, _> = (0..s.len())
		.step_by(2)
		.map(|i| u8::from_str_radix(&s[i..i + 2], 16))
		.collect();

	result.map_err(|_| ())
}

#[derive(Deserialize, Debug)]
pub struct RuntimeVersion {
	#[serde(rename = "specName")]
	pub spec_name: String,
	#[serde(rename = "implName")]
	pub impl_name: String,
	#[serde(rename = "authoringVersion")]
	pub authoring_version: u32,
	#[serde(rename = "specVersion")]
	pub spec_version: u32,
	#[serde(rename = "implVersion")]
	pub impl_version: u32,
	pub apis: Vec<(String, u32)>,
	#[serde(rename = "transactionVersion")]
	pub transaction_version: u32,
	#[serde(rename = "stateVersion")]
	pub state_version: u8,
}

pub type BlockNumber = u32;

pub type Bytes = Vec<u8>;

struct Params(pub Option<Box<JsonRawValue>>);

impl ToRpcParams for Params {
	fn to_rpc_params(self) -> Result<Option<Box<JsonRawValue>>, serde_json::Error> {
		Ok(self.0)
	}
}

mod test {
	use super::*;
	use client::http::Client;
	use crypto::{Keypair, SecretUri, Ss58Codec};
	use std::str::FromStr;
	use transaction::{ExtrinsicExtra, Mortality};

	#[test]
	pub fn something() {
		let rt = tokio::runtime::Builder::new_current_thread()
			.enable_all()
			.build()
			.unwrap();

		rt.block_on(async {
			let a = String::from("aabbcc");
			let call =
				interface::calls::data_availability::create_application_key(a.as_bytes().to_vec());

			let secret_uri = SecretUri::from_str("//Alice").unwrap();
			let account = Keypair::from_uri(&secret_uri).unwrap();
			let account_id = account.public_key().to_account_id();
			let client = Client::new("http://127.0.0.1:9944");

			let a = ExtrinsicExtra::new();

			let payload = client.build_payload(call, account_id, a).await;
			let signature = payload.sign(&account);
			let tx = client.sign(&payload, account.public_key(), signature);
			dbg!(tx.get_hash().to_hex_string());
			let a = client.rpc_author_submit_extrinsic(tx.data).await;
			dbg!(a.to_hex_string());
		});

		/* 	let a = String::from("aabbcc");

		let pallet_index: u8 = 29;
		let call_index: u8 = 0;
		let field = CallFields::DACreateKey(a.as_bytes().to_vec());
		dbg!(pallet_index.encode());
		dbg!(call_index.encode());
		dbg!(field.encode());

		let c = Call {
			pallet_index,
			call_index,
			fields: field,
		};
		let cc = c.encode();
		dbg!(&cc);
		dbg!(hex::encode(&cc));
		println!("{:02X?}", cc); */
	}
}

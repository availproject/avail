use super::{AlreadyEncoded, H256};
use crate::types::payload_fields::Call;
use parity_scale_codec::Encode;
use serde::{Deserialize, Deserializer};

pub use super::payload_fields::Period;
pub use super::payload_fields::Phase;
pub use calls::data_availability as DataAvailabilityCalls;

pub type Tip = u128;
pub type Nonce = u32;
pub type AppId = u32;
pub type BlockNumber = u32;
pub type BlockHeader = block::Header;

#[repr(u8)]
pub enum Pallet {
	DataAvailability = 29,
}

pub mod calls {
	use super::*;
	pub mod data_availability {
		use super::*;

		#[repr(u8)]
		pub enum Dispatchable {
			CreateApplicationKey = 0,
			SubmitData = 1,
		}

		pub fn create_application_key(key: Vec<u8>) -> Call {
			Call::new(
				Pallet::DataAvailability as u8,
				Dispatchable::CreateApplicationKey as u8,
				AlreadyEncoded(key.encode()),
			)
		}

		pub fn submit_data(data: Vec<u8>) -> Call {
			Call::new(
				Pallet::DataAvailability as u8,
				Dispatchable::SubmitData as u8,
				AlreadyEncoded(data.encode()),
			)
		}
	}
}

pub mod block {
	use super::*;

	#[derive(Debug, Clone, Deserialize)]
	#[serde(rename_all = "camelCase")]
	pub struct Header {
		#[serde(deserialize_with = "block_hash_from_string")]
		pub parent_hash: H256,
		#[serde(deserialize_with = "number_from_hex")]
		pub number: BlockNumber,
		#[serde(deserialize_with = "block_hash_from_string")]
		pub state_root: H256,
		#[serde(deserialize_with = "block_hash_from_string")]
		pub extrinsics_root: H256,
		#[serde(skip_deserializing)]
		pub digest: Option<u32>,
		#[serde(skip_deserializing)]
		pub extension: Option<u32>,
	}

	fn block_hash_from_string<'de, D>(deserializer: D) -> Result<H256, D::Error>
	where
		D: Deserializer<'de>,
	{
		let buf = String::deserialize(deserializer)?;
		Ok(H256::from_hex_string(&buf).unwrap())
	}

	fn number_from_hex<'de, D>(deserializer: D) -> Result<u32, D::Error>
	where
		D: Deserializer<'de>,
	{
		let buf = String::deserialize(deserializer)?;
		let without_prefix = buf.trim_start_matches("0x");
		Ok(u32::from_str_radix(without_prefix, 16).unwrap())
	}
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

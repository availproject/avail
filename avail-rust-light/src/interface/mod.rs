use crate::crypto::blake2_256;
use crate::transaction::AlreadyEncoded;
use crate::transaction::Call;
use crate::BlockNumber;
use crate::Bytes;
use parity_scale_codec::Encode;
use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;

pub use H256 as BlockHash;
pub use H256 as TransactionHash;

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct H256(pub [u8; 32]);
impl H256 {
	pub fn to_hex_string(&self) -> String {
		std::format!("0x{}", hex::encode(self.0))
	}

	pub fn from_hex_string(mut s: &str) -> Result<Self, ()> {
		if s.starts_with("0x") {
			s = &s[2..];
		}

		if s.len() != 64 {
			return Err(());
		}

		let block_hash = hex::decode(s).unwrap();

		if block_hash.len() != 32 {
			return Err(());
		}

		let block_hash = TryInto::<[u8; 32]>::try_into(block_hash).map_err(|_| ())?;

		Ok(H256(block_hash))
	}
}
impl Encode for H256 {
	fn size_hint(&self) -> usize {
		self.0.size_hint()
	}

	fn encode_to<T: parity_scale_codec::Output + ?Sized>(&self, dest: &mut T) {
		self.0.encode_to(dest);
	}
}

#[derive(Debug, Clone)]
pub struct OpaqueTransaction {
	pub data: AlreadyEncoded,
}
impl OpaqueTransaction {
	pub fn new(data: AlreadyEncoded) -> Self {
		Self { data }
	}

	pub fn to_hex_string(&self) -> String {
		self.data.to_hex_string()
	}

	pub fn get_hash(&self) -> BlockHash {
		BlockHash(blake2_256(&self.data.0))
	}
}

#[repr(u8)]
pub enum Pallet {
	DataAvailability = 29,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Header {
	#[serde(deserialize_with = "block_hash_from_string")]
	pub parent_hash: BlockHash,
	#[serde(deserialize_with = "number_from_hex")]
	pub number: BlockNumber,
	#[serde(deserialize_with = "block_hash_from_string")]
	pub state_root: BlockHash,
	#[serde(deserialize_with = "block_hash_from_string")]
	pub extrinsics_root: BlockHash,
	#[serde(skip_deserializing)]
	pub digest: Option<u32>,
	#[serde(skip_deserializing)]
	pub extension: Option<u32>,
}

fn block_hash_from_string<'de, D>(deserializer: D) -> Result<BlockHash, D::Error>
where
	D: Deserializer<'de>,
{
	let buf = String::deserialize(deserializer)?;
	Ok(BlockHash::from_hex_string(&buf).unwrap())
}

fn number_from_hex<'de, D>(deserializer: D) -> Result<u32, D::Error>
where
	D: Deserializer<'de>,
{
	let buf = String::deserialize(deserializer)?;
	let without_prefix = buf.trim_start_matches("0x");
	Ok(u32::from_str_radix(without_prefix, 16).unwrap())
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

		pub fn create_application_key(key: Bytes) -> Call {
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

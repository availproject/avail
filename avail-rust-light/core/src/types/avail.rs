use super::{AlreadyEncoded, H256};
use crate::types::payload_fields::Call;
use parity_scale_codec::{Decode, Encode};
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

#[derive(Debug, Deserialize)]
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

pub mod block {
	use super::*;

	/// Consensus engine unique ID.
	pub type ConsensusEngineId = [u8; 4];
	/// The encoded justification specific to a consensus engine.
	pub type EncodedJustification = Vec<u8>;
	/// An abstraction over justification for a block's validity under a consensus algorithm.
	/// Essentially a finality proof.
	pub type Justification = (ConsensusEngineId, EncodedJustification);

	#[derive(Debug, Clone, Deserialize)]
	pub struct SignedBlock {
		pub block: Block,
		pub justifications: Option<Justifications>,
	}

	#[derive(Debug, Clone, Deserialize)]
	pub struct Block {
		pub header: Header,
		#[serde(deserialize_with = "decode_extrinsics")]
		pub extrinsics: Vec<String>,
	}

	fn decode_extrinsics<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
	where
		D: Deserializer<'de>,
	{
		Vec::deserialize(deserializer)
	}

	#[derive(Debug, Clone, Deserialize)]
	pub struct Justifications(pub Vec<Justification>);

	#[derive(Debug, Clone, Deserialize)]
	#[serde(rename_all = "camelCase")]
	pub struct Header {
		pub parent_hash: H256,
		#[serde(deserialize_with = "number_from_hex")]
		pub number: BlockNumber,
		pub state_root: H256,
		pub extrinsics_root: H256,
		pub digest: Digest,
		pub extension: HeaderExtension,
	}

	fn number_from_hex<'de, D>(deserializer: D) -> Result<u32, D::Error>
	where
		D: Deserializer<'de>,
	{
		let buf = String::deserialize(deserializer)?;
		let without_prefix = buf.trim_start_matches("0x");
		Ok(u32::from_str_radix(without_prefix, 16).unwrap())
	}

	#[derive(Debug, Clone, Deserialize)]
	pub struct Digest {
		#[serde(deserialize_with = "digest_item_from_hex")]
		pub logs: Vec<DigestItem>,
	}
	impl Digest {
		pub fn to_human_readable(&self) -> String {
			let mut result = String::new();

			for log in &self.logs {
				result.push_str(&log.to_human_readable());
				result.push('\n');
			}

			result
		}
	}

	#[derive(Debug, Clone, Deserialize)]
	pub enum DigestItem {
		PreRuntime([u8; 4usize], Vec<u8>),
		Consensus([u8; 4usize], Vec<u8>),
		Seal([u8; 4usize], Vec<u8>),
		Other(Vec<u8>),
		RuntimeEnvironmentUpdated,
	}
	impl DigestItem {
		pub fn to_human_readable(&self) -> String {
			let mut result = String::new();

			match self {
				DigestItem::PreRuntime(x, y) => {
					let name = String::from_utf8(x.to_vec()).unwrap();
					let value = std::format!("0x{}", hex::encode(y));
					result.push_str(&std::format!(
						"DigestItem::PreRuntime [ {} {:?} ]",
						name,
						value
					));
				},
				DigestItem::Consensus(x, y) => {
					let name = String::from_utf8(x.to_vec()).unwrap();
					let value = std::format!("0x{}", hex::encode(y));
					result.push_str(&std::format!(
						"DigestItem::Consensus [ {} {:?} ]",
						name,
						value
					));
				},
				DigestItem::Seal(x, y) => {
					let name = String::from_utf8(x.to_vec()).unwrap();
					let value = std::format!("0x{}", hex::encode(y));
					result.push_str(&std::format!("DigestItem::Seal [ {} {:?} ]", name, value));
				},
				DigestItem::Other(x) => {
					let value = std::format!("0x{}", hex::encode(x));
					result.push_str(&std::format!("DigestItem::Other [ {:?} ]", value));
				},
				DigestItem::RuntimeEnvironmentUpdated => {
					result.push_str("DigestItem::RuntimeEnvironmentUpdated");
				},
			}

			result
		}
	}

	fn digest_item_from_hex<'de, D>(deserializer: D) -> Result<Vec<DigestItem>, D::Error>
	where
		D: Deserializer<'de>,
	{
		let buf: Vec<String> = Vec::deserialize(deserializer)?;

		let mut items: Vec<DigestItem> = Vec::with_capacity(buf.len());
		for encoded_item in buf {
			let encoded_item = encoded_item.trim_start_matches("0x");
			let hex_decoded_item = hex::decode(encoded_item).unwrap();
			let mut bytes = hex_decoded_item.as_slice();
			let item = DigestItem::decode(&mut bytes).unwrap();
			items.push(item);
		}

		Ok(items)
	}

	/* impl Encode for DigestItem {
		fn size_hint(&self) -> usize {
			let size = match self {
				Self::PreRuntime(x, y) => x.size_hint() + y.size_hint(),
				Self::Consensus(x, y) => x.size_hint() + y.size_hint(),
				Self::Seal(x, y) => x.size_hint() + y.size_hint(),
				Self::Other(x) => x.size_hint(),
				Self::RuntimeEnvironmentUpdated => 0usize,
			};

			size + 1
		}

		fn encode_to<T: parity_scale_codec::Output + ?Sized>(&self, dest: &mut T) {
			match self {
				Self::Other(_) => 0u8.encode_to(dest),
				Self::Consensus(_, _) => 4u8.encode_to(dest),
				Self::Seal(_, _) => 5u8.encode_to(dest),
				Self::PreRuntime(_, _) => 6u8.encode_to(dest),
				Self::RuntimeEnvironmentUpdated => 8u8.encode_to(dest),
			}

			match self {
				Self::PreRuntime(x, y) => {
					x.encode_to(dest);
					y.encode_to(dest);
				},
				Self::Consensus(x, y) => {
					x.encode_to(dest);
					y.encode_to(dest);
				},
				Self::Seal(x, y) => {
					x.encode_to(dest);
					y.encode_to(dest);
				},
				Self::Other(x) => {
					x.encode_to(dest);
				},
				Self::RuntimeEnvironmentUpdated => (),
			}
		}
	} */

	impl Decode for DigestItem {
		fn decode<I: parity_scale_codec::Input>(
			input: &mut I,
		) -> Result<Self, parity_scale_codec::Error> {
			use parity_scale_codec::Decode as ParityDecode;
			let index: u8 = u8::decode(input)?;

			match index {
				0 => {
					let value: Vec<u8> = <Vec<u8> as ParityDecode>::decode(input)?;
					Ok(DigestItem::Other(value))
				},
				4 => {
					let x: [u8; 4usize] = <[u8; 4usize] as ParityDecode>::decode(input)?;
					let y: Vec<u8> = <Vec<u8> as parity_scale_codec::Decode>::decode(input)?;
					Ok(DigestItem::Consensus(x, y))
				},
				5 => {
					let x: [u8; 4usize] = <[u8; 4usize] as ParityDecode>::decode(input)?;
					let y: Vec<u8> = <Vec<u8> as parity_scale_codec::Decode>::decode(input)?;
					Ok(DigestItem::Seal(x, y))
				},
				6 => {
					let x: [u8; 4usize] = <[u8; 4usize] as ParityDecode>::decode(input)?;
					let y: Vec<u8> = <Vec<u8> as parity_scale_codec::Decode>::decode(input)?;
					Ok(DigestItem::PreRuntime(x, y))
				},
				8 => Ok(DigestItem::RuntimeEnvironmentUpdated),
				_ => Err(parity_scale_codec::Error::from("Unknown Digest Index")),
			}
		}
	}

	#[derive(Debug, Clone, Deserialize)]
	#[repr(u8)]
	pub enum HeaderExtension {
		V3(V3HeaderExtension) = 2,
	}
	impl HeaderExtension {
		pub fn to_human_readable(&self) -> String {
			match self {
				HeaderExtension::V3(x) => x.to_human_readable(),
			}
		}
	}

	#[derive(Debug, Clone, Deserialize)]
	#[serde(rename_all = "camelCase")]
	pub struct V3HeaderExtension {
		pub app_lookup: DataLookup,
		pub commitment: KateCommitment,
	}
	impl V3HeaderExtension {
		pub fn to_human_readable(&self) -> String {
			todo!()
		}
	}

	#[derive(Debug, Clone, Deserialize)]
	pub struct DataLookup {
		pub size: u32,
		pub index: Vec<DataLookupItem>,
	}
	impl DataLookup {
		pub fn to_human_readable(&self) -> String {
			todo!()
		}
	}

	#[derive(Debug, Clone, Deserialize)]
	pub struct DataLookupItem {
		pub app_id: AppId,
		pub start: u32,
	}
	impl DataLookupItem {
		pub fn to_human_readable(&self) -> String {
			todo!()
		}
	}

	#[derive(Debug, Clone, Deserialize)]
	#[serde(rename_all = "camelCase")]
	pub struct KateCommitment {
		/// Rows
		pub rows: u16,
		/// Cols
		pub cols: u16,
		/// Plonk commitment.
		pub commitment: Vec<u8>,
		/// The merkle root of the data submitted
		pub data_root: H256,
	}
	impl KateCommitment {
		pub fn to_human_readable(&self) -> String {
			todo!()
		}
	}
}

pub mod events {
	use super::*;

	#[derive(Debug, Clone)]
	pub struct EventRecord {
		pub phase: Phase,
		//#[serde(deserialize_with = "event_record_custom_deserializing")]
		pub event: RuntimeEvent,
		//#[serde(skip)]
		pub topics: Vec<u32>,
	}

	impl<'de> Deserialize<'de> for EventRecord {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			let buf = serde_json::Value::deserialize(deserializer)?;
			dbg!(buf);
			todo!()
		}
	}

	fn event_record_custom_deserializing<'de, D>(deserializer: D) -> Result<RuntimeEvent, D::Error>
	where
		D: Deserializer<'de>,
	{
		let buf = serde_json::Value::deserialize(deserializer)?;
		dbg!(buf);

		todo!()
	}

	#[derive(Debug, Clone, Deserialize)]
	pub enum Phase {
		/// Applying an extrinsic.
		ApplyExtrinsic(u32),
		/// Finalizing the block.
		Finalization,
		/// Initializing the block.
		Initialization,
	}
	impl Default for Phase {
		fn default() -> Self {
			Self::Initialization
		}
	}

	impl Decode for Phase {
		fn decode<I: parity_scale_codec::Input>(
			input: &mut I,
		) -> Result<Self, parity_scale_codec::Error> {
			let index: u8 = u8::decode(input)?;

			match index {
				0 => {
					let value: u32 = u32::decode(input)?;
					Ok(Self::ApplyExtrinsic(value))
				},
				1 => Ok(Self::Finalization),
				2 => Ok(Self::Initialization),
				_ => Err(parity_scale_codec::Error::from("Unknown Phase Index")),
			}
		}
	}

	#[derive(Debug, Clone, Deserialize)]
	pub enum RuntimeEvent {
		System(FrameSystemEvent),
		Other,
	}

	#[derive(Debug, Clone, Deserialize)]
	#[repr(u8)]
	pub enum FrameSystemEvent {
		//#[codec(index = 0)]
		#[doc = "An extrinsic completed successfully."]
		ExtrinsicSuccess = 0,
		//#[codec(index = 1)]
		#[doc = "An extrinsic failed."]
		ExtrinsicFailed = 1,
	}

	/// Storage change set
	#[derive(Debug, Clone, Deserialize)]
	pub struct StorageChangeSet {
		/// Block hash
		pub block: String,
		/// A list of changes; tuples of storage key and optional storage data.
		pub changes: Vec<Vec<String>>,
	}
}

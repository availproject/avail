use std::fmt::Debug;

use avail::runtime_types::{
	da_primitives::{
		asdr::data_lookup::DataLookup as AvailDataLookup, header::Header as AvailHeader,
		kate_commitment::KateCommitment as AvailKateCommitment,
	},
	sp_runtime::{
		generic::digest::{Digest as AvailDigest, DigestItem as AvailDigestItem},
		traits::BlakeTwo256 as AvailBlakeTwo256,
	},
};
use codec::{Codec, Compact, Decode, Encode, EncodeLike, Error as DecodeError, Input};
use parity_util_mem::MallocSizeOf;
use scale_info::TypeInfo;
use serde::{Deserialize, Deserializer, Serialize};
use subxt::{
	ext::{
		sp_core::H256,
		sp_runtime::{
			traits::{BlakeTwo256, Extrinsic, Hash, Header},
			AccountId32, Digest, DigestItem, MultiAddress, MultiSignature,
		},
	},
	tx::{Era, ExtrinsicParams, PlainTip},
	Config,
};

#[subxt::subxt(runtime_metadata_path = "avail.dev.metadata.scale")]
pub mod avail {}

#[derive(Clone, Debug, Default)]
pub struct AvailConfig;

impl Config for AvailConfig {
	type AccountId = AccountId32;
	type Address = MultiAddress<Self::AccountId, u32>;
	type BlockNumber = u32;
	type Extrinsic = AvailExtrinsic;
	type ExtrinsicParams = AvailExtrinsicParams;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type Header = DaHeader;
	type Index = u32;
	type Signature = MultiSignature;
}

#[derive(Serialize, Debug, Clone)]
pub enum AvailExtrinsic {
	AvailDataExtrinsic {
		signature: MultiSignature,
		data: Vec<u8>,
		#[serde(skip_serializing)]
		address: MultiAddress<AccountId32, u32>,
		#[serde(skip_serializing)]
		extra_params: AvailExtrinsicParams,
	},
	RawExtrinsic {
		encoded_data: Vec<u8>,
	},
}
impl Eq for AvailExtrinsic {}

impl PartialEq for AvailExtrinsic {
	fn eq(&self, other: &Self) -> bool {
		match (self, other) {
			(
				Self::AvailDataExtrinsic {
					signature: l_signature,
					data: l_data,
					address: l_address,
					extra_params: l_extra_params,
				},
				Self::AvailDataExtrinsic {
					signature: r_signature,
					data: r_data,
					address: r_address,
					extra_params: r_extra_params,
				},
			) => {
				l_signature == r_signature
					&& l_data == r_data && l_address == r_address
					&& l_extra_params.app_id == r_extra_params.app_id
			},
			(
				Self::RawExtrinsic {
					encoded_data: l_encoded_data,
				},
				Self::RawExtrinsic {
					encoded_data: r_encoded_data,
				},
			) => l_encoded_data == r_encoded_data,
			_ => false,
		}
	}
}

const EXTRINSIC_VERSION: u8 = 4;
impl Decode for AvailExtrinsic {
	fn decode<I: Input>(input: &mut I) -> Result<AvailExtrinsic, DecodeError> {
		// This is a little more complicated than usual since the binary format must be compatible
		// with substrate's generic `Vec<u8>` type. Basically this just means accepting that there
		// will be a prefix of vector length (we don't need
		// to use this).
		let _length_do_not_remove_me_see_above: Compact<u32> = Decode::decode(input)?;

		let version = input.read_byte()?;

		let is_signed = version & 0b1000_0000 != 0;
		let version = version & 0b0111_1111;
		if version != EXTRINSIC_VERSION {
			return Err("Invalid transaction version".into());
		}
		let (signature, address, extra) = if is_signed {
			let address = <MultiAddress<AccountId32, u32>>::decode(input)?;
			let sig = MultiSignature::decode(input)?;
			let (mortality, nonce, tip, app_id) =
				<(Era, Compact<u32>, Compact<u128>, u32)>::decode(input)?;
			let extra = AvailExtrinsicParams {
				nonce: nonce.0,
				tip: PlainTip::new(tip.0),
				app_id,
				mortality,
				..Default::default()
			};
			(sig, address, extra)
		} else {
			return Err("NOTE: Not signed".into());
		};

		let section: u8 = Decode::decode(input)?;
		let method: u8 = Decode::decode(input)?;

		let data: Vec<u8> = match (section, method) {
			(29, 1) => Decode::decode(input)?,
			(_a, _b) => {
				return Err("NOTE: Not Avail Data extrinsic".into());
			},
		};

		Ok(Self::AvailDataExtrinsic {
			signature,
			data,
			address,
			extra_params: extra,
		})
	}
}

impl Encode for AvailExtrinsic {
	fn encode(&self) -> Vec<u8> {
		match self {
			AvailExtrinsic::AvailDataExtrinsic {
				signature,
				data,
				address,
				extra_params,
			} => {
				let mut tmp = Vec::new();

				tmp.push(EXTRINSIC_VERSION | 0b1000_0000);
				address.encode_to(&mut tmp);
				signature.encode_to(&mut tmp);
				extra_params.encode_extra_to(&mut tmp);
				(29u8, 1u8).encode_to(&mut tmp);
				data.encode_to(&mut tmp);
				let compact_len = codec::Compact::<u32>(tmp.len() as u32);
				let mut output = Vec::with_capacity(compact_len.size_hint() + tmp.len());
				compact_len.encode_to(&mut output);
				output.extend(tmp);
				output
			},
			AvailExtrinsic::RawExtrinsic { encoded_data } => encoded_data.clone(),
		}
	}
}

impl EncodeLike for AvailExtrinsic {}

impl<'a> Deserialize<'a> for AvailExtrinsic {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'a>,
	{
		let r = subxt::ext::sp_core::bytes::deserialize(deserializer)?;
		match Decode::decode(&mut &r[..]) {
			Ok(xt) => Ok(xt),
			Err(e) => {
				if e.to_string().contains("NOTE") {
					Ok(AvailExtrinsic::RawExtrinsic { encoded_data: r })
				} else {
					Err(serde::de::Error::custom(format!("Decode error: {}", e)))
				}
			},
		}
	}
}

impl Extrinsic for AvailExtrinsic {
	type Call = ();
	type SignaturePayload = ();

	fn is_signed(&self) -> Option<bool> {
		if let Self::AvailDataExtrinsic {
			signature: _,
			data: _,
			address: _,
			extra_params: _,
		} = self
		{
			Some(true)
		} else {
			None
		}
	}

	fn new(_call: Self::Call, _signed_data: Option<Self::SignaturePayload>) -> Option<Self> { None }
}

impl MallocSizeOf for AvailExtrinsic {
	fn size_of(&self, _ops: &mut parity_util_mem::MallocSizeOfOps) -> usize {
		// self.app_id.size_of(ops)
		// Implement this if necessary
		todo!()
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Encode, Decode)]
pub struct KateCommitment {
	/// The merkle root of the extrinsics.
	pub hash: H256,
	/// Plonk commitment.
	pub commitment: Vec<u8>,
	/// Rows
	pub rows: u16,
	/// Cols
	pub cols: u16,
	/// The merkle root of the data submissions
	#[serde(rename = "dataRoot")]
	pub data_root: [u8; 32],
}

impl MallocSizeOf for KateCommitment {
	fn size_of(&self, ops: &mut parity_util_mem::MallocSizeOfOps) -> usize {
		self.hash.size_of(ops)
			+ self.commitment.size_of(ops)
			+ self.rows.size_of(ops)
			+ self.cols.size_of(ops)
			+ self.data_root.size_of(ops)
	}
}

pub type AppId = u32;

#[derive(
	Debug, PartialEq, Eq, Clone, Encode, Decode, Default, TypeInfo, Serialize, Deserialize,
)]
pub struct DataLookup {
	pub size: u32,
	pub index: Vec<(AppId, u32)>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Encode, Decode)]
#[serde(rename_all = "camelCase")]
pub struct DaHeader {
	pub parent_hash: H256,
	#[serde(deserialize_with = "number_from_hex")]
	pub number: u32,
	pub state_root: H256,
	pub extrinsics_root: KateCommitment,
	pub digest: Digest,
	pub app_data_lookup: DataLookup,
}

// Type conversions for turning serializable Avail objects into avail
// config-specific types.
impl From<DaHeader> for AvailHeader<u32, AvailBlakeTwo256> {
	fn from(da_header: DaHeader) -> Self {
		Self {
			number: da_header.number,
			state_root: da_header.state_root,
			parent_hash: da_header.parent_hash,
			digest: da_header.digest.into(),
			extrinsics_root: da_header.extrinsics_root.into(),
			app_data_lookup: da_header.app_data_lookup.into(),
			__subxt_unused_type_params: Default::default(),
		}
	}
}

impl From<KateCommitment> for AvailKateCommitment<H256> {
	fn from(commitment: KateCommitment) -> Self {
		Self {
			hash: commitment.hash,
			data_root: commitment.data_root,
			commitment: commitment.commitment,
			rows: commitment.rows,
			cols: commitment.cols,
		}
	}
}

impl From<DataLookup> for AvailDataLookup {
	fn from(lookup: DataLookup) -> Self {
		Self {
			size: lookup.size,
			index: lookup.index,
		}
	}
}

impl From<Digest> for AvailDigest {
	fn from(digest: Digest) -> Self {
		Self {
			logs: digest.logs.into_iter().map(|l| l.into()).collect(),
		}
	}
}

impl From<DigestItem> for AvailDigestItem {
	fn from(item: DigestItem) -> Self {
		match item {
			DigestItem::PreRuntime(id, vec) => Self::PreRuntime(id, vec),
			DigestItem::Consensus(id, vec) => Self::Consensus(id, vec),
			DigestItem::Seal(id, vec) => Self::Seal(id, vec),
			DigestItem::Other(vec) => Self::Other(vec),
			DigestItem::RuntimeEnvironmentUpdated => Self::RuntimeEnvironmentUpdated,
		}
	}
}

fn number_from_hex<'de, D>(deserializer: D) -> Result<u32, D::Error>
where
	D: Deserializer<'de>,
{
	let buf = String::deserialize(deserializer)?;
	let without_prefix = buf.trim_start_matches("0x");
	Ok(u32::from_str_radix(without_prefix, 16).unwrap())
}

pub trait Parameter: Codec + EncodeLike + Clone + Eq + Debug {}
impl<T> Parameter for T where T: Codec + EncodeLike + Clone + Eq + Debug {}

impl MallocSizeOf for DaHeader {
	fn size_of(&self, ops: &mut parity_util_mem::MallocSizeOfOps) -> usize {
		self.parent_hash.size_of(ops)
			+ self.number.size_of(ops)
			+ self.state_root.size_of(ops)
			+ self.extrinsics_root.size_of(ops)
			+ self.digest.size_of(ops)
	}
}

impl Header for DaHeader {
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type Number = u32;

	fn new(
		number: Self::Number,
		extrinsics_root: Self::Hash,
		state_root: Self::Hash,
		parent_hash: Self::Hash,
		digest: Digest,
	) -> Self {
		Self {
			parent_hash,
			number,
			state_root,
			extrinsics_root: KateCommitment {
				hash: extrinsics_root,
				commitment: vec![],
				rows: 0,
				cols: 0,
				data_root: [0; 32],
			},
			digest,
			app_data_lookup: DataLookup {
				size: 0,
				index: vec![],
			},
		}
	}

	fn number(&self) -> &Self::Number { &self.number }

	fn set_number(&mut self, number: Self::Number) { self.number = number as u32; }

	fn extrinsics_root(&self) -> &Self::Hash { &self.extrinsics_root.hash }

	fn set_extrinsics_root(&mut self, root: Self::Hash) { self.extrinsics_root.hash = root; }

	fn state_root(&self) -> &Self::Hash { &self.state_root }

	fn set_state_root(&mut self, root: Self::Hash) { self.state_root = root; }

	fn parent_hash(&self) -> &Self::Hash { &self.parent_hash }

	fn set_parent_hash(&mut self, hash: Self::Hash) { self.parent_hash = hash; }

	fn digest(&self) -> &Digest { &self.digest }

	fn digest_mut(&mut self) -> &mut Digest { &mut self.digest }

	fn hash(&self) -> Self::Hash { <Self::Hashing as Hash>::hash_of(self) }
}
#[derive(Debug, Clone)]
pub struct AvailExtrinsicParams {
	pub spec_version: u32,
	pub tx_version: u32,
	pub nonce: u32,
	pub mortality: Era,
	pub genesis_hash: H256,
	pub tip: PlainTip,
	pub app_id: u32,
}

impl ExtrinsicParams<u32, H256> for AvailExtrinsicParams {
	type OtherParams = AvailExtrinsicParams;

	fn new(
		spec_version: u32,
		tx_version: u32,
		nonce: u32,
		genesis_hash: H256,
		other_params: Self::OtherParams,
	) -> Self {
		Self {
			spec_version,
			tx_version,
			nonce,
			mortality: other_params.mortality,
			genesis_hash,
			tip: other_params.tip,
			app_id: other_params.app_id,
		}
	}

	fn encode_extra_to(&self, v: &mut Vec<u8>) {
		(self.mortality, Compact(self.nonce), self.tip, self.app_id).encode_to(v);
	}

	fn encode_additional_to(&self, v: &mut Vec<u8>) {
		(
			self.spec_version,
			self.tx_version,
			self.genesis_hash,
			self.genesis_hash,
		)
			.encode_to(v);
	}
}

impl Default for AvailExtrinsicParams {
	fn default() -> Self {
		Self {
			spec_version: Default::default(),
			tx_version: Default::default(),
			nonce: Default::default(),
			mortality: Era::Immortal,
			genesis_hash: Default::default(),
			tip: Default::default(),
			app_id: Default::default(),
		}
	}
}
impl AvailExtrinsicParams {
	/// Create params with the addition of tip and app_id
	pub fn new_with_tip_and_app_id(tip: u128, app_id: u32) -> Self {
		Self {
			tip: PlainTip::new(tip),
			app_id,
			..Default::default()
		}
	}

	/// Create params with the addition of app_id
	pub fn new_with_app_id(app_id: u32) -> Self {
		Self {
			app_id,
			..Default::default()
		}
	}
}

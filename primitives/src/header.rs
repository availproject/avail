use codec::{Codec, Decode, Encode, EncodeAsRef, Error, HasCompact, Input, Output};
#[cfg(feature = "std")]
use parity_util_mem::{MallocSizeOf, MallocSizeOfOps};
use scale_info::TypeInfo;
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_core::{RuntimeDebug, U256};
use sp_runtime::{
	traits::{
		AtLeast32BitUnsigned, Hash as HashT, Header as HeaderT, MaybeDisplay, MaybeFromStr,
		MaybeMallocSizeOf, MaybeSerialize, MaybeSerializeDeserialize, Member, SimpleBitOps,
	},
	Digest,
};
use sp_runtime_interface::pass_by::{Codec as PassByCodecImpl, PassBy};
use sp_std::{convert::TryFrom, fmt::Debug, hash::Hash as StdHash};

use crate::{
	asdr::DataLookup,
	traits::{ExtendedHeader, ExtrinsicsWithCommitment as _},
	KateCommitment,
};

pub trait HeaderNumberTrait:
	Member
	+ AtLeast32BitUnsigned
	+ Codec
	+ MaybeSerializeDeserialize
	+ MaybeDisplay
	+ MaybeFromStr
	+ MaybeFromStr
	+ MaybeMallocSizeOf
	+ StdHash
	+ Copy
	+ Into<U256>
	+ TryFrom<U256>
	+ Debug
	+ Eq
{
}

impl<
		T: Member
			+ AtLeast32BitUnsigned
			+ Codec
			+ MaybeSerializeDeserialize
			+ MaybeDisplay
			+ MaybeFromStr
			+ MaybeMallocSizeOf
			+ StdHash
			+ Copy
			+ Into<U256>
			+ TryFrom<U256>
			+ Debug
			+ Eq,
	> HeaderNumberTrait for T
{
}

pub trait KateHashTrait: HashT {}
impl<T: HashT> KateHashTrait for T {}

pub trait KateHashOutputTrait:
	MaybeDisplay + Decode + MaybeMallocSizeOf + SimpleBitOps + Ord
{
}

impl<T: MaybeDisplay + Decode + MaybeMallocSizeOf + SimpleBitOps + Ord> KateHashOutputTrait for T {}

/// Abstraction over a block header for a substrate chain.
#[derive(PartialEq, Eq, Clone, RuntimeDebug, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "std", serde(deny_unknown_fields, rename_all = "camelCase"))]
pub struct Header<Number: HeaderNumberTrait, Hash: KateHashTrait> {
	/// The parent hash.
	pub parent_hash: Hash::Output,
	/// The block number.
	#[cfg_attr(feature = "std", serde(with = "number_serde"))]
	pub number: Number,
	/// The state trie merkle root
	pub state_root: Hash::Output,
	/// Hash and Kate Commitment
	pub extrinsics_root: KateCommitment<Hash::Output>,
	/// A chain-specific digest of data useful for light clients or referencing auxiliary data.
	pub digest: Digest,
	/// Application specific data index.
	pub app_data_lookup: DataLookup,
}

impl<N, H> Default for Header<N, H>
where
	N: HeaderNumberTrait,
	H: KateHashTrait,
	<H as sp_runtime::traits::Hash>::Output: From<[u8; 32]>,
{
	fn default() -> Self {
		Self {
			number: 1u32.into(),
			extrinsics_root: Default::default(),
			state_root: Default::default(),
			parent_hash: Default::default(),
			digest: Default::default(),
			app_data_lookup: Default::default(),
		}
	}
}

impl<Number, Hash> PassBy for Header<Number, Hash>
where
	Number: HeaderNumberTrait,
	Hash: KateHashTrait,
{
	type PassBy = PassByCodecImpl<Header<Number, Hash>>;
}

/// This module adds serialization support to `Header::number` field.
#[cfg(feature = "std")]
mod number_serde {
	use serde::{Deserializer, Serializer};

	use super::*;

	pub fn serialize<N, S>(n: &N, serializer: S) -> Result<S::Ok, S::Error>
	where
		N: HeaderNumberTrait,
		S: Serializer,
	{
		let u256: U256 = (*n).into();
		serde::Serialize::serialize(&u256, serializer)
	}

	pub fn deserialize<'de, D, T>(d: D) -> Result<T, D::Error>
	where
		T: HeaderNumberTrait,
		D: Deserializer<'de>,
	{
		let u256: U256 = serde::Deserialize::deserialize(d)?;
		TryFrom::try_from(u256).map_err(|_| serde::de::Error::custom("Try from failed"))
	}
}

#[cfg(feature = "std")]
impl<Number, Hash> MallocSizeOf for Header<Number, Hash>
where
	Number: HeaderNumberTrait,
	Hash: KateHashTrait,
	Hash::Output: KateHashOutputTrait,
{
	fn size_of(&self, ops: &mut MallocSizeOfOps) -> usize {
		self.parent_hash.size_of(ops)
			+ self.number.size_of(ops)
			+ self.state_root.size_of(ops)
			+ self.extrinsics_root.size_of(ops)
			+ self.digest.size_of(ops)
			+ self.app_data_lookup.size_of(ops)
	}
}

impl<Number, Hash> Decode for Header<Number, Hash>
where
	Number: HeaderNumberTrait,
	Hash: KateHashTrait,
	Hash::Output: Decode,
{
	fn decode<I: Input>(input: &mut I) -> Result<Self, Error> {
		Ok(Self {
			parent_hash: Decode::decode(input)?,
			number: <<Number as HasCompact>::Type>::decode(input)?.into(),
			state_root: Decode::decode(input)?,
			extrinsics_root: Decode::decode(input)?,
			digest: Decode::decode(input)?,
			app_data_lookup: Decode::decode(input)?,
		})
	}
}

impl<Number, Hash> Encode for Header<Number, Hash>
where
	Number: HeaderNumberTrait,
	Hash: KateHashTrait,
	Hash::Output: Encode,
{
	fn encode_to<T: Output + ?Sized>(&self, dest: &mut T) {
		self.parent_hash.encode_to(dest);
		<<<Number as HasCompact>::Type as EncodeAsRef<_>>::RefType>::from(&self.number)
			.encode_to(dest);
		self.state_root.encode_to(dest);
		self.extrinsics_root.encode_to(dest);
		self.digest.encode_to(dest);
		self.app_data_lookup.encode_to(dest);
	}
}

impl<Number, Hash> codec::EncodeLike for Header<Number, Hash>
where
	Number: HeaderNumberTrait,
	Hash: KateHashTrait,
	Hash::Output: Encode,
{
}

impl<Number, Hash> HeaderT for Header<Number, Hash>
where
	Number: Member
		+ MaybeSerializeDeserialize
		+ Debug
		+ sp_std::hash::Hash
		+ MaybeDisplay
		+ AtLeast32BitUnsigned
		+ Codec
		+ Copy
		+ Into<U256>
		+ TryFrom<U256>
		+ sp_std::str::FromStr
		+ MaybeMallocSizeOf,
	Hash: HashT,
	Hash::Output: Default
		+ sp_std::hash::Hash
		+ Copy
		+ Member
		+ Ord
		+ MaybeSerialize
		+ Debug
		+ MaybeDisplay
		+ SimpleBitOps
		+ Codec
		+ MaybeMallocSizeOf,
{
	type Hash = <Hash as HashT>::Output;
	type Hashing = Hash;
	type Number = Number;

	fn number(&self) -> &Self::Number { &self.number }

	fn set_number(&mut self, num: Self::Number) { self.number = num }

	fn extrinsics_root(&self) -> &Self::Hash { self.extrinsics_root.hash() }

	fn set_extrinsics_root(&mut self, _root: Self::Hash) { todo!() }

	fn state_root(&self) -> &Self::Hash { &self.state_root }

	fn set_state_root(&mut self, root: Self::Hash) { self.state_root = root }

	fn parent_hash(&self) -> &Self::Hash { &self.parent_hash }

	fn set_parent_hash(&mut self, hash: Self::Hash) { self.parent_hash = hash }

	fn digest(&self) -> &Digest { &self.digest }

	fn digest_mut(&mut self) -> &mut Digest {
		#[cfg(feature = "std")]
		log::debug!(target: "header", "Retrieving mutable reference to digest");
		&mut self.digest
	}

	fn new(
		number: Self::Number,
		extrinsics_root_hash: Self::Hash,
		state_root: Self::Hash,
		parent_hash: Self::Hash,
		digest: Digest,
	) -> Self {
		let extrinsics_root = extrinsics_root_hash.into();
		Self {
			number,
			parent_hash,
			state_root,
			digest,
			extrinsics_root,
			app_data_lookup: Default::default(),
		}
	}
}

impl<N, H> ExtendedHeader for Header<N, H>
where
	N: HeaderNumberTrait,
	H: KateHashTrait,
{
	type Hash = <H as HashT>::Output;
	type Number = N;
	type Root = KateCommitment<Self::Hash>;

	fn extrinsics_root(&self) -> &Self::Root { &self.extrinsics_root }

	fn set_extrinsics_root(&mut self, root: Self::Root) { self.extrinsics_root = root; }

	fn data_lookup(&self) -> &DataLookup { &self.app_data_lookup }

	/// Creates new header.
	fn new(
		number: Self::Number,
		extrinsics_root: Self::Root,
		state_root: Self::Hash,
		parent_hash: Self::Hash,
		digest: Digest,
		app_data_lookup: DataLookup,
	) -> Self {
		// TODO @miguel: Default app_data_lookup?
		Self {
			number,
			extrinsics_root,
			state_root,
			parent_hash,
			digest,
			app_data_lookup,
		}
	}
}

impl<Number, Hash> Header<Number, Hash>
where
	Number: HeaderNumberTrait,
	Hash: KateHashTrait,
{
	/// Convenience helper for computing the hash of the header without having
	/// to import the trait.
	pub fn hash(&self) -> Hash::Output { Hash::hash_of(self) }
}

#[cfg(all(test, feature = "std"))]
mod tests {
	use super::*;

	#[test]
	fn should_serialize_numbers() {
		fn serialize(num: u128) -> String {
			let mut v = vec![];
			{
				let mut ser = serde_json::Serializer::new(std::io::Cursor::new(&mut v));
				number_serde::serialize(&num, &mut ser).unwrap();
			}
			String::from_utf8(v).unwrap()
		}

		assert_eq!(serialize(0), "\"0x0\"".to_owned());
		assert_eq!(serialize(1), "\"0x1\"".to_owned());
		assert_eq!(
			serialize(u64::max_value() as u128),
			"\"0xffffffffffffffff\"".to_owned()
		);
		assert_eq!(
			serialize(u64::max_value() as u128 + 1),
			"\"0x10000000000000000\"".to_owned()
		);
	}

	#[test]
	fn should_deserialize_number() {
		fn deserialize(num: &str) -> u128 {
			let mut der = serde_json::Deserializer::new(serde_json::de::StrRead::new(num));
			number_serde::deserialize(&mut der).unwrap()
		}

		assert_eq!(deserialize("\"0x0\""), 0);
		assert_eq!(deserialize("\"0x1\""), 1);
		assert_eq!(
			deserialize("\"0xffffffffffffffff\""),
			u64::max_value() as u128
		);
		assert_eq!(
			deserialize("\"0x10000000000000000\""),
			u64::max_value() as u128 + 1
		);
	}
	#[test]
	fn ensure_format_is_unchanged() {
		use sp_runtime::{
			generic::{Digest, DigestItem},
			traits::BlakeTwo256,
		};

		use crate::KateCommitment;
		let extrinsic_root = KateCommitment {
			hash: BlakeTwo256::hash(b"4"),
			rows: 1,
			cols: 4,
			commitment: vec![
				128, 233, 73, 235, 218, 245, 193, 62, 9, 100, 156, 88, 124, 107, 25, 5, 251, 119,
				11, 74, 104, 67, 171, 170, 198, 180, 19, 227, 167, 64, 93, 152, 37, 172, 118, 77,
				178, 52, 29, 185, 183, 150, 89, 101, 7, 62, 151, 89, 128, 233, 73, 235, 218, 245,
				193, 62, 9, 100, 156, 88, 124, 107, 25, 5, 251, 119, 11, 74, 104, 67, 171, 170,
				198, 180, 19, 227, 167, 64, 93, 152, 37, 172, 118, 77, 178, 52, 29, 185, 183, 150,
				89, 101, 7, 62, 151, 89,
			],
			data_root: [
				63, 191, 50, 39, 146, 108, 250, 63, 65, 103, 119, 30, 90, 217, 28, 250, 44, 45,
				112, 144, 102, 124, 224, 30, 145, 28, 169, 11, 79, 49, 91, 17,
			],
		};
		let data_lookup = DataLookup {
			size: 1,
			index: vec![],
		};
		let header = Header::<u32, BlakeTwo256> {
			parent_hash: BlakeTwo256::hash(b"1"),
			number: 2,
			state_root: BlakeTwo256::hash(b"3"),
			extrinsics_root: extrinsic_root,
			digest: Digest {
				logs: vec![DigestItem::Other(b"5".to_vec())],
			},
			app_data_lookup: data_lookup,
		};
		let encoded = header.encode();
		assert_eq!(encoded, vec![
			146, 205, 245, 120, 196, 112, 133, 165, 153, 34, 86, 240, 220, 249, 125, 11, 25, 241,
			241, 201, 222, 77, 95, 227, 12, 58, 206, 97, 145, 182, 229, 219, 8, 88, 19, 72, 51,
			123, 15, 62, 20, 134, 32, 23, 61, 170, 165, 249, 77, 0, 216, 129, 112, 93, 203, 240,
			170, 131, 239, 218, 186, 97, 210, 237, 225, 235, 134, 73, 33, 73, 151, 87, 78, 32, 196,
			100, 56, 138, 23, 36, 32, 210, 84, 3, 104, 43, 187, 184, 12, 73, 104, 49, 200, 204, 31,
			143, 13, 129, 1, 128, 233, 73, 235, 218, 245, 193, 62, 9, 100, 156, 88, 124, 107, 25,
			5, 251, 119, 11, 74, 104, 67, 171, 170, 198, 180, 19, 227, 167, 64, 93, 152, 37, 172,
			118, 77, 178, 52, 29, 185, 183, 150, 89, 101, 7, 62, 151, 89, 128, 233, 73, 235, 218,
			245, 193, 62, 9, 100, 156, 88, 124, 107, 25, 5, 251, 119, 11, 74, 104, 67, 171, 170,
			198, 180, 19, 227, 167, 64, 93, 152, 37, 172, 118, 77, 178, 52, 29, 185, 183, 150, 89,
			101, 7, 62, 151, 89, 1, 0, 4, 0, 63, 191, 50, 39, 146, 108, 250, 63, 65, 103, 119, 30,
			90, 217, 28, 250, 44, 45, 112, 144, 102, 124, 224, 30, 145, 28, 169, 11, 79, 49, 91,
			17, 4, 0, 4, 53, 1, 0, 0, 0, 0
		],);
	}
}

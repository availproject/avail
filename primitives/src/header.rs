use codec::{Codec, Decode, Encode, EncodeAsRef, Error, HasCompact, Input, Output};
use scale_info::TypeInfo;
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_core::U256;
use sp_runtime::{
	traits::{
		AtLeast32BitUnsigned, Hash as HashT, Header, MaybeDisplay, MaybeFromStr, MaybeMallocSizeOf,
		MaybeSerialize, MaybeSerializeDeserialize, Member, SimpleBitOps,
	},
	Digest,
};
use sp_std::{convert::TryFrom, fmt::Debug, hash::Hash as StdHash};

use crate::{
	traits::{ExtrinsicsRoot, Rooted},
	KateExtrinsicsRoot,
};

pub trait KateNumberTrait:
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
	> KateNumberTrait for T
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
#[derive(PartialEq, Eq, Clone, sp_core::RuntimeDebug, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "std", serde(deny_unknown_fields))]
pub struct KateHeader<Number: KateNumberTrait, Hash: KateHashTrait> {
	/// The parent hash.
	pub parent_hash: Hash::Output,
	/// The block number.
	#[cfg_attr(feature = "std", serde(with = "number_serde"))]
	pub number: Number,
	/// The state trie merkle root
	pub state_root: Hash::Output,
	/// Hash and Kate Commitment
	pub extrinsics_root: KateExtrinsicsRoot<Hash::Output>,
	/// A chain-specific digest of data useful for light clients or referencing auxiliary data.
	pub digest: Digest,
}

/// This module adds serialization support to `KateHeader::number` field.
#[cfg(feature = "std")]
mod number_serde {
	use serde::{Deserializer, Serializer};

	use super::*;

	pub fn serialize<N, S>(n: &N, serializer: S) -> Result<S::Ok, S::Error>
	where
		N: KateNumberTrait,
		S: Serializer,
	{
		let u256: U256 = (*n).into();
		serde::Serialize::serialize(&u256, serializer)
	}

	pub fn deserialize<'de, D, T>(d: D) -> Result<T, D::Error>
	where
		T: KateNumberTrait,
		D: Deserializer<'de>,
	{
		let u256: U256 = serde::Deserialize::deserialize(d)?;
		TryFrom::try_from(u256).map_err(|_| serde::de::Error::custom("Try from failed"))
	}
}

#[cfg(feature = "std")]
impl<Number, Hash> parity_util_mem::MallocSizeOf for KateHeader<Number, Hash>
where
	Number: KateNumberTrait,
	Hash: KateHashTrait,
	Hash::Output: KateHashOutputTrait,
{
	fn size_of(&self, ops: &mut parity_util_mem::MallocSizeOfOps) -> usize {
		self.parent_hash.size_of(ops)
			+ self.number.size_of(ops)
			+ self.state_root.size_of(ops)
			+ self.extrinsics_root.size_of(ops)
			+ self.digest.size_of(ops)
	}
}

impl<Number, Hash> Decode for KateHeader<Number, Hash>
where
	Number: KateNumberTrait,
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
		})
	}
}

impl<Number, Hash> Encode for KateHeader<Number, Hash>
where
	Number: KateNumberTrait,
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
	}
}

impl<Number, Hash> codec::EncodeLike for KateHeader<Number, Hash>
where
	Number: KateNumberTrait,
	Hash: KateHashTrait,
	Hash::Output: Encode,
{
}

impl<Number, Hash> Header for KateHeader<Number, Hash>
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
		}
	}
}

impl<N, H> Rooted for KateHeader<N, H>
where
	N: KateNumberTrait,
	H: KateHashTrait,
{
	type Hash = <H as HashT>::Output;
	type Number = N;
	type Root = KateExtrinsicsRoot<Self::Hash>;

	fn extrinsics_root(&self) -> &Self::Root { &self.extrinsics_root }

	fn set_extrinsics_root(&mut self, root: Self::Root) { self.extrinsics_root = root; }

	/// Creates new header.
	fn new(
		number: Self::Number,
		extrinsics_root: Self::Root,
		state_root: Self::Hash,
		parent_hash: Self::Hash,
		digest: Digest,
	) -> Self {
		Self {
			number,
			extrinsics_root,
			state_root,
			parent_hash,
			digest,
		}
	}
}

impl<Number, Hash> KateHeader<Number, Hash>
where
	Number: KateNumberTrait,
	Hash: KateHashTrait,
	/*Hash::Output:
	Default + sp_std::hash::Hash + Copy + Member + MaybeDisplay + SimpleBitOps + Codec,*/
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
				serialize_number(&num, &mut ser).unwrap();
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
			deserialize_number(&mut der).unwrap()
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
}

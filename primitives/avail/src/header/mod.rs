// This file is part of Substrate.

// Copyright (C) 2017-2022 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Data-Avail implementation of a block header.

use codec::{Codec, Decode, Encode};
use scale_info::TypeInfo;
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_core::{RuntimeDebug, U256};
use sp_runtime::{
	traits::{
		AtLeast32BitUnsigned, Hash as HashT, Header as HeaderT, MaybeDisplay, MaybeSerialize,
		MaybeSerializeDeserialize, Member, SimpleBitOps,
	},
	Digest,
};
use sp_runtime_interface::pass_by::{Codec as PassByCodecImpl, PassBy};
use sp_std::{convert::TryFrom, fmt::Debug};

use crate::traits::{ExtendedHeader, HeaderBlockNumber, HeaderHash};

const LOG_TARGET: &str = "header";

pub mod extension;
pub use extension::HeaderExtension;

/// Abstraction over a block header for a substrate chain.
#[derive(PartialEq, Eq, Clone, RuntimeDebug, TypeInfo, Encode, Decode)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "std", serde(deny_unknown_fields, rename_all = "camelCase"))]
pub struct Header<Number: HeaderBlockNumber, Hash: HeaderHash> {
	/// The parent hash.
	pub parent_hash: Hash::Output,
	/// The block number.
	#[cfg_attr(feature = "std", serde(with = "number_serde"))]
	#[codec(compact)]
	pub number: Number,
	/// The state trie merkle root
	pub state_root: Hash::Output,
	/// The merkle root of the extrinsics.
	pub extrinsics_root: Hash::Output,
	/// A chain-specific digest of data useful for light clients or referencing auxiliary data.
	pub digest: Digest,
	/// Data Availability header extension.
	pub extension: HeaderExtension,
}

/// This module adds serialization support to `Header::number` field.
#[cfg(feature = "std")]
mod number_serde {
	use serde::{Deserializer, Serializer};

	use super::*;

	pub fn serialize<N, S>(n: &N, serializer: S) -> Result<S::Ok, S::Error>
	where
		N: HeaderBlockNumber,
		S: Serializer,
	{
		let u256: U256 = (*n).into();
		serde::Serialize::serialize(&u256, serializer)
	}

	pub fn deserialize<'de, D, T>(d: D) -> Result<T, D::Error>
	where
		T: HeaderBlockNumber,
		D: Deserializer<'de>,
	{
		let u256: U256 = serde::Deserialize::deserialize(d)?;
		TryFrom::try_from(u256).map_err(|_| serde::de::Error::custom("Try from failed"))
	}
}

impl<N: HeaderBlockNumber, H: HeaderHash> Header<N, H> {
	/// Creates a header V1
	#[inline]
	pub fn new(
		number: N,
		extrinsics_root: H::Output,
		state_root: H::Output,
		parent_hash: H::Output,
		digest: Digest,
		extension: HeaderExtension,
	) -> Self {
		Self {
			parent_hash,
			number,
			state_root,
			extrinsics_root,
			digest,
			extension,
		}
	}

	/// Convenience helper for computing the hash of the header without having
	/// to import the trait.
	#[inline]
	pub fn hash(&self) -> H::Output {
		H::hash_of(self)
	}
}

impl<N, H> Default for Header<N, H>
where
	N: HeaderBlockNumber + Default,
	H: HeaderHash,
{
	fn default() -> Self {
		Self {
			parent_hash: Default::default(),
			number: Default::default(),
			state_root: Default::default(),
			extrinsics_root: Default::default(),
			digest: Default::default(),
			extension: Default::default(),
		}
	}
}

impl<N: HeaderBlockNumber, H: HeaderHash> PassBy for Header<N, H> {
	type PassBy = PassByCodecImpl<Header<N, H>>;
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
		+ sp_std::str::FromStr,
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
		+ Codec,
{
	type Hash = <Hash as HashT>::Output;
	type Hashing = Hash;
	type Number = Number;

	fn number(&self) -> &Self::Number {
		&self.number
	}

	fn set_number(&mut self, num: Self::Number) {
		self.number = num
	}

	fn extrinsics_root(&self) -> &Self::Hash {
		&self.extrinsics_root
	}

	fn set_extrinsics_root(&mut self, root: Self::Hash) {
		self.extrinsics_root = root
	}

	fn state_root(&self) -> &Self::Hash {
		&self.state_root
	}

	fn set_state_root(&mut self, root: Self::Hash) {
		self.state_root = root
	}

	fn parent_hash(&self) -> &Self::Hash {
		&self.parent_hash
	}

	fn set_parent_hash(&mut self, hash: Self::Hash) {
		self.parent_hash = hash
	}

	fn digest(&self) -> &Digest {
		&self.digest
	}

	fn digest_mut(&mut self) -> &mut Digest {
		#[cfg(feature = "std")]
		log::debug!(target: LOG_TARGET, "Retrieving mutable reference to digest");
		&mut self.digest
	}

	fn new(
		number: Self::Number,
		extrinsics_root: Self::Hash,
		state_root: Self::Hash,
		parent_hash: Self::Hash,
		digest: Digest,
	) -> Self {
		Self {
			number,
			parent_hash,
			state_root,
			digest,
			extrinsics_root,
			extension: Default::default(),
		}
	}
}

impl<N: HeaderBlockNumber, H: HeaderHash> ExtendedHeader for Header<N, H> {
	type Hash = <H as HashT>::Output;
	type Number = N;

	/// Creates new header.
	fn new(
		n: Self::Number,
		extrinsics: Self::Hash,
		state: Self::Hash,
		parent: Self::Hash,
		digest: Digest,
		extension: HeaderExtension,
	) -> Self {
		Header::<N, H>::new(n, extrinsics, state, parent, digest, extension)
	}

	fn extension(&self) -> &HeaderExtension {
		&self.extension
	}

	fn set_extension(&mut self, extension: HeaderExtension) {
		self.extension = extension;
	}
}

#[cfg(test)]
mod tests {
	use codec::Error;
	use hex_literal::hex;
	use sp_runtime::{traits::BlakeTwo256, DigestItem};
	use test_case::test_case;

	use super::*;
	use crate::kate_commitment::KateCommitment;

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
		let commitment = KateCommitment {
				rows: 1,
				cols: 4,
				data_root: hex!("3fbf3227926cfa3f4167771e5ad91cfa2c2d7090667ce01e911ca90b4f315b11").into(),
				commitment: hex!("80e949ebdaf5c13e09649c587c6b1905fb770b4a6843abaac6b413e3a7405d9825ac764db2341db9b7965965073e975980e949ebdaf5c13e09649c587c6b1905fb770b4a6843abaac6b413e3a7405d9825ac764db2341db9b7965965073e9759").to_vec()
		};

		let extension = extension::v1::HeaderExtension {
			commitment,
			..Default::default()
		};

		let header = Header::<u32, BlakeTwo256> {
			parent_hash: BlakeTwo256::hash(b"1"),
			number: 2,
			state_root: BlakeTwo256::hash(b"3"),
			extrinsics_root: BlakeTwo256::hash(b"4"),
			digest: Digest {
				logs: vec![DigestItem::Other(b"5".to_vec())],
			},
			extension: extension.into(),
		};

		let encoded = header.encode();
		assert_eq!(encoded, hex!("92cdf578c47085a5992256f0dcf97d0b19f1f1c9de4d5fe30c3ace6191b6e5db08581348337b0f3e148620173daaa5f94d00d881705dcbf0aa83efdaba61d2ede1eb8649214997574e20c464388a172420d25403682bbbb80c496831c8cc1f8f0d040004350004103fbf3227926cfa3f4167771e5ad91cfa2c2d7090667ce01e911ca90b4f315b11810180e949ebdaf5c13e09649c587c6b1905fb770b4a6843abaac6b413e3a7405d9825ac764db2341db9b7965965073e975980e949ebdaf5c13e09649c587c6b1905fb770b4a6843abaac6b413e3a7405d9825ac764db2341db9b7965965073e97590000").to_vec());
	}

	fn header_v1() -> Header<u32, BlakeTwo256> {
		let commitment = KateCommitment {
				commitment: hex!("80e949ebdaf5c13e09649c587c6b1905fb770b4a6843abaac6b413e3a7405d9825ac764db2341db9b7965965073e975980e949ebdaf5c13e09649c587c6b1905fb770b4a6843abaac6b413e3a7405d9825ac764db2341db9b7965965073e9759").to_vec(),
				data_root: hex!("3fbf3227926cfa3f4167771e5ad91cfa2c2d7090667ce01e911ca90b4f315b11").into(),
				..Default::default()
			};
		let extension = extension::v1::HeaderExtension {
			commitment,
			..Default::default()
		};

		Header::<u32, BlakeTwo256> {
			extension: extension.into(),
			..Default::default()
		}
	}

	#[cfg(not(feature = "header-backward-compatibility-test"))]
	fn header_test() -> Header<u32, BlakeTwo256> {
		header_v1()
	}

	#[cfg(feature = "header-backward-compatibility-test")]
	fn header_test() -> Header<u32, BlakeTwo256> {
		let mut header = header_v1();
		header.extension = extension::v_test::HeaderExtension {
			new_field: b"New field for testing".to_vec(),
			..Default::default()
		}
		.into();

		header
	}

	#[test_case( header_v1().encode().as_ref() => Ok(header_v1()) ; "Decode V1 header")]
	#[test_case( header_test().encode().as_ref() => Ok(header_test()) ; "Decode test header")]
	fn header_decoding(mut encoded_header: &[u8]) -> Result<Header<u32, BlakeTwo256>, Error> {
		Header::decode(&mut encoded_header)
	}

	fn header_serde_encode<N: HeaderBlockNumber, H: HeaderHash>(header: Header<N, H>) -> String {
		serde_json::to_string(&header).unwrap_or_default()
	}

	#[test_case( header_serde_encode(header_v1()) => Ok(header_v1()) ; "Serde V1 header")]
	#[test_case( header_serde_encode(header_test()) => Ok(header_test()) ; "Serde test header")]
	fn header_serde(json_header: String) -> Result<Header<u32, BlakeTwo256>, String> {
		serde_json::from_str(&json_header).map_err(|serde_err| format!("{}", serde_err))
	}
}

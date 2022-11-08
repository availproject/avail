use core::marker::PhantomData;

use codec::{Decode, Encode};
use parity_util_mem::MallocSizeOf;
use serde::{Deserialize, Deserializer, Serialize};
use subxt::ext::{
	sp_core::H256,
	sp_runtime::{
		traits::{BlakeTwo256, Hash, Header as SPHeader},
		Digest as XtDigest, DigestItem as XtDigestItem,
	},
};

use crate::api::runtime_types::{
	da_primitives::header::{extension::HeaderExtension, Header as ApiHeader},
	sp_runtime::generic::digest::{Digest as ApiDigest, DigestItem as ApiDigestItem},
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Encode, Decode)]
#[serde(rename_all = "camelCase")]
pub struct Header {
	pub parent_hash: H256,
	#[serde(deserialize_with = "number_from_hex")]
	#[codec(compact)]
	pub number: u32,
	pub state_root: H256,
	pub extrinsics_root: H256,
	pub digest: XtDigest,
	pub extension: HeaderExtension,
}

impl Header {
	pub fn data_root(&self) -> H256 {
		match &self.extension {
			HeaderExtension::V1(ext) => ext.commitment.data_root,
		}
	}
}

impl MallocSizeOf for Header {
	fn size_of(&self, ops: &mut parity_util_mem::MallocSizeOfOps) -> usize {
		self.parent_hash.size_of(ops)
			+ self.number.size_of(ops)
			+ self.state_root.size_of(ops)
			+ self.extrinsics_root.size_of(ops)
			+ self.digest.size_of(ops)
	}
}

impl SPHeader for Header {
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type Number = u32;

	fn new(
		number: Self::Number,
		extrinsics_root: Self::Hash,
		state_root: Self::Hash,
		parent_hash: Self::Hash,
		digest: XtDigest,
	) -> Self {
		Self {
			parent_hash,
			number,
			state_root,
			extrinsics_root,
			digest,
			extension: HeaderExtension::V1(Default::default()),
		}
	}

	fn number(&self) -> &Self::Number { &self.number }

	fn set_number(&mut self, number: Self::Number) { self.number = number; }

	fn extrinsics_root(&self) -> &Self::Hash { &self.extrinsics_root }

	fn set_extrinsics_root(&mut self, root: Self::Hash) { self.extrinsics_root = root; }

	fn state_root(&self) -> &Self::Hash { &self.state_root }

	fn set_state_root(&mut self, root: Self::Hash) { self.state_root = root; }

	fn parent_hash(&self) -> &Self::Hash { &self.parent_hash }

	fn set_parent_hash(&mut self, hash: Self::Hash) { self.parent_hash = hash; }

	fn digest(&self) -> &XtDigest { &self.digest }

	fn digest_mut(&mut self) -> &mut XtDigest { &mut self.digest }

	fn hash(&self) -> Self::Hash { <Self::Hashing as Hash>::hash_of(self) }
}

fn number_from_hex<'de, D>(deserializer: D) -> Result<u32, D::Error>
where
	D: Deserializer<'de>,
{
	let buf = String::deserialize(deserializer)?;
	let without_prefix = buf.trim_start_matches("0x");
	Ok(u32::from_str_radix(without_prefix, 16).unwrap())
}

impl<B, H> From<Header> for ApiHeader<B, H>
where
	B: From<u32>,
{
	fn from(h: Header) -> Self {
		Self {
			parent_hash: h.parent_hash,
			number: h.number.into(),
			state_root: h.state_root,
			extrinsics_root: h.extrinsics_root,
			digest: h.digest.into(),
			extension: h.extension,
			__subxt_unused_type_params: PhantomData::default(),
		}
	}
}

impl From<XtDigest> for ApiDigest {
	fn from(d: XtDigest) -> Self {
		let logs = d
			.logs
			.into_iter()
			.map(|xt_item| xt_item.into())
			.collect::<Vec<_>>();
		Self { logs }
	}
}

impl From<XtDigestItem> for ApiDigestItem {
	fn from(di: XtDigestItem) -> Self {
		match di {
			XtDigestItem::PreRuntime(id, data) => ApiDigestItem::PreRuntime(id, data),
			XtDigestItem::Consensus(id, data) => ApiDigestItem::Consensus(id, data),
			XtDigestItem::Seal(id, data) => ApiDigestItem::Seal(id, data),
			XtDigestItem::Other(data) => ApiDigestItem::Other(data),
			XtDigestItem::RuntimeEnvironmentUpdated => ApiDigestItem::RuntimeEnvironmentUpdated,
		}
	}
}

use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::ensure;
use nomad_core::keccak256_concat;
use scale_info::TypeInfo;
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_core::{RuntimeDebug, H256};

use super::{error::TreeError, Merkle, MerkleProof, Proof, TREE_DEPTH, ZERO_HASHES};

/// Const assertions at `LightMerkle` struct.
struct AssertLightMerkleN<const N: usize>;
impl<const N: usize> AssertLightMerkleN<N> {
	/// `N` must be less or equal to `TREE_DEPTH`.
	pub const N_LESS_EQ_TREE_DEPTH: usize = TREE_DEPTH - N;
}

/// Verify at compilation time that `N <= TREE_DEPTH`
#[inline]
#[allow(path_statements)]
const fn const_assert_n_is_valid<const N: usize>() {
	AssertLightMerkleN::<N>::N_LESS_EQ_TREE_DEPTH;
}

/// An incremental merkle tree, modeled on the eth2 deposit contract
#[derive(Clone, Copy, Encode, Decode, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct LightMerkle<const N: usize> {
	#[cfg_attr(feature = "std", serde(with = "arrays"))]
	branch: [H256; N],
	count: u32, // TODO: is this reasonable over usize due to scale limitations
}

impl<const N: usize> Default for LightMerkle<N> {
	fn default() -> Self {
		const_assert_n_is_valid::<N>();
		let mut branch: [H256; N] = [Default::default(); N];
		branch
			.iter_mut()
			.enumerate()
			.for_each(|(i, elem)| *elem = ZERO_HASHES[i]);
		Self { branch, count: 0 }
	}
}

impl<const N: usize> Merkle for LightMerkle<N> {
	type Proof = Proof<N>;

	/// Return the maximum number of leaves in this tree
	fn max_elements() -> u32 {
		const_assert_n_is_valid::<N>();
		2u32.saturating_pow(N as u32)
	}

	fn count(&self) -> u32 { self.count }

	fn root(&self) -> H256 {
		let mut node: H256 = Default::default();
		let mut size = self.count;

		self.branch.iter().enumerate().for_each(|(i, elem)| {
			let (left, right) = if (size & 1) == 1 {
				(elem.as_bytes(), node.as_bytes())
			} else {
				(node.as_bytes(), ZERO_HASHES[i].as_bytes())
			};
			node = keccak256_concat!(left, right);
			size /= 2;
		});

		node
	}

	fn depth(&self) -> usize { N }

	fn ingest(&mut self, element: H256) -> Result<H256, TreeError> {
		ensure!(Self::max_elements() > self.count, TreeError::MerkleTreeFull);

		let mut node = element;

		self.count += 1;
		let mut size = self.count;
		for i in 0..TREE_DEPTH {
			if (size & 1) == 1 {
				self.branch[i] = node;
				return Ok(self.root());
			}
			node = keccak256_concat!(self.branch[i].as_bytes(), node.as_bytes());
			size /= 2;
		}

		Err(TreeError::Invalid)
	}
}

impl<const N: usize> LightMerkle<N> {
	/// Instantiate a new tree with a known depth and a starting leaf-set
	pub fn from_leaves(leaves: &[H256]) -> Result<Self, TreeError> {
		let mut tree = Self::default();

		for leaf in leaves.iter() {
			tree.ingest(*leaf).map_err(|_| TreeError::MerkleTreeFull)?;
		}

		Ok(tree)
	}

	/// Calculate the initital root of a tree of this depth
	pub fn initial_root() -> H256 { LightMerkle::<N>::default().root() }

	/// Get the leading-edge branch.
	pub fn branch(&self) -> &[H256; N] { &self.branch }

	/// Verify a incremental merkle proof of inclusion
	pub fn verify(&self, proof: &Proof<N>) -> bool { proof.root() == self.root() }
}

#[cfg(feature = "std")]
mod arrays {
	use std::{convert::TryInto, marker::PhantomData};

	use serde::{
		de::{SeqAccess, Visitor},
		ser::SerializeTuple,
		Deserialize, Deserializer, Serialize, Serializer,
	};
	pub fn serialize<S: Serializer, T: Serialize, const N: usize>(
		data: &[T; N],
		ser: S,
	) -> Result<S::Ok, S::Error> {
		let mut s = ser.serialize_tuple(N)?;
		for item in data {
			s.serialize_element(item)?;
		}
		s.end()
	}
	struct ArrayVisitor<T, const N: usize>(PhantomData<T>);
	impl<'de, T, const N: usize> Visitor<'de> for ArrayVisitor<T, N>
	where
		T: Deserialize<'de>,
	{
		type Value = [T; N];

		fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
			formatter.write_str(&format!("an array of length {}", N))
		}

		#[inline]
		fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
		where
			A: SeqAccess<'de>,
		{
			// can be optimized using MaybeUninit
			let mut data = Vec::with_capacity(N);
			for _ in 0..N {
				match (seq.next_element())? {
					Some(val) => data.push(val),
					None => return Err(serde::de::Error::invalid_length(N, &self)),
				}
			}
			data.try_into()
				.map_err(|_| serde::de::Error::custom("Failed to convert sized Vec<T> into [T; N]"))
		}
	}
	pub fn deserialize<'de, D, T, const N: usize>(deserializer: D) -> Result<[T; N], D::Error>
	where
		D: Deserializer<'de>,
		T: Deserialize<'de>,
	{
		deserializer.deserialize_tuple(N, ArrayVisitor::<T, N>(PhantomData))
	}
}

#[cfg(test)]
mod test {
	use ethers_core::utils::hash_message;

	use super::*;
	use crate::{test_utils, NomadLightMerkle};

	#[test]
	fn it_calculates_the_initial_root() {
		assert_eq!(
			NomadLightMerkle::initial_root(),
			"0x27ae5ba08d7291c96c8cbddcc148bf48a6d68c7974b94356f53754ef6171d757"
				.parse()
				.unwrap()
		);
	}

	#[test]
	fn it_computes_branch_roots() {
		let test_cases = test_utils::load_merkle_test_json();
		for test_case in test_cases.iter() {
			let mut tree = NomadLightMerkle::default();
			// insert the leaves
			for leaf in test_case.leaves.iter() {
				// TODO: ethers core and crate have different primitive types
				// versions. Must get inner bytes and call into().
				let hashed_leaf = hash_message(leaf).0.into();
				tree.ingest(hashed_leaf).expect("!ingest");
			}
			// assert the tree has the proper leaf count
			assert_eq!(tree.count() as usize, test_case.leaves.len());
			// assert the tree generates the proper root
			let root = tree.root(); // root is type H256
			assert_eq!(root, test_case.expected_root);
			for n in 0..test_case.leaves.len() {
				// check that the tree can verify the proof for this leaf
				assert!(tree.verify(&test_case.proofs[n]));
			}
		}
	}

	#[test]
	fn it_n_less_than_max_tree_depth() {
		const TREE_DEPTH_MINUS_ONE: usize = TREE_DEPTH - 1;

		let _ = LightMerkle::<0>::default();
		let _ = LightMerkle::<1>::default();
		let _ = LightMerkle::<TREE_DEPTH_MINUS_ONE>::default();
		let _ = LightMerkle::<TREE_DEPTH>::default();

		// Following code does not compile due to static asserts.
		// const TREE_DEPTH_PLUS_ONE :usize = TREE_DEPTH +1;
		// let _ = LightMerkle::<TREE_DEPTH_PLUS_ONE>::default();
	}
}

use codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;
use sp_core::{RuntimeDebug, H256, U256};

use super::{
	error::IngestionError, utils::hash_concat, Merkle, MerkleProof, Proof, TREE_DEPTH, ZERO_HASHES,
};

/// An incremental merkle tree, modeled on the eth2 deposit contract
#[derive(Clone, Copy, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct LightMerkle<const N: usize> {
	branch: [H256; N],
	count: u32, // TODO: is this reasonable over usize due to scale limitations
}

impl<const N: usize> Default for LightMerkle<N> {
	fn default() -> Self {
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
	fn max_elements() -> U256 { super::utils::max_leaves(N) }

	fn count(&self) -> u32 { self.count }

	fn root(&self) -> H256 {
		let mut node: H256 = Default::default();
		let mut size = self.count;

		self.branch.iter().enumerate().for_each(|(i, elem)| {
			node = if (size & 1) == 1 {
				super::utils::hash_concat(elem, node)
			} else {
				super::utils::hash_concat(node, ZERO_HASHES[i])
			};
			size /= 2;
		});

		node
	}

	fn depth(&self) -> usize { N }

	fn ingest(&mut self, element: H256) -> Result<H256, IngestionError> {
		let mut node = element;
		if Self::max_leaves() <= self.count.into() {
			return Err(IngestionError::MerkleTreeFull);
		}
		assert!(self.count < u32::MAX);
		self.count += 1;
		let mut size = self.count;
		for i in 0..TREE_DEPTH {
			if (size & 1) == 1 {
				self.branch[i] = node;
				return Ok(self.root());
			}
			node = hash_concat(self.branch[i], node);
			size /= 2;
		}
		unreachable!()
	}
}

impl<const N: usize> LightMerkle<N> {
	/// Return the maximum number of leaves in this tree
	pub fn max_leaves() -> U256 { super::utils::max_leaves(N) }

	/// Instantiate a new tree with a known depth and a starting leaf-set
	pub fn from_leaves(leaves: &[H256]) -> Self {
		let mut tree = Self::default();

		for leaf in leaves.iter() {
			tree.ingest(*leaf).unwrap();
		}

		tree
	}

	/// Calculate the initital root of a tree of this depth
	pub fn initial_root() -> H256 { LightMerkle::<N>::default().root() }

	/// Get the leading-edge branch.
	pub fn branch(&self) -> &[H256; N] { &self.branch }

	/// Verify a incremental merkle proof of inclusion
	pub fn verify(&self, proof: &Proof<N>) -> bool { proof.root() == self.root() }
}

#[cfg(test)]
mod test {
	use ethers_core::utils::{hash_message, keccak256};

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
				let hashed_leaf = keccak256(leaf);
				tree.ingest(hashed_leaf.into()).expect("!ingest");
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
}

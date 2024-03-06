use bounded_collections::BoundedVec;
use codec::{Decode, Encode};
use derive_more::Constructor;
use scale_info::TypeInfo;
use sp_core::{ConstU32, H256};
use sp_std::vec::Vec;
use thiserror_no_std::Error;

/*
#[cfg(feature = "runtime")]
use crate::keccak256_concat;
*/
#[cfg(feature = "runtime")]
use binary_merkle_tree::MerkleProof;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "runtime")]
use sp_io::hashing::keccak_256;

/// Max data supported on bidge (Ethereum calldata limits)
pub const BOUNDED_DATA_MAX_LENGTH: u32 = 102_400;
/// Maximum size of data allowed in the bridge
pub type BoundedData = BoundedVec<u8, ConstU32<BOUNDED_DATA_MAX_LENGTH>>;

pub mod message;
pub use message::{AddressedMessage, Message, MessageType};

pub mod message_ref;
pub use message_ref::{AddressedMessageRef, MessageRef};

/// Unique Tx identifier based on its block number and index.
pub fn tx_uid(block: u32, tx_index: u32) -> u64 {
	let mut buf = [0u8; 8];
	buf[..4].copy_from_slice(&block.to_be_bytes());
	buf[4..].copy_from_slice(&tx_index.to_be_bytes());
	u64::from_be_bytes(buf)
}

pub fn tx_uid_deconstruct(uid: u64) -> (u32, u32) {
	const SLICE_ERR: &str = "Valid slice .qed";

	let id: [u8; 8] = uid.to_be_bytes();
	let block = u32::from_be_bytes(id[..4].try_into().expect(SLICE_ERR));
	let tx_index = u32::from_be_bytes(id[4..].try_into().expect(SLICE_ERR));

	(block, tx_index)
}

#[derive(Clone, Debug, Encode, Decode, TypeInfo, Constructor)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct ProofResponse {
	pub data_proof: DataProofV2,
	#[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
	pub message: Option<Message>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum SubTrie {
	DataSubmit,
	Bridge,
}

#[derive(Debug, Clone, Copy, Encode, Decode, PartialEq, Eq, Default, TypeInfo)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TxDataRoots {
	/// Global Merkle root
	pub root: H256,
	/// Merkle root hash of submitted data
	pub submitted: H256,
	/// Merkle root of bridged data
	pub bridged: H256,
}

#[cfg(feature = "runtime")]
impl TxDataRoots {
	pub fn new(submitted: H256, bridged: H256) -> Self {
		// keccak_256(submitted, bridged)
		let sub_roots = [submitted.to_fixed_bytes(), bridged.to_fixed_bytes()].concat();
		let root = keccak_256(sub_roots.as_slice()).into();

		Self {
			root,
			submitted,
			bridged,
		}
	}
}

/// Wrapper of `binary-merkle-tree::MerkleProof` with codec support.
#[derive(Clone, Debug, PartialEq, Eq, Encode, Decode, TypeInfo, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct DataProofV2 {
	pub roots: TxDataRoots,
	/// Proof items (does not contain the leaf hash, nor the root obviously).
	///
	/// This vec contains all inner node hashes necessary to reconstruct the root hash given the
	/// leaf hash.
	pub proof: Vec<H256>,
	/// Number of leaves in the original tree.
	///
	/// This is needed to detect a case where we have an odd number of leaves that "get promoted"
	/// to upper layers.
	#[codec(compact)]
	pub number_of_leaves: u32,
	/// Index of the leaf the proof is for (0-based).
	#[codec(compact)]
	pub leaf_index: u32,
	/// Leaf content.
	pub leaf: H256,
}

#[cfg(feature = "runtime")]
impl DataProofV2 {
	pub fn new(roots: TxDataRoots, m_proof: MerkleProof<H256, Vec<u8>>) -> Self {
		let leaf = keccak_256(m_proof.leaf.as_slice()).into();

		Self {
			roots,
			leaf,
			proof: m_proof.proof,
			number_of_leaves: m_proof.number_of_leaves as u32,
			leaf_index: m_proof.leaf_index as u32,
		}
	}
}

/// Conversion error from `binary-merkle-tree::MerkleProof`.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Error)]
pub enum DataProofV2TryFromError {
	/// Root cannot be converted into `H256`.
	#[error("Root cannot be converted into `H256`")]
	InvalidRoot,
	/// Leaf cannot be converted into `H256`.
	#[error("Leaf cannot be converted into `H256`")]
	InvalidLeaf,
	/// The given index of proofs cannot be converted into `H256`.
	#[error("Proof at {0} cannot be converted into `H256`")]
	InvalidProof(usize),
	/// Number of leaves overflowed.
	#[error("Number of leaves overflowed")]
	OverflowedNumberOfLeaves,
	/// Number of leaves must be greater than zero.
	#[error("Number of leaves cannot be zero")]
	InvalidNumberOfLeaves,
	/// Leaf index overflowed.
	#[error("Leaf index overflowed")]
	OverflowedLeafIndex,
	/// Leaf index overflowed or invalid (greater or equal to `number_of_leaves`)
	#[error("Leaf index is invalid")]
	InvalidLeafIndex,
}

#[cfg(all(test, feature = "runtime"))]
mod test {
	use crate::Keccak256;
	use hex_literal::hex;
	use sp_io::hashing::keccak_256;
	use sp_std::cmp::min;
	use test_case::test_case;

	use super::*;

	fn leaves() -> Vec<Vec<u8>> {
		(0u8..7)
			.map(|idx| H256::repeat_byte(idx).to_fixed_bytes().to_vec())
			.collect::<Vec<_>>()
	}

	/// Creates a merkle proof of `leaf_index`.
	///
	/// If `leaf_index >= number_of_leaves`, it will create a fake proof using the latest possible
	/// index and overwriting the proof. That case is used to test transformations into
	/// `DataProofV2`.
	fn merkle_proof_idx(
		leaf_index: usize,
		root: H256,
		sub_trie: SubTrie,
	) -> (MerkleProof<H256, Vec<u8>>, H256, SubTrie) {
		let leaves = leaves();
		let index = min(leaf_index, leaves.len() - 1);
		let mut proof = binary_merkle_tree::merkle_proof::<Keccak256, _, _>(leaves, index);
		proof.leaf_index = leaf_index;

		(proof, root, sub_trie)
	}

	fn invalid_merkle_proof_zero_leaves() -> (MerkleProof<H256, Vec<u8>>, H256, SubTrie) {
		(
			MerkleProof {
				root: H256::default(),
				proof: vec![],
				number_of_leaves: 0,
				leaf_index: 0,
				leaf: H256::default().to_fixed_bytes().to_vec(),
			},
			H256::zero(),
			SubTrie::DataSubmit,
		)
	}

	fn expected_data_proof_1(
		root: H256,
		sub_trie: SubTrie,
	) -> Result<DataProofV2, DataProofV2TryFromError> {
		let data_root = expected_root(&sub_trie, root);
		let mut data_proof = DataProofV2 {
			data_root,
			proof: vec![
				hex!("290decd9548b62a8d60345a988386fc84ba6bc95484008f6362f93160ef3e563").into(),
				hex!("b54faa1de855f3a59f4b1fdd40d8fd1c5825be5e767edd3c2712eaa2db44e419").into(),
				hex!("8b109df8272c258d0bee1ebd1ec97979ce0dc19f0dcbfb329a79323bffcc23d1").into(),
			],
			number_of_leaves: 7,
			leaf_index: 1,
			leaf: H256::repeat_byte(1),
			..Default::default()
		};

		if sub_trie == SubTrie::Bridge {
			data_proof.bridged_root = root;
			data_proof.leaf = H256::repeat_byte(1);
			data_proof.submitted_root = H256(hex!(
				"c93decb6f246d173698f24c03ffe19694f9c1633cf40ae35862816f1255c6516"
			));
		} else {
			data_proof.submitted_root = root;
			data_proof.leaf = keccak_256(H256::repeat_byte(1).as_bytes()).into();
			data_proof.bridged_root = H256(hex!(
				"c93decb6f246d173698f24c03ffe19694f9c1633cf40ae35862816f1255c6516"
			));
		}

		Ok(data_proof)
	}

	fn expected_data_proof_0(
		root: H256,
		sub_trie: SubTrie,
	) -> Result<DataProofV2, DataProofV2TryFromError> {
		let data_root = expected_root(&sub_trie, root);
		let mut data_proof = DataProofV2 {
			data_root,
			proof: vec![
				hex!("cebc8882fecbec7fb80d2cf4b312bec018884c2d66667c67a90508214bd8bafc").into(),
				hex!("b54faa1de855f3a59f4b1fdd40d8fd1c5825be5e767edd3c2712eaa2db44e419").into(),
				hex!("8b109df8272c258d0bee1ebd1ec97979ce0dc19f0dcbfb329a79323bffcc23d1").into(),
			],
			number_of_leaves: 7,
			leaf_index: 0,
			leaf: H256::repeat_byte(0),
			..Default::default()
		};

		if sub_trie == SubTrie::Bridge {
			data_proof.bridged_root = root;
			data_proof.submitted_root = H256(hex!(
				"c93decb6f246d173698f24c03ffe19694f9c1633cf40ae35862816f1255c6516"
			));
			data_proof.leaf = H256::repeat_byte(0);
		} else {
			data_proof.leaf = keccak_256(H256::repeat_byte(0).as_bytes()).into();
			data_proof.submitted_root = root;
			data_proof.bridged_root = H256(hex!(
				"c93decb6f246d173698f24c03ffe19694f9c1633cf40ae35862816f1255c6516"
			));
		}

		Ok(data_proof)
	}

	fn expected_data_proof_6(
		root: H256,
		sub_trie: SubTrie,
	) -> Result<DataProofV2, DataProofV2TryFromError> {
		let data_root = expected_root(&sub_trie, root);
		let mut data_proof = DataProofV2 {
			data_root,
			proof: vec![
				hex!("98e3d39eab95363a52c1a9269a1840a6fc86bdae17f333ba1c64c123d77f5e1f").into(),
				hex!("dd553f5e3808fd45b691f2ab61c50b52764085451f6ad64484c05632ad4c9bc8").into(),
			],
			number_of_leaves: 7,
			leaf_index: 6,
			leaf: H256::repeat_byte(6),
			..Default::default()
		};

		if sub_trie == SubTrie::Bridge {
			data_proof.bridged_root = root;
			data_proof.leaf = H256::repeat_byte(6);
			data_proof.submitted_root = H256(hex!(
				"c93decb6f246d173698f24c03ffe19694f9c1633cf40ae35862816f1255c6516"
			));
		} else {
			data_proof.submitted_root = root;
			data_proof.leaf = keccak_256(H256::repeat_byte(6).as_bytes()).into();
			data_proof.bridged_root = H256(hex!(
				"c93decb6f246d173698f24c03ffe19694f9c1633cf40ae35862816f1255c6516"
			));
		}

		Ok(data_proof)
	}

	fn expected_root(sub_trie: &SubTrie, sub_trie_root: H256) -> H256 {
		let data_root: H256;
		let root = hex!("c93decb6f246d173698f24c03ffe19694f9c1633cf40ae35862816f1255c6516");
		match sub_trie {
			SubTrie::DataSubmit => {
				data_root = keccak256_concat!(sub_trie_root.as_bytes(), root);
			},
			SubTrie::Bridge => {
				data_root = keccak256_concat!(root, sub_trie_root.as_bytes());
			},
		}
		data_root
	}

	#[test_case(merkle_proof_idx(0, H256::zero(), SubTrie::DataSubmit) => expected_data_proof_0(H256::zero(), SubTrie::DataSubmit); "From merkle proof 0 left sub trie")]
	#[test_case(merkle_proof_idx(1, H256::zero(), SubTrie::DataSubmit) => expected_data_proof_1(H256::zero(), SubTrie::DataSubmit); "From merkle proof 1 left sub trie")]
	#[test_case(merkle_proof_idx(6, H256::zero(), SubTrie::DataSubmit) => expected_data_proof_6(H256::zero(), SubTrie::DataSubmit); "From merkle proof 6 left sub trie")]
	#[test_case(merkle_proof_idx(0, H256::zero(), SubTrie::Bridge) => expected_data_proof_0(H256::zero(), SubTrie::Bridge); "From merkle proof 0 right sub trie")]
	#[test_case(merkle_proof_idx(1, H256::zero(), SubTrie::Bridge) => expected_data_proof_1(H256::zero(), SubTrie::Bridge); "From merkle proof 1 right sub trie")]
	#[test_case(merkle_proof_idx(6, H256::zero(), SubTrie::Bridge) => expected_data_proof_6(H256::zero(), SubTrie::Bridge); "From merkle proof 6 right sub trie")]
	#[test_case(merkle_proof_idx(0, H256::repeat_byte(1), SubTrie::DataSubmit) => expected_data_proof_0(H256::repeat_byte(1), SubTrie::DataSubmit); "From merkle proof 0 left sub trie non zero")]
	#[test_case(merkle_proof_idx(1, H256::repeat_byte(1), SubTrie::DataSubmit) => expected_data_proof_1(H256::repeat_byte(1), SubTrie::DataSubmit); "From merkle proof 1 left sub trie non zero")]
	#[test_case(merkle_proof_idx(6, H256::repeat_byte(1), SubTrie::DataSubmit) => expected_data_proof_6(H256::repeat_byte(1), SubTrie::DataSubmit); "From merkle proof 6 left sub trie non zero")]
	#[test_case(merkle_proof_idx(0, H256::repeat_byte(1), SubTrie::Bridge) => expected_data_proof_0(H256::repeat_byte(1), SubTrie::Bridge); "From merkle proof 0 right sub trie non zero")]
	#[test_case(merkle_proof_idx(1, H256::repeat_byte(1), SubTrie::Bridge) => expected_data_proof_1(H256::repeat_byte(1), SubTrie::Bridge); "From merkle proof 1 right sub trie non zero")]
	#[test_case(merkle_proof_idx(6, H256::repeat_byte(1), SubTrie::Bridge) => expected_data_proof_6(H256::repeat_byte(1), SubTrie::Bridge); "From merkle proof 6 right sub trie non zero")]
	#[test_case(merkle_proof_idx(7, H256::zero(), SubTrie::DataSubmit) => Err(DataProofV2TryFromError::InvalidLeafIndex); "From invalid leaf index left sub trie")]
	#[test_case(merkle_proof_idx(7, H256::zero(), SubTrie::Bridge) => Err(DataProofV2TryFromError::InvalidLeafIndex); "From invalid leaf index right sub trie")]
	#[test_case(invalid_merkle_proof_zero_leaves() => Err(DataProofV2TryFromError::InvalidNumberOfLeaves); "From invalid number of leaves")]
	fn from_binary(
		binary_proof: (MerkleProof<H256, Vec<u8>>, H256, SubTrie),
	) -> Result<DataProofV2, DataProofV2TryFromError> {
		let (proof, root, sub_trie) = binary_proof;
		let data_proof = DataProofV2::try_from((&proof, root, sub_trie))?;
		Ok(data_proof)
	}
}

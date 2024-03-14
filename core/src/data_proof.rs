use bounded_collections::BoundedVec;
use codec::{Decode, Encode};
use derive_more::Constructor;
use scale_info::TypeInfo;
use sp_core::{ConstU32, H256};
use sp_std::vec::Vec;

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

/// Unique Tx identifier based on its block number and index.
pub fn tx_uid(block: u32, tx_index: u32) -> u64 {
	let mut buf = [0u8; 8];
	buf[..4].copy_from_slice(&block.to_be_bytes());
	buf[4..].copy_from_slice(&tx_index.to_be_bytes());
	u64::from_be_bytes(buf)
}

/// Deconstructs the Unique Tx identifier into its block number and index.
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
	pub data_proof: DataProof,
	#[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
	pub message: Option<AddressedMessage>,
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
pub struct DataProof {
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
impl DataProof {
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

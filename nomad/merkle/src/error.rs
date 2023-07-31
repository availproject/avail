use sp_core::H256;
use thiserror_no_std::Error;

/// Tree Errors
#[derive(Debug, Clone, Copy, Error)]
pub enum VerifyingError {
	/// Failed proof verification
	#[error("Proof verification failed. Root is {expected}, produced is {actual}")]
	#[allow(dead_code)]
	VerificationFailed {
		/// The expected root (this tree's current root)
		expected: H256,
		/// The root produced by branch evaluation
		actual: H256,
	},
}

/// Error type for merkle tree ops.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Error)]
pub enum TreeError {
	/// Trying to push in a leaf
	#[error("Trying to push in a leaf")]
	LeafReached,
	/// No more space in the MerkleTree
	#[error("No more space in the MerkleTree")]
	MerkleTreeFull,
	/// MerkleTree is invalid
	#[error("MerkleTree is invalid")]
	Invalid,
	/// Incorrect Depth provided
	#[error("Incorrect Depth provided")]
	DepthTooSmall,
	/// Depth provided too large
	#[error("Provided tree depth exceeded 32")]
	DepthTooLarge,
}

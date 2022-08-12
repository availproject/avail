use primitive_types::H256;

/// Tree Errors
#[derive(Debug, thiserror_no_std::Error, Clone, Copy)]
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
#[derive(Debug, PartialEq, Clone, Copy, thiserror_no_std::Error)]
pub enum IngestionError {
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
}

use sp_core::{H256, U256};
use tiny_keccak::{Hasher, Keccak};

/// Return the keccak256 digest of the preimage
pub fn hash(preimage: impl AsRef<[u8]>) -> H256 {
	let mut output = [0u8; 32];
	let mut hasher = Keccak::v256();
	hasher.update(preimage.as_ref());
	hasher.finalize(&mut output);
	output.into()
}

/// Return the keccak256 disgest of the concatenation of the arguments
pub fn hash_concat(left: impl AsRef<[u8]>, right: impl AsRef<[u8]>) -> H256 {
	let mut vec = left.as_ref().to_vec();
	vec.extend_from_slice(right.as_ref());
	hash(vec)
}

/// Max number of leaves in a tree
pub(crate) fn max_leaves(n: usize) -> U256 {
	U256::from(2).pow(n.into()) - 1
}

/// Compute a root hash from a leaf and a Merkle proof.
pub fn merkle_root_from_branch(leaf: H256, branch: &[H256], depth: usize, index: usize) -> H256 {
	assert_eq!(branch.len(), depth, "proof length should equal depth");

	let mut current = leaf;

	for (i, next) in branch.iter().enumerate().take(depth) {
		let ith_bit = (index >> i) & 0x01;
		if ith_bit == 1 {
			current = hash_concat(next, current);
		} else {
			current = hash_concat(current, next);
		}
	}

	current
}

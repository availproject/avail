use frame_support::ensure;
use sp_core::H256;
use sp_runtime::traits::{Hash, Keccak256};

use crate::{TreeError, TREE_DEPTH};

/// Return the keccak256 disgest of the concatenation of the arguments
pub fn hash_concat(left: impl AsRef<[u8]>, right: impl AsRef<[u8]>) -> H256 {
	let mut vec = left.as_ref().to_vec();
	vec.extend_from_slice(right.as_ref());
	Keccak256::hash(vec.as_ref())
}

/// Max number of leaves in a tree. Returns `None` if overflow occurred
/// (i.e. n > 32).
pub(crate) fn max_leaves(n: usize) -> Result<u32, TreeError> {
	ensure!(n <= TREE_DEPTH, TreeError::DepthTooLarge);

	Ok(if n == 32 {
		u32::MAX
	} else {
		2u32.pow(n as u32) - 1
	})
}

/// Compute a root hash from a leaf and a Merkle proof.
pub fn merkle_root_from_branch<const N: usize>(
	leaf: H256,
	branch: &[H256; N],
	index: usize,
) -> H256 {
	let mut current = leaf;

	for (i, next) in branch.iter().enumerate().take(N) {
		let ith_bit = (index >> i) & 0x01;
		if ith_bit == 1 {
			current = hash_concat(next, current);
		} else {
			current = hash_concat(current, next);
		}
	}

	current
}

use frame_support::ensure;
use sp_core::H256;
use tiny_keccak::{Hasher, Keccak};

use crate::{TreeError, TREE_DEPTH};

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

use sp_core::H256;
use tiny_keccak::{Hasher, Keccak};

/// Return the keccak256 disgest of the concatenation of the arguments
pub fn hash_concat(left: impl AsRef<[u8]>, right: impl AsRef<[u8]>) -> H256 {
	let mut output = [0u8; 32];
	let mut hasher = Keccak::v256();
	hasher.update(left.as_ref());
	hasher.update(right.as_ref());
	hasher.finalize(&mut output);
	output.into()
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

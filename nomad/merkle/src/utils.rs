use nomad_core::keccak256_concat;
use sp_core::H256;

/// Compute a root hash from a leaf and a Merkle proof.
pub fn merkle_root_from_branch<const N: usize>(
	leaf: H256,
	branch: &[H256; N],
	index: usize,
) -> H256 {
	let mut current = leaf;

	for (i, next) in branch.iter().enumerate().take(N) {
		let ith_bit = (index >> i) & 0x01;
		let (left, right) = if ith_bit == 1 {
			(next.as_bytes(), current.as_bytes())
		} else {
			(current.as_bytes(), next.as_bytes())
		};
		current = keccak256_concat!(left, right);
	}

	current
}

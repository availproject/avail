#![cfg_attr(not(feature = "std"), no_std)]
#![warn(missing_debug_implementations)]
#![warn(missing_copy_implementations)]

extern crate alloc;

/// Hashing utils
pub mod utils;

/// Common error types for the merkle trees.
pub mod error;

/// A lightweight incremental merkle, suitable for running on-chain. Stores O
/// (1) data
pub mod light;
/// Merkle Proof struct
pub mod proof;

/// Test utils
#[cfg(test)]
pub(crate) mod test_utils;

use avail_core::ensure;
use sp_core::H256;

/// Tree depth
pub const TREE_DEPTH: usize = 32;
/// An incremental Nomad protocol standard-depth tree
pub type NomadLightMerkle = light::LightMerkle<TREE_DEPTH>;
/// A Nomad protocol standard-depth proof
pub type NomadProof = proof::Proof<TREE_DEPTH>;

pub use error::{TreeError, VerifyingError, VerifyingError::VerificationFailed};
use hex_literal::hex;
pub use light::*;
pub use proof::*;
pub use utils::*;

/// A cache of the zero hashes for each layer of the tree.
/// See `zero_hashes_pre_build()` test to check how it is calculated.
pub const ZERO_HASHES: [H256; TREE_DEPTH + 1] = [
	H256::zero(),
	H256(hex!(
		"ad3228b676f7d3cd4284a5443f17f1962b36e491b30a40b2405849e597ba5fb5"
	)),
	H256(hex!(
		"b4c11951957c6f8f642c4af61cd6b24640fec6dc7fc607ee8206a99e92410d30"
	)),
	H256(hex!(
		"21ddb9a356815c3fac1026b6dec5df3124afbadb485c9ba5a3e3398a04b7ba85"
	)),
	H256(hex!(
		"e58769b32a1beaf1ea27375a44095a0d1fb664ce2dd358e7fcbfb78c26a19344"
	)),
	H256(hex!(
		"0eb01ebfc9ed27500cd4dfc979272d1f0913cc9f66540d7e8005811109e1cf2d"
	)),
	H256(hex!(
		"887c22bd8750d34016ac3c66b5ff102dacdd73f6b014e710b51e8022af9a1968"
	)),
	H256(hex!(
		"ffd70157e48063fc33c97a050f7f640233bf646cc98d9524c6b92bcf3ab56f83"
	)),
	H256(hex!(
		"9867cc5f7f196b93bae1e27e6320742445d290f2263827498b54fec539f756af"
	)),
	H256(hex!(
		"cefad4e508c098b9a7e1d8feb19955fb02ba9675585078710969d3440f5054e0"
	)),
	H256(hex!(
		"f9dc3e7fe016e050eff260334f18a5d4fe391d82092319f5964f2e2eb7c1c3a5"
	)),
	H256(hex!(
		"f8b13a49e282f609c317a833fb8d976d11517c571d1221a265d25af778ecf892"
	)),
	H256(hex!(
		"3490c6ceeb450aecdc82e28293031d10c7d73bf85e57bf041a97360aa2c5d99c"
	)),
	H256(hex!(
		"c1df82d9c4b87413eae2ef048f94b4d3554cea73d92b0f7af96e0271c691e2bb"
	)),
	H256(hex!(
		"5c67add7c6caf302256adedf7ab114da0acfe870d449a3a489f781d659e8becc"
	)),
	H256(hex!(
		"da7bce9f4e8618b6bd2f4132ce798cdc7a60e7e1460a7299e3c6342a579626d2"
	)),
	H256(hex!(
		"2733e50f526ec2fa19a22b31e8ed50f23cd1fdf94c9154ed3a7609a2f1ff981f"
	)),
	H256(hex!(
		"e1d3b5c807b281e4683cc6d6315cf95b9ade8641defcb32372f1c126e398ef7a"
	)),
	H256(hex!(
		"5a2dce0a8a7f68bb74560f8f71837c2c2ebbcbf7fffb42ae1896f13f7c7479a0"
	)),
	H256(hex!(
		"b46a28b6f55540f89444f63de0378e3d121be09e06cc9ded1c20e65876d36aa0"
	)),
	H256(hex!(
		"c65e9645644786b620e2dd2ad648ddfcbf4a7e5b1a3a4ecfe7f64667a3f0b7e2"
	)),
	H256(hex!(
		"f4418588ed35a2458cffeb39b93d26f18d2ab13bdce6aee58e7b99359ec2dfd9"
	)),
	H256(hex!(
		"5a9c16dc00d6ef18b7933a6f8dc65ccb55667138776f7dea101070dc8796e377"
	)),
	H256(hex!(
		"4df84f40ae0c8229d0d6069e5c8f39a7c299677a09d367fc7b05e3bc380ee652"
	)),
	H256(hex!(
		"cdc72595f74c7b1043d0e1ffbab734648c838dfb0527d971b602bc216c9619ef"
	)),
	H256(hex!(
		"0abf5ac974a1ed57f4050aa510dd9c74f508277b39d7973bb2dfccc5eeb0618d"
	)),
	H256(hex!(
		"b8cd74046ff337f0a7bf2c8e03e10f642c1886798d71806ab1e888d9e5ee87d0"
	)),
	H256(hex!(
		"838c5655cb21c6cb83313b5a631175dff4963772cce9108188b34ac87c81c41e"
	)),
	H256(hex!(
		"662ee4dd2dd7b2bc707961b1e646c4047669dcb6584f0d8d770daf5d7e7deb2e"
	)),
	H256(hex!(
		"388ab20e2573d171a88108e79d820e98f26c0b84aa8b2f4aa4968dbb818ea322"
	)),
	H256(hex!(
		"93237c50ba75ee485f4c22adf2f741400bdf8d6a9cc7df7ecae576221665d735"
	)),
	H256(hex!(
		"8448818bb4ae4562849e949e17ac16e0be16688e156b5cf15e098c627c0056a9"
	)),
	H256(hex!(
		"27ae5ba08d7291c96c8cbddcc148bf48a6d68c7974b94356f53754ef6171d757"
	)),
];

/// A merkle proof
pub trait MerkleProof {
	/// Calculate the merkle root of this proof's branch
	fn root(&self) -> H256;
}

/// A simple trait for merkle-based accumulators
pub trait Merkle: core::fmt::Debug + Default {
	/// A proof of some leaf in this tree
	type Proof: MerkleProof;

	/// The maximum number of elements the tree can ingest
	fn max_elements() -> u32;

	/// The number of elements currently in the tree
	fn count(&self) -> u32;

	/// Calculate the root hash of this Merkle tree.
	fn root(&self) -> H256;

	/// Get the tree's depth.
	fn depth(&self) -> usize;

	/// Push a leaf to the tree
	fn ingest(&mut self, element: H256) -> Result<H256, TreeError>;

	/// Verify a proof against this tree's root.
	fn verify(&self, proof: &Self::Proof) -> Result<(), VerifyingError> {
		let actual = proof.root();
		let expected = self.root();
		ensure!(expected == actual, VerificationFailed { expected, actual });

		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use nomad_core::keccak256_concat;
	use sp_core::H256;

	use super::{TREE_DEPTH, ZERO_HASHES};

	/// Ensures that `ZERO_HASHES` pre-build image is valid.
	#[test]
	fn zero_hashes_pre_build() {
		// Build image.
		let mut hashes = [H256::zero(); TREE_DEPTH + 1];
		for i in 0..TREE_DEPTH {
			hashes[i + 1] = keccak256_concat!(hashes[i], hashes[i]);
		}

		assert_eq!(hashes, ZERO_HASHES);
	}
}

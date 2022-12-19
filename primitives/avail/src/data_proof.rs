use beefy_merkle_tree::MerkleProof;
use codec::{Decode, Encode};
use frame_support::ensure;
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_core::H256;
use sp_io::hashing::sha2_256;
use sp_std::{convert::TryFrom, vec::Vec};
use thiserror_no_std::Error;

/// Wrapper of `beefy-merkle-tree::MerkleProof` with codec support.
#[derive(Clone, Debug, PartialEq, Eq, Encode, Decode, Default)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct DataProof {
	/// Root hash of generated merkle tree.
	pub root: H256,
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

/// Conversion error from `beefy-merkle-tree::MerkleProof`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Error)]
pub enum DataProofTryFromError {
	/// Root cannot be converted into `H256`.
	#[error("Root cannot be converted into `H256`")]
	InvalidRoot,
	/// The given index of proofs cannot be converted into `H256`.
	#[error("Proof at {0} cannot be converted into `H256`")]
	InvalidProof(usize),
	/// Number of leaves overflowed
	#[error("Number of leaves overflowed")]
	OverflowedNumberOfLeaves,
	/// Number of leaves must be greater than zero.
	#[error("Number of leaves cannot be zero")]
	InvalidNumberOfLeaves,
	/// Leaf index overflowed
	#[error("Leaf index overflowed")]
	OverflowedLeafIndex,
	/// Leaf index overflowed or invalid (greater or equal to `number_of_leaves`)
	#[error("Leaf index is invalid")]
	InvalidLeafIndex,
}

impl<H, T> TryFrom<&MerkleProof<H, T>> for DataProof
where
	T: AsRef<[u8]>,
	H: PartialEq + Eq + AsRef<[u8]>,
{
	type Error = DataProofTryFromError;

	fn try_from(merkle_proof: &MerkleProof<H, T>) -> Result<Self, Self::Error> {
		use DataProofTryFromError::*;

		let root = <[u8; 32]>::try_from(merkle_proof.root.as_ref())
			.map_err(|_| InvalidRoot)?
			.into();
		let leaf = sha2_256(merkle_proof.leaf.as_ref()).into();
		let proof = merkle_proof
			.proof
			.iter()
			.enumerate()
			.map(|(idx, proof)| {
				<[u8; 32]>::try_from(proof.as_ref())
					.map_err(|_| InvalidProof(idx))
					.map(H256::from)
			})
			.collect::<Result<Vec<H256>, _>>()?;
		let number_of_leaves =
			u32::try_from(merkle_proof.number_of_leaves).map_err(|_| OverflowedNumberOfLeaves)?;
		ensure!(number_of_leaves != 0, InvalidNumberOfLeaves);

		let leaf_index = u32::try_from(merkle_proof.leaf_index).map_err(|_| OverflowedLeafIndex)?;
		ensure!(leaf_index < number_of_leaves, InvalidLeafIndex);

		Ok(Self {
			proof,
			root,
			leaf,
			number_of_leaves,
			leaf_index,
		})
	}
}

#[cfg(test)]
mod test {
	use crate::ShaTwo256;
	use hex_literal::hex;
	use sp_core::H512;
	use sp_std::cmp::min;
	use test_case::test_case;

	use super::*;

	fn leaves() -> Vec<Vec<u8>> {
		(0u8..7)
			.map(|idx| H512::repeat_byte(idx).to_fixed_bytes().to_vec())
			.collect::<Vec<_>>()
	}

	/// Creates a merkle proof of `leaf_index`.
	///
	/// If `leaf_index >= number_of_leaves`, it will create a fake proof using the latest possible
	/// index and overwriting the proof. That case is used to test transformations into
	/// `DataProof`.
	fn merkle_proof_idx(leaf_index: usize) -> MerkleProof<H256, Vec<u8>> {
		let leaves = leaves();
		let index = min(leaf_index, leaves.len() - 1);

		let mut proof = beefy_merkle_tree::merkle_proof::<ShaTwo256, _, _>(leaves, index);
		proof.leaf_index = leaf_index;
		proof
	}

	fn invalid_merkle_proof_zero_leaves() -> MerkleProof<H256, Vec<u8>> {
		MerkleProof {
			root: H256::default(),
			proof: vec![],
			number_of_leaves: 0,
			leaf_index: 0,
			leaf: H256::default().to_fixed_bytes().to_vec(),
		}
	}

	fn expected_data_proof_1() -> Result<DataProof, DataProofTryFromError> {
		Ok(DataProof {
			root: hex!("e18e5f531a15090555c2d3539b5d93a5a872ffc3422bd9b9410776549d71f6f6").into(),
			proof: vec![
				hex!("f5a5fd42d16a20302798ef6ed309979b43003d2320d9f0e8ea9831a92759fb4b").into(),
				hex!("e59d380e38bc66ab4e5452df8ee47bb4611e719efb8985c2a5e6598784e3d642").into(),
				hex!("fc7ad74dc17cb03a8464bbfb12fd037cceaef8ef5973d9f1772b4913503bff6e").into(),
			],
			number_of_leaves: 7,
			leaf_index: 1,
			leaf: sha2_256(H512::repeat_byte(1).as_bytes()).into(),
		})
	}

	fn expected_data_proof_0() -> Result<DataProof, DataProofTryFromError> {
		Ok(DataProof {
			root: hex!("e18e5f531a15090555c2d3539b5d93a5a872ffc3422bd9b9410776549d71f6f6").into(),
			proof: vec![
				hex!("7c8975e1e60a5c8337f28edf8c33c3b180360b7279644a9bc1af3c51e6220bf5").into(),
				hex!("e59d380e38bc66ab4e5452df8ee47bb4611e719efb8985c2a5e6598784e3d642").into(),
				hex!("fc7ad74dc17cb03a8464bbfb12fd037cceaef8ef5973d9f1772b4913503bff6e").into(),
			],
			number_of_leaves: 7,
			leaf_index: 0,
			leaf: sha2_256(H512::repeat_byte(0).as_bytes()).into(),
		})
	}

	fn expected_data_proof_6() -> Result<DataProof, DataProofTryFromError> {
		Ok(DataProof {
			root: hex!("e18e5f531a15090555c2d3539b5d93a5a872ffc3422bd9b9410776549d71f6f6").into(),
			proof: vec![
				hex!("6b19c42f81575abc499679f91bb649e0aa8af83d9634aab78af04b5e13b04e5f").into(),
				hex!("e4117bb4906266f46977187ca43a9151b88928ab1aa03283ddf5ead4b33c3e78").into(),
			],
			number_of_leaves: 7,
			leaf_index: 6,
			leaf: sha2_256(H512::repeat_byte(6).as_bytes()).into(),
		})
	}

	#[test_case( merkle_proof_idx(0) => expected_data_proof_0(); "From merkle proof 0")]
	#[test_case( merkle_proof_idx(1) => expected_data_proof_1(); "From merkle proof 1")]
	#[test_case( merkle_proof_idx(6) => expected_data_proof_6(); "From merkle proof 6")]
	#[test_case( merkle_proof_idx(7) => Err(DataProofTryFromError::InvalidLeafIndex); "From invalid leaf index")]
	#[test_case( invalid_merkle_proof_zero_leaves() => Err(DataProofTryFromError::InvalidNumberOfLeaves); "From invalid number of leaves")]
	fn from_beefy(
		beefy_proof: MerkleProof<H256, Vec<u8>>,
	) -> Result<DataProof, DataProofTryFromError> {
		let data_proof = DataProof::try_from(&beefy_proof)?;
		Ok(data_proof)
	}
}

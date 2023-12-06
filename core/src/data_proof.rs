#[cfg(feature = "runtime")]
use binary_merkle_tree::MerkleProof;
use codec::{Decode, Encode};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use sp_core::H256;
use sp_std::vec::Vec;
use thiserror_no_std::Error;

/// Wrapper of `binary-merkle-tree::MerkleProof` with codec support.
#[derive(Clone, Debug, PartialEq, Eq, Encode, Decode, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct DataProof {
    /// Root hash of generated merkle tree.
    pub data_root: H256,
    /// Root hash of generated blob root.
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

/// Conversion error from `binary-merkle-tree::MerkleProof`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Error)]
pub enum DataProofTryFromError {
    /// Root cannot be converted into `H256`.
    #[error("Root cannot be converted into `H256`")]
    InvalidRoot,
    /// The given index of proofs cannot be converted into `H256`.
    #[error("Proof at {0} cannot be converted into `H256`")]
    InvalidProof(usize),
    /// Number of leaves overflowed.
    #[error("Number of leaves overflowed")]
    OverflowedNumberOfLeaves,
    /// Number of leaves must be greater than zero.
    #[error("Number of leaves cannot be zero")]
    InvalidNumberOfLeaves,
    /// Leaf index overflowed.
    #[error("Leaf index overflowed")]
    OverflowedLeafIndex,
    /// Leaf index overflowed or invalid (greater or equal to `number_of_leaves`)
    #[error("Leaf index is invalid")]
    InvalidLeafIndex,
}

#[cfg(feature = "runtime")]
impl<H, T> core::convert::TryFrom<(MerkleProof<H, T>, H256)> for DataProof
    where
        T: AsRef<[u8]>,
        H: PartialEq + Eq + AsRef<[u8]>,
{
    type Error = DataProofTryFromError;

    fn try_from(merkle_proof_data: (MerkleProof<H, T>, H256)) -> Result<Self, Self::Error> {
        use crate::ensure;
        use DataProofTryFromError::*;

        use sp_io::hashing::keccak_256;

        let merkle_proof = merkle_proof_data.0;
        let root = merkle_proof_data.1;

        let data_root = <[u8; 32]>::try_from(merkle_proof.root.as_ref())
            .map_err(|_| InvalidRoot)?
            .into();
        let leaf = keccak_256(merkle_proof.leaf.as_ref()).into();

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
            data_root,
            root,
            leaf,
            number_of_leaves,
            leaf_index,
        })
    }
}

#[cfg(all(test, feature = "runtime"))]
mod test {
    use crate::Keccak256;
    use hex_literal::hex;
    use sp_core::H512;
    use sp_io::hashing::keccak_256;
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
    fn merkle_proof_idx(leaf_index: usize) -> (MerkleProof<H256, Vec<u8>>, H256) {
        let leaves = leaves();
        let index = min(leaf_index, leaves.len() - 1);

        let mut proof = binary_merkle_tree::merkle_proof::<Keccak256, _, _>(leaves, index);
        proof.leaf_index = leaf_index;

        (proof, H256::zero())
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
            data_root: Default::default(),
            root: hex!("08a1133e47edacdc5a7a37f7301aad3c725fbf5698ca5e35acb7915ad1784b95").into(),
            proof: vec![
                hex!("ad3228b676f7d3cd4284a5443f17f1962b36e491b30a40b2405849e597ba5fb5").into(),
                hex!("8421b50025cb27f1412ed7103442ecdd09d4aa1e4a1ba777597ae921e48b31e1").into(),
                hex!("08f1f28658e6a37fa6fd9be84bd7315c3ca1eceb0849ec88cbd5bf9a69160653").into(),
            ],
            number_of_leaves: 7,
            leaf_index: 1,
            leaf: keccak_256(H512::repeat_byte(1).as_bytes()).into(),
        })
    }

    fn expected_data_proof_0() -> Result<DataProof, DataProofTryFromError> {
        Ok(DataProof {
            data_root: Default::default(),
            root: hex!("08a1133e47edacdc5a7a37f7301aad3c725fbf5698ca5e35acb7915ad1784b95").into(),
            proof: vec![
                hex!("401617bc4f769381f86be40df0207a0a3e31ae0839497a5ac6d4252dfc35577f").into(),
                hex!("8421b50025cb27f1412ed7103442ecdd09d4aa1e4a1ba777597ae921e48b31e1").into(),
                hex!("08f1f28658e6a37fa6fd9be84bd7315c3ca1eceb0849ec88cbd5bf9a69160653").into(),
            ],
            number_of_leaves: 7,
            leaf_index: 0,
            leaf: keccak_256(H512::repeat_byte(0).as_bytes()).into(),
        })
    }

    fn expected_data_proof_6() -> Result<DataProof, DataProofTryFromError> {
        Ok(DataProof {
            data_root: Default::default(),
            root: hex!("08a1133e47edacdc5a7a37f7301aad3c725fbf5698ca5e35acb7915ad1784b95").into(),
            proof: vec![
                hex!("8663c7e2962f98579b883bf5e2179f9200ae3615ec6fc3bd8027a0de9973606a").into(),
                hex!("b225b28cd9168524306b0d944342b11bb21d37e9156cdbf42073d4e51b2f0a41").into(),
            ],
            number_of_leaves: 7,
            leaf_index: 6,
            leaf: keccak_256(H512::repeat_byte(6).as_bytes()).into(),
        })
    }

    #[test_case(merkle_proof_idx(0) => expected_data_proof_0(); "From merkle proof 0")]
    #[test_case(merkle_proof_idx(1) => expected_data_proof_1(); "From merkle proof 1")]
    #[test_case(merkle_proof_idx(6) => expected_data_proof_6(); "From merkle proof 6")]
    #[test_case(merkle_proof_idx(7) => Err(DataProofTryFromError::InvalidLeafIndex); "From invalid leaf index")]
    #[test_case(invalid_merkle_proof_zero_leaves() => Err(DataProofTryFromError::InvalidNumberOfLeaves); "From invalid number of leaves")]
    fn from_binary(
        binary_proof: (MerkleProof<H256, Vec<u8>>, H256),
    ) -> Result<DataProof, DataProofTryFromError> {
        let data_proof = DataProof::try_from(binary_proof)?;
        Ok(data_proof)
    }
}

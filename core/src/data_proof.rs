#[cfg(feature = "runtime")]
use binary_merkle_tree::MerkleProof;
use codec::{Decode, Encode};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use sp_core::H256;
use sp_std::vec::Vec;
use thiserror_no_std::Error;
#[cfg(feature = "runtime")]
use nomad_core::keccak256_concat;


#[derive(PartialEq)]
pub enum SubTrie {
    Left,
    Right,
}

/// Wrapper of `binary-merkle-tree::MerkleProof` with codec support.
#[derive(Clone, Debug, PartialEq, Eq, Encode, Decode, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct DataProof {
    /// Root hash of generated merkle tree.
    pub data_root: H256,
    /// Root hash of generated blob root.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blob_root: Option<H256>,
    /// Root hash of generated bridge root.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bridge_root: Option<H256>,
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
impl<H, T> core::convert::TryFrom<(&MerkleProof<H, T>, H256, SubTrie)> for DataProof
    where
        T: AsRef<[u8]>,
        H: PartialEq + Eq + AsRef<[u8]>,
{
    type Error = DataProofTryFromError;

    fn try_from(merkle_proof_data: (&MerkleProof<H, T>, H256, SubTrie)) -> Result<Self, Self::Error> {
        use crate::ensure;
        use DataProofTryFromError::*;

        use sp_io::hashing::keccak_256;

        let (merkle_proof, sub_trie_root, sub_trie) = merkle_proof_data;

        let root: H256 = <[u8; 32]>::try_from(merkle_proof.root.as_ref())
            .map_err(|_| InvalidRoot)?
            .into();
        let leaf = merkle_proof.leaf.as_ref().into();

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

        let data_root: H256;
        let mut blob_root: Option<H256> = None;
        let mut bridge_root: Option<H256> = None;
        match sub_trie {
            SubTrie::Right => {
                data_root = keccak256_concat!(root, sub_trie_root.as_bytes());
                bridge_root = Some(root);

            }
            SubTrie::Left => {
                data_root = keccak256_concat!(sub_trie_root.as_bytes(), root);
                blob_root = Some(root);
            }
        }

        Ok(Self {
            proof,
            data_root,
            blob_root,
            bridge_root,
            leaf,
            number_of_leaves,
            leaf_index,
        })
    }
}

#[cfg(all(test, feature = "runtime"))]
mod test {
    use crate::{Keccak256};
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
    fn merkle_proof_idx(leaf_index: usize, root: H256, sub_trie: SubTrie) -> (MerkleProof<H256, Vec<u8>>, H256, SubTrie) {
        let leaves = leaves();
        let index = min(leaf_index, leaves.len() - 1);
        let mut proof = binary_merkle_tree::merkle_proof::<Keccak256, _, _>(leaves, index);
        proof.leaf_index = leaf_index;

        (proof, root, sub_trie)
    }

    fn invalid_merkle_proof_zero_leaves() -> (MerkleProof<H256, Vec<u8>>, H256, SubTrie) {
        (MerkleProof {
            root: H256::default(),
            proof: vec![],
            number_of_leaves: 0,
            leaf_index: 0,
            leaf: H256::default().to_fixed_bytes().to_vec(),
        },
         H256::zero(),
         SubTrie::Left
        )
    }

    fn expected_data_proof_1(root: H256, sub_trie: SubTrie) -> Result<DataProof, DataProofTryFromError> {
        let data_root = expected_root(&sub_trie, root);
        let mut data_proof = DataProof {
            data_root,
            blob_root: None,
            bridge_root: None,
            proof: vec![
                hex!("ad3228b676f7d3cd4284a5443f17f1962b36e491b30a40b2405849e597ba5fb5").into(),
                hex!("8421b50025cb27f1412ed7103442ecdd09d4aa1e4a1ba777597ae921e48b31e1").into(),
                hex!("08f1f28658e6a37fa6fd9be84bd7315c3ca1eceb0849ec88cbd5bf9a69160653").into(),
            ],
            number_of_leaves: 7,
            leaf_index: 1,
            leaf: keccak_256(H512::repeat_byte(1).as_bytes()).into(),
        };

        if sub_trie == SubTrie::Left {
            data_proof.bridge_root = Some(hex!("08a1133e47edacdc5a7a37f7301aad3c725fbf5698ca5e35acb7915ad1784b95").into());
        } else {
            data_proof.blob_root = Some(hex!("08a1133e47edacdc5a7a37f7301aad3c725fbf5698ca5e35acb7915ad1784b95").into());
        }

        Ok(data_proof)
    }

    fn expected_data_proof_0(root: H256, sub_trie: SubTrie) -> Result<DataProof, DataProofTryFromError> {
        let data_root = expected_root(&sub_trie, root);
        let mut data_proof = DataProof {
            data_root,
            blob_root: None,
            bridge_root: None,
            proof: vec![
                hex!("401617bc4f769381f86be40df0207a0a3e31ae0839497a5ac6d4252dfc35577f").into(),
                hex!("8421b50025cb27f1412ed7103442ecdd09d4aa1e4a1ba777597ae921e48b31e1").into(),
                hex!("08f1f28658e6a37fa6fd9be84bd7315c3ca1eceb0849ec88cbd5bf9a69160653").into(),
            ],
            number_of_leaves: 7,
            leaf_index: 0,
            leaf: keccak_256(H512::repeat_byte(0).as_bytes()).into(),
        };

        if sub_trie == SubTrie::Left {
            data_proof.bridge_root = Some(hex!("08a1133e47edacdc5a7a37f7301aad3c725fbf5698ca5e35acb7915ad1784b95").into());
        } else {
            data_proof.blob_root = Some(hex!("08a1133e47edacdc5a7a37f7301aad3c725fbf5698ca5e35acb7915ad1784b95").into());
        }

        Ok(data_proof)
    }

    fn expected_data_proof_6(root: H256, sub_trie: SubTrie) -> Result<DataProof, DataProofTryFromError> {
        let data_root = expected_root(&sub_trie, root);
        let mut data_proof = DataProof {
            data_root,
            blob_root: None,
            bridge_root: None,
            proof: vec![
                hex!("8663c7e2962f98579b883bf5e2179f9200ae3615ec6fc3bd8027a0de9973606a").into(),
                hex!("b225b28cd9168524306b0d944342b11bb21d37e9156cdbf42073d4e51b2f0a41").into(),
            ],
            number_of_leaves: 7,
            leaf_index: 6,
            leaf: keccak_256(H512::repeat_byte(6).as_bytes()).into(),
        };

        if sub_trie == SubTrie::Left {
            data_proof.bridge_root = Some(hex!("08a1133e47edacdc5a7a37f7301aad3c725fbf5698ca5e35acb7915ad1784b95").into());
        } else {
            data_proof.blob_root = Some(hex!("08a1133e47edacdc5a7a37f7301aad3c725fbf5698ca5e35acb7915ad1784b95").into());
        }

        Ok(data_proof)
    }


    fn expected_root(sub_trie: &SubTrie, sub_trie_root: H256) -> H256 {
        let data_root: H256;
        let root = hex!("08a1133e47edacdc5a7a37f7301aad3c725fbf5698ca5e35acb7915ad1784b95");
        match sub_trie {
            SubTrie::Left => {
                data_root = keccak256_concat!(sub_trie_root.as_bytes(), root);
            }
            SubTrie::Right => {
                data_root = keccak256_concat!(root, sub_trie_root.as_bytes());
            }
        }
        data_root
    }

    #[test_case(merkle_proof_idx(0, H256::zero(), SubTrie::Left) => expected_data_proof_0(H256::zero(), SubTrie::Left); "From merkle proof 0 left sub trie")]
    #[test_case(merkle_proof_idx(1, H256::zero(), SubTrie::Left) => expected_data_proof_1(H256::zero(), SubTrie::Left); "From merkle proof 1 left sub trie")]
    #[test_case(merkle_proof_idx(6, H256::zero(), SubTrie::Left) => expected_data_proof_6(H256::zero(), SubTrie::Left); "From merkle proof 6 left sub trie")]
    #[test_case(merkle_proof_idx(0, H256::zero(), SubTrie::Right) => expected_data_proof_0(H256::zero(), SubTrie::Right); "From merkle proof 0 right sub trie")]
    #[test_case(merkle_proof_idx(1, H256::zero(), SubTrie::Right) => expected_data_proof_1(H256::zero(), SubTrie::Right); "From merkle proof 1 right sub trie")]
    #[test_case(merkle_proof_idx(6, H256::zero(), SubTrie::Right) => expected_data_proof_6(H256::zero(), SubTrie::Right); "From merkle proof 6 right sub trie")]
    #[test_case(merkle_proof_idx(0, H256::repeat_byte(1), SubTrie::Left) => expected_data_proof_0(H256::repeat_byte(1), SubTrie::Left); "From merkle proof 0 left sub trie non zero")]
    #[test_case(merkle_proof_idx(1, H256::repeat_byte(1), SubTrie::Left) => expected_data_proof_1(H256::repeat_byte(1), SubTrie::Left); "From merkle proof 1 left sub trie non zero")]
    #[test_case(merkle_proof_idx(6, H256::repeat_byte(1), SubTrie::Left) => expected_data_proof_6(H256::repeat_byte(1), SubTrie::Left); "From merkle proof 6 left sub trie non zero")]
    #[test_case(merkle_proof_idx(0, H256::repeat_byte(1), SubTrie::Right) => expected_data_proof_0(H256::repeat_byte(1), SubTrie::Right); "From merkle proof 0 right sub trie non zero")]
    #[test_case(merkle_proof_idx(1, H256::repeat_byte(1), SubTrie::Right) => expected_data_proof_1(H256::repeat_byte(1), SubTrie::Right); "From merkle proof 1 right sub trie non zero")]
    #[test_case(merkle_proof_idx(6, H256::repeat_byte(1), SubTrie::Right) => expected_data_proof_6(H256::repeat_byte(1), SubTrie::Right); "From merkle proof 6 right sub trie non zero")]
    #[test_case(merkle_proof_idx(7, H256::zero(), SubTrie::Left) => Err(DataProofTryFromError::InvalidLeafIndex); "From invalid leaf index left sub trie")]
    #[test_case(merkle_proof_idx(7, H256::zero(), SubTrie::Right) => Err(DataProofTryFromError::InvalidLeafIndex); "From invalid leaf index right sub trie")]
    #[test_case(invalid_merkle_proof_zero_leaves() => Err(DataProofTryFromError::InvalidNumberOfLeaves); "From invalid number of leaves")]
    fn from_binary(
        binary_proof: (MerkleProof<H256, Vec<u8>>, H256, SubTrie),
    ) -> Result<DataProof, DataProofTryFromError> {
        let (proof, root, sub_trie) = binary_proof;
        let data_proof = DataProof::try_from((&proof, root, sub_trie))?;
        Ok(data_proof)
    }
}

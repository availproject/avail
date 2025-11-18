use super::HeaderExtensionDataFilter;
use avail_core::OpaqueExtrinsic;
use avail_core::{
	data_proof::{AddressedMessage, SubTrie, TxDataRoots},
	Keccak256,
};
use avail_core::{traits::GetAppId, AppId};
use binary_merkle_tree::{merkle_proof, merkle_root, MerkleProof};
use codec::{Decode, Encode};
use derive_more::Constructor;
use sp_core::H256;
use sp_runtime_interface::pass_by::PassByCodec;
use sp_std::{iter::repeat, vec::Vec};

#[derive(Constructor, Debug, Encode, Decode, Clone, PartialEq, Eq)]
pub struct BridgedData {
	pub tx_index: u32,
	pub addr_msg: AddressedMessage,
}

#[derive(Debug, Constructor, Encode, Decode, PartialEq, Eq, Clone)]
pub struct SubmittedData {
	pub id: AppId,
	pub tx_index: u32,
	pub hash: H256,
	pub commitments: Vec<u8>,
}

impl GetAppId for SubmittedData {
	fn app_id(&self) -> AppId {
		self.id
	}
}

#[derive(Debug, Default)]
pub struct ExtractedTxData {
	pub submitted_data: Option<SubmittedData>,
	pub bridge_data: Option<BridgedData>,
}

#[derive(Debug, Default, PassByCodec, Encode, Decode)]
pub struct HeaderExtensionBuilderData {
	pub data_submissions: Vec<SubmittedData>,
	pub bridge_messages: Vec<BridgedData>,
}

impl HeaderExtensionBuilderData {
	pub fn from_raw_extrinsics<F: HeaderExtensionDataFilter>(
		block: u32,
		extrinsics: &[Vec<u8>],
		cols: u32,
		rows: u32,
	) -> Self {
		let opaques: Vec<OpaqueExtrinsic> = extrinsics
			.iter()
			.filter_map(|e| OpaqueExtrinsic::from_bytes(e).ok())
			.collect();

		Self::from_opaque_extrinsics::<F>(block, &opaques, cols, rows)
	}

	pub fn from_opaque_extrinsics<F: HeaderExtensionDataFilter>(
		block: u32,
		opaques: &[OpaqueExtrinsic],
		cols: u32,
		rows: u32,
	) -> Self {
		let failed_transactions = F::get_failed_transaction_ids(opaques);

		let extracted_tx_datas: Vec<ExtractedTxData> = opaques
			.into_iter()
			.enumerate()
			.filter_map(|(idx, opaque)| {
				F::filter(&failed_transactions, opaque.clone(), block, idx, cols, rows)
			})
			.collect();

		HeaderExtensionBuilderData::from(extracted_tx_datas)
	}

	pub fn to_submitted_datas(&self) -> Vec<SubmittedData> {
		self.data_submissions.clone()
	}

	pub fn is_empty(&self) -> bool {
		self.data_submissions.is_empty() && self.bridge_messages.is_empty()
	}

	pub fn roots(&self) -> TxDataRoots {
		let submitted = self.submitted_root();
		let bridged = self.bridged_root();

		TxDataRoots::new(submitted, bridged)
	}

	/// Generates the root of sub-tries.
	pub fn data_root(&self) -> H256 {
		self.roots().data_root
	}

	/// Generates the bridge root, using `bridges` as leaves and after balancing the merkle tree with
	/// `H256::zero` leaves.
	/// If `bridges` is empty, it will return `H256::zero()`.
	pub fn bridged_root(&self) -> H256 {
		if self.bridge_messages.is_empty() {
			return H256::zero();
		}

		merkle_root::<Keccak256, _>(self.balanced_bridged())
	}

	/// Generates a merkle root of **Data Submit** extrinsics from `submitted` leaves after balancing the merkle tree with
	/// `H256::zero` leaves.
	/// If `submitted` is empty, it will return `H256::zero()`.
	/// Note that data is hashed twice (`keccak_256(keccak_256(data))`, because:
	/// - We want to prove inclusion of blobs without revealing the blobs, otherwise,
	///   we end up overpaying L1 for data attestation, where we want to prove data inclusion
	///   on Avail.
	/// - It should not be possible to pass an internal node as a blob leaf.
	pub fn submitted_root(&self) -> H256 {
		if self.data_submissions.is_empty() {
			return H256::zero();
		}

		merkle_root::<Keccak256, _>(self.balanced_submitted())
	}

	pub fn submitted_proof_of(&self, leaf_idx: usize) -> Option<MerkleProof<H256, Vec<u8>>> {
		if self.data_submissions.is_empty() || leaf_idx >= self.data_submissions.len() {
			return None;
		}

		let proof = merkle_proof_to_owned(self.balanced_submitted(), leaf_idx);
		Some(proof)
	}

	pub fn bridged_proof_of(&self, leaf_idx: usize) -> Option<MerkleProof<H256, Vec<u8>>> {
		if self.bridge_messages.is_empty() || leaf_idx >= self.bridge_messages.len() {
			return None;
		}
		let proof = merkle_proof_to_owned(self.balanced_bridged(), leaf_idx);
		Some(proof)
	}

	pub fn leaf_idx(&self, tx_idx: u32) -> Option<(usize, SubTrie)> {
		if let Some(idx) = self
			.data_submissions
			.iter()
			.position(|s| s.tx_index == tx_idx)
		{
			return Some((idx, SubTrie::DataSubmit));
		}
		if let Some(idx) = self
			.bridge_messages
			.iter()
			.position(|b| b.tx_index == tx_idx)
		{
			return Some((idx, SubTrie::Bridge));
		}
		None
	}
}

impl HeaderExtensionBuilderData {
	fn balanced_submitted(&self) -> impl Iterator<Item = H256> + '_ {
		let balanced_len = next_power_of_two(&self.data_submissions);
		self.data_submissions
			.iter()
			.map(|s| s.hash)
			.chain(repeat(H256::zero()))
			.take(balanced_len)
	}

	fn balanced_bridged(&self) -> impl Iterator<Item = Vec<u8>> + '_ {
		let value: Vec<u8> = H256::zero().to_fixed_bytes().into();
		let balanced_len = next_power_of_two(&self.bridge_messages);

		self.bridge_messages
			.iter()
			.map(|b| b.addr_msg.abi_encode())
			.chain(repeat(value))
			.take(balanced_len)
	}
}

impl From<Vec<ExtractedTxData>> for HeaderExtensionBuilderData {
	fn from(value: Vec<ExtractedTxData>) -> Self {
		let mut data_submissions = Vec::new();
		let mut bridge_messages = Vec::new();

		for val in value {
			if let Some(data_submission) = val.submitted_data {
				data_submissions.push(data_submission);
			}

			if let Some(bridge_message) = val.bridge_data {
				bridge_messages.push(bridge_message);
			}
		}

		// Sort data_submissions by app_id once
		data_submissions.sort_by_key(|data| data.id);

		Self {
			data_submissions,
			bridge_messages,
		}
	}
}

#[inline]
fn next_power_of_two<T>(s: &[T]) -> usize {
	s.len().checked_next_power_of_two().unwrap_or(1)
}

/// Creates the Merkle Proof of `leaves` and for `leaf_idx` index using `Keccak256` hasher.
/// # Panics
/// If the `leaf_idx` is greater than the number of leaves in the proof.
///
/// # TODO
/// - The `merkle_proof` requires `ExactSizeIterator`, forcing to use `collect`. Try to remove
/// this.
fn merkle_proof_to_owned<I, T>(leaf_iter: I, leaf_idx: usize) -> MerkleProof<H256, Vec<u8>>
where
	I: Iterator<Item = T>,
	T: AsRef<[u8]>,
{
	let leaves = leaf_iter.collect::<Vec<_>>();
	let mp = merkle_proof::<Keccak256, _, T>(leaves, leaf_idx);
	// NOTE: As we are using refrences for the leaves, like `&'a [u8]`, we need to
	// convert them to `Vec<u8>`.
	MerkleProof {
		root: mp.root,
		proof: mp.proof,
		number_of_leaves: mp.number_of_leaves,
		leaf_index: mp.leaf_index,
		leaf: mp.leaf.as_ref().to_vec(),
	}
}

#[cfg(test)]
mod tests {
	use avail_core::from_substrate::keccak_256;

	use super::*;
	// use sp_core::H256;

	#[test]
	fn test_from_raw_extrinsics() {
		let extrinsics: Vec<Vec<u8>> = vec![vec![1, 2, 3], vec![4, 5, 6]];
		let builder_data =
			HeaderExtensionBuilderData::from_raw_extrinsics::<()>(1, &extrinsics, 1024, 4096);
		assert_eq!(builder_data.data_submissions.len(), 0);
		assert_eq!(builder_data.bridge_messages.len(), 0);
	}

	#[test]
	fn test_data_root() {
		let builder_data = HeaderExtensionBuilderData {
			data_submissions: vec![SubmittedData {
				id: AppId::default(),
				tx_index: 0,
				commitments: vec![],
				hash: H256::from(keccak_256(&vec![1, 2, 3])),
			}],
			bridge_messages: vec![],
		};
		assert_ne!(builder_data.data_root(), H256::zero());
	}

	#[test]
	fn test_submitted_proof_of() {
		let builder_data = HeaderExtensionBuilderData::default();
		assert!(builder_data.submitted_proof_of(0).is_none());

		let builder_data = HeaderExtensionBuilderData {
			data_submissions: vec![SubmittedData {
				id: AppId::default(),
				tx_index: 0,
				hash: H256::from(keccak_256(&vec![1, 2, 3])),
				commitments: vec![],
			}],
			bridge_messages: vec![],
		};
		assert!(builder_data.submitted_proof_of(0).is_some());
	}

	#[test]
	fn test_leaf_idx() {
		let builder_data = HeaderExtensionBuilderData::default();
		assert!(builder_data.leaf_idx(0).is_none());

		let builder_data = HeaderExtensionBuilderData {
			data_submissions: vec![SubmittedData {
				id: AppId::default(),
				tx_index: 0,
				hash: H256::from(keccak_256(&vec![1, 2, 3])),
				commitments: vec![],
			}],
			bridge_messages: vec![],
		};
		assert_eq!(builder_data.leaf_idx(0), Some((0, SubTrie::DataSubmit)));
	}

	#[test]
	fn test_data_submissions_sorted_by_app_id() {
		let data_submissions = vec![
			SubmittedData {
				id: AppId(3),
				tx_index: 0,
				hash: H256::from(keccak_256(&vec![1, 2, 3])),
				commitments: vec![],
			},
			SubmittedData {
				id: AppId(1),
				tx_index: 1,
				hash: H256::from(keccak_256(&vec![4, 5, 6])),
				commitments: vec![],
			},
			SubmittedData {
				id: AppId(2),
				tx_index: 2,
				hash: H256::from(keccak_256(&vec![7, 8, 9])),
				commitments: vec![],
			},
			SubmittedData {
				id: AppId(1),
				tx_index: 3,
				hash: H256::from(keccak_256(&vec![7, 8, 9])),
				commitments: vec![],
			},
		];

		let mut builder_data = HeaderExtensionBuilderData {
			data_submissions,
			bridge_messages: vec![],
		};

		// Sort data_submissions by app_id
		builder_data.data_submissions.sort_by_key(|data| data.id);

		let sorted_ids: Vec<(AppId, u32)> = builder_data
			.data_submissions
			.iter()
			.map(|d| (d.id, d.tx_index))
			.collect();
		assert_eq!(
			sorted_ids,
			vec![(AppId(1), 1), (AppId(1), 3), (AppId(2), 2), (AppId(3), 0)]
		);
	}
}

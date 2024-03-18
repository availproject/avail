use crate::data_root::{BridgedData, SubmittedData};
use avail_core::{
	app_extrinsic::AppExtrinsic,
	data_proof::{SubTrie, TxDataRoots},
	Keccak256,
};

use binary_merkle_tree::{merkle_proof, merkle_root, MerkleProof};
use codec::{Decode, Encode};
use derive_more::Constructor;
use itertools::Itertools;
use sp_core::H256;
use sp_io::hashing::keccak_256;
use sp_runtime_interface::pass_by::PassByCodec;
use sp_std::{iter::repeat, vec, vec::Vec};

#[derive(Debug, Default, Constructor, PassByCodec, Encode, Decode, Eq, PartialEq)]
pub struct TxData {
	pub submitted: Vec<SubmittedData>,
	pub bridged: Vec<BridgedData>,
	failed_send_msg_txs: Vec<u32>,
}

impl TxData {
	pub fn failed_send_msg_txs(failed_send_msg_txs: Vec<u32>) -> Self {
		Self {
			failed_send_msg_txs,
			..Default::default()
		}
	}

	pub fn to_app_extrinsics(self) -> Vec<AppExtrinsic> {
		self.submitted
			.into_iter()
			.map(|s| AppExtrinsic::new(s.id, s.data))
			.collect()
	}

	pub fn is_empty(&self) -> bool {
		self.submitted.is_empty() && self.bridged.is_empty()
	}

	pub fn roots(&self) -> TxDataRoots {
		let submitted = self.submitted_root();
		let bridged = self.bridged_root();

		TxDataRoots::new(submitted, bridged)
	}

	/// Generates the root of sub-tries.
	pub fn root(&self) -> H256 {
		self.roots().data_root
	}

	/// Generates the bridge root, using `bridges` as leaves and after balancing the merkle tree with
	/// `H256::zero` leaves.
	/// If `bridges` is empty, it will return `H256::zero()`.
	pub fn bridged_root(&self) -> H256 {
		if self.bridged.is_empty() {
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
		if self.submitted.is_empty() {
			return H256::zero();
		}

		merkle_root::<Keccak256, _>(self.balanced_submitted())
	}

	pub fn submitted_proof_of(&self, leaf_idx: usize) -> Option<MerkleProof<H256, Vec<u8>>> {
		if self.submitted.is_empty() || leaf_idx >= self.submitted.len() {
			return None;
		}

		let proof = merkle_proof_to_owned(self.balanced_submitted(), leaf_idx);
		Some(proof)
	}

	pub fn bridged_proof_of(&self, leaf_idx: usize) -> Option<MerkleProof<H256, Vec<u8>>> {
		if self.bridged.is_empty() || leaf_idx >= self.bridged.len() {
			return None;
		}
		let proof = merkle_proof_to_owned(self.balanced_bridged(), leaf_idx);
		Some(proof)
	}

	pub fn leaf_idx(&self, tx_idx: u32) -> Option<(usize, SubTrie)> {
		if let Some(idx) = self.submitted.iter().position(|s| s.tx_index == tx_idx) {
			return Some((idx, SubTrie::DataSubmit));
		}
		if let Some(idx) = self.bridged.iter().position(|b| b.tx_index == tx_idx) {
			return Some((idx, SubTrie::Bridge));
		}
		None
	}
}

impl TxData {
	fn balanced_submitted(&self) -> impl Iterator<Item = H256> + '_ {
		let balanced_len = next_power_of_two(&self.submitted);
		self.submitted
			.iter()
			.map(|s| H256(keccak_256(&s.data)))
			.chain(repeat(H256::zero()))
			.take(balanced_len)
	}

	fn balanced_bridged(&self) -> impl Iterator<Item = Vec<u8>> + '_ {
		let value: Vec<u8> = H256::zero().to_fixed_bytes().into();
		let balanced_len = next_power_of_two(&self.bridged);

		self.bridged
			.iter()
			.map(|b| b.addr_msg.abi_encode())
			.chain(repeat(value))
			.take(balanced_len)
	}
}

impl From<SubmittedData> for TxData {
	fn from(s: SubmittedData) -> Self {
		Self {
			submitted: vec![s],
			..Default::default()
		}
	}
}

impl From<BridgedData> for TxData {
	fn from(b: BridgedData) -> Self {
		Self {
			bridged: vec![b],
			..Default::default()
		}
	}
}

impl FromIterator<TxData> for TxData {
	fn from_iter<I: IntoIterator<Item = TxData>>(iter: I) -> Self {
		let (submitted, bridged, failed_send_msg_txs): (Vec<_>, Vec<_>, Vec<_>) = iter
			.into_iter()
			.map(|tx| (tx.submitted, tx.bridged, tx.failed_send_msg_txs))
			.multiunzip();

		let mut failed_send_msg_txs = failed_send_msg_txs
			.into_iter()
			.flatten()
			.collect::<Vec<_>>();
		failed_send_msg_txs.sort();
		failed_send_msg_txs.dedup();

		// Filter empty data submissions.
		let submitted = submitted
			.into_iter()
			.flatten()
			.filter(|s| !s.data.is_empty())
			.collect::<Vec<_>>();

		// Filter failed Txs.
		let bridged = bridged
			.into_iter()
			.flatten()
			.filter(|b| !failed_send_msg_txs.contains(&b.tx_index))
			.collect::<Vec<_>>();

		Self {
			submitted,
			bridged,
			failed_send_msg_txs,
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

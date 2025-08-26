use super::HeaderExtensionDataFilter;
use avail_core::OpaqueExtrinsic;
use avail_core::{
	app_extrinsic::AppExtrinsic,
	data_proof::{AddressedMessage, SubTrie, TxDataRoots},
	Keccak256,
};
use avail_core::{traits::GetAppId, AppId};
use binary_merkle_tree::{merkle_proof, merkle_root, MerkleProof};
use codec::{Decode, Encode};
use derive_more::Constructor;
use sp_core::H256;
use sp_io::hashing::keccak_256;
use sp_runtime_interface::pass_by::PassByCodec;
use sp_std::{iter::repeat, vec::Vec};

#[derive(Constructor, Debug, Encode, Decode, Clone, PartialEq, Eq)]
pub struct BridgedData {
	pub tx_index: u32,
	pub addr_msg: AddressedMessage,
}

#[derive(Debug, Constructor, Encode, Decode, PartialEq, Eq)]
pub struct SubmittedData {
	pub id: AppId,
	pub tx_index: u32,
	pub data: Vec<u8>,
}

impl GetAppId for SubmittedData {
	fn app_id(&self) -> AppId {
		self.id
	}
}

#[derive(Debug, Default)]
pub struct ExtractedTxData {
	pub app_extrinsic: Option<AppExtrinsic>,
	pub submitted_data: Option<SubmittedData>,
	pub bridge_data: Option<BridgedData>,
}

#[derive(Debug, Default, PassByCodec, Encode, Decode)]
pub struct HeaderExtensionBuilderData {
	pub app_extrinsics: Vec<AppExtrinsic>,
	pub data_submissions: Vec<SubmittedData>,
	pub bridge_messages: Vec<BridgedData>,
}

impl HeaderExtensionBuilderData {
	pub fn from_raw_extrinsics<F: HeaderExtensionDataFilter>(
		block: u32,
		extrinsics: &[Vec<u8>],
	) -> Self {
		let opaques: Vec<OpaqueExtrinsic> = extrinsics
			.iter()
			.filter_map(|e| OpaqueExtrinsic::from_bytes(e).ok())
			.collect();

		Self::from_opaque_extrinsics::<F>(block, &opaques)
	}

	pub fn from_opaque_extrinsics<F: HeaderExtensionDataFilter>(
		block: u32,
		opaques: &[OpaqueExtrinsic],
	) -> Self {
		let failed_transactions = opaques
			.iter()
			.rev()
			.find_map(|o| F::get_failed_transaction_ids(o));
		let failed_transactions = failed_transactions.unwrap_or_else(|| Vec::new());

		let extracted_tx_datas: Vec<ExtractedTxData> = opaques
			.into_iter()
			.enumerate()
			.filter_map(|(idx, opaque)| F::filter(&failed_transactions, opaque.clone(), block, idx))
			.collect();

		HeaderExtensionBuilderData::from(extracted_tx_datas)
	}

	pub fn to_app_extrinsics(&self) -> Vec<AppExtrinsic> {
		self.app_extrinsics.clone()
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
			.map(|s| H256(keccak_256(&s.data)))
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
		let mut app_extrinsics = Vec::new();

		for val in value {
			if let Some(data_submission) = val.submitted_data {
				data_submissions.push(data_submission);
			}

			if let Some(bridge_message) = val.bridge_data {
				bridge_messages.push(bridge_message);
			}

			if let Some(app_extrinsic) = val.app_extrinsic {
				app_extrinsics.push(app_extrinsic);
			}
		}

		Self {
			data_submissions,
			bridge_messages,
			app_extrinsics,
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
	// NOTE: As we are using references for the leaves, like `&'a [u8]`, we need to
	// convert them to `Vec<u8>`.
	MerkleProof {
		root: mp.root,
		proof: mp.proof,
		number_of_leaves: mp.number_of_leaves,
		leaf_index: mp.leaf_index,
		leaf: mp.leaf.as_ref().to_vec(),
	}
}

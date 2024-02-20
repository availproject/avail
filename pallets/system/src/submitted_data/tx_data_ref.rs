use crate::submitted_data::TxData;
use avail_core::{
	data_proof_v2::{AddressedMessage, AddressedMessageRef, MessageRef},
	Keccak256,
};

use binary_merkle_tree::{merkle_proof, merkle_root, MerkleProof};
use codec::{Decode, Encode};
use derive_more::Constructor;
use sp_core::H256;
use sp_io::hashing::keccak_256;
use sp_std::{iter::repeat, vec::Vec};

#[derive(Debug, Clone, Copy, Constructor, Encode, Decode)]
pub struct TxDataRoots {
	pub root: H256,
	pub submit: H256,
	pub bridge: H256,
}

/// It contains references (no copies) of the data used to calculate roots.
/// Data referenced is from the following extrinsics:
/// - `da::submit_data`
/// - `bridge::send_message`
#[derive(Default)]
pub struct TxDataRef<'a> {
	submitted: Vec<&'a [u8]>,
	bridged: Vec<AddressedMessageRef<'a>>,
}

impl<'a> TxDataRef<'a> {
	pub fn new(mut submitted: Vec<&'a [u8]>, mut bridged: Vec<AddressedMessageRef<'a>>) -> Self {
		submitted.retain(|d| !d.is_empty());
		bridged.retain(|m| match m.message {
			MessageRef::Data(data) => !data.is_empty(),
			MessageRef::FungibleToken { .. } => true,
		});

		Self { submitted, bridged }
	}

	pub fn roots(&self) -> TxDataRoots {
		// Keccak of `data_submit` and balance with `keccak_256(0x00..0)`.
		let submit = self.submitted_data_root();
		let bridge = self.bridged_messages_root();

		// keccak_256(submit_root, bridge_root)
		let sub_roots = [submit.to_fixed_bytes(), bridge.to_fixed_bytes()].concat();
		let root = keccak_256(sub_roots.as_slice()).into();

		TxDataRoots::new(root, submit, bridge)
	}

	/// Generates the root of sub-tries.
	pub fn root(&self) -> H256 {
		self.roots().root
	}

	pub fn submitted_data(&self) -> &Vec<&'a [u8]> {
		&self.submitted
	}

	pub fn bridged_messages(&self) -> &Vec<AddressedMessageRef<'a>> {
		&self.bridged
	}

	pub fn is_empty(&self) -> bool {
		self.submitted.is_empty() && self.bridged.is_empty()
	}

	/// Generates a merkle root of **Data Submit** extrinsics from `submitted` leaves after balancing the merkle tree with
	/// `H256::zero` leaves.
	/// If `submitted` is empty, it will return `H256::zero()`.
	/// Note that data is hashed twice (`keccak_256(keccak_256(data))`, because:
	/// - We want to prove inclusion of blobs without revealing the blobs, otherwise,
	///   we end up overpaying L1 for data attestation, where we want to prove data inclusion
	///   on Avail.
	/// - It should not be possible to pass an internal node as a blob leaf.
	pub fn submitted_data_root(&self) -> H256 {
		if self.submitted.is_empty() {
			return H256::zero();
		}

		merkle_root::<Keccak256, _>(self.balanced_submitted())
	}

	pub fn submitted_data_proof(&self, leaf_idx: usize) -> Option<MerkleProof<H256, Vec<u8>>> {
		if self.submitted.is_empty() || leaf_idx >= self.submitted.len() {
			return None;
		}

		let proof = merkle_proof_to_owned(self.balanced_submitted(), leaf_idx);
		Some(proof)
	}

	fn balanced_submitted(&self) -> impl Iterator<Item = H256> + '_ {
		let balanced_len = next_power_of_two(&self.submitted);
		self.submitted
			.iter()
			.map(|d| H256(keccak_256(d)))
			.chain(repeat(H256::zero()))
			.take(balanced_len)
	}

	/// Generates the bridge root, using `bridges` as leaves and after balancing the merkle tree with
	/// `H256::zero` leaves.
	/// If `bridges` is empty, it will return `H256::zero()`.
	pub fn bridged_messages_root(&self) -> H256 {
		if self.bridged.is_empty() {
			return H256::zero();
		}

		merkle_root::<Keccak256, _>(self.balanced_bridged())
	}

	pub fn bridged_messages_proof(&self, leaf_idx: usize) -> Option<MerkleProof<H256, Vec<u8>>> {
		if self.bridged.is_empty() || leaf_idx >= self.bridged.len() {
			return None;
		}
		let proof = merkle_proof_to_owned(self.balanced_bridged(), leaf_idx);
		Some(proof)
	}

	fn balanced_bridged(&self) -> impl Iterator<Item = Vec<u8>> + '_ {
		let value: Vec<u8> = H256::zero().to_fixed_bytes().into();
		let balanced_len = next_power_of_two(&self.bridged);
		self.bridged
			.iter()
			.map(|m| m.abi_encode())
			.chain(repeat(value))
			.take(balanced_len)
	}
}

impl<'a> FromIterator<TxDataRef<'a>> for TxDataRef<'a> {
	fn from_iter<I: IntoIterator<Item = TxDataRef<'a>>>(iter: I) -> Self {
		let (submitted, bridge): (Vec<_>, Vec<_>) = iter
			.into_iter()
			.map(|tx| (tx.submitted, tx.bridged))
			.unzip();
		let submitted = submitted.into_iter().flatten().collect::<Vec<_>>();
		let bridge = bridge.into_iter().flatten().collect::<Vec<_>>();
		Self::new(submitted, bridge)
	}
}

impl<'a> From<TxDataRef<'a>> for TxData {
	fn from(r: TxDataRef<'a>) -> Self {
		let submitted = r.submitted.into_iter().map(<[u8]>::to_vec).collect();
		let bridged = r.bridged.into_iter().map(AddressedMessage::from).collect();
		Self { submitted, bridged }
	}
}

impl<'a> From<&'a TxData> for TxDataRef<'a> {
	fn from(tx: &'a TxData) -> Self {
		let submitted = tx.submitted.iter().map(AsRef::as_ref).collect();
		let bridged = tx.bridged.iter().map(AddressedMessageRef::from).collect();
		Self { submitted, bridged }
	}
}

fn next_power_of_two<T>(s: &[T]) -> usize {
	s.len().checked_next_power_of_two().unwrap_or(1)
}

/// Creates the Merkle Proof of `leaves` and for `leaf_idx` index using `Keccak256` hasher.
/// # Panics
/// If the `leaf_idx` is greater than the number of leaves in the proof.
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

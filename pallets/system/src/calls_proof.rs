use crate::data_root::{Metrics, TxData, TxDataFilter};

use avail_core::{
	data_proof_v2::{AddressedMessage, SubTrie},
	traits::{GetAppId, MaybeCaller},
};

use binary_merkle_tree::{verify_proof, Leaf, MerkleProof};
use codec::Decode;
use derive_more::Constructor;
use frame_support::traits::ExtrinsicCall;
use sp_core::H256;
use sp_runtime::traits::Keccak256;
use sp_std::vec::Vec;

#[derive(Constructor)]
pub struct CallsProof {
	pub proof: MerkleProof<H256, Vec<u8>>,
	pub root: H256,
	pub message: Option<AddressedMessage>,
}

/// Creates the Merkle Proof of the submitted data items in `calls` filtered by `F` and
/// the given `data_index`.
///
/// If `transaction_index` is greater than the number transactions in the block, it will return `None`.
/// If `data_index` is greater than the number of Merkle leaves, it will return `None`.
///
/// # TODO
/// - The `merkle_proof` requires `ExactSizeIterator`, forcing to load all submitted data into
/// memory. That would increase the memory footprint of the node significantly. We could fix this
/// adding the number of submitted data items at `System` pallet.
pub fn calls_proof<'a, F, E, A, I>(
	block: u32,
	extrinsics: I,
	leaf_idx: usize,
	call_type: SubTrie,
) -> Option<CallsProof>
where
	F: TxDataFilter<A, E::Call>,
	E: ExtrinsicCall + MaybeCaller<A> + GetAppId + Decode,
	I: Iterator<Item = &'a Vec<u8>> + 'a,
{
	let mut metrics = Metrics::default();

	let tx_data = extrinsics
		.enumerate()
		.filter_map(|(idx, raw_extrinsic)| {
			let ext = E::decode(&mut raw_extrinsic.as_slice()).ok()?;
			let caller = ext.caller()?;
			let app_id = ext.app_id();
			let call = ext.call();
			F::filter(caller, call, app_id, block, idx, &mut metrics)
		})
		.collect::<TxData>();

	let message = tx_data
		.bridged
		.get(leaf_idx)
		.map(|bridged| bridged.addr_msg.clone());

	let (proof, root) = match call_type {
		SubTrie::DataSubmit => {
			let proof = tx_data.submitted_proof_of(leaf_idx)?;
			let root = tx_data.bridged_root();
			(proof, root)
		},
		SubTrie::Bridge => {
			let proof = tx_data.bridged_proof_of(leaf_idx)?;
			let root = tx_data.submitted_root();
			(proof, root)
		},
	};

	Some(CallsProof::new(proof, root, message))
}

/// Verify Merkle Proof correctness versus given root hash.
///
/// The proof is NOT expected to contain leaf hash as the first
/// element, but only all adjacent nodes required to eventually by process of
/// concatenating and hashing end up with given root hash.
///
/// The proof must not contain the root hash.
#[allow(dead_code)]
pub fn verify<I>(
	root: H256,
	proof: I,
	number_of_submitted_data: u32,
	data_index: u32,
	data_hash: H256,
) -> bool
where
	I: IntoIterator<Item = H256>,
{
	let leaf = Leaf::Hash(data_hash);
	verify_proof::<Keccak256, _, _>(
		&root,
		proof,
		number_of_submitted_data as usize,
		data_index as usize,
		leaf,
	)
}

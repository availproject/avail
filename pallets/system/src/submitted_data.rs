pub use avail_core::data_proof_v2::{AddressedMessage, AddressedMessageRef, BoundedData};
use avail_core::{data_proof_v2::SubTrie, traits::MaybeCaller};

use binary_merkle_tree::{verify_proof, Leaf, MerkleProof};
use frame_support::traits::ExtrinsicCall;
use sp_core::H256;
use sp_runtime::{traits::Keccak256, AccountId32};
use sp_std::vec::Vec;

mod tx_data_ref;
pub use tx_data_ref::TxDataRef;

mod tx_data;
pub use tx_data::TxData;

mod metrics;
pub use metrics::{Metrics, RcMetrics};

pub mod traits;
pub use traits::TxDataFilter;

#[cfg(test)]
mod tests;

pub fn tx_data_root<F, E, A>(block: u32, indexed_ext: &[(usize, E)]) -> H256
where
	F: TxDataFilter<A, E::Call>,
	E: ExtrinsicCall + MaybeCaller<A>,
{
	let mut metrics = Metrics::default();

	let tx_data = indexed_ext
		.iter()
		.filter_map(|(idx, ext)| {
			let caller = ext.caller()?;
			let call = ext.call();
			F::filter(caller, call, block, *idx, &mut metrics)
		})
		.collect::<TxDataRef<'_>>();

	tx_data.root()
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
pub fn calls_proof<'a, F, I, C: 'a>(
	calls: I,
	block: u32,
	leaf_idx: usize,
	call_type: SubTrie,
) -> Option<(MerkleProof<H256, Vec<u8>>, H256, Option<AddressedMessage>)>
where
	F: TxDataFilter<AccountId32, C>,
	I: Iterator<Item = (&'a AccountId32, usize, &'a C)> + 'a,
{
	let mut metrics = Metrics::default();
	let tx_data = calls
		.filter_map(|(caller, tx_idx, call)| F::filter(caller, call, block, tx_idx, &mut metrics))
		.collect::<TxDataRef<'_>>();

	let message = tx_data
		.bridged_messages()
		.get(leaf_idx)
		.cloned()
		.map(AddressedMessage::from);

	let (proof, root) = match call_type {
		SubTrie::DataSubmit => {
			let proof = tx_data.submitted_data_proof(leaf_idx)?;
			let root = tx_data.bridged_messages_root();
			(proof, root)
		},
		SubTrie::Bridge => {
			let proof = tx_data.bridged_messages_proof(leaf_idx)?;
			let root = tx_data.submitted_data_root();
			(proof, root)
		},
	};

	Some((proof, root, message))
}

/// Verify Merkle Proof correctness versus given root hash.
///
/// The proof is NOT expected to contain leaf hash as the first
/// element, but only all adjacent nodes required to eventually by process of
/// concatenating and hashing end up with given root hash.
///
/// The proof must not contain the root hash.
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

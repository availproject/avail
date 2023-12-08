use avail_core::data_proof::SubTrie;
use core::fmt::Debug;

use avail_core::OpaqueExtrinsic;
use binary_merkle_tree::{merkle_proof, merkle_root, verify_proof, Leaf, MerkleProof};
use frame_support::Hashable;
use sp_core::H256;
use sp_io::hashing::keccak_256;
use sp_io::trie::keccak_256_root;
use sp_runtime::traits::Keccak256;
use sp_std::vec;
use sp_std::{cell::RefCell, rc::Rc, vec::Vec};

const LOG_TARGET: &str = "runtime::system::submitted_data";

/// Information about `submitted_data_root` and `submitted_data_proof` methods.
#[derive(Default, Debug)]
pub struct Metrics {
	/// Number of extrinsics containing one or more submitted data.
	pub data_submit_extrinsics: u32,
	/// Total number of submitted data.
	pub data_submit_leaves: u32,
	/// Total number of analysed extrinsic.
	pub total_extrinsics: u32,
}

pub type RcMetrics = Rc<RefCell<Metrics>>;

impl Metrics {
	/// Creates a shared metric with internal mutability.
	pub fn new_shared() -> RcMetrics {
		Rc::new(RefCell::new(Self::default()))
	}
}

/// Extracts the `data` field from some types of extrinsics.
pub trait Extractor {
	type Error: Debug;
	/// Returns the `data` field of `encoded_extrinsic` if it contains one, like a
	/// `Avail::SubmitData` call.
	///
	/// The `metrics` will be used to write accountability information about the whole process.
	fn extract(
		extrinsic: &OpaqueExtrinsic,
		metrics: RcMetrics,
	) -> Result<(Vec<Vec<u8>>, Vec<Vec<u8>>), Self::Error>;
}

#[cfg(any(feature = "std", test))]
impl Extractor for () {
	type Error = ();

	fn extract(_: &OpaqueExtrinsic, _: RcMetrics) -> Result<(Vec<Vec<u8>>, Vec<Vec<u8>>), ()> {
		Ok((vec![], vec![]))
	}
}

/// It is similar to `Extractor` but it uses `C` type for calls, instead of `AppExtrinsic`.
pub trait Filter<C> {
	/// Returns the `data` field of `call` if it is a one or multiple valid `da_ctrl::submit_data` call.
	fn filter(call: C, metrics: RcMetrics) -> (Vec<Vec<u8>>, Vec<Vec<u8>>);

	/// This function processes a list of calls and returns their data as Vec<Vec<u8>>
	fn process_calls(calls: Vec<C>, metrics: &RcMetrics) -> (Vec<Vec<u8>>, Vec<Vec<u8>>);
}

#[cfg(any(feature = "std", test))]
impl<C> Filter<C> for () {
	fn filter(_: C, _: RcMetrics) -> (Vec<Vec<u8>>, Vec<Vec<u8>>) {
		(vec![], vec![])
	}

	fn process_calls(_: Vec<C>, _: &RcMetrics) -> (Vec<Vec<u8>>, Vec<Vec<u8>>) {
		(vec![], vec![])
	}
}

fn extract_and_inspect<E>(
	opaque: &OpaqueExtrinsic,
	metrics: RcMetrics,
) -> (Vec<Vec<u8>>, Vec<Vec<u8>>)
where
	E: Extractor,
	E::Error: Debug,
{
	let extracted = E::extract(opaque, metrics);
	if let Err(e) = extracted.as_ref() {
		log::error!("Extractor cannot decode opaque: {e:?}");
	}

	let extracted_data = extracted.unwrap_or_default();

	let blob_root = extracted_data
		.0
		.into_iter()
		.filter(|data| !data.is_empty())
		.collect();
	let data_root = extracted_data
		.1
		.into_iter()
		.filter(|data| !data.is_empty())
		.collect();

	(blob_root, data_root)
}

/// Construct a root hash of Binary Merkle Tree created from given filtered `app_extrincs`.
pub fn extrinsics_root<'a, E, I>(opaque_itr: I) -> H256
where
	E: Extractor,
	E::Error: Debug,
	I: Iterator<Item = &'a OpaqueExtrinsic>,
{
	let metrics = Metrics::new_shared();

	let (blob_data, bridge_data): (Vec<_>, Vec<_>) = opaque_itr
		.map(|ext| extract_and_inspect::<E>(ext, Rc::clone(&metrics)))
		.unzip();

	let blob_data = blob_data.into_iter().flatten();
	let bridge_data = bridge_data.into_iter().flatten();

	let root_blob_data = blob_data
		.into_iter()
		.filter(|v| !v.is_empty())
		.map(|leaf| keccak_256(leaf.as_slice()).as_slice().to_vec())
		.collect::<Vec<_>>();

	let root_bridge_data = bridge_data
		.into_iter()
		.filter(|v| !v.is_empty())
		.map(|leaf| keccak_256(leaf.as_slice()).as_slice().to_vec())
		.collect::<Vec<_>>();

	let binding = root(root_blob_data.into_iter(), Rc::clone(&metrics));
	let blob_root = binding.as_bytes();

	let binding = root(root_bridge_data.into_iter(), Rc::clone(&metrics));

	let bridge_root = binding.as_bytes();

	let mut concat = vec![];
	// keccak_256(blob_root, bridge_root)
	concat.extend_from_slice(blob_root);
	concat.extend_from_slice(bridge_root);
	let hash = keccak_256(concat.as_slice());
	H256(hash)
}

/// Construct a root hash of a Binary Merkle Tree created from given leaves and stores
/// information about the process into `metrics`.
///
/// In case an empty list of leaves is passed the function returns a 0-filled hash.
pub fn root<I: Iterator<Item = Vec<u8>> + core::fmt::Debug>(
	submitted_data: I,
	metrics: RcMetrics,
) -> H256 {
	let root = merkle_root::<Keccak256, _>(submitted_data);
	log::debug!(
		target: LOG_TARGET,
		"Build submitted data root: {:?}, metrics: {:?}",
		root,
		metrics
	);

	root
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
pub fn calls_proof<F, I, C>(
	calls: I,
	transaction_index: u32,
	call_type: SubTrie,
) -> Option<(MerkleProof<H256, Vec<u8>>, H256)>
where
	F: Filter<C>,
	I: Iterator<Item = C>,
{
	let metrics = Metrics::new_shared();

	let transaction_index = usize::try_from(transaction_index).ok()?;
	let tx_max = transaction_index.checked_add(1)?;

	let (blob_data, bridge_data): (Vec<_>, Vec<_>) = calls
		.map(|ext| {
			let (l, r) = F::filter(ext, Rc::clone(&metrics));
			(
				l.into_iter().flatten().collect::<Vec<_>>(),
				r.into_iter().flatten().collect::<Vec<_>>(),
			)
		})
		.unzip();

	let mut submitted_data = vec![];
	let mut root_data = vec![];

	match call_type {
		SubTrie::Left => {
			submitted_data = blob_data;
			root_data = bridge_data;
		},
		SubTrie::Right => {
			submitted_data = bridge_data;
			root_data = blob_data;
		},
	}

	match submitted_data.get(transaction_index) {
		None => return None,
		Some(data) if data.is_empty() => return None,
		_ => (),
	};

	let data_index = submitted_data
		.iter()
		.take(tx_max)
		.filter(|data| !data.is_empty())
		.count() - 1;

	// clean root data
	let data = submitted_data
		.into_iter()
		.filter(|v| !v.is_empty())
		.map(|leaf| keccak_256(leaf.as_slice()).as_slice().to_vec())
		.collect::<Vec<_>>();

	// clean root data
	let root_data = root_data
		.into_iter()
		.filter(|v| !v.is_empty())
		.map(|leaf| keccak_256(leaf.as_slice()).as_slice().to_vec())
		.collect::<Vec<_>>();

	let root = root(root_data.into_iter(), Rc::clone(&metrics));
	let data_index = u32::try_from(data_index).ok()?;
	return if let Some(proof) = proof(data, data_index, Rc::clone(&metrics)) {
		Some((proof, root))
	} else {
		None
	};
}

/// Construct a Merkle Proof for `submit_data` given by `data_index` and stores
/// information about the process into `metrics`.
///
/// If `data_index` is greater than the number of Merkle leaves, it will return `None`.
fn proof(
	submitted_data: Vec<Vec<u8>>,
	data_index: u32,
	metrics: RcMetrics,
) -> Option<MerkleProof<H256, Vec<u8>>> {
	let data_index = data_index as usize;
	// NOTE: `merkle_proof` panics if `data_index > leaves`.
	if data_index >= submitted_data.len() {
		return None;
	}

	let proof = merkle_proof::<Keccak256, _, _>(submitted_data, data_index);
	log::debug!(
		target: LOG_TARGET,
		"Build submitted data proof of index {data_index}: {:?} metrics: {:?}",
		proof,
		metrics
	);

	Some(proof)
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

#[cfg(test)]
mod test {
	use hex_literal::hex;
	use sp_core::{keccak_256, H256};
	use std::vec;

	use crate::submitted_data::{calls_proof, proof, root, Filter, Metrics, RcMetrics};

	// dummy filter implementation that skips empty strings in vector
	impl<C> Filter<C> for String
	where
		String: From<C>,
	{
		fn filter(d: C, _: RcMetrics) -> (Vec<Vec<u8>>, Vec<Vec<u8>>) {
			let s = String::try_from(d).unwrap();
			if s.is_empty() {
				(vec![], vec![])
			} else {
				(vec![s.into_bytes()], vec![])
			}
		}

		fn process_calls(_: Vec<C>, _: &RcMetrics) -> (Vec<Vec<u8>>, Vec<Vec<u8>>) {
			(vec![], vec![])
		}
	}

	#[test]
	fn test_() {
		let tx1_data: String = String::from("0");
		let tx2_data: String = String::from("1");
		let tx3_data: String = String::from("2");

		let tx1_hash = keccak_256(String::from("0").as_bytes());
		let tx2_hash = keccak_256(String::from("1").as_bytes());
		let tx3_hash = keccak_256(String::from("2").as_bytes());

		println!("0 {:?}", H256(tx1_hash));
		println!("1 {:?}", H256(tx2_hash));
		println!("2 {:?}", H256(tx3_hash));

		println!("hashed 0 {:?}", H256(keccak_256(tx1_hash.as_slice())));
		println!("hashed 1 {:?}", H256(keccak_256(tx2_hash.as_slice())));
		println!("hashed 2 {:?}", H256(keccak_256(tx3_hash.as_slice())));

		let submitted_data: Vec<u8> = vec![];
		let metrics = Metrics::new_shared();

		// let proof = proof(submitted_data, 0, metrics);
		let blob_root = H256(hex!(
			"044852b2a670ade5407e78fb2863c51de9fcb96542a07186fe3aeda6bb8a116d"
		))
		.as_bytes()
		.to_vec();
		let bridge_root = H256::zero().as_bytes().to_vec();
		let root1 = root(
			vec![blob_root.clone(), bridge_root.clone()].into_iter(),
			metrics,
		); // 0x9c6b2c1b0d0b25a008e6c882cc7b415f309965c72ad2b944ac0931048ca31cd5 which vec[vec[]] is not the same as H256::zero
		let root2 = root(
			vec![bridge_root.clone(), blob_root.clone()].into_iter(),
			Metrics::new_shared(),
		); // 0x9c6b2c1b0d0b25a008e6c882cc7b415f309965c72ad2b944ac0931048ca31cd5 which vec[vec[]] is not the same as H256::zero
   // let root1 = root(vec![].into_iter(), metrics);

		println!("{:?}", root1);
		println!("{:?}", root2);
	}

	// #[test]
	// fn test_data_proof_with_skipped_tx() {
	//     let tx1_data: String = String::from("0");
	//     let tx2_data: String = String::new(); // tx should be skipped
	//     let tx3_data: String = String::from("1");
	//     let tx4_data: String = String::from("2");
	//
	//     let submitted_data = vec![tx1_data, tx2_data, tx3_data, tx4_data];
	//
	//     // leaf 0 044852b2a670ade5407e78fb2863c51de9fcb96542a07186fe3aeda6bb8a116d
	//     // leaf 1 c89efdaa54c0f20c7adf612882df0950f5a951637e0307cdcb4c672f298b8bc6
	//     // leaf 2 ad7c5bef027816a800da1736444fb58a807ef4c9603b7848673f7e3a68eb14a5
	//     // intermediate root (leaf[0], leaf[1])  0b4aa17bff8fc189efb37609ac5ea9fca0df4c834a6fbac74b24c8119c40fef2
	//     // data_root 0f1d00f4d84258b8d99bfc4748ff45b8039f108f43ca47e22ac5a1eab2e8c02d
	//
	//     if let Some(da_proof) = calls_proof::<String, _, _>(submitted_data.clone().into_iter(), 0) {
	//         assert_eq!(da_proof.leaf_index, 0);
	//         assert_eq!(
	//             format!("{:#x}", da_proof.root),
	//             "0x0f1d00f4d84258b8d99bfc4748ff45b8039f108f43ca47e22ac5a1eab2e8c02d"
	//         );
	//         assert_eq!(da_proof.proof.len(), 2);
	//         assert_eq!(
	//             format!("{:#x}", da_proof.proof[0]),
	//             "0xc89efdaa54c0f20c7adf612882df0950f5a951637e0307cdcb4c672f298b8bc6"
	//         );
	//         assert_eq!(
	//             format!("{:#x}", da_proof.proof[1]),
	//             "0xad7c5bef027816a800da1736444fb58a807ef4c9603b7848673f7e3a68eb14a5"
	//         );
	//         assert_eq!(da_proof.number_of_leaves, 3);
	//     } else {
	//         panic!("Proof not generated for the transaction index 0!");
	//     }
	//
	//     // proof should not be generated when there is not data
	//     assert_eq!(
	//         None,
	//         calls_proof::<String, _, _>(submitted_data.clone().into_iter(), 1)
	//     );
	//
	//     if let Some(da_proof) = calls_proof::<String, _, _>(submitted_data.clone().into_iter(), 2) {
	//         assert_eq!(da_proof.leaf_index, 1);
	//         assert_eq!(
	//             format!("{:#x}", da_proof.root),
	//             "0x0f1d00f4d84258b8d99bfc4748ff45b8039f108f43ca47e22ac5a1eab2e8c02d"
	//         );
	//         assert_eq!(da_proof.proof.len(), 2);
	//         assert_eq!(
	//             format!("{:#x}", da_proof.proof[0]),
	//             "0x044852b2a670ade5407e78fb2863c51de9fcb96542a07186fe3aeda6bb8a116d"
	//         );
	//         assert_eq!(
	//             format!("{:#x}", da_proof.proof[1]),
	//             "0xad7c5bef027816a800da1736444fb58a807ef4c9603b7848673f7e3a68eb14a5"
	//         );
	//         assert_eq!(da_proof.number_of_leaves, 3);
	//     } else {
	//         panic!("Proof not generated for the transaction index 2!");
	//     }
	//
	//     if let Some(da_proof) = calls_proof::<String, _, _>(submitted_data.clone().into_iter(), 3) {
	//         assert_eq!(da_proof.leaf_index, 2);
	//         assert_eq!(
	//             format!("{:#x}", da_proof.root),
	//             "0x0f1d00f4d84258b8d99bfc4748ff45b8039f108f43ca47e22ac5a1eab2e8c02d"
	//         );
	//         assert_eq!(da_proof.proof.len(), 1);
	//         assert_eq!(
	//             format!("{:#x}", da_proof.proof[0]),
	//             "0x0b4aa17bff8fc189efb37609ac5ea9fca0df4c834a6fbac74b24c8119c40fef2"
	//         );
	//         assert_eq!(da_proof.number_of_leaves, 3);
	//     } else {
	//         panic!("Proof not generated for the transaction index 3!");
	//     }
	//
	//     // submit index that does not exists and proof should not be generated
	//     assert_eq!(
	//         None,
	//         calls_proof::<String, _, _>(submitted_data.clone().into_iter(), 15)
	//     );
	// }
}

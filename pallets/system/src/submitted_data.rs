use avail_core::data_proof_v2::SubTrie;
pub use avail_core::data_proof_v2::{BoundedData, Message, MessageType};
use avail_core::OpaqueExtrinsic;
use binary_merkle_tree::{merkle_proof, merkle_root, verify_proof, Leaf, MerkleProof};
use core::fmt::Debug;
use sp_core::H256;
use sp_io::hashing::keccak_256;
use sp_runtime::traits::Keccak256;
use sp_runtime::AccountId32;
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
	fn new_shared() -> RcMetrics {
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
	// Note: This should be deprecated in the upcoming versions in favour of extract_v2
	fn extract(
		extrinsic: &OpaqueExtrinsic,
		metrics: RcMetrics,
	) -> Result<Vec<Vec<u8>>, Self::Error>;

	/// Returns the `data` or `Message` based on whether the given extrinsic is `da::submit_data`
	/// or `bridge::send_message` Call respectively. It supports both v1 & v2 headers.
	///
	/// The `metrics` will be used to write accountability information about the whole process.
	#[allow(clippy::type_complexity)]
	fn extract_v2(
		extrinsic: &OpaqueExtrinsic,
		metrics: RcMetrics,
	) -> Result<(Vec<Vec<u8>>, Vec<Message>), Self::Error>;
}

#[cfg(any(feature = "std", test))]
impl Extractor for () {
	type Error = ();

	fn extract(_: &OpaqueExtrinsic, _: RcMetrics) -> Result<Vec<Vec<u8>>, ()> {
		Ok(vec![])
	}

	fn extract_v2(_: &OpaqueExtrinsic, _: RcMetrics) -> Result<(Vec<Vec<u8>>, Vec<Message>), ()> {
		Ok((vec![], vec![]))
	}
}

/// It is similar to `Extractor` but it uses `C` type for calls, instead of `AppExtrinsic`.
pub trait Filter<C> {
	/// Returns the `data` field of `call` if it is a one or multiple valid `da_ctrl::submit_data` call.
	fn filter(call: C, metrics: RcMetrics) -> Vec<Vec<u8>>;

	fn filter_v2(call: C, metrics: RcMetrics, caller: AccountId32) -> (Vec<Vec<u8>>, Vec<Message>);

	/// This function processes a list of calls and returns their data as Vec<Vec<u8>>
	fn process_calls(calls: Vec<C>, metrics: &RcMetrics) -> Vec<Vec<u8>>;

	fn process_calls_v2(
		calls: Vec<C>,
		metrics: &RcMetrics,
		caller: AccountId32,
	) -> (Vec<Vec<u8>>, Vec<Message>);
}

#[cfg(any(feature = "std", test))]
impl<C> Filter<C> for () {
	fn filter(_: C, _: RcMetrics) -> Vec<Vec<u8>> {
		vec![]
	}

	fn process_calls(_: Vec<C>, _: &RcMetrics) -> Vec<Vec<u8>> {
		vec![]
	}

	fn filter_v2(_: C, _: RcMetrics, _: AccountId32) -> (Vec<Vec<u8>>, Vec<Message>) {
		(vec![], vec![])
	}

	fn process_calls_v2(_: Vec<C>, _: &RcMetrics, _: AccountId32) -> (Vec<Vec<u8>>, Vec<Message>) {
		(vec![], vec![])
	}
}

// This should be deprecated in upcoming version in favour of extract_and_inspect_v2
fn extract_and_inspect<E>(opaque: &OpaqueExtrinsic, metrics: RcMetrics) -> Vec<Vec<u8>>
where
	E: Extractor,
	E::Error: Debug,
{
	let extracted = E::extract(opaque, metrics);
	if let Err(e) = extracted.as_ref() {
		log::error!("Extractor cannot decode opaque: {e:?}");
	}
	extracted
		.unwrap_or_default()
		.into_iter()
		.filter(|data| !data.is_empty())
		.collect()
}

// Supports both v1 & v2 headers
fn extract_and_inspect_v2<E>(
	opaque: &OpaqueExtrinsic,
	metrics: RcMetrics,
) -> (Vec<Vec<u8>>, Vec<Message>)
where
	E: Extractor,
	E::Error: Debug,
{
	let extracted = E::extract_v2(opaque, metrics);
	if let Err(e) = extracted.as_ref() {
		log::error!("Extractor cannot decode opaque: {e:?}");
	}

	let (blob_root_data, bridge_root_data) = extracted.unwrap_or_default();

	// This filtering is required for data extraction of v1 header
	let blob_root = blob_root_data
		.into_iter()
		.filter(|data| !data.is_empty())
		.collect();

	(blob_root, bridge_root_data)
}

/// Construct a root hash of Binary Merkle Tree created from given filtered `app_extrincs`.
pub fn extrinsics_root<'a, E, I>(opaque_itr: I) -> H256
where
	E: Extractor,
	E::Error: Debug,
	I: Iterator<Item = &'a OpaqueExtrinsic> + core::fmt::Debug,
{
	let metrics = Metrics::new_shared();
	let submitted_data =
		opaque_itr.flat_map(|ext| extract_and_inspect::<E>(ext, Rc::clone(&metrics)));

	root(submitted_data, Rc::clone(&metrics))
}

pub fn extrinsics_root_v2<'a, E, I>(opaque_itr: I, nonce: u64) -> (H256, u64)
where
	E: Extractor,
	E::Error: Debug,
	I: Iterator<Item = &'a OpaqueExtrinsic>,
{
	let mut bridge_nonce = nonce;
	let metrics = Metrics::new_shared();

	let (blob_data, bridge_data): (Vec<_>, Vec<_>) = opaque_itr
		.map(|ext| extract_and_inspect_v2::<E>(ext, Rc::clone(&metrics)))
		.unzip();

	let blob_data = blob_data.into_iter().flatten();
	let bridge_data = bridge_data.into_iter().flatten();

	let root_blob_data = blob_data
		.into_iter()
		.filter(|v| !v.is_empty())
		.map(|leaf| keccak_256(leaf.as_slice()).as_slice().to_vec())
		.collect::<Vec<_>>();

	let root_bridge_data: Vec<_> = bridge_data
		.into_iter()
		.map(|mut m| {
			bridge_nonce += 1;
			m.id = bridge_nonce;
			m.abi_encode().to_vec()
		})
		.collect();

	// make leaves 2^n
	let root_data_balanced = calculate_balance_trie(root_blob_data).unwrap_or_default();
	let data_filtered_balanced = calculate_balance_trie(root_bridge_data).unwrap_or_default();

	let blob_root = root(root_data_balanced.into_iter(), Rc::clone(&metrics));
	let bridge_root = root(data_filtered_balanced.into_iter(), Rc::clone(&metrics));

	log::debug!("bridge root {:?}", bridge_root);
	log::debug!("blob root {:?}", blob_root);

	let mut concat = vec![];
	// keccak_256(blob_root, bridge_root)
	concat.extend_from_slice(blob_root.as_bytes());
	concat.extend_from_slice(bridge_root.as_bytes());
	let hash = keccak_256(concat.as_slice());
	(H256(hash), bridge_nonce)
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
pub fn calls_proof<F, I, C>(calls: I, transaction_index: u32) -> Option<MerkleProof<H256, Vec<u8>>>
where
	F: Filter<C>,
	I: Iterator<Item = C>,
{
	let metrics = Metrics::new_shared();

	let transaction_index = usize::try_from(transaction_index).ok()?;
	let tx_max = transaction_index.checked_add(1)?;

	let submitted_data = calls
		.map(|c| {
			F::filter(c, Rc::clone(&metrics))
				.into_iter()
				.flatten()
				.collect::<Vec<_>>()
		})
		.collect::<Vec<_>>();

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

	let data = submitted_data
		.into_iter()
		.filter(|v| !v.is_empty())
		.collect::<Vec<_>>();

	let data_index = u32::try_from(data_index).ok()?;
	proof(data, data_index, Rc::clone(&metrics))
}

#[allow(clippy::type_complexity)]
pub fn calls_proof_v2<F, I, C>(
	calls: I,
	callers: Vec<AccountId32>,
	transaction_index: u32,
	bridge_nonce: u64,
	call_type: SubTrie,
) -> Option<(MerkleProof<H256, Vec<u8>>, H256, Option<Message>)>
where
	F: Filter<C>,
	I: Iterator<Item = C>,
{
	let metrics = Metrics::new_shared();

	let transaction_index = usize::try_from(transaction_index).ok()?;
	let tx_max = transaction_index.checked_add(1)?;
	let mut nonce = bridge_nonce;
	let mut message_data: Option<Message> = None;

	let (blob_data, bridge_data): (Vec<_>, Vec<_>) = calls
		.zip(callers)
		.enumerate()
		.map(|(index, (ext, caller))| {
			let (l, r) = F::filter_v2(ext, Rc::clone(&metrics), caller);
			let r_with_id: Vec<_> = r
				.into_iter()
				.flat_map(|mut m| {
					nonce += 1;
					m.id = nonce;
					if index == transaction_index {
						message_data = Some(m.clone());
					}
					m.abi_encode()
				})
				.collect();
			(l.into_iter().flatten().collect::<Vec<_>>(), r_with_id)
		})
		.unzip();

	let submitted_data: Vec<Vec<u8>>;
	let root_data: Vec<Vec<u8>>;

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
	let data_filtered = submitted_data
		.into_iter()
		.filter(|v| !v.is_empty())
		.map(|leaf| {
			if call_type == SubTrie::Right {
				leaf
			} else {
				keccak_256(leaf.as_slice()).to_vec()
			}
		})
		.collect::<Vec<_>>();

	// clean root data
	let root_data_filtered = root_data
		.into_iter()
		.filter(|v| !v.is_empty())
		.map(|leaf| {
			if call_type == SubTrie::Left {
				leaf
			} else {
				keccak_256(leaf.as_slice()).to_vec()
			}
		})
		.collect::<Vec<_>>();

	// make leaves 2^n
	let root_data_balanced = calculate_balance_trie(root_data_filtered).or(None)?;
	let data_filtered_balanced = calculate_balance_trie(data_filtered).or(None)?;

	let root = root(root_data_balanced.into_iter(), Rc::clone(&metrics));

	let data_index = u32::try_from(data_index).ok()?;

	proof(data_filtered_balanced, data_index, Rc::clone(&metrics))
		.map(|proof| (proof, root, message_data))
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

/// calculate_balance_trie extends trie to the nearest pow of 2 number of laves
pub fn calculate_balance_trie(mut data: Vec<Vec<u8>>) -> Option<Vec<Vec<u8>>> {
	if data.is_empty() {
		return Some(data);
	}
	let card = u32::try_from(data.len()).ok()?;
	let next_pow_2 = libm::ceil(libm::log2(card as f64)) as u32;
	let leafs_to_append = usize::try_from(2u32.pow(next_pow_2) - card).ok()?;
	let to_append = vec![H256::zero().as_bytes().to_vec(); leafs_to_append];

	data.extend(to_append);

	Some(data)
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
	use crate::submitted_data::SubTrie;
	use codec::Encode;
	use frame_support::traits::DefensiveTruncateFrom;
	use hex_literal::hex;
	use sp_core::{keccak_256, H256, U256};
	use sp_runtime::{AccountId32, BoundedVec};
	use std::vec;

	use crate::submitted_data::{
		calculate_balance_trie, calls_proof_v2, Filter, Message, MessageType, RcMetrics,
	};

	// dummy filter implementation that skips empty strings in vector
	impl<C> Filter<C> for String
	where
		String: From<C>,
	{
		fn filter(d: C, _: RcMetrics) -> Vec<Vec<u8>> {
			let s = String::try_from(d).unwrap();
			if s.is_empty() {
				vec![]
			} else {
				vec![s.into_bytes()]
			}
		}

		fn process_calls(_: Vec<C>, _: &RcMetrics) -> Vec<Vec<u8>> {
			vec![]
		}

		fn filter_v2(d: C, _: RcMetrics, _: AccountId32) -> (Vec<Vec<u8>>, Vec<Message>) {
			let s = String::try_from(d).unwrap();
			if s.is_empty() {
				(vec![], vec![])
			} else {
				(vec![s.into_bytes()], vec![])
			}
		}
		fn process_calls_v2(
			_: Vec<C>,
			_: &RcMetrics,
			_: AccountId32,
		) -> (Vec<Vec<u8>>, Vec<Message>) {
			(vec![], vec![])
		}
	}

	#[test]
	fn test_left_data_proof_with_one_tx() {
		let tx1_data: String = String::from("0");
		let submitted_data = vec![tx1_data];
		// leaf 0 keccak256(044852b2a670ade5407e78fb2863c51de9fcb96542a07186fe3aeda6bb8a116d)
		//                  40105d5bc10105c17fd72b93a8f73369e2ee6eee4d4714b7bf7bf3c2f156e601

		let callers: Vec<AccountId32> = vec![AccountId32::new([0u8; 32])];
		let bridge_nonce: u64 = 0u64;

		if let Some((da_proof, root, _)) = calls_proof_v2::<String, _, _>(
			submitted_data.clone().into_iter(),
			callers.clone(),
			0,
			bridge_nonce,
			SubTrie::Left,
		) {
			assert_eq!(root, H256::zero());
			assert_eq!(da_proof.leaf_index, 0);
			assert_eq!(
				format!("{:#x}", da_proof.root),
				"0x40105d5bc10105c17fd72b93a8f73369e2ee6eee4d4714b7bf7bf3c2f156e601"
			);
			assert_eq!(da_proof.proof.len(), 0);
			assert_eq!(da_proof.number_of_leaves, 1);
		} else {
			panic!("Proof not generated for the transaction index 0!");
		}
	}

	#[test]
	fn test_left_data_proof_with_two_tx() {
		let tx1_data: String = String::from("0");
		let tx2_data: String = String::from("1");

		let submitted_data = vec![tx1_data, tx2_data];
		// leaf 0 keccak256(044852b2a670ade5407e78fb2863c51de9fcb96542a07186fe3aeda6bb8a116d)
		//                  40105d5bc10105c17fd72b93a8f73369e2ee6eee4d4714b7bf7bf3c2f156e601
		// leaf 1 keccak256(c89efdaa54c0f20c7adf612882df0950f5a951637e0307cdcb4c672f298b8bc6)
		//                  4aeff0db81e3146828378be230d377356e57b6d599286b4b517dbf8941b3e1b2

		let callers: Vec<AccountId32> =
			vec![AccountId32::new([0u8; 32]), AccountId32::new([0u8; 32])];
		let bridge_nonce: u64 = 0u64;

		if let Some((da_proof, root, _)) = calls_proof_v2::<String, _, _>(
			submitted_data.clone().into_iter(),
			callers.clone(),
			0,
			bridge_nonce,
			SubTrie::Left,
		) {
			assert_eq!(root, H256::zero());
			assert_eq!(da_proof.leaf_index, 0);
			assert_eq!(
				format!("{:#x}", da_proof.root),
				"0xdb0ccc7a2d6559682303cc9322d4b79a7ad619f0c87d5f94723a33015550a64e"
			);
			assert_eq!(da_proof.proof.len(), 1);
			assert_eq!(da_proof.number_of_leaves, 2);

			assert_eq!(
				format!("{:#x}", da_proof.proof[0]),
				"0x4aeff0db81e3146828378be230d377356e57b6d599286b4b517dbf8941b3e1b2"
			);
		} else {
			panic!("Proof not generated for the transaction index 0!");
		}
	}

	#[test]
	fn test_left_data_proof_with_skipped_tx() {
		let tx1_data: String = String::from("0");
		let tx2_data: String = String::new(); // tx should be skipped
		let tx3_data: String = String::from("1");
		let tx4_data: String = String::from("2");
		let callers: Vec<AccountId32> = vec![
			AccountId32::new([0u8; 32]),
			AccountId32::new([0u8; 32]),
			AccountId32::new([0u8; 32]),
			AccountId32::new([0u8; 32]),
		];
		let bridge_nonce: u64 = 0u64;

		let submitted_data = vec![tx1_data, tx2_data, tx3_data, tx4_data];

		// leaf 0 keccak256(044852b2a670ade5407e78fb2863c51de9fcb96542a07186fe3aeda6bb8a116d)
		//                  40105d5bc10105c17fd72b93a8f73369e2ee6eee4d4714b7bf7bf3c2f156e601
		// leaf 1 keccak256(c89efdaa54c0f20c7adf612882df0950f5a951637e0307cdcb4c672f298b8bc6)
		//                  4aeff0db81e3146828378be230d377356e57b6d599286b4b517dbf8941b3e1b2
		// leaf 2 keccak256(ad7c5bef027816a800da1736444fb58a807ef4c9603b7848673f7e3a68eb14a5)
		//                  1204b3dcd975ba0a68eafbf4d2ca0d13cc7b5e3709749c1dc36e6e74934270b3
		//  leaf appended in in order to have 2^n number of leaves
		// leaf 3 (appended) keccak256(0000000000000000000000000000000000000000000000000000000000000000)
		//                  290decd9548b62a8d60345a988386fc84ba6bc95484008f6362f93160ef3e563

		// intermediate root (leaf[0], leaf[1]) db0ccc7a2d6559682303cc9322d4b79a7ad619f0c87d5f94723a33015550a64e
		// intermediate root (leaf[2], leaf[3]) 3c86bde3a90d18efbcf23e27e9b6714012aa055263fe903a72333aa9caa37f1b
		// data_root keccak256(db0ccc7a2d6559682303cc9322d4b79a7ad619f0c87d5f94723a33015550a64e, 3c86bde3a90d18efbcf23e27e9b6714012aa055263fe903a72333aa9caa37f1b)
		//                                                       (877f9ed6aa67f160e9b9b7794bb851998d15b65d11bab3efc6ff444339a3d750)

		if let Some((da_proof, root, _)) = calls_proof_v2::<String, _, _>(
			submitted_data.clone().into_iter(),
			callers.clone(),
			0,
			bridge_nonce,
			SubTrie::Left,
		) {
			assert_eq!(root, H256::zero());
			assert_eq!(da_proof.leaf_index, 0);
			assert_eq!(
				format!("{:#x}", da_proof.root),
				"0x877f9ed6aa67f160e9b9b7794bb851998d15b65d11bab3efc6ff444339a3d750"
			);
			assert_eq!(da_proof.proof.len(), 2);
			assert_eq!(
				format!("{:#x}", da_proof.proof[0]),
				"0x4aeff0db81e3146828378be230d377356e57b6d599286b4b517dbf8941b3e1b2"
			);
			assert_eq!(
				format!("{:#x}", da_proof.proof[1]),
				"0x3c86bde3a90d18efbcf23e27e9b6714012aa055263fe903a72333aa9caa37f1b"
			);

			assert_eq!(
				H256::from_slice(da_proof.leaf.as_slice()),
				H256(hex!(
					"044852b2a670ade5407e78fb2863c51de9fcb96542a07186fe3aeda6bb8a116d"
				))
			);

			assert_eq!(da_proof.number_of_leaves, 4);
		} else {
			panic!("Proof not generated for the transaction index 0!");
		}

		// proof should not be generated when there is not data
		assert_eq!(
			None,
			calls_proof_v2::<String, _, _>(
				submitted_data.clone().into_iter(),
				callers.clone(),
				1,
				bridge_nonce,
				SubTrie::Left,
			)
		);

		if let Some((da_proof, root, _)) = calls_proof_v2::<String, _, _>(
			submitted_data.clone().into_iter(),
			callers.clone(),
			2,
			bridge_nonce,
			SubTrie::Left,
		) {
			assert_eq!(root, H256::zero());
			assert_eq!(da_proof.leaf_index, 1);
			assert_eq!(
				format!("{:#x}", da_proof.root),
				"0x877f9ed6aa67f160e9b9b7794bb851998d15b65d11bab3efc6ff444339a3d750"
			);
			assert_eq!(da_proof.proof.len(), 2);
			assert_eq!(
				format!("{:#x}", da_proof.proof[0]),
				"0x40105d5bc10105c17fd72b93a8f73369e2ee6eee4d4714b7bf7bf3c2f156e601"
			);
			assert_eq!(
				format!("{:#x}", da_proof.proof[1]),
				"0x3c86bde3a90d18efbcf23e27e9b6714012aa055263fe903a72333aa9caa37f1b"
			);
			assert_eq!(da_proof.number_of_leaves, 4);
		} else {
			panic!("Proof not generated for the transaction index 2!");
		}

		if let Some((da_proof, root, _)) = calls_proof_v2::<String, _, _>(
			submitted_data.clone().into_iter(),
			callers.clone(),
			3,
			bridge_nonce,
			SubTrie::Left,
		) {
			assert_eq!(root, H256::zero());
			assert_eq!(da_proof.leaf_index, 2);
			assert_eq!(
				format!("{:#x}", da_proof.root),
				"0x877f9ed6aa67f160e9b9b7794bb851998d15b65d11bab3efc6ff444339a3d750"
			);
			assert_eq!(da_proof.proof.len(), 2);
			assert_eq!(
				format!("{:#x}", da_proof.proof[0]),
				"0x290decd9548b62a8d60345a988386fc84ba6bc95484008f6362f93160ef3e563"
			);
			assert_eq!(
				format!("{:#x}", da_proof.proof[1]),
				"0xdb0ccc7a2d6559682303cc9322d4b79a7ad619f0c87d5f94723a33015550a64e"
			);
			assert_eq!(da_proof.number_of_leaves, 4);
		} else {
			panic!("Proof not generated for the transaction index 3!");
		}

		// submit index that does not exists and proof should not be generated
		assert_eq!(
			None,
			calls_proof_v2::<String, _, _>(
				submitted_data.clone().into_iter(),
				callers.clone(),
				15,
				bridge_nonce,
				SubTrie::Left,
			)
		);
	}

	#[test]
	fn test_pow_2_elements() {
		let empty: Vec<Vec<u8>> = vec![];
		if let Some(balanced) = calculate_balance_trie(empty) {
			assert_eq!(balanced.len(), 0)
		} else {
			panic!("Trie leaves must be empty!");
		}

		let mut leaves = vec![0u32.to_be_bytes().to_vec()];
		for i in 1..20u32 {
			if let Some(balanced) = calculate_balance_trie(leaves.clone()) {
				let before = leaves.len() as u32;
				let after = balanced.len() as u32;
				let next_pow_2 = (before as f64).log2().ceil() as u32;
				let missing = 2u32.pow(next_pow_2) - before;

				assert_eq!(after - before, missing);
				leaves.push(i.to_be_bytes().to_vec())
			} else {
				panic!("Trie leaves must be empty!");
			}
		}
	}

	#[test]
	fn test_message_encoding() {
		let expected_encoded_message = hex!("00000000000000000000000000000000000000000000000000000000000000200200000000000000000000000000000000000000000000000000000000000000681257bed628425a28b469114dc21a7c30205cfd00000000000000000000000000000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000000e00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000004000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000de0b6b3a7640000").to_vec();

		// Message(0x02, bytes32(bytes20(0x681257BED628425a28B469114Dc21A7c30205cFD)), bytes32(uint256(1)), 2, 1, abi.encode(bytes32(0), 1 ether), 0);
		let data = &[
			ethabi::Token::FixedBytes(H256::zero().encode()),
			ethabi::Token::Uint(U256::from(1000000000000000000u128)),
		];

		let encoded_data = BoundedVec::try_from(ethabi::encode(data)).unwrap();

		let message = Message {
			message_type: MessageType::FungibleToken,
			from: H256(hex!(
				"681257BED628425a28B469114Dc21A7c30205cFD000000000000000000000000"
			)),
			to: H256(hex!(
				"0000000000000000000000000000000000000000000000000000000000000001"
			)),
			origin_domain: 2,
			destination_domain: 1,
			data: encoded_data,
			id: 0,
		};

		let abi_encoded = message.abi_encode();
		assert_eq!(expected_encoded_message, abi_encoded);
	}

	#[test]
	fn test_message_encoding_from_avail_with_hash() {
		// Message is : Message {
		// message_type: FungibleToken,
		// from: 0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d,
		// to: 0x0000000000000000000000000000000000000000000000000000000000000001,
		// origin_domain: 1,
		// destination_domain: 2,
		// data: BoundedVec([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 102400),
		// id: 1 }

		let data = &[
			ethabi::Token::FixedBytes(H256::zero().encode()),
			ethabi::Token::Uint(U256::from(1u128)),
		];

		let encoded_data = BoundedVec::defensive_truncate_from(ethabi::encode(data));

		let message = Message {
			message_type: MessageType::FungibleToken,
			from: H256(hex!(
				"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d"
			)),
			to: H256(hex!(
				"0000000000000000000000000000000000000000000000000000000000000001"
			)),
			origin_domain: 1,
			destination_domain: 2,
			data: encoded_data,
			id: 1,
		};

		let encoded = message.abi_encode();

		let leaf_hash = H256(hex!(
			"ccd6cb2b400270449e283f0f9e4fdf1dbfeb44fa5d86468272d6834d2be7574f"
		));

		assert_eq!(leaf_hash, H256(keccak_256(encoded.as_slice())));
	}

	#[test]
	fn test_message_encoding_from_avail_with_hash1() {
		let data = &[
			ethabi::Token::FixedBytes(H256::zero().encode()),
			ethabi::Token::Uint(U256::from(1000000000000000000u128)),
		];

		let encoded_data = BoundedVec::defensive_truncate_from(ethabi::encode(data));

		let message = Message {
			message_type: MessageType::FungibleToken,
			from: H256(hex!(
				"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d"
			)),
			to: H256(hex!(
				"0000000000000000000000000000000000000000000000000000000000000001"
			)),
			origin_domain: 1,
			destination_domain: 2,
			data: encoded_data,
			id: 1,
		};

		let encoded = message.abi_encode();
		let leaf_hash = H256(hex!(
			"94491650baa28a6f0db3c5e9495e12e43b7f1b2726fa5c5dabed2619514bd7b5"
		));

		assert_eq!(leaf_hash, H256(keccak_256(encoded.as_slice())));
	}

	#[test]
	fn test_amb_message_encoding() {
		let expected_encoding = hex!("000000000000000000000000000000000000000000000000000000000000002001000000000000000000000000000000000000000000000000000000000000008f8d47bf15953e26c622f36f3366e43e26b9b78b000000000000000000000000c437b127628aa7984af0f001dc7ac023eee266f0df6356ef9243f340af8842360000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000000e0000000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000002b67");
		let expected_hash =
			hex!("ad79a34ee43ea39301b1f190558ea279122328ebd66b342a49a131ee5befd3b5");

		let data = H256(hex!(
			"0000000000000000000000000000000000000000000000000000000000002b67"
		));
		let encoded_data = BoundedVec::defensive_truncate_from(data.as_bytes().to_vec());

		let message = Message {
			message_type: MessageType::ArbitraryMessage,
			from: H256(hex!(
				"8f8d47bf15953e26c622f36f3366e43e26b9b78b000000000000000000000000"
			)),
			to: H256(hex!(
				"c437b127628aa7984af0f001dc7ac023eee266f0df6356ef9243f340af884236"
			)),
			origin_domain: 2,
			destination_domain: 1,
			data: encoded_data,
			id: 1,
		};
		let encoded_message = message.abi_encode();
		assert_eq!(expected_encoding.to_vec(), encoded_message.to_vec());
		assert_eq!(expected_hash, keccak_256(encoded_message.as_slice()));
	}

	#[test]
	fn test_amb_message_encoding_with_hash_check() {
		let expected_encoding = hex!("00000000000000000000000000000000000000000000000000000000000000200100000000000000000000000000000000000000000000000000000000000000681257bed628425a28b469114dc21a7c30205cfd0000000000000000000000003547517355657647456b6f7847444a5044576251694b4478714b6d675a3570470000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000000e00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000d48656c6c6f2c20576f726c642100000000000000000000000000000000000000");
		let expected_hash =
			hex!("5774ba3f9618e2da3885b0e2853e4005c3e836625e8be0f69bf3d93f51fac58d");

		let encoded_data = BoundedVec::defensive_truncate_from("Hello, World!".as_bytes().to_vec());

		let receipient = H256(hex!(
			"3547517355657647456b6f7847444a5044576251694b4478714b6d675a357047"
		));

		let message = Message {
			message_type: MessageType::ArbitraryMessage,
			from: H256(hex!(
				"681257BED628425a28B469114Dc21A7c30205cFD000000000000000000000000"
			)),
			to: receipient,
			origin_domain: 2,
			destination_domain: 1,
			data: encoded_data,
			id: 0,
		};

		let encoded_message = message.abi_encode();
		assert_eq!(expected_encoding.to_vec(), encoded_message.to_vec());
		assert_eq!(expected_hash, keccak_256(encoded_message.as_slice()));
	}
}

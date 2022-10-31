use beefy_merkle_tree::{merkle_proof, merkle_root, verify_proof, Leaf, MerkleProof};
use da_primitives::{asdr::AppExtrinsic, data_proof::HasherSha256};
use sp_core::H256;
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
	fn new_shared() -> RcMetrics { Rc::new(RefCell::new(Self::default())) }
}

/// Extracts the `data` field from some types of extrinsics.
pub trait Extractor {
	/// Returns the `data` field of `app_ext` if it contains one.
	/// The `metrics` will be used to write accountability information about the whole process.
	fn extract(app_ext: AppExtrinsic, metrics: RcMetrics) -> Option<Vec<u8>>;
}

impl Extractor for () {
	fn extract(_: AppExtrinsic, _: RcMetrics) -> Option<Vec<u8>> { None }
}

/// It is similar to `Extractor` but it uses `C` type for calls, instead of `AppExtrinsic`.
pub trait Filter<C> {
	/// Returns the `data` field of `call` if it is a valid `da_ctrl::submit_data` call.
	fn filter(call: C, metrics: RcMetrics) -> Option<Vec<u8>>;
}

impl<C> Filter<C> for () {
	fn filter(_: C, _: RcMetrics) -> Option<Vec<u8>> { None }
}

/// Construct a root hash of Binary Merkle Tree created from given filtered `app_extrincs`.
pub fn extrinsics_root<E, I>(app_extrinsics: I) -> H256
where
	E: Extractor,
	I: Iterator<Item = AppExtrinsic>,
{
	let metrics = Metrics::new_shared();
	let submitted_data = app_extrinsics.filter_map(|ext| E::extract(ext, Rc::clone(&metrics)));
	root(submitted_data, Rc::clone(&metrics))
}

/// Construct a root hash of Binary Merkle Tree created from given filtered `calls`.
pub fn calls_root<F, C, I>(calls: I) -> H256
where
	F: Filter<C>,
	I: Iterator<Item = C>,
{
	let metrics = Metrics::new_shared();
	let submitted_data = calls.filter_map(|c| F::filter(c, Rc::clone(&metrics)));
	root(submitted_data, Rc::clone(&metrics))
}

/// Construct a root hash of a Binary Merkle Tree created from given leaves and stores
/// information about the process into `metrics`.
///
/// In case an empty list of leaves is passed the function returns a 0-filled hash.
fn root<I: Iterator<Item = Vec<u8>>>(submitted_data: I, metrics: RcMetrics) -> H256 {
	#[cfg(not(feature = "force-rs-merkle"))]
	let root = merkle_root::<HasherSha256, _, _>(submitted_data).into();
	#[cfg(feature = "force-rs-merkle")]
	let root = rs_merkle_root(submitted_data).into();
	log::debug!(
		target: LOG_TARGET,
		"Build submitted data root: {:?}, metrics: {:?}",
		root,
		metrics
	);

	root
}

/// Calculates the merkle root using `Sha256` and `rs_merkle` crate.
#[cfg(feature = "force-rs-merkle")]
fn rs_merkle_root<I>(leaves: I) -> H256
where
	I: Iterator<Item = Vec<u8>>,
{
	use rs_merkle::{algorithms::Sha256, Hasher, MerkleTree};

	let mut tree = MerkleTree::<Sha256>::new();
	leaves.for_each(|leave| {
		let leave_hash = Sha256::hash(leave.as_slice());
		tree.insert(leave_hash);
	});

	tree.commit();
	tree.root().unwrap_or_default().into()
}

/// Creates the Merkle Proof of the submitted data items in `app_extrinsics` filtered and
/// extracted by `E` and the given `data_index`.
///
/// If `data_index` is greater than the number of Merkle leaves, it will return `None`.
///
/// # TODO
/// - The `merkle_proof` requires `ExactSizeIterator`, forcing to load all submitted data into
/// memory. That would increase the memory footprint of the node significantly. We could fix this
/// adding the number of submitted data items at `System` pallet.
pub fn extrinsics_proof<E, I>(app_extrinsics: I, data_index: u32) -> Option<MerkleProof<Vec<u8>>>
where
	E: Extractor,
	I: Iterator<Item = AppExtrinsic>,
{
	let metrics = Metrics::new_shared();
	let submitted_data = app_extrinsics
		.filter_map(|ext| E::extract(ext, Rc::clone(&metrics)))
		.collect::<Vec<_>>();

	proof(submitted_data, data_index, Rc::clone(&metrics))
}

/// Creates the Merkle Proof of the submitted data items in `calls` filtered by `F` and
/// the given `data_index`.
///
/// If `data_index` is greater than the number of Merkle leaves, it will return `None`.
///
/// # TODO
/// - The `merkle_proof` requires `ExactSizeIterator`, forcing to load all submitted data into
/// memory. That would increase the memory footprint of the node significantly. We could fix this
/// adding the number of submitted data items at `System` pallet.
pub fn calls_proof<F, I, C>(calls: I, data_index: u32) -> Option<MerkleProof<Vec<u8>>>
where
	F: Filter<C>,
	I: Iterator<Item = C>,
{
	let metrics = Metrics::new_shared();
	let submitted_data = calls
		.filter_map(|c| F::filter(c, Rc::clone(&metrics)))
		.collect::<Vec<_>>();

	proof(submitted_data, data_index, Rc::clone(&metrics))
}

/// Construct a Merkle Proof for `submit_data` given by `data_index` and stores
/// information about the process into `metrics`.
///
/// If `data_index` is greater than the number of Merkle leaves, it will return `None`.
fn proof(
	submitted_data: Vec<Vec<u8>>,
	data_index: u32,
	metrics: RcMetrics,
) -> Option<MerkleProof<Vec<u8>>> {
	let data_index = data_index as usize;
	// NOTE: `merkle_proof` panics if `data_index > leaves`.
	if data_index >= submitted_data.len() {
		return None;
	}

	let proof = merkle_proof::<HasherSha256, _, _>(submitted_data, data_index);
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
	let leaf = Leaf::Hash(data_hash.0);
	verify_proof::<HasherSha256, _, _>(
		root.as_fixed_bytes(),
		proof.into_iter().map(|hash| hash.to_fixed_bytes()),
		number_of_submitted_data as usize,
		data_index as usize,
		leaf,
	)
}

#[cfg(all(test, feature = "force-rs-merkle"))]
mod test {
	use da_primitives::asdr::AppId;
	use hex_literal::hex;
	use rs_merkle::{algorithms::Sha256, Hasher, MerkleTree};
	use test_case::test_case;

	use super::*;

	mod nomad {
		use codec::{Compact, Error as DecodeError, Input};
		use sp_runtime::{AccountId32, MultiAddress, MultiSignature};

		use super::*;

		#[derive(Debug, Clone, PartialEq, Eq, Default)]
		pub struct AvailExtrinsic {
			pub app_id: u32,
			pub signature: Option<MultiSignature>,
			pub data: Vec<u8>,
		}

		pub type AvailSignedExtra = ((), (), (), AvailMortality, Nonce, (), Balance, u32);

		#[derive(Decode)]
		pub struct Balance(#[codec(compact)] u128);

		#[derive(Decode)]
		pub struct Nonce(#[codec(compact)] u32);

		pub enum AvailMortality {
			Immortal,
			Mortal(u64, u64),
		}

		impl Decode for AvailMortality {
			fn decode<I: Input>(input: &mut I) -> Result<Self, DecodeError> {
				let first = input.read_byte()?;
				if first == 0 {
					Ok(Self::Immortal)
				} else {
					let encoded = first as u64 + ((input.read_byte()? as u64) << 8);
					let period = 2 << (encoded % (1 << 4));
					let quantize_factor = (period >> 12).max(1);
					let phase = (encoded >> 4) * quantize_factor;
					if period >= 4 && phase < period {
						Ok(Self::Mortal(period, phase))
					} else {
						Err("Invalid period and phase".into())
					}
				}
			}
		}

		const EXTRINSIC_VERSION: u8 = 4;
		impl Decode for AvailExtrinsic {
			fn decode<I: Input>(input: &mut I) -> Result<AvailExtrinsic, DecodeError> {
				// This is a little more complicated than usual since the binary format must be compatible
				// with substrate's generic `Vec<u8>` type. Basically this just means accepting that there
				// will be a prefix of vector length (we don't need
				// to use this).
				let _length_do_not_remove_me_see_above: Compact<u32> = Decode::decode(input)?;

				let version = input.read_byte()?;

				let is_signed = version & 0b1000_0000 != 0;
				let version = version & 0b0111_1111;
				if version != EXTRINSIC_VERSION {
					return Err("Invalid transaction version".into());
				}
				let (app_id, signature) = if is_signed {
					let _address = <MultiAddress<AccountId32, u32>>::decode(input)?;
					let sig = MultiSignature::decode(input)?;
					let extra = <AvailSignedExtra>::decode(input)?;
					let app_id = extra.7;

					(app_id, Some(sig))
				} else {
					return Err("Not signed".into());
				};

				let section: u8 = Decode::decode(input)?;
				let method: u8 = Decode::decode(input)?;

				let data: Vec<u8> = match (section, method) {
					// TODO: Define these pairs as enums or better yet - make a dependency on substrate enums if possible
					(29, 1) => Decode::decode(input)?,
					_ => return Err("Not Avail Extrinsic".into()),
				};

				Ok(Self {
					app_id,
					signature,
					data,
				})
			}
		}
	}

	fn encoded_timestamp_call() -> AppExtrinsic {
		AppExtrinsic {
			app_id: 0.into(),
			data: hex!("280403000BC26208378301").into(),
		}
	}

	fn encoded_fillblock_call<A: Into<AppId>>(app_id: A) -> AppExtrinsic {
		let data = hex!("5D0284001CBD2D43530A44705AD088AF313E18F80B53EF16B36177CD4B77B846F2A5F07C01C44755794EA949E9410390CB4CE07FE2D8068656185B5AB9B43EEF934C3680478968C1F83E360A5D942FE75E9D58E49106A8E8B23601CBC6A633D80E5D089D83A4000400030000001D01A46868616A6B616E636B61206C61682069616B6A206361697568206162206169616A6820612067616861").to_vec();
		AppExtrinsic {
			app_id: app_id.into(),
			data,
		}
	}

	fn encoded_tx_bob() -> AppExtrinsic {
		let data = hex!("490284001cbd2d43530a44705ad088af313e18f80b53ef16b36177cd4b77b846f2a5f07c0166de9fcb3903fa119cb6d23dd903b93a67719f76922b2b4c15a2539d11021102b75f4c452595b65b3bacef0e852430bbfa44bd38133b16cd5d48edb45962568204010000000000000600008eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a4802093d00").to_vec();
		AppExtrinsic {
			app_id: 0.into(),
			data,
		}
	}

	fn dr_input_1() -> Vec<AppExtrinsic> {
		vec![encoded_timestamp_call(), encoded_fillblock_call(3)]
	}

	fn dr_output_1() -> H256 {
		hex!("DDF368647A902A6F6AB9F53B32245BE28EDC99E92F43F0004BBC2CB359814B2A").into()
	}

	/*
	#[test_case( dr_input_1() => dr_output_1())]
	#[test_case( vec![encoded_timestamp_call()] => H256::zero(); "Empty block")]
	#[test_case( vec![encoded_tx_bob()] => H256::zero(); "Signed Native Tx")]
	fn it_build_data_root(app_extrinsics: Vec<AppExtrinsic>) -> H256 {
		build_data_root(&app_extrinsics)
	}*/

	#[test]
	fn test_merkle_proof() {
		let avail_data: Vec<Vec<u8>> = vec![
			hex!("3033333166613733656565636362653465323235").into(),
			hex!("3630646564316635616236373261373132376261").into(),
			hex!("3262313166316464333935353666623261623432").into(),
		];

		let leaves = avail_data
			.iter()
			.map(|xt| Sha256::hash(&xt))
			.collect::<Vec<[u8; 32]>>();

		let data_tree = MerkleTree::<Sha256>::from_leaves(&leaves);
		let proof = data_tree.proof(&[1usize]);
		let root_proof = proof.proof_hashes().to_vec();
		assert_eq!(root_proof, vec![
			hex!("754B9412E0ED7907BDF4B7CA5D2A22F5E129A03DEB1F4E1C1FE42D322FDEE90E"),
			hex!("8D6E30E494D17D7675A94C3C614467FF8CCE35201C1056751A6E9A100515DAF9")
		]);
	}

	#[test]
	fn test_single_merkle_proof() {
		let empty_vec: Vec<[u8; 32]> = vec![];

		let avail_data: Vec<Vec<u8>> =
			vec![hex!("3435346666383063303838616137666162396531").to_vec()];

		let leaves = avail_data
			.iter()
			.map(|xt| Sha256::hash(&xt))
			.collect::<Vec<[u8; 32]>>();

		let data_tree = MerkleTree::<Sha256>::from_leaves(&leaves);
		let proof = data_tree.proof(&[0usize]);
		let root_proof = proof.proof_hashes().to_vec();
		// here the proof is shown empty because the root itself is the proof as there is only one appdata extrinsic
		assert_eq!(root_proof, empty_vec);
	}

	///using rs-merkle proof verify function
	#[test]
	fn verify_merkle_proof() {
		let avail_data: Vec<Vec<u8>> = vec![
			hex!("3033333166613733656565636362653465323235").into(),
			hex!("3630646564316635616236373261373132376261").into(),
			hex!("3262313166316464333935353666623261623432").into(),
			hex!("6433326630643762346634306264346563323665").into(),
		];
		let leaves = avail_data
			.iter()
			.map(|xt| Sha256::hash(&xt))
			.collect::<Vec<[u8; 32]>>();

		let merkle_tree = MerkleTree::<Sha256>::from_leaves(&leaves);
		let indices_to_prove = vec![3];
		let leaves_to_prove = leaves.get(3..4).ok_or("can't get leaves to prove").unwrap();

		let proof = merkle_tree.proof(&indices_to_prove);
		let root = merkle_tree
			.root()
			.ok_or("couldn't get the merkle root")
			.unwrap();

		assert!(proof.verify(root, &indices_to_prove, leaves_to_prove, leaves.len()));
	}

	#[test]
	fn verify_nodata_merkle_proof() {
		let avail_data: Vec<Vec<u8>> = vec![];

		let leaves = avail_data
			.iter()
			.map(|xt| Sha256::hash(&xt))
			.collect::<Vec<[u8; 32]>>();
		let leaves_to_prove = if let Ok(leaves) = leaves.get(0).ok_or("can't get leaves to prove") {
			leaves
		} else {
			&[0u8; 32]
		};
		assert_eq!(leaves_to_prove, &[0u8; 32]);
	}

	fn encoded_submit_call<A: Into<AppId>>(app_id: A) -> AppExtrinsic {
		let data = hex!("5D0284001CBD2D43530A44705AD088AF313E18F80B53EF16B36177CD4B77B846F2A5F07C01C44755794EA949E9410390CB4CE07FE2D8068656185B5AB9B43EEF934C3680478968C1F83E360A5D942FE75E9D58E49106A8E8B23601CBC6A633D80E5D089D83A4000400030000001D01A46868616A6B616E636B61206C61682069616B6A206361697568206162206169616A6820612067616861").to_vec();
		AppExtrinsic {
			app_id: app_id.into(),
			data,
		}
	}

	/*
	#[test_case( encoded_submit_call(0) => H256(hex!("ddf368647a902a6f6ab9f53b32245be28edc99e92f43f0004bbc2cb359814b2a")); "Submit data 0")]
	#[test_case( encoded_submit_call(1) => H256(hex!("ddf368647a902a6f6ab9f53b32245be28edc99e92f43f0004bbc2cb359814b2a")); "Submit data 1")]
	fn nomad_merkle_root_compatibility(extrinsic: AppExtrinsic) -> H256 {
		build_data_root(&[extrinsic])
	}*/
}

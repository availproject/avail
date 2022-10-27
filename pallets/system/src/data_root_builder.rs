use codec::Decode;
use da_primitives::asdr::AppExtrinsic;
use frame_support::traits::ExtrinsicCall;
use sp_core::H256;
use sp_runtime::traits::Extrinsic;
use sp_std::vec::Vec;

const LOG_TARGET: &str = "runtime::system::data_root_builder";

pub type DRFOutput = Option<Vec<u8>>;
pub type DRFCallOf<T> = <T as Extrinsic>::Call;

pub trait DataRootFilter {
	type UncheckedExtrinsic: Decode + ExtrinsicCall;

	fn filter(call: &DRFCallOf<Self::UncheckedExtrinsic>) -> DRFOutput;
}

pub trait DataRootBuilder<F: DataRootFilter> {
	fn build<'a, I>(app_extrinsics: I) -> H256
	where
		I: IntoIterator<Item = &'a AppExtrinsic>,
	{
		let mut used_extrinsics_count = 0u32;
		let mut total_extrinsics_count = 0u32;

		let filtered_iter = app_extrinsics
			.into_iter()
			.enumerate()
			.filter_map(|(idx, app_extrinsic)| {
				// Decode call and log any failure.
				total_extrinsics_count += 1;
				match F::UncheckedExtrinsic::decode(&mut app_extrinsic.data.as_slice()) {
					Ok(app_unchecked_extrinsic) => Some(app_unchecked_extrinsic),
					Err(err) => {
						// NOTE: Decodification issue is like a `unrecheable` because this
						// extrinsic was decoded previously, when node executed the block.
						// We will keep this here just to have a trace if we update
						// `System::note_extrinsic` in the future.
						log::error!(
							target: LOG_TARGET,
							"Extrinsic {} cannot be decoded: {:?}",
							idx,
							err
						);
						None
					},
				}
			})
			.filter_map(|app_unchecked_extrinsic| {
				// Filter calls and traces removed calls
				// @TODO: We could avoid the the copy of data from filter
				// once `beefy_merkle_tree::merkelize` becomes public. In that case,
				// we could work with iterator and lifescopes, having something like:
				//
				// ```Rust
				// pub trait DataRootFilter {
				//	type UncheckedExtrinsic: Decode + ExtrinsicCall;
				//	fn filter<'a>(call: &'a <Self::UncheckedExtrinsic as Extrinsic>::Call) -> Option<&'a [u8]>;
				//	}
				// ```
				let maybe_data = F::filter(app_unchecked_extrinsic.call());
				if maybe_data.is_some() {
					used_extrinsics_count += 1;
				}

				maybe_data
			});

		let root = Self::merkle_root(filtered_iter);

		log::debug!(
			target: LOG_TARGET,
			"Used {} extrinsics of {}",
			used_extrinsics_count,
			total_extrinsics_count
		);

		root.into()
	}

	#[cfg(not(feature = "force-rs-merkle"))]
	fn merkle_root<I>(leaves: I) -> H256
	where
		I: Iterator<Item = Vec<u8>>,
	{
		use beefy_merkle_tree::{merkle_root, Hash, Hasher};
		use sp_io::hashing::sha2_256;

		#[derive(Copy, Clone)]
		struct Sha2_256 {}

		impl Hasher for Sha2_256 {
			fn hash(data: &[u8]) -> Hash { sha2_256(data) }
		}

		merkle_root::<Sha2_256, _, _>(leaves).into()
	}

	#[cfg(feature = "force-rs-merkle")]
	fn merkle_root<'a, I>(leaves: I) -> H256
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
}

impl<F: DataRootFilter> DataRootBuilder<F> for F {}

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

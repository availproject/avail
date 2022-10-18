use codec::{Compact, Decode, Encode, Error as DecodeError, Input};
use da_primitives::{
	asdr::{AppExtrinsic, DataLookup},
	traits::ExtendedHeader,
	HeaderExtension, KateCommitment,
};
use frame_support::{ensure, traits::Randomness};
pub use kate::Seed;
use rs_merkle::{algorithms::Sha256, Hasher, MerkleTree};
use scale_info::TypeInfo;
use sp_core::H256;
use sp_runtime::{traits::Hash, AccountId32, MultiAddress, MultiSignature, SaturatedConversion};
use sp_runtime_interface::{pass_by::PassByCodec, runtime_interface};
use sp_std::vec::Vec;

use crate::{generic::Digest, limits::BlockLength, Config, LOG_TARGET};

pub mod da {
	use core::marker::PhantomData;

	use da_primitives::Header as DaHeader;
	use sp_runtime::traits::BlakeTwo256;

	use super::{AppExtrinsic, BlockLength, Config, DigestWrapper, Vec};

	pub type BlockNumber = u32;
	pub type Hash = sp_core::H256;
	pub type Hasher = BlakeTwo256;
	pub type Header = DaHeader<BlockNumber, Hasher>;

	/// Data-Avail Header builder.
	pub struct HeaderBuilder<T: Config>(PhantomData<T>);

	impl<T: Config> super::HeaderBuilder for HeaderBuilder<T> {
		type Header = Header;

		#[inline]
		fn build(
			app_extrinsics: Vec<AppExtrinsic>,
			parent_hash: Hash,
			digest: DigestWrapper,
			block_length: BlockLength,
			block_number: BlockNumber,
		) -> Header {
			let seed = Self::random_seed::<T>();

			super::hosted_header_builder::build(
				app_extrinsics,
				parent_hash,
				digest,
				block_length,
				block_number,
				seed,
			)
		}
	}
}

/// It is just a wapper to support `PassBy` on `Digest` type.
#[derive(Clone, TypeInfo, Encode, Decode, PassByCodec)]
pub struct DigestWrapper(pub Digest);

impl From<Digest> for DigestWrapper {
	fn from(d: Digest) -> Self { Self(d) }
}

/// Trait for header builder.
pub trait HeaderBuilder {
	type Header: sp_runtime::traits::Header + ExtendedHeader;

	/// Creates the header using the given parameters.
	fn build(
		app_extrinsics: Vec<AppExtrinsic>,
		parent_hash: <Self::Header as sp_runtime::traits::Header>::Hash,
		digest: DigestWrapper,
		block_length: BlockLength,
		block_number: <Self::Header as sp_runtime::traits::Header>::Number,
	) -> Self::Header;

	/// Generates a random seed using the _epoch seed_ and the _current block_ returned by
	/// `T::Randomness` type.
	fn random_seed<T: Config>() -> Seed {
		let (epoch_seed, block_number) = <T as Config>::Randomness::random_seed();
		let seed = <T as Config>::Hashing::hash_of(&(&epoch_seed, &block_number));

		log::trace!(
			target: LOG_TARGET,
			"Header builder seed {:?} from epoch seed {:?} and block {:?}",
			seed,
			epoch_seed,
			block_number
		);

		seed.into()
	}
}

fn build_base(
	app_extrinsics: &[AppExtrinsic],
	parent_hash: da::Hash,
	digest: DigestWrapper,
	block_length: BlockLength,
	block_number: da::BlockNumber,
	seed: Seed,
	build_extension: fn(&[AppExtrinsic], BlockLength, Seed) -> HeaderExtension,
) -> da::Header {
	use sp_io::storage::{changes_root, root};

	use crate::generic::DigestItem;

	let extension = build_extension(app_extrinsics, block_length, seed);

	// @TODO Miguel: Is it possible to avoid copies here?
	let extrinsics = app_extrinsics
		.iter()
		.map(|e| e.data.clone())
		.collect::<Vec<_>>();
	let extrinsics_root = da::Hasher::ordered_trie_root(extrinsics);

	let mut digest = digest.0;
	let storage_root =
		da::Hash::decode(&mut &root()[..]).expect("Node is configured to use the same hash; qed");

	// we can't compute changes trie root earlier && put it to the Digest
	// because it will include all currently existing temporaries.
	if let Some(storage_changes_root) = changes_root(&parent_hash.encode()) {
		let hash_changes_root = da::Hash::decode(&mut &storage_changes_root[..])
			.expect("Node is configured to use the same hash; qed");
		let item = DigestItem::Other(hash_changes_root.as_ref().to_vec());
		digest.push(item);
	}

	<da::Header as ExtendedHeader>::new(
		block_number,
		extrinsics_root,
		storage_root,
		parent_hash,
		digest,
		extension,
	)
}

/// Builds commitments using `Plonk v0.8.9`
#[cfg(all(feature = "std", feature = "header-backward-compatibility-test"))]
fn build_extension_v_test(
	app_extrinsics: &[AppExtrinsic],
	block_length: BlockLength,
	seed: Seed,
) -> HeaderExtension {
	let extension_v1 = build_extension_v1(app_extrinsics, block_length, seed);
	match extension_v1 {
		HeaderExtension::V1(extension) => HeaderExtension::VTest(extension.into()),
		r @ HeaderExtension::VTest(_) => r,
	}
}

#[cfg(feature = "std")]
fn build_extension_v1(
	app_extrinsics: &[AppExtrinsic],
	block_length: BlockLength,
	seed: Seed,
) -> HeaderExtension {
	use da_primitives::header::extension::v1;

	let (xts_layout, commitment, block_dims, _data_matrix) = kate::com::par_build_commitments(
		block_length.rows as usize,
		block_length.cols as usize,
		block_length.chunk_size() as usize,
		app_extrinsics,
		seed,
	)
	.expect("Build commitments cannot fail .qed");
	let app_lookup =
		DataLookup::try_from(xts_layout.as_slice()).expect("Extrinsic size cannot overflow .qed");
	let data_root = build_data_root(app_extrinsics);

	log::debug!(
		target: LOG_TARGET,
		"Build Commitment (v1): Data Root: {:?}, App Lookup : {:?}",
		data_root,
		app_lookup
	);

	let commitment = KateCommitment {
		rows: block_dims.rows.saturated_into::<u16>(),
		cols: block_dims.cols.saturated_into::<u16>(),
		commitment,
		data_root,
	};

	HeaderExtension::V1(v1::HeaderExtension {
		commitment,
		app_lookup,
	})
}

/// Hosted function to build the header using `kate` commitments.
#[runtime_interface]
pub trait HostedHeaderBuilder {
	/// Creates the header using the given parameters.
	/// *NOTE:* Version 1 uses `dusk-plonk v0.8.2`
	#[version(1)]
	fn build(
		app_extrinsics: Vec<AppExtrinsic>,
		parent_hash: da::Hash,
		digest: DigestWrapper,
		block_length: BlockLength,
		block_number: da::BlockNumber,
		seed: Seed,
	) -> da::Header {
		build_base(
			app_extrinsics.as_slice(),
			parent_hash,
			digest,
			block_length,
			block_number,
			seed,
			build_extension_v1,
		)
	}

	/*
	#[version(2)]
	fn build(
		app_extrinsics: Vec<AppExtrinsic>,
		parent_hash: da::Hash,
		digest: DigestWrapper,
		block_length: BlockLength,
		block_number: da::BlockNumber,
		seed: Seed,
	) -> da::Header {
		build_base(
			app_extrinsics.as_slice(),
			parent_hash,
			digest,
			block_length,
			block_number,
			seed,
			build_extension_v_test,
		)
	}*/
}

/*
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct AvailExtrinsic {
	pub app_id: u32,
	pub signature: Option<MultiSignature>,
	pub data: Vec<u8>,
}
*/

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

/*
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
}*/

fn filter_map_submit_data(input: &mut &[u8]) -> Result<<Sha256 as Hasher>::Hash, DecodeError> {
	const EXTRINSIC_VERSION: u8 = 4;

	// This is a little more complicated than usual since the binary format must be compatible
	// with substrate's generic `Vec<u8>` type. Basically this just means accepting that there
	// will be a prefix of vector length (we don't need
	// to use this).
	let _length_do_not_remove_me_see_above: Compact<u32> = Decode::decode(input)?;

	let version = input.read_byte()?;
	let is_signed = version & 0b1000_0000 != 0;
	ensure!(is_signed, "Not signed");

	let version = version & 0b0111_1111;
	ensure!(version == EXTRINSIC_VERSION, "Invalid transaction version");

	let _address = <MultiAddress<AccountId32, u32>>::decode(input)?;
	let _signature = MultiSignature::decode(input)?;
	let _extra = <AvailSignedExtra>::decode(input)?;

	let pallet: u8 = Decode::decode(input)?;
	let method: u8 = Decode::decode(input)?;
	ensure!(pallet == 29 && method == 1, "Not DaCtrl::submit_data");

	let data_size = Compact::<u32>::decode(input)?.0 as usize;
	ensure!(input.len() >= data_size, "Corrupted extrinsic");
	Ok(Sha256::hash(&input[..data_size]))
}

fn build_data_root(app_ext: &[AppExtrinsic]) -> H256 {
	let mut tree = MerkleTree::<Sha256>::new();

	app_ext
		.iter()
		// NOTE: `AvailExtrinsic` decode fn will filter onlye `Da_Control::submit_data` extrinsics.
		// TODO: It implies a circular dependency atm between system & DA control pallets.
		.filter_map(|e| filter_map_submit_data(&mut e.data.as_slice()).ok())
		.for_each(|hash| {
			tree.insert(hash);
		});

	tree.commit();
	tree.root().unwrap_or_default().into()
}

#[cfg(test)]
mod tests {
	use da_primitives::asdr::AppId;
	use hex_literal::hex;
	use sp_core::H256;
	use test_case::test_case;

	use super::*;

	fn encoded_timestamp_call() -> AppExtrinsic {
		AppExtrinsic {
			app_id: 0.into(),
			data: hex!("280403000BC26208378301").into(),
		}
	}

	fn encoded_submit_data<A: Into<AppId>>(app_id: A) -> AppExtrinsic {
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

	fn dr_input_1() -> Vec<AppExtrinsic> { vec![encoded_timestamp_call(), encoded_submit_data(3)] }

	fn dr_output_1() -> H256 {
		hex!("DDF368647A902A6F6AB9F53B32245BE28EDC99E92F43F0004BBC2CB359814B2A").into()
	}

	#[test_case( dr_input_1() => dr_output_1())]
	#[test_case( vec![encoded_timestamp_call()] => H256::zero(); "Empty block")]
	#[test_case( vec![encoded_tx_bob()] => H256::zero(); "Signed Native Tx")]
	fn it_build_data_root(app_extrinsics: Vec<AppExtrinsic>) -> H256 {
		build_data_root(&app_extrinsics).into()
	}

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
}

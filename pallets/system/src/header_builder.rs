use codec::{Compact, Decode, Encode, Error as DecodeError, Input};
use da_primitives::{asdr::AppExtrinsic, traits::ExtendedHeader};
use frame_support::traits::Randomness;
pub use kate::Seed;
use rs_merkle::{algorithms::Sha256, Hasher, MerkleTree};
use scale_info::TypeInfo;
use sp_core::H256;
use sp_runtime::{traits::Hash, AccountId32, MultiAddress, MultiSignature};
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

/// Hosted function to build the header using `kate` commitments.
#[runtime_interface]
pub trait HostedHeaderBuilder {
	/// Creates the header using the given parameters.
	fn build(
		app_extrinsics: Vec<AppExtrinsic>,
		parent_hash: da::Hash,
		digest: DigestWrapper,
		block_length: BlockLength,
		block_number: da::BlockNumber,
		seed: Seed,
	) -> da::Header {
		use da_primitives::{asdr::DataLookup, traits::ExtrinsicsWithCommitment as _};
		use sp_runtime::traits::Hash;

		use crate::generic::DigestItem;

		let (kate_commitment, block_dims, data_index) = {
			let (xts_layout, kate_commitment, block_dims, _data_matrix) =
				kate::com::par_build_commitments(
					block_length.rows as usize,
					block_length.cols as usize,
					block_length.chunk_size() as usize,
					app_extrinsics.as_slice(),
					seed,
				)
				.expect("Build commitments cannot fail .qed");
			let data_index = DataLookup::try_from(xts_layout.as_slice())
				.expect("Extrinsic size cannot overflow .qed");

			log::debug!(target: LOG_TARGET, "App DataLookup: {:?}", data_index);

			(kate_commitment, block_dims, data_index)
		};

		let extrinsics: Vec<Vec<u8>> = app_extrinsics.clone().into_iter().map(|e| e.data).collect();
		let data_root = build_data_root(&app_extrinsics);

		log::debug!("Avail Data Root: {:?}\n", data_root);

		let root_hash = da::Hasher::ordered_trie_root(extrinsics);

		let storage_root = da::Hash::decode(&mut &sp_io::storage::root()[..])
			.expect("Node is configured to use the same hash; qed");
		let storage_changes_root = sp_io::storage::changes_root(&parent_hash.encode());

		let mut digest = digest.0;
		// we can't compute changes trie root earlier && put it to the Digest
		// because it will include all currently existing temporaries.
		if let Some(storage_changes_root) = storage_changes_root {
			let hash_changes_root = da::Hash::decode(&mut &storage_changes_root[..])
				.expect("Node is configured to use the same hash; qed");
			let item = DigestItem::Other(hash_changes_root.as_ref().to_vec());
			digest.push(item);
		}

		let extrinsics_root = <da::Header as ExtendedHeader>::Root::new_with_commitment(
			root_hash,
			kate_commitment,
			block_dims.rows as u16,
			block_dims.cols as u16,
			data_root,
		);

		<da::Header as ExtendedHeader>::new(
			block_number,
			extrinsics_root,
			storage_root,
			parent_hash,
			digest,
			data_index,
		)
	}
}

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

fn build_data_root(app_ext: &[AppExtrinsic]) -> H256 {
	let mut tree = MerkleTree::<Sha256>::new();

	app_ext
		.iter()
		// NOTE: `AvailExtrinsic` decode fn will filter onlye `Da_Control::submit_data` extrinsics.
		// TODO: It implies a circular dependency atm between system & DA control pallets.
		.map(|x| &x.data)
		.filter_map(|e| AvailExtrinsic::decode(&mut &e[..]).ok())
		.for_each(|ext| {
			let ext_hash = Sha256::hash(&ext.data);
			tree.insert(ext_hash);
		});

	tree.commit();
	tree.root().unwrap_or_default().into()
}

#[cfg(test)]
mod tests {
	use da_primitives::asdr::AppExtrinsic;
	use hex_literal::hex;
	use rs_merkle::{algorithms::Sha256, Hasher, MerkleTree};

	use super::build_data_root;
	/// tests for data root is created correctly
	#[test]
	fn data_root_test() {
		let avail_ext: Vec<AppExtrinsic> = vec![
			AppExtrinsic {
				app_id: 0,
				data: hex!("280403000BC26208378301").into(), 
			},
			AppExtrinsic {
				app_id: 3,
				data: hex!("5D0284001CBD2D43530A44705AD088AF313E18F80B53EF16B36177CD4B77B846F2A5F07C01C44755794EA949E9410390CB4CE07FE2D8068656185B5AB9B43EEF934C3680478968C1F83E360A5D942FE75E9D58E49106A8E8B23601CBC6A633D80E5D089D83A4000400030000001D01A46868616A6B616E636B61206C61682069616B6A206361697568206162206169616A6820612067616861").into()
			},
		];
		let no_app_data: Vec<AppExtrinsic> = vec![AppExtrinsic {
			app_id: 0,
			data: hex!("280403000BA51408378301").into(),
		}];

		let data_root = build_data_root(&avail_ext);
		let no_app_data_root = build_data_root(&no_app_data);
		//test for data root for appdata extrinsics
		assert_eq!(
			data_root,
			hex!("DDF368647A902A6F6AB9F53B32245BE28EDC99E92F43F0004BBC2CB359814B2A").into()
		);
		//test for data root for single extrinsic without appdata
		assert_eq!(no_app_data_root, [0u8; 32].into());
	}

	#[test]
	fn test_merkle_proof() {
		let avail_data: Vec<Vec<u8>> = vec![
			[
				48, 51, 51, 49, 102, 97, 55, 51, 101, 101, 101, 99, 99, 98, 101, 52, 101, 50, 50,
				53,
			]
			.to_vec(),
			[
				54, 48, 100, 101, 100, 49, 102, 53, 97, 98, 54, 55, 50, 97, 55, 49, 50, 55, 98, 97,
			]
			.to_vec(),
			[
				50, 98, 49, 49, 102, 49, 100, 100, 51, 57, 53, 53, 54, 102, 98, 50, 97, 98, 52, 50,
			]
			.to_vec(),
		];

		let leaves = avail_data
			.iter()
			.map(|xt| Sha256::hash(&xt))
			.collect::<Vec<[u8; 32]>>();

		let data_tree = MerkleTree::<Sha256>::from_leaves(&leaves);
		let proof = data_tree.proof(&[1usize]);
		let root_proof = proof.proof_hashes().to_vec();
		assert_eq!(root_proof, vec![
			[
				117, 75, 148, 18, 224, 237, 121, 7, 189, 244, 183, 202, 93, 42, 34, 245, 225, 41,
				160, 61, 235, 31, 78, 28, 31, 228, 45, 50, 47, 222, 233, 14
			],
			[
				141, 110, 48, 228, 148, 209, 125, 118, 117, 169, 76, 60, 97, 68, 103, 255, 140,
				206, 53, 32, 28, 16, 86, 117, 26, 110, 154, 16, 5, 21, 218, 249
			]
		]);
	}

	#[test]
	fn test_single_merkle_proof() {
		let empty_vec: Vec<[u8; 32]> = vec![];

		let avail_data: Vec<Vec<u8>> = vec![[
			52, 53, 52, 102, 102, 56, 48, 99, 48, 56, 56, 97, 97, 55, 102, 97, 98, 57, 101, 49,
		]
		.to_vec()];

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
			[
				48, 51, 51, 49, 102, 97, 55, 51, 101, 101, 101, 99, 99, 98, 101, 52, 101, 50, 50,
				53,
			]
			.to_vec(),
			[
				54, 48, 100, 101, 100, 49, 102, 53, 97, 98, 54, 55, 50, 97, 55, 49, 50, 55, 98, 97,
			]
			.to_vec(),
			[
				50, 98, 49, 49, 102, 49, 100, 100, 51, 57, 53, 53, 54, 102, 98, 50, 97, 98, 52, 50,
			]
			.to_vec(),
			[
				100, 51, 50, 102, 48, 100, 55, 98, 52, 102, 52, 48, 98, 100, 52, 101, 99, 50, 54,
				101,
			]
			.to_vec(),
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

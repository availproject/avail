use codec::{Decode, Encode};
use da_primitives::{asdr::AppExtrinsic, traits::ExtendedHeader};
use frame_support::traits::Randomness;
pub use kate::Seed;
use scale_info::TypeInfo;
use sp_runtime::traits::Hash;
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

		let mut sorted_app_extrinsics = app_extrinsics.clone();
		sorted_app_extrinsics.sort_by(|a, b| a.app_id.cmp(&b.app_id));
		let (kate_commitment, block_dims, data_index) = {
			let (xts_layout, kate_commitment, block_dims, _data_matrix) =
				kate::com::build_commitments(
					block_length.rows as usize,
					block_length.cols as usize,
					block_length.chunk_size() as usize,
					sorted_app_extrinsics.as_slice(),
					seed,
				)
				.expect("Build commitments cannot fail .qed");
			let data_index = DataLookup::try_from(xts_layout.as_slice())
				.expect("Extrinsic size cannot overflow .qed");

			log::debug!(target: LOG_TARGET, "App DataLookup: {:?}", data_index);

			(kate_commitment, block_dims, data_index)
		};

		let extrinsics = app_extrinsics.into_iter().map(|e| e.data).collect();
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

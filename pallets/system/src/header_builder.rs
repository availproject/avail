use crate::{limits::BlockLength, Config, LOG_TARGET};
use avail_core::{
	app_extrinsic::AppExtrinsic, header::HeaderExtension, traits::ExtendedHeader, HeaderVersion,
};
pub use kate::{
	metrics::{IgnoreMetrics, Metrics},
	com::Error as KateError,
	Seed,
};

use codec::{Decode, Encode};
use frame_support::traits::Randomness;
use scale_info::TypeInfo;
use sp_core::H256;
use sp_runtime::traits::Hash;
use sp_runtime_interface::{pass_by::PassByCodec, runtime_interface};
use sp_std::vec::Vec;
use core::num::TryFromIntError;
use thiserror_no_std::Error;



#[cfg(feature = "std")]
mod v3;

pub mod da {
	use core::marker::PhantomData;

	use avail_core::header::{Header as DaHeader, HeaderExtension};
	use sp_runtime::traits::BlakeTwo256;

	use super::*;

	pub type Hash = sp_core::H256;
	pub type BlockNumber = u32;

	/// avail-node Header builder.
	pub struct HeaderExtensionBuilder<T: Config>(PhantomData<T>);

	impl<T: Config> super::HeaderExtensionBuilder for HeaderExtensionBuilder<T> {
		type Header = DaHeader<BlockNumber, BlakeTwo256>;

		#[inline]
		fn build(
			submitted: Vec<AppExtrinsic>,
			data_root: H256,
			block_length: BlockLength,
			block_number: u32,
		) -> HeaderExtension {
			let seed = Self::random_seed::<T>();

			super::hosted_header_builder::build(
				submitted,
				data_root,
				block_length,
				block_number,
				seed,
			)
		}

		fn grid(
			submitted: Vec<AppExtrinsic>,
			max_width: u32,
			max_height: u32,
			rows: Vec<u32>,
		) -> Result<Vec<Vec<[u8; 32]>>, Error> {
			let seed = Self::random_seed::<T>();
			super::hosted_header_builder::grid(submitted, max_width, max_height, seed, rows)
		}
	}
}


/// Trait for header builder.
pub trait HeaderExtensionBuilder {
	type Header: ExtendedHeader<Extension = HeaderExtension>;

	/// Creates the header using the given parameters.
	fn build(
		app_extrinsics: Vec<AppExtrinsic>,
		data_root: H256,
		block_length: BlockLength,
		block_number: u32,
	) -> HeaderExtension;

	fn grid(
		submitted: Vec<AppExtrinsic>,
		max_width: u32,
		max_height: u32,
		rows: Vec<u32>,
	) -> Result<Vec<Vec<[u8; 32]>>, Error>; 

	/// Generates a random seed using the _epoch seed_ and the _current block_ returned by
	/// `T::Randomness` type.
	fn random_seed<T: Config>() -> Seed {
		let seed = if cfg!(feature = "secure_padding_fill") {
			let (epoch_seed, block_number) = <T as Config>::Randomness::random_seed();
			let seed = <T as Config>::Hashing::hash_of(&(&epoch_seed, &block_number));
			log::trace!(
				target: LOG_TARGET,
				"Header builder seed {seed:?} from epoch seed {epoch_seed:?} and block {block_number:?}");
			seed
		} else {
			<T as Config>::Hash::default()
		};

		seed.into()
	}
}

/// Hosted function to build the header using `kate` commitments.
#[runtime_interface]
pub trait HostedHeaderBuilder {
	#[version(1)]
	fn build(
		submitted: Vec<AppExtrinsic>,
		data_root: H256,
		block_length: BlockLength,
		block_number: u32,
		seed: Seed,
	) -> HeaderExtension {
		v3::build_extension(
			submitted,
			data_root,
			block_length,
			block_number,
			seed,
			HeaderVersion::V3,
		)
	}

	fn grid(
		submitted: Vec<AppExtrinsic>,
		max_width: u32,
		max_height: u32,
		seed: Seed,
		rows: Vec<u32>,
	) -> Result<Vec<Vec<[u8;32]>>, Error> {
		let max_width = usize::try_from(max_width)?;
		let max_height = usize::try_from(max_height)?;
		let rows = rows
			.into_iter()
			.map(usize::try_from)
			.collect::<Result<_, _>>()?;

		v3::grid(submitted, max_width, max_height, seed, rows)
	}

    fn data(
        submitted: Vec<AppExtrinsic>,
        max_width: u32,
        max_height: u32,
        seed: Seed,
        rows: Vec<u32>,
    ) -> Result<Vec<Vec<[u8;32]>>, Error> {
        let max_width = usize::try_from(max_width)?;
        let max_height = usize::try_from(max_height)?;
        let rows = rows
            .into_iter()
            .map(usize::try_from)
            .collect::<Result<_, _>>()?;
        v3::grid(submitted, max_width, max_height, seed, rows)
    }
}

#[derive(Error, Encode, Decode, TypeInfo, PassByCodec)]
pub enum Error {
	#[error("Invalid integer conversion")]
	TryFromInt,
	#[error("Missing row {0}")]
	MissingRow(u32),
	#[error("Invalid scalar at row {0}")]
	InvalidScalarAtRow(u32),
	#[error("Grid generation error")]
	KateGrid,
}

impl From<TryFromIntError> for Error {
	fn from(_: TryFromIntError) -> Self {
		Self::TryFromInt
	}
}

impl From<KateError> for Error {
	fn from(_: KateError) -> Self {
		Self::KateGrid
	}
}

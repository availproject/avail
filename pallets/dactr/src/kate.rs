use crate::{Config, LOG_TARGET};
use avail_core::{AppExtrinsic, AppId};
use frame_system::{limits::BlockLength, Config as SysConfig};
use kate::Seed;
#[cfg(feature = "std")]
use kate::{com::Error as KateError, gridgen::AppRowError as KateAppRowError};

use codec::{Decode, Encode};
use core::{marker::PhantomData, num::TryFromIntError};
use derive_more::From;
use frame_support::traits::Randomness;
use scale_info::TypeInfo;
use sp_core::U256;
use sp_runtime::traits::Hash;
use sp_runtime_interface::pass_by::{PassByCodec, PassByInner};
use sp_std::vec::Vec;
use thiserror_no_std::Error;

#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

pub type GRawScalar = U256;
pub type GRow = Vec<GRawScalar>;
pub type GDataProof = (GRawScalar, GProof);

/// # NOTE
/// `Serde` requires a custom implementation for `GProof` due to the array size (greater than `[T;32]`).
/// In this case, we transform into a `Vec<u8>` as intermediate step to serialize/deserialize.
#[derive(Encode, Decode, TypeInfo, PassByInner, Debug, From, Clone, Copy)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "std", serde(try_from = "Vec<u8>", into = "Vec<u8>"))]
pub struct GProof([u8; 48]);

impl From<GProof> for Vec<u8> {
	fn from(proof: GProof) -> Self {
		proof.0.to_vec()
	}
}

impl TryFrom<Vec<u8>> for GProof {
	type Error = u32;
	fn try_from(data: Vec<u8>) -> Result<Self, Self::Error> {
		if data.len() != 48 {
			return Err(data.len() as u32);
		};

		let mut proof = [0u8; 48];
		proof.copy_from_slice(&data);
		Ok(GProof(proof))
	}
}

pub mod hosted_kate;
pub use hosted_kate::hosted_kate::{
	app_data as hosted_app_data, grid as hosted_grid, proof as hosted_proof,
};

#[derive(Default)]
pub struct RTKate<T: Config>(PhantomData<T>);

impl<T: Config> RTKate<T> {
	fn random_seed() -> Seed {
		let seed = if cfg!(feature = "secure_padding_fill") {
			let (epoch_seed, block_number) = <T as SysConfig>::Randomness::random_seed();
			let seed = <T as SysConfig>::Hashing::hash_of(&(&epoch_seed, &block_number));
			log::trace!(
				target: LOG_TARGET,
				"RTKate seed {seed:?} from epoch seed {epoch_seed:?} and block {block_number:?}");
			seed
		} else {
			<T as SysConfig>::Hash::default()
		};

		seed.into()
	}

	pub fn grid(
		submitted: Vec<AppExtrinsic>,
		block_length: BlockLength,
		selected_rows: Vec<u32>,
	) -> Result<Vec<GRow>, Error> {
		let seed = Self::random_seed();
		hosted_grid(submitted, block_length, seed, selected_rows)
	}

	pub fn app_data(
		submitted: Vec<AppExtrinsic>,
		block_length: BlockLength,
		app_id: AppId,
	) -> Result<Vec<Option<GRow>>, Error> {
		let seed = Self::random_seed();
		hosted_app_data(submitted, block_length, seed, app_id.0)
	}

	pub fn proof(
		extrinsics: Vec<AppExtrinsic>,
		block_len: BlockLength,
		cells: Vec<(u32, u32)>,
	) -> Result<Vec<GDataProof>, Error> {
		let seed = Self::random_seed();
		hosted_proof(extrinsics, block_len, seed, cells)
	}
}

#[derive(Error, Encode, Decode, TypeInfo, PassByCodec, Debug)]
pub enum Error {
	#[error("Invalid integer conversion")]
	TryFromInt,
	#[error("Missing row {0}")]
	MissingRow(u32),
	#[error("Invalid scalar at row {0}")]
	InvalidScalarAtRow(u32),
	#[error("Grid generation error")]
	KateGrid,
	#[error("Invalid grid dimension")]
	InvalidDimension,
	#[error("App Data row error")]
	AppRow,
	#[error("Missing cell {row} {col}")]
	MissingCell { row: u32, col: u32 },
	#[error("MultiProof error")]
	Proof,
}

impl From<TryFromIntError> for Error {
	fn from(_: TryFromIntError) -> Self {
		Self::TryFromInt
	}
}

#[cfg(feature = "std")]
impl From<KateError> for Error {
	fn from(_: KateError) -> Self {
		Self::KateGrid
	}
}

#[cfg(feature = "std")]
impl From<KateAppRowError> for Error {
	fn from(_: KateAppRowError) -> Self {
		Self::AppRow
	}
}

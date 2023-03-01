#![cfg_attr(not(feature = "std"), no_std)]

use da_primitives::{BlockLengthColumns, BlockLengthRows};
#[cfg(feature = "std")]
pub use dusk_plonk::{commitment_scheme::kzg10::PublicParameters, prelude::BlsScalar};
use frame_support::sp_runtime::SaturatedConversion;
#[cfg(feature = "std")]
use kate_recovery::matrix::Dimensions;
use static_assertions::const_assert_ne;

use crate::config::DATA_CHUNK_SIZE;

pub const LOG_TARGET: &str = "kate";
pub type Seed = [u8; 32];

pub mod config {
	use super::{BlockLengthColumns, BlockLengthRows};

	pub const SCALAR_SIZE_WIDE: usize = 64;
	pub const SCALAR_SIZE: usize = 32;
	pub const DATA_CHUNK_SIZE: usize = 31; // Actual chunk size is 32 after 0 padding is done
	pub const EXTENSION_FACTOR: u32 = 2;
	pub const PROVER_KEY_SIZE: u32 = 48;
	pub const PROOF_SIZE: usize = 48;
	// MINIMUM_BLOCK_SIZE, MAX_BLOCK_ROWS and MAX_BLOCK_COLUMNS have to be a power of 2 because of the FFT functions requirements
	pub const MINIMUM_BLOCK_SIZE: usize = 128;
	pub const MAX_BLOCK_ROWS: BlockLengthRows = if cfg!(feature = "extended-columns") {
		BlockLengthRows(128)
	} else {
		BlockLengthRows(256)
	};
	pub const MAX_BLOCK_COLUMNS: BlockLengthColumns = if cfg!(feature = "extended-columns") {
		BlockLengthColumns(512)
	} else {
		BlockLengthColumns(256)
	};
	pub const MAXIMUM_BLOCK_SIZE: bool = cfg!(feature = "maximum-block-size");
}

/// TODO
///  - Dedup this from `kate-recovery` once that library support `no-std`.
#[cfg(feature = "std")]
pub mod testnet {
	use super::{BlockLengthColumns, PublicParameters};
	use once_cell::sync::Lazy;
	use rand::SeedableRng;
	use rand_chacha::ChaChaRng;
	use std::{collections::HashMap, sync::Mutex};

	static SRS_DATA: Lazy<Mutex<HashMap<u32, PublicParameters>>> =
		Lazy::new(|| Mutex::new(HashMap::new()));

	pub fn public_params(max_degree: BlockLengthColumns) -> PublicParameters {
		let mut srs_data_locked = SRS_DATA.lock().unwrap();
		srs_data_locked
			.entry(max_degree.0)
			.or_insert_with(|| {
				let mut rng = ChaChaRng::seed_from_u64(42);
				PublicParameters::setup(max_degree.as_usize(), &mut rng).unwrap()
			})
			.clone()
	}
}

pub mod metrics;

#[cfg(feature = "std")]
pub mod com;
/// Precalculate the length of padding IEC 9797 1.
///
/// # NOTE
/// There is a unit test to ensure this formula match with the current
/// IEC 9797 1 algorithm we implemented. See `fn pad_iec_9797_1`
#[inline]
fn padded_len_of_pad_iec_9797_1(len: u32) -> u32 {
	let len_plus_one = len.saturating_add(1);
	let offset = (DATA_CHUNK_SIZE - (len_plus_one as usize % DATA_CHUNK_SIZE)) % DATA_CHUNK_SIZE;
	let offset: u32 = offset.saturated_into();

	len_plus_one.saturating_add(offset)
}

/// Calculates the padded len based of initial `len`.
pub fn padded_len(len: u32, chunk_size: u32) -> u32 {
	let iec_9797_1_len = padded_len_of_pad_iec_9797_1(len);

	const_assert_ne!(DATA_CHUNK_SIZE, 0);
	debug_assert!(
		chunk_size >= DATA_CHUNK_SIZE as u32,
		"`BlockLength.chunk_size` is valid by design .qed"
	);
	let diff_per_chunk = chunk_size - DATA_CHUNK_SIZE as u32;
	let pad_to_chunk_extra = if diff_per_chunk != 0 {
		let chunks_count = iec_9797_1_len / DATA_CHUNK_SIZE as u32;
		chunks_count * diff_per_chunk
	} else {
		0
	};

	iec_9797_1_len + pad_to_chunk_extra
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct BlockDimensions {
	pub rows: BlockLengthRows,
	pub cols: BlockLengthColumns,
	pub chunk_size: u32,
}

impl BlockDimensions {
	pub fn size(&self) -> usize {
		self.rows
			.0
			.saturating_mul(self.cols.0)
			.saturating_mul(self.chunk_size) as usize
	}

	pub fn new<R, C>(rows: R, cols: C, chunk_size: u32) -> Self
	where
		R: Into<BlockLengthRows>,
		C: Into<BlockLengthColumns>,
	{
		Self {
			rows: rows.into(),
			cols: cols.into(),
			chunk_size,
		}
	}
}

#[derive(PartialEq, Eq, Debug)]
pub enum TryFromBlockDimensionsError {
	InvalidRowsOrColumns(sp_std::num::TryFromIntError),
	InvalidDimensions,
}

impl From<sp_std::num::TryFromIntError> for TryFromBlockDimensionsError {
	fn from(error: sp_std::num::TryFromIntError) -> Self {
		TryFromBlockDimensionsError::InvalidRowsOrColumns(error)
	}
}

#[cfg(feature = "std")]
impl sp_std::convert::TryInto<Dimensions> for BlockDimensions {
	type Error = TryFromBlockDimensionsError;

	fn try_into(self) -> Result<Dimensions, Self::Error> {
		let rows = self.rows.0.try_into()?;
		let cols = self.cols.0.try_into()?;

		Dimensions::new(rows, cols).ok_or(TryFromBlockDimensionsError::InvalidDimensions)
	}
}

// vim: set noet nowrap

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
pub use dusk_plonk::{commitment_scheme::kzg10::PublicParameters, prelude::BlsScalar};
use static_assertions::const_assert_ne;

use crate::config::DATA_CHUNK_SIZE;

pub const LOG_TARGET: &str = "kate";
pub type Seed = [u8; 32];

pub mod config {
	pub const SCALAR_SIZE_WIDE: usize = 64;
	pub const SCALAR_SIZE: usize = 32;
	pub const DATA_CHUNK_SIZE: usize = 31; // Actual chunk size is 32 after 0 padding is done
	pub const EXTENSION_FACTOR: usize = 2;
	pub const PROVER_KEY_SIZE: usize = 48;
	pub const PROOF_SIZE: usize = 48;
	// MINIMUM_BLOCK_SIZE, MAX_BLOCK_ROWS and MAX_BLOCK_COLUMNS have to be a power of 2 because of the FFT functions requirements
	pub const MINIMUM_BLOCK_SIZE: usize = 128;
	pub const MAX_BLOCK_ROWS: u32 = if cfg!(feature = "extended-columns") {
		128
	} else {
		256
	};
	pub const MAX_BLOCK_COLUMNS: u32 = if cfg!(feature = "extended-columns") {
		512
	} else {
		256
	};
	pub const MAXIMUM_BLOCK_SIZE: bool = cfg!(feature = "maximum-block-size");
}

#[cfg(feature = "std")]
pub mod com;
/// Precalculate the length of padding IEC 9797 1.
///
/// # NOTE
/// There is a unit test to ensure this formula match with the current
/// IEC 9797 1 algorithm we implemented. See `fn pad_iec_9797_1`
#[inline]
fn padded_len_of_pad_iec_9797_1(len: u32) -> u32 {
	(len + 1)
		+ (DATA_CHUNK_SIZE as u32 - ((len + 1) % DATA_CHUNK_SIZE as u32)) % DATA_CHUNK_SIZE as u32
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
	pub rows: usize,
	pub cols: usize,
	pub chunk_size: usize,
}

impl BlockDimensions {
	pub fn size(&self) -> usize {
		self.rows
			.saturating_mul(self.cols)
			.saturating_mul(self.chunk_size)
	}
}

#[cfg(feature = "std")]
pub mod testnet {
	use std::{collections::HashMap, sync::Mutex};

	use dusk_plonk::commitment_scheme::kzg10::PublicParameters;
	use once_cell::sync::Lazy;
	use rand::SeedableRng;
	use rand_chacha::ChaChaRng;

	static SRS_DATA: Lazy<Mutex<HashMap<usize, PublicParameters>>> =
		Lazy::new(|| Mutex::new(HashMap::new()));

	pub fn public_params(max_degree: usize) -> PublicParameters {
		let mut srs_data_locked = SRS_DATA.lock().unwrap();
		srs_data_locked
			.entry(max_degree)
			.or_insert_with(|| {
				let mut rng = ChaChaRng::seed_from_u64(42);
				PublicParameters::setup(max_degree, &mut rng).unwrap()
			})
			.clone()
	}
}

// vim: set noet nowrap

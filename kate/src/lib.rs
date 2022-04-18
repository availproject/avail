#![cfg_attr(not(feature = "std"), no_std)]

pub type Seed = [u8; 32];

pub mod config {
	pub const SCALAR_SIZE_WIDE: usize = 64;
	pub const SCALAR_SIZE: usize = 32;
	pub const DATA_CHUNK_SIZE: usize = 31; // Actual chunk size is 32 after 0 padding is done
	pub const EXTENSION_FACTOR: usize = 2;
	pub const PROVER_KEY_SIZE: usize = 48;
	pub const PROOF_SIZE: usize = 48;
	pub const MAX_PROOFS_REQUEST: usize = 30;
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
}

#[cfg(feature = "std")]
pub mod com;

#[cfg(all(feature = "std", feature = "testnet"))]
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

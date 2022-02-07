#![cfg_attr(not(feature = "std"), no_std)]

pub mod config {
	pub const SCALAR_SIZE_WIDE: usize = 64;
	pub const SCALAR_SIZE: usize = 32;
	pub const EXTENSION_FACTOR: usize = 2;
	pub const PROVER_KEY_SIZE: usize = 48;
	pub const PROOF_SIZE: usize = 48;
	pub const MAX_PROOFS_REQUEST: usize = 30;
	pub const MINIMUM_BLOCK_SIZE: usize = 256;
}

#[cfg(feature = "std")]
pub mod com;

#[cfg(feature = "alloc")]
pub mod testnet {
	use dusk_plonk::commitment_scheme::kzg10::PublicParameters;
	use rand::SeedableRng;
	use rand_chacha::ChaChaRng;

	use std::{sync::Mutex, collections::HashMap};
	use once_cell::sync::Lazy;

	static SRS_DATA: Lazy<Mutex<HashMap<usize, PublicParameters>>> = Lazy::new(|| {
		Mutex::new(HashMap::new())
	});

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

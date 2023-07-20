use std::{collections::HashMap, sync::Mutex};

use dusk_plonk::commitment_scheme::kzg10::PublicParameters;
use once_cell::sync::Lazy;

static SRS_DATA: Lazy<Mutex<HashMap<usize, PublicParameters>>> =
	Lazy::new(|| Mutex::new(HashMap::new()));

pub fn public_params(max_degree: usize) -> PublicParameters {
	let mut srs_data_locked = SRS_DATA.lock().unwrap();
	srs_data_locked
		.entry(max_degree)
		.or_insert_with(|| {
			use rand_chacha::{rand_core::SeedableRng as _, ChaChaRng};

			let mut rng = ChaChaRng::seed_from_u64(42);
			PublicParameters::setup(max_degree, &mut rng).unwrap()
		})
		.clone()
}

use std::{collections::HashMap, sync::Mutex};

use dusk_plonk::commitment_scheme::kzg10::PublicParameters;
use once_cell::sync::Lazy;

static SRS_DATA: Lazy<Mutex<HashMap<usize, PublicParameters>>> =
	Lazy::new(|| Mutex::new(HashMap::new()));

pub fn public_params(_max_degree: usize) -> PublicParameters {
	let pp_bytes = include_bytes!("../../src/pp_1024.data");
	PublicParameters::from_slice(pp_bytes).expect("Error is deserialising public parameters")
}

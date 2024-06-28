use dusk_plonk::commitment_scheme::kzg10::PublicParameters;

pub fn public_params() -> PublicParameters {
	let pp_bytes = include_bytes!("../../src/pp_1024.data");
	PublicParameters::from_slice(pp_bytes)
		.expect("Deserializing of public parameters should work for serialized pp")
}

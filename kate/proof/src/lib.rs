use std::convert::TryInto;

use anyhow::{anyhow, Context};
use dusk_bytes::Serializable;
use dusk_plonk::{
	bls12_381::G1Affine,
	commitment_scheme::kzg10::{commitment::Commitment, proof::Proof},
	fft::EvaluationDomain,
};
use merlin::Transcript;

mod testnet {
	use dusk_plonk::commitment_scheme::kzg10::PublicParameters;
	use rand::SeedableRng;
	use rand_chacha::ChaChaRng;

	pub fn public_params(max_degree: usize) -> PublicParameters {
		let mut rng = ChaChaRng::seed_from_u64(42);
		PublicParameters::setup(max_degree, &mut rng).unwrap()
	}
}

pub struct ProofVerification {
	pub status: Result<(), dusk_plonk::error::Error>,
	pub public_params: Vec<u8>,
}

// code for light client to verify incoming kate proofs
// args - now - column number, response (witness + evaluation_point = 48 + 32 bytes), commitment (as bytes)
// args - in future - multiple sets of these
pub fn kc_verify_proof(
	col_num: u16,
	response: &[u8],
	commitment: &[u8],
	total_rows: usize,
	total_cols: usize,
) -> anyhow::Result<ProofVerification> {
	// let total_rows = 128;
	let _extended_total_rows = total_rows * 2;
	// let total_cols = 256;

	let public_params = testnet::public_params(256);

	let (_, verifier_key) = public_params.trim(total_cols).unwrap();

	let row_eval_domain = EvaluationDomain::new(total_cols).unwrap();
	let mut row_dom_x_pts = Vec::with_capacity(row_eval_domain.size());
	row_dom_x_pts.extend(row_eval_domain.elements());

	let (witness, eval) = response.split_at(48);

	// log::info!("{:?} {:?}", witness.len(), eval.len());

	let commitment_point = G1Affine::from_bytes(
		commitment
			.try_into()
			.context("commitment slice with incorrect length")?,
	)
	.expect("Invalid commitment point");
	let eval_point = dusk_plonk::prelude::BlsScalar::from_bytes(
		eval.try_into()
			.context("evaluation point slice with incorrect length")?,
	)
	.unwrap();
	let witness_point = G1Affine::from_bytes(
		witness
			.try_into()
			.context("witness slice with incorrect length")?,
	)
	.map_err(|_| anyhow!("Invalid witness point"))?;
	// Discarding error due to unimplemented traits which prevents us to use context

	let proof = Proof {
		commitment_to_witness: Commitment::from(witness_point),
		evaluated_point: eval_point,
		commitment_to_polynomial: Commitment::from(commitment_point),
	};

	let point = row_dom_x_pts[col_num as usize];
	let status = verifier_key.batch_check(&[point], &[proof], &mut Transcript::new(b""));
	let raw_pp = public_params.to_raw_var_bytes();

	Ok(ProofVerification {
		status,
		public_params: raw_pp,
	})
}

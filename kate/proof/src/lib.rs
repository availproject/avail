use std::convert::TryInto;

use anyhow::{anyhow, Context};
use dusk_bytes::Serializable;
use dusk_plonk::{
	bls12_381::G1Affine,
	commitment_scheme::kzg10::{commitment::Commitment, proof::Proof, PublicParameters},
	fft::EvaluationDomain,
};

pub mod testnet {
	use dusk_plonk::commitment_scheme::kzg10::PublicParameters;
	use rand::SeedableRng;
	use rand_chacha::ChaChaRng;

	pub fn public_params(max_degree: usize) -> PublicParameters {
		let mut rng = ChaChaRng::seed_from_u64(42);
		PublicParameters::setup(max_degree, &mut rng).unwrap()
	}
}

// code for light client to verify incoming kate proofs
// args - now - column number, response (witness + evaluation_point = 48 + 32 bytes), commitment (as bytes)
// args - in future - multiple sets of these
pub fn kc_verify_proof(
	col_num: u32,          // column identifier index ( 0 -based )
	response: &[u8],       // witness ( 48B ) + evaluation point ( 32B ) = proof ( 80B )
	commitment: &[u8],     // commitment generated by prover ( 48B )
	_total_rows: usize,    // # -of rows in data matrix against which proof/ commitment are generated
	total_cols: usize,     // # -of cols in data matrix against which proof/ commitment are generated
	pp: &PublicParameters, // public parameters with max degree >= total_cols [ensure this]
) -> anyhow::Result<bool> {
	let (_, verifier_key) = pp.trim(total_cols).context("trimming failed")?;

	let row_eval_domain = EvaluationDomain::new(total_cols).unwrap();
	let mut row_dom_x_pts = Vec::with_capacity(row_eval_domain.size());
	row_dom_x_pts.extend(row_eval_domain.elements());

	let (witness, eval) = response.split_at(48);

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
	let status = verifier_key.check(point, proof);

<<<<<<< HEAD
	Ok(status)
=======
	// Ok(ProofVerification {
	// 	status
	// // 	let public_params_hash =  hex::encode(sp_core::blake2_128(&raw_pp));
	// // 	let public_params_len =  hex::encode(raw_pp).len();
	// // 	public_params: raw_pp,
	// })
	Ok(status)
	
>>>>>>> 6906a76 (verify_proof return changes)
}

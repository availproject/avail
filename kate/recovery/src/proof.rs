use dusk_bytes::Serializable;
use dusk_plonk::{
	bls12_381::G1Affine,
	commitment_scheme::kzg10::{commitment::Commitment, proof::Proof, PublicParameters},
	fft::EvaluationDomain,
	prelude::BlsScalar,
};
use thiserror::Error;

use crate::{config::COMMITMENT_SIZE, data::Cell, matrix::Dimensions};

#[derive(Error, Debug)]
pub enum Error {
	#[error("Proof, data or commitment is not valid: {0}")]
	InvalidData(String),
	#[error("Evaluation domain is not valid for given dimensions: {0}")]
	InvalidDomain(String),
	#[error("Public parameters degree is to small for given dimensions: {0}")]
	InvalidDegree(String),
}

impl From<dusk_bytes::Error> for Error {
	fn from(error: dusk_bytes::Error) -> Self {
		Error::InvalidData(format!("{error:?}"))
	}
}

/// Verifies proof for given cell
pub fn verify(
	public_parameters: &PublicParameters,
	dimensions: &Dimensions,
	commitment: &[u8; COMMITMENT_SIZE],
	cell: &Cell,
) -> Result<bool, Error> {
	let commitment_to_witness = G1Affine::from_bytes(&cell.proof()).map(Commitment::from)?;

	let evaluated_point = BlsScalar::from_bytes(&cell.data())?;

	let commitment_to_polynomial = G1Affine::from_bytes(commitment).map(Commitment::from)?;

	let proof = Proof {
		commitment_to_witness,
		evaluated_point,
		commitment_to_polynomial,
	};

	let point = EvaluationDomain::new(dimensions.cols().into())
		.map_err(|error| Error::InvalidDomain(format!("{error:?}")))?
		.elements()
		.nth(cell.position.col.into())
		.ok_or_else(|| Error::InvalidDomain("Position isn't in domain".to_string()))?;

	public_parameters
		.trim(dimensions.cols().into())
		.map(|(_, verifier_key)| verifier_key.check(point, proof))
		.map_err(|error| Error::InvalidDegree(format!("{error:?}")))
}

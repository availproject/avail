#[cfg(feature = "std")]
use dusk_bytes::Serializable;
#[cfg(feature = "std")]
use dusk_plonk::{
	bls12_381::G1Affine,
	commitment_scheme::kzg10::{commitment::Commitment, proof::Proof, PublicParameters},
	fft::EvaluationDomain,
	prelude::BlsScalar,
};
use thiserror_no_std::Error;

use crate::{data::Cell, matrix::Dimensions};
use avail_core::constants::kate::COMMITMENT_SIZE;

#[derive(Error, Debug)]
pub enum Error {
	#[error("Proof, data or commitment is not valid")]
	InvalidData,
	#[error("Evaluation domain is not valid for given dimensions")]
	InvalidDomain,
	#[error("Public parameters degree is to small for given dimensions")]
	InvalidDegree,
	#[error("Position isn't in domain")]
	InvalidPositionInDomain,
}

#[cfg(feature = "std")]
impl From<dusk_bytes::Error> for Error {
	fn from(_: dusk_bytes::Error) -> Self {
		Error::InvalidData
	}
}

/// Verifies proof for given cell
#[cfg(feature = "std")]
pub fn verify(
	public_parameters: &PublicParameters,
	dimensions: Dimensions,
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

	let cols: usize = dimensions.width();
	let point = EvaluationDomain::new(cols)
		.map_err(|_| Error::InvalidDomain)?
		.elements()
		.nth(cell.position.col.into())
		.ok_or(Error::InvalidPositionInDomain)?;

	Ok(public_parameters.opening_key().check(point, proof))
}

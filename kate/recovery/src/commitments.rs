use std::{array::TryFromSliceError, convert::TryFrom};

use dusk_bytes::Serializable;
use dusk_plonk::{
	fft::{EvaluationDomain, Evaluations},
	prelude::{BlsScalar, CommitKey, PublicParameters},
};
use thiserror::Error;

const SCALAR_SIZE: usize = 32;
const COMMITMENT_SIZE: usize = 48;

#[derive(Error, Debug)]
pub enum Error {
	#[error("Invalid data: {0}")]
	InvalidData(String),
}

impl From<TryFromSliceError> for Error {
	fn from(e: TryFromSliceError) -> Self { Self::InvalidData(format!("{}", e)) }
}

impl From<dusk_bytes::Error> for Error {
	fn from(e: dusk_bytes::Error) -> Self {
		match e {
			dusk_bytes::Error::InvalidData => {
				Self::InvalidData("Scalar data is not valid".to_string())
			},
			dusk_bytes::Error::BadLength { .. } => {
				Self::InvalidData("Invlaid scalar data length".to_string())
			},
			dusk_bytes::Error::InvalidChar { .. } => {
				Self::InvalidData("Scalar data contains invalid character".to_string())
			},
		}
	}
}

impl From<dusk_plonk::error::Error> for Error {
	fn from(e: dusk_plonk::error::Error) -> Self { Self::InvalidData(format!("{}", e)) }
}

fn try_into_scalar(chunk: &[u8]) -> Result<BlsScalar, Error> {
	let sized_chunk = <[u8; SCALAR_SIZE]>::try_from(chunk)?;
	BlsScalar::from_bytes(&sized_chunk).map_err(From::from)
}

fn try_into_scalars(data: &[u8]) -> Result<Vec<BlsScalar>, Error> {
	// TODO: Replace with error
	assert!(data.len() % SCALAR_SIZE == 0);

	data.chunks_exact(SCALAR_SIZE)
		.map(try_into_scalar)
		.collect::<Result<Vec<BlsScalar>, Error>>()
}

fn verify_row(
	prover_key: &CommitKey,
	domain: EvaluationDomain,
	commitment: &[u8],
	row: &Option<Vec<u8>>,
) -> Result<bool, Error> {
	match row {
		None => Ok(true),
		Some(row) => {
			let scalars = try_into_scalars(row)?;
			let poly = Evaluations::from_vec_and_domain(scalars, domain).interpolate();
			let row_commitment = prover_key.commit(&poly)?;
			Ok(row_commitment.to_bytes() == commitment)
		},
	}
}

pub fn verify(
	public_params: &PublicParameters,
	commitments: &[u8],
	cols_num: usize,
	rows: Vec<Option<Vec<u8>>>,
) -> Result<bool, Error> {
	// TODO: Replace with error
	assert!(commitments.len() / COMMITMENT_SIZE == rows.len());
	assert!(commitments.len() % COMMITMENT_SIZE == 0);

	let (prover_key, _) = public_params.trim(cols_num)?;
	let domain = EvaluationDomain::new(cols_num)?;

	let verifications = commitments
		.chunks_exact(COMMITMENT_SIZE)
		.zip(rows.iter())
		.map(|(commitment, row)| verify_row(&prover_key, domain, commitment, row))
		.collect::<Result<Vec<bool>, Error>>()?;

	Ok(verifications.iter().all(|&v| v))
}

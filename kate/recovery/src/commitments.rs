use std::{array::TryFromSliceError, convert::TryFrom};

use dusk_bytes::Serializable;
use dusk_plonk::{
	fft::{EvaluationDomain, Evaluations},
	prelude::{BlsScalar, CommitKey, PublicParameters},
};
use thiserror::Error;

use crate::com::{CHUNK_SIZE, COMMITMENT_SIZE};

#[derive(Error, Debug)]
pub enum Error {
	#[error("Invalid data: {0}")]
	InvalidData(String),
	#[error("Cannot create {0}-bytes chunks from {1} bytes")]
	InvalidChunksData(usize, usize),
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
				Self::InvalidData("Invalid scalar data length".to_string())
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
	let sized_chunk = <[u8; CHUNK_SIZE]>::try_from(chunk)?;
	BlsScalar::from_bytes(&sized_chunk).map_err(From::from)
}

fn try_into_scalars(data: &[u8]) -> Result<Vec<BlsScalar>, Error> {
	let chunks = data.chunks_exact(CHUNK_SIZE);
	if !chunks.remainder().is_empty() {
		return Err(Error::InvalidData("Invalid data length".to_string()));
	}
	chunks
		.map(try_into_scalar)
		.collect::<Result<Vec<BlsScalar>, Error>>()
}

fn row_commitment(
	prover_key: &CommitKey,
	domain: EvaluationDomain,
	row: &[u8],
) -> Result<[u8; COMMITMENT_SIZE], Error> {
	let scalars = try_into_scalars(row)?;
	let polynomial = Evaluations::from_vec_and_domain(scalars, domain).interpolate();
	Ok(prover_key.commit(&polynomial)?.to_bytes())
}

/// Verifies given commitments and row commitments equality. Commitments are verified only for specified rows,
/// which means that unspecified rows will be assumed as verified.
///
/// # Arguments
///
/// * `public_params` - Public parameters
/// * `commitments` - Commitments represented as byte array (as in header)
/// * `cols_num` - Number of columns
/// * `rows` - Array of optional rows
pub fn verify_equality(
	public_params: &PublicParameters,
	commitments: &[u8],
	cols_num: usize,
	rows: &[Option<Vec<u8>>],
) -> Result<bool, Error> {
	if commitments.len() / COMMITMENT_SIZE != rows.len() {
		return Err(Error::InvalidData(
			"Rows and commitments count mismatch".to_string(),
		));
	}

	if commitments.len() % COMMITMENT_SIZE > 0 {
		return Err(Error::InvalidData("Invalid commitments data".to_string()));
	}

	let (prover_key, _) = public_params.trim(cols_num)?;
	let domain = EvaluationDomain::new(cols_num)?;

	let verifications = commitments
		.chunks_exact(COMMITMENT_SIZE)
		.zip(rows.iter())
		.map(|(commitment, row)| match row {
			None => Ok(true),
			Some(row) => Ok(row_commitment(&prover_key, domain, row)? == commitment),
		})
		.collect::<Result<Vec<bool>, Error>>()?;

	Ok(verifications.iter().all(|&v| v))
}

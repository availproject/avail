use std::{array::TryFromSliceError, convert::TryFrom};

use dusk_bytes::Serializable;
use dusk_plonk::{
	fft::{EvaluationDomain, Evaluations},
	prelude::{BlsScalar, CommitKey, PublicParameters},
};
use thiserror::Error;

use crate::config;

#[derive(Error, Debug)]
pub enum DataError {
	#[error("Scalar slice error: {0}")]
	SliceError(TryFromSliceError),
	#[error("Scalar data is not valid")]
	ScalarDataError,
	#[error("Invalid scalar data length")]
	BadScalarDataLen,
	#[error("Scalar data contains invalid character")]
	BadScalarData,
	#[error("Bad data len")]
	BadLen,
	#[error("Plonk error: {0}")]
	PlonkError(dusk_plonk::error::Error),
	#[error("Row and commitments count mismatch")]
	RowAndCommitmentsMismatch,
	#[error("Bad commitments data")]
	BadCommitmentsData,
}

#[derive(Error, Debug)]
pub enum Error {
	#[error("Invalid data: {0}")]
	InvalidData(DataError),
}

impl From<TryFromSliceError> for Error {
	fn from(e: TryFromSliceError) -> Self { Self::InvalidData(DataError::SliceError(e)) }
}

impl From<dusk_bytes::Error> for Error {
	fn from(e: dusk_bytes::Error) -> Self {
		match e {
			dusk_bytes::Error::InvalidData => Self::InvalidData(DataError::ScalarDataError),
			dusk_bytes::Error::BadLength { .. } => Self::InvalidData(DataError::BadScalarDataLen),
			dusk_bytes::Error::InvalidChar { .. } => Self::InvalidData(DataError::BadScalarData),
		}
	}
}

impl From<dusk_plonk::error::Error> for Error {
	fn from(e: dusk_plonk::error::Error) -> Self { Self::InvalidData(DataError::PlonkError(e)) }
}

fn try_into_scalar(chunk: &[u8]) -> Result<BlsScalar, Error> {
	let sized_chunk = <[u8; config::CHUNK_SIZE]>::try_from(chunk)?;
	BlsScalar::from_bytes(&sized_chunk).map_err(From::from)
}

fn try_into_scalars(data: &[u8]) -> Result<Vec<BlsScalar>, Error> {
	let chunks = data.chunks_exact(config::CHUNK_SIZE);
	if !chunks.remainder().is_empty() {
		return Err(Error::InvalidData(DataError::BadLen));
	}
	chunks
		.map(try_into_scalar)
		.collect::<Result<Vec<BlsScalar>, Error>>()
}

fn row_commitment(
	prover_key: &CommitKey,
	domain: EvaluationDomain,
	row: &[u8],
) -> Result<[u8; config::COMMITMENT_SIZE], Error> {
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
	if commitments.len() / config::COMMITMENT_SIZE != rows.len() {
		return Err(Error::InvalidData(DataError::RowAndCommitmentsMismatch));
	}

	if commitments.len() % config::COMMITMENT_SIZE > 0 {
		return Err(Error::InvalidData(DataError::BadCommitmentsData));
	}

	let (prover_key, _) = public_params.trim(cols_num)?;
	let domain = EvaluationDomain::new(cols_num)?;

	let verifications = commitments
		.chunks_exact(config::COMMITMENT_SIZE)
		.zip(rows.iter())
		.map(|(commitment, row)| match row {
			None => Ok(true),
			Some(row) => Ok(row_commitment(&prover_key, domain, row)? == commitment),
		})
		.collect::<Result<Vec<bool>, Error>>()?;

	Ok(verifications.iter().all(|&v| v))
}

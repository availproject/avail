use std::{array::TryFromSliceError, convert::TryFrom, num::TryFromIntError};

use dusk_bytes::Serializable;
use dusk_plonk::{
	fft::{EvaluationDomain, Evaluations},
	prelude::{BlsScalar, PublicParameters},
};
use thiserror::Error;

use crate::{com, config, index, matrix};

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
	#[error("Bad commitments data")]
	BadCommitmentsData,
	#[error("Bad rows data")]
	BadRowsData,
}

#[derive(Error, Debug)]
pub enum Error {
	#[error("Invalid data: {0}")]
	InvalidData(DataError),
}

impl From<TryFromSliceError> for Error {
	fn from(e: TryFromSliceError) -> Self {
		Self::InvalidData(DataError::SliceError(e))
	}
}

impl From<TryFromIntError> for Error {
	fn from(_: TryFromIntError) -> Self {
		Self::InvalidData(DataError::BadCommitmentsData)
	}
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
	fn from(e: dusk_plonk::error::Error) -> Self {
		Self::InvalidData(DataError::PlonkError(e))
	}
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

/// Verifies given commitments and row commitments equality.
/// Commitments are verified only for app specific data rows.
/// Function returns pair of verified and missing data rows, or an error.
///
/// # Arguments
///
/// * `public_params` - Public parameters
/// * `commitments` - Commitments represented as byte array (as in header)
/// * `rows` - Array of optional rows
/// * `index` - Application data index
/// * `dimensions` - Extended matrix dimensions
/// * `app_id` - Application ID
pub fn verify_equality(
	public_params: &PublicParameters,
	commitments: &[u8],
	rows: &[Option<Vec<u8>>],
	index: &index::AppDataIndex,
	dimensions: &matrix::Dimensions,
	app_id: u32,
) -> Result<(Vec<u16>, Vec<u16>), Error> {
	if commitments.len() % config::COMMITMENT_SIZE > 0 {
		return Err(Error::InvalidData(DataError::BadCommitmentsData));
	}

	if <u32>::try_from(commitments.len() / config::COMMITMENT_SIZE)? != dimensions.extended_rows() {
		return Err(Error::InvalidData(DataError::BadCommitmentsData));
	}

	if <u32>::try_from(rows.len())? != dimensions.extended_rows() {
		return Err(Error::InvalidData(DataError::BadRowsData));
	}
	let commitment_chunks = commitments.chunks_exact(config::COMMITMENT_SIZE);

	let app_rows = com::app_specific_rows(index, dimensions, app_id);

	let present_rows = rows
		.iter()
		.zip(0u32..)
		.filter(|(row, _)| row.is_some())
		.collect::<Vec<_>>();

	if present_rows.len() != app_rows.len() {
		return Err(Error::InvalidData(DataError::BadRowsData));
	}

	let app_rows_present = present_rows
		.into_iter()
		.zip(app_rows.iter())
		.all(|((_, a), &b)| a == b);

	if !app_rows_present {
		return Err(Error::InvalidData(DataError::BadRowsData));
	}

	let (prover_key, _) = public_params.trim(dimensions.cols() as usize)?;
	let domain = EvaluationDomain::new(dimensions.cols() as usize)?;

	// This is a single-threaded implementation.
	// At some point we should benchmark and decide
	// if we need parallel commitments verification.
	let verifications = commitment_chunks
		.zip(rows.iter())
		.enumerate()
		.filter_map(|(index, (commitment, row))| row.as_ref().map(|row| (index, commitment, row)))
		.map(|(index, commitment, row)| {
			let scalars = try_into_scalars(row)?;
			let polynomial = Evaluations::from_vec_and_domain(scalars, domain).interpolate();
			let row_commitment = prover_key.commit(&polynomial)?.to_bytes();
			Ok((index as u16, row_commitment == commitment))
		})
		.collect::<Result<Vec<(u16, bool)>, Error>>()?;

	let (verified, missing): (Vec<_>, Vec<_>) =
		verifications.iter().partition(|(_, is_equal)| *is_equal);

	Ok((
		verified.into_iter().map(|(i, _)| i).collect::<Vec<u16>>(),
		missing.into_iter().map(|(i, _)| i).collect::<Vec<u16>>(),
	))
}

#[cfg(test)]
mod tests {
	use dusk_plonk::prelude::PublicParameters;
	use once_cell::sync::Lazy;
	use rand::SeedableRng;
	use rand_chacha::ChaChaRng;

	use crate::{index, matrix};

	static PUBLIC_PARAMETERS: Lazy<PublicParameters> =
		Lazy::new(|| PublicParameters::setup(256, &mut ChaChaRng::seed_from_u64(42)).unwrap());

	#[test]
	fn verify_equality() {
		let _ = super::verify_equality(
			&PUBLIC_PARAMETERS,
			&[],
			&[],
			&index::AppDataIndex::default(),
			&matrix::Dimensions::new(1, 1).unwrap(),
			0,
		)
		.is_err();
	}
}

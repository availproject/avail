#![cfg(feature = "std")]

use avail_core::{AppExtrinsic, DaCommitments};
use frame_system::limits::BlockLength;
use kate::{
	couscous::multiproof_params,
	gridgen::{AsBytes, EvaluationGrid},
	pmp::m1_blst::M1NoPrecomp,
	Seed,
};
use sp_runtime::SaturatedConversion;
use std::{sync::OnceLock, vec::Vec};
use thiserror_no_std::Error;

pub const MIN_WIDTH: usize = 4;

static PMP: OnceLock<M1NoPrecomp> = OnceLock::new();

#[derive(Error, Debug)]
pub enum DaCommitmentsError {
	#[error("Grid construction failed: {0}")]
	GridConstructionFailed(String),
	#[error("Make polynomial grid failed: {0}")]
	MakePolynomialGridFailed(String),
	#[error("Grid extension failed: {0}")]
	GridExtensionFailed(String),
	#[error("Commitment serialization failed: {0}")]
	CommitmentSerializationFailed(String),
}

fn build_grid(
	data: Vec<u8>,
	block_length: BlockLength,
	seed: Seed,
) -> Result<EvaluationGrid, DaCommitmentsError> {
	let app_ext = AppExtrinsic::from(data);
	let grid = EvaluationGrid::from_extrinsics(
		vec![app_ext],
		MIN_WIDTH,
		block_length.cols.0.saturated_into(), // even if we run on a u16 target this is fine
		block_length.rows.0.saturated_into(),
		seed,
	)
	.map_err(|e| DaCommitmentsError::GridConstructionFailed(format!("{:?}", e)))?;

	Ok(grid)
}

fn build_commitment(grid: &EvaluationGrid) -> Result<Vec<u8>, DaCommitmentsError> {
	let pmp = PMP.get_or_init(multiproof_params);

	let poly_grid = grid
		.make_polynomial_grid()
		.map_err(|e| DaCommitmentsError::MakePolynomialGridFailed(format!("{:?}", e)))?;

	let extended_grid = poly_grid
		.extended_commitments(pmp, 2)
		.map_err(|e| DaCommitmentsError::GridExtensionFailed(format!("{:?}", e)))?;

	let mut commitment = Vec::new();
	for c in extended_grid.iter() {
		match c.to_bytes() {
			Ok(bytes) => commitment.extend(bytes),
			Err(e) => {
				return Err(DaCommitmentsError::CommitmentSerializationFailed(format!(
					"{:?}",
					e
				)))
			},
		}
	}

	Ok(commitment)
}

pub fn build_da_commitments(
	data: Vec<u8>,
	block_length: BlockLength,
	seed: Seed,
) -> Result<DaCommitments, DaCommitmentsError> {
	let grid = build_grid(data, block_length, seed)?;

	let commitments = build_commitment(&grid)?;
	let da_commitments = commitments
		.chunks(48)
		.map(|chunk| {
			let mut array = [0u8; 48];
			array.copy_from_slice(chunk);
			array
		})
		.collect();
	Ok(da_commitments)
}

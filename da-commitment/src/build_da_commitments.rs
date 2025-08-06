#![cfg(feature = "std")]

use anyhow::{anyhow, Result};
use kate::{
	couscous::multiproof_params,
	gridgen::core::{AsBytes, EvaluationGrid},
	pmp::{ark_bls12_381::Bls12_381, Commitment},
	M1NoPrecomp, Seed,
};
use std::{sync::OnceLock, vec::Vec};
use thiserror_no_std::Error;

pub type DaCommitments = Vec<u8>;
pub type ArkCommitment = Commitment<Bls12_381>;

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
	max_width: usize,
	max_height: usize,
	seed: Seed,
) -> Result<EvaluationGrid, DaCommitmentsError> {
	let grid = EvaluationGrid::from_data(data, max_width, max_width, max_height, seed)
		.map_err(|e| DaCommitmentsError::GridConstructionFailed(format!("{:?}", e)))?;

	Ok(grid)
}

fn build_commitment(grid: &EvaluationGrid) -> Result<Vec<u8>, DaCommitmentsError> {
	let pmp = PMP.get_or_init(multiproof_params);

	let poly_grid = grid
		.make_polynomial_grid()
		.map_err(|e| DaCommitmentsError::MakePolynomialGridFailed(format!("{:?}", e)))?;

	let extended_grid = poly_grid
		.commitments(pmp)
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
	max_width: usize,
	max_height: usize,
	seed: Seed,
) -> DaCommitments {
	let grid = match build_grid(data, max_width, max_height, seed) {
		Ok(grid) => grid,
		Err(e) => {
			log::error!("Grid construction failed: {:?}", e);
			return DaCommitments::new();
		},
	};

	let commitments = match build_commitment(&grid) {
		Ok(commitments) => commitments,
		Err(e) => {
			log::error!("Commitment generation failed: {:?}", e);
			return DaCommitments::new();
		},
	};

	commitments
}

pub fn build_extended_commitments(commitments: Vec<u8>) -> Result<Vec<[u8; 48]>> {
	let original_commitments: Vec<ArkCommitment> = commitments
		.chunks_exact(48)
		.enumerate()
		.map(|(i, chunk)| {
			let chunk_array: [u8; 48] = chunk
				.try_into()
				.map_err(|_| anyhow!("Chunk at index {i} is not 48 bytes long"))?;

			ArkCommitment::from_bytes(&chunk_array)
				.map_err(|e| anyhow!("Invalid commitment at index {i}: {e}"))
		})
		.collect::<Result<_, _>>()?;

	let extended_commitments =
		ArkCommitment::extend_commitments(&original_commitments, original_commitments.len() * 2)
			.expect("extending commitments should work if dimensions are valid");

	let commitments_bytes: Vec<_> = extended_commitments
		.into_iter()
		.map(|c| {
			c.to_bytes()
				.expect("Valid commitments should be serialisable")
		})
		.collect();

	Ok(commitments_bytes)
}

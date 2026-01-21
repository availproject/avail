#![cfg(feature = "std")]

use anyhow::Result;
use kate::gridgen::core::PolynomialGrid;
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
	data: &[u8],
	max_width: usize,
	max_height: usize,
	seed: Seed,
) -> Result<EvaluationGrid, DaCommitmentsError> {
	let grid = EvaluationGrid::from_data(data, max_width, max_width, max_height, seed)
		.map_err(|e| DaCommitmentsError::GridConstructionFailed(format!("{:?}", e)))?;

	Ok(grid)
}

pub fn build_polynomial_grid(
	data: &[u8],
	max_width: usize,
	max_height: usize,
	seed: Seed,
) -> PolynomialGrid {
	let grid = match build_grid(data, max_width, max_height, seed) {
		Ok(grid) => grid,
		Err(e) => {
			log::error!("Grid construction failed: {:?}", e);
			panic!("Failed to build grild");
		},
	};

	let Ok(poly_grid) = grid.into_polynomial_grid() else {
		panic!("Failed to build polynominal grid")
	};

	poly_grid
}

pub fn build_commitments_from_polynomial_grid(grid: PolynomialGrid) -> Vec<u8> {
	let pmp = PMP.get_or_init(multiproof_params);

	let Ok(extended_grid) = grid.commitments(pmp) else {
		panic!("Failed to build commitments");
	};

	let mut commitment = Vec::new();
	for c in extended_grid.iter() {
		match c.to_bytes() {
			Ok(bytes) => commitment.extend(bytes),
			Err(_) => {
				panic!("Failed to get bytes from comitment");
			},
		}
	}

	commitment
}

fn build_commitment(grid: EvaluationGrid) -> Result<Vec<u8>, DaCommitmentsError> {
	let pmp = PMP.get_or_init(multiproof_params);

	let poly_grid = grid
		.into_polynomial_grid()
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
	data: &[u8],
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

	let commitments = match build_commitment(grid) {
		Ok(commitments) => commitments,
		Err(e) => {
			log::error!("Commitment generation failed: {:?}", e);
			return DaCommitments::new();
		},
	};

	commitments
}

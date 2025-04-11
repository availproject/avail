use avail_core::{AppExtrinsic, BlockLengthColumns, BlockLengthRows};
use kate::{
	com::Cell, couscous::multiproof_params, gridgen::{AsBytes, EvaluationGrid}, pmp::m1_blst::M1NoPrecomp, Seed
};
use kate_recovery::matrix::Dimensions;
use std::{sync::OnceLock, vec::Vec};
use thiserror_no_std::Error;

pub type DaCommitments = Vec<u8>;

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
	#[error("Multiproof generation failed: {0}")]
	MultiproofGenerationFailed(String),
}

fn build_grid(
	data: Vec<u8>,
	max_width: usize,
	max_height: usize,
	seed: Seed,
) -> Result<EvaluationGrid, DaCommitmentsError> {
	// let app_ext = AppExtrinsic::from(data);
	let grid = EvaluationGrid::from_data(data, max_width, max_width, max_height, seed)
		.map_err(|e| DaCommitmentsError::GridConstructionFailed(format!("{:?}", e)))?;

	Ok(grid)
}

fn build_commitment(grid: &EvaluationGrid) -> Result<Vec<u8>, DaCommitmentsError> {
	let start = std::time::Instant::now();
	let pmp = PMP.get_or_init(multiproof_params);
	let pmp_time = start.elapsed();
	println!("PMP initialization time: {:?}", pmp_time);

	let poly_grid = grid
		.make_polynomial_grid()
		.map_err(|e| DaCommitmentsError::MakePolynomialGridFailed(format!("{:?}", e)))?;

	let poly_grid_time = start.elapsed();
	println!("Make polynomial grid time: {:?}", poly_grid_time - pmp_time);

	let extended_grid = poly_grid
		.commitments(pmp)
		.map_err(|e| DaCommitmentsError::GridExtensionFailed(format!("{:?}", e)))?;

	let extended_grid_time = start.elapsed();
	println!(
		" Commitments build time: {:?}",
		extended_grid_time - poly_grid_time
	);
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

	let commitment_time = start.elapsed();
	println!(
		"Commitment serialization time: {:?}",
		commitment_time - extended_grid_time
	);
	Ok(commitment)
}

// Generate multiproof
fn generate_proof(grid: &EvaluationGrid) -> Result<[u8; 48], DaCommitmentsError> {
	let start = std::time::Instant::now();
	let pmp = PMP.get_or_init(multiproof_params);
	let pmp_time = start.elapsed();
	println!("PMP initialization time: {:?}", pmp_time);

	let poly_grid = grid
		.make_polynomial_grid()
		.map_err(|e| DaCommitmentsError::MakePolynomialGridFailed(format!("{:?}", e)))?;

	let poly_grid_time = start.elapsed();
	println!("Make polynomial grid time: {:?}", poly_grid_time - pmp_time);

	let target_dims = Dimensions::new_from(grid.dims().rows().get(), 1).unwrap();
	let multiproof = poly_grid.multiproof(&pmp, &Cell::new(BlockLengthRows(0), BlockLengthColumns(0)), &grid, target_dims).map_err(|e| DaCommitmentsError::MultiproofGenerationFailed(format!("{:?}", e)))?;
	let proof = multiproof.proof;
	// convert proof to bytes
	let proof_bytes = proof.to_bytes().unwrap();
	Ok(proof_bytes)
}

pub fn build_da_commitments(
	data: Vec<u8>,
	max_width: usize,
	max_height: usize,
	seed: Seed,
) -> Result<DaCommitments, DaCommitmentsError> {
	let start = std::time::Instant::now();
	let grid = build_grid(data, max_width, max_height, seed)?;

	let commitments = build_commitment(&grid)?;
	let da_commitments_time = start.elapsed();
	println!("DA Commitments time: {:?}", da_commitments_time);
	// build multiproof for (rows * 1) target grid for cell(0, 0)

	Ok(commitments)
}

pub fn build_da_commitments_with_proof(
	data: Vec<u8>,
	max_width: usize,
	max_height: usize,
	seed: Seed,
) -> Result<(DaCommitments, [u8; 48]), DaCommitmentsError> {
	let start = std::time::Instant::now();
	let grid = build_grid(data, max_width, max_height, seed)?;

	let commitments = build_commitment(&grid)?;
	let proof = generate_proof(&grid)?;
	let da_commitments_time = start.elapsed();
	println!("DA Commitments with proof time: {:?}", da_commitments_time);

	Ok((commitments, proof))
}

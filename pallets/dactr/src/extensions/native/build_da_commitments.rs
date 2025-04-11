#![cfg(feature = "std")]

// use avail_core::AppExtrinsic;
use frame_system::limits::BlockLength;
use kate::{
	couscous::multiproof_params,
	gridgen::{self, AsBytes, Dimensions, EvaluationGrid},
	pmp::m1_blst::M1NoPrecomp,
	pmp::{merlin::Transcript, traits::PolyMultiProofNoPrecomp},
	Seed,
};
use sp_runtime::SaturatedConversion;
use std::{sync::OnceLock, vec::Vec};
use thiserror_no_std::Error;

pub type DaCommitments = Vec<u8>;
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
	#[error("Multiproof generation failed: {0}")]
	MultiproofVerificationFailed(String),
}

fn build_grid(
	data: Vec<u8>,
	block_length: BlockLength,
	seed: Seed,
) -> Result<EvaluationGrid, DaCommitmentsError> {
	// let app_ext = AppExtrinsic::from(data);
	// let grid = EvaluationGrid::from_extrinsics(
	let grid = EvaluationGrid::from_data(
		data,
		// MIN_WIDTH,
		block_length.cols.0.saturated_into(), // To make sure every row is of fixed length by padding
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


fn verify(
	data: Vec<u8>,
	block_length: BlockLength,
	seed: Seed,
	commitments: Vec<u8>,
	proof: &[u8; 48],
) -> Result<bool, DaCommitmentsError> {
	let start = std::time::Instant::now();
	let pmp = PMP.get_or_init(multiproof_params);

	// Build grid
	let grid = EvaluationGrid::from_data(
		data,
		block_length.cols.0.saturated_into(),
		block_length.cols.0.saturated_into(),
		block_length.rows.0.saturated_into(),
		seed,
	)
	.map_err(|e| DaCommitmentsError::GridConstructionFailed(format!("{:?}", e)))?;

	// Generate domain points
	let points = gridgen::domain_points(block_length.cols.0.saturated_into())
		.map_err(|e| DaCommitmentsError::MultiproofVerificationFailed(format!("{:?}", e)))?;

	// Create target dimensions
	let target_dims = Dimensions::new_from(grid.dims().rows(), 1).ok_or_else(|| {
		DaCommitmentsError::MultiproofVerificationFailed(
			"Failed to construct target grid".to_string(),
		)
	})?;

	// Create multiproof block
	let mp_block = gridgen::multiproof_block(0, 0, grid.dims(), target_dims).ok_or_else(|| {
		DaCommitmentsError::MultiproofVerificationFailed(
			"Failed to construct target block cell".to_string(),
		)
	})?;

	// Extract evaluation data from the grid
	let grid_evals: Vec<Vec<_>> = (mp_block.start_y..mp_block.end_y)
		.map(|y| {
			grid.row(y)
				.ok_or_else(|| {
					DaCommitmentsError::MultiproofVerificationFailed(
						"Row index out of bounds".to_string(),
					)
				})
				.map(|row| row[mp_block.start_x..mp_block.end_x].to_vec())
		})
		.collect::<Result<Vec<_>, _>>()?;

	// Deserialize the commitments
	let commits = commitments
		.chunks_exact(48)
		.skip(mp_block.start_y)
		.take(mp_block.end_y - mp_block.start_y)
		.map(|c| {
			let bytes: [u8; 48] = c
            .try_into()
            .map_err(|_| DaCommitmentsError::MultiproofVerificationFailed("Invalid commitment length".to_string()))?;

        	kate::pmp::Commitment::from_bytes(&bytes)
            .map_err(|e| DaCommitmentsError::MultiproofVerificationFailed(format!("{:?}", e)))
		})
		.collect::<Result<Vec<_>, _>>()
		.map_err(|e| DaCommitmentsError::MultiproofVerificationFailed(format!("{:?}", e)))?;

	// Deserialize the proof
	let proof = kate::pmp::m1_blst::Proof::from_bytes(proof)
		.map_err(|e| DaCommitmentsError::MultiproofVerificationFailed(format!("{:?}", e)))?;

	// Verify the multiproof
	let result = pmp
		.verify(
			&mut Transcript::new(b"avail-mp"),
			&commits,
			&points[mp_block.start_x..mp_block.end_x],
			&grid_evals,
			&proof,
		)
		.map_err(|e| DaCommitmentsError::MultiproofVerificationFailed(format!("{:?}", e)))?;

	let verification_time = start.elapsed();
	log::info!("Verification time: {:?}", verification_time);
	Ok(result)
}

// Verify multiproof
pub fn verify_multiproof(
	data: Vec<u8>,
	block_length: BlockLength,
	seed: Seed,
	commitments: Vec<u8>,
	proof: &[u8; 48],
) -> bool {
	match verify(data, block_length, seed, commitments, proof) {
		Ok(result) => result,
		Err(e) => {
			log::error!("Multiproof verification failed: {:?}", e);
			false
		},
	}
}

pub fn build_da_commitments(data: Vec<u8>, block_length: BlockLength, seed: Seed) -> DaCommitments {
	let start = std::time::Instant::now();
	let grid = match build_grid(data, block_length, seed) {
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

	let commitment_time = start.elapsed();
	log::info!("Commitment generation time: {:?}", commitment_time);

	commitments
}

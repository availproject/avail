// !!!!
// If the logic is changed in this file it will break Turing/Mainnet. Do not change it.
// If the logic is changed in avail-core it will break Turing/Mainnet as well. Do no change it.
// !!!!
#![cfg(feature = "std")]

use super::hosted_header_builder::MIN_WIDTH;
use crate::limits::BlockLength;
use avail_base::header_extension::SubmittedData;
use avail_core::{
	app_extrinsic::AppExtrinsic,
	header::{extension as he, HeaderExtension},
	kate_commitment as kc, DataLookup, HeaderVersion,
};
use kate::{
	couscous::multiproof_params,
	gridgen::{AsBytes, EvaluationGrid},
	pmp::m1_blst::M1NoPrecomp,
	Seed,
};
use sp_core::H256;
use sp_runtime::SaturatedConversion;
use std::{sync::OnceLock, vec::Vec};

#[cfg(feature = "testing-environment")]
use avail_base::testing_env::*;

static PMP: OnceLock<M1NoPrecomp> = OnceLock::new();

#[allow(dead_code)]
fn build_grid(
	submitted: Vec<AppExtrinsic>,
	block_length: BlockLength,
	seed: Seed,
) -> Result<EvaluationGrid, String> {
	#[cfg(feature = "testing-environment")]
	{
		unsafe {
			if ENABLE_TEST_GRID_FAILURE {
				return Err(String::from("ENABLE_TEST_GRID_FAILURE"));
			}
		}
	}

	let grid = EvaluationGrid::from_extrinsics(
		submitted,
		MIN_WIDTH,
		block_length.cols.0.saturated_into(), // even if we run on a u16 target this is fine
		block_length.rows.0.saturated_into(),
		seed,
	)
	.map_err(|e| format!("Grid construction failed: {e:?}"))?;

	Ok(grid)
}

#[allow(dead_code)]
fn build_commitment(grid: &EvaluationGrid) -> Result<Vec<u8>, String> {
	#[cfg(feature = "testing-environment")]
	{
		unsafe {
			if ENABLE_TEST_COMMITMENT_FAILURE {
				return Err(String::from("ENABLE_TEST_COMMITMENT_FAILURE"));
			}
		}
	}

	// couscous has pp for degree upto 1024
	let pmp = PMP.get_or_init(multiproof_params);

	let poly_grid = grid
		.make_polynomial_grid()
		.map_err(|e| format!("Make polynomial grid failed: {e:?}"))?;

	let extended_grid = poly_grid
		.extended_commitments(pmp, 2)
		.map_err(|e| format!("Grid extension failed: {e:?}"))?;

	let mut commitment = Vec::new();
	for c in extended_grid.iter() {
		match c.to_bytes() {
			Ok(bytes) => commitment.extend(bytes),
			Err(e) => return Err(format!("Commitment serialization failed: {:?}", e)),
		}
	}

	Ok(commitment)
}

#[allow(unused_mut)]
pub fn build_extension(
	mut submitted: Vec<SubmittedData>,
	data_root: H256,
	_block_length: BlockLength,
	_block_number: u32,
	_seed: Seed,
	version: HeaderVersion,
) -> HeaderExtension {
	#[cfg(feature = "testing-environment")]
	{
		unsafe {
			if ENABLE_TEST_EXTENSION_FAILURE {
				return HeaderExtension::get_faulty_header(data_root, version);
			}

			if let Some(new_extrinsics) = &TEST_POPULATE_GRID {
				submitted = new_extrinsics.clone();
			}
		}
	}

	// Blocks with non-DA extrinsics will have empty commitments
	if submitted.is_empty() {
		return HeaderExtension::get_empty_header(data_root, version);
	}

	let app_lookup = DataLookup::default();
	let total_commitments: usize = submitted
		.iter()
		.map(|da_call| da_call.commitments.len())
		.sum();
	let mut commitment = Vec::with_capacity(total_commitments);
	for da_call in submitted.iter() {
		commitment.extend(da_call.commitments.clone());
	}

	match version {
		HeaderVersion::V3 => {
			let commitment = kc::v3::KateCommitment::new(0, 0, data_root, commitment);
			he::v3::HeaderExtension {
				app_lookup,
				commitment,
			}
			.into()
		},
	}
}

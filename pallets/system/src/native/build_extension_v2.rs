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
	kate::COMMITMENT_SIZE,
	kate_commitment as kc, AppId, DataLookup, HeaderVersion,
};
use avail_observability::metrics::avail::{
	HeaderExtensionBuilderMetrics as Metrics, MetricObserver, ObserveKind,
};
use kate::{
	couscous::multiproof_params,
	gridgen::core::{AsBytes, EvaluationGrid},
	M1NoPrecomp, Seed,
};
use sp_core::H256;
use sp_runtime::SaturatedConversion;
use std::{sync::OnceLock, vec::Vec};

#[cfg(feature = "testing-environment")]
use avail_base::testing_env::*;

static PMP: OnceLock<M1NoPrecomp> = OnceLock::new();

fn build_grid(
	submitted: Vec<AppExtrinsic>,
	block_length: BlockLength,
	seed: Seed,
) -> Result<EvaluationGrid, String> {
	let _metric_observer = MetricObserver::new(ObserveKind::HEGrid);

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

fn build_commitment(grid: &EvaluationGrid) -> Result<Vec<u8>, String> {
	let _metric_observer = MetricObserver::new(ObserveKind::HECommitment);

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

/// V3 header_extension
#[allow(unused_mut)]
pub fn build_extension(
	mut submitted: Vec<AppExtrinsic>,
	data_root: H256,
	block_length: BlockLength,
	_block_number: u32,
	seed: Seed,
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

	let _metric_observer = MetricObserver::new(ObserveKind::HETotalExecutionTime);

	// Build the grid
	let maybe_grid = build_grid(submitted, block_length, seed);

	// We get the grid or return an empty header in case of an error
	let grid = match maybe_grid {
		Ok(res) => res,
		Err(message) => {
			log::error!("NODE_CRITICAL_ERROR_001 - A critical error has occured: {message:?}.");
			log::error!("NODE_CRITICAL_ERROR_001 - If you see this, please warn Avail team and raise an issue.");
			return HeaderExtension::get_faulty_header(data_root, version);
		},
	};

	let maybe_commitment = build_commitment(&grid);

	// We get the commitment or return an empty header in case of an error
	let commitment = match maybe_commitment {
		Ok(res) => res,
		Err(message) => {
			log::error!("NODE_CRITICAL_ERROR_002 - A critical error has occured: {message:?}.");
			log::error!("NODE_CRITICAL_ERROR_002 - If you see this, please warn Avail team and raise an issue.");
			return HeaderExtension::get_faulty_header(data_root, version);
		},
	};

	// Note that this uses the original dims, _not the extended ones_
	let rows = grid.dims().rows().get();
	let cols = grid.dims().cols().get();

	// Grid Metrics
	Metrics::observe_grid_rows(rows as f64);
	Metrics::observe_grid_cols(cols as f64);

	let app_lookup = grid.lookup().clone();

	let commitment = kc::v3::KateCommitment::new(rows, cols, data_root, commitment);
	he::v3::HeaderExtension {
		app_lookup,
		commitment,
	}
	.into()
}

#[allow(unused_mut)]
pub fn build_extension_v4(
	mut submitted: Vec<SubmittedData>,
	data_root: H256,
	block_length: BlockLength,
	version: HeaderVersion,
) -> HeaderExtension {
	// Blocks with non-DA extrinsics will have empty commitments
	if submitted.is_empty() {
		return HeaderExtension::get_empty_header(data_root, version);
	}

	let max_columns = block_length.cols.0 as usize;
	if max_columns == 0 {
		// Blocks with 0 columns will have empty commitments, ideally we should never reach here
		return HeaderExtension::get_empty_header(data_root, version);
	}

	let total_commitments: usize = submitted
		.iter()
		.map(|da_call| da_call.commitments.len())
		.sum();
	let mut commitment = Vec::with_capacity(total_commitments);

	let mut app_rows: Vec<(AppId, usize)> = Vec::with_capacity(submitted.len());

	for da_call in submitted.iter() {
		commitment.extend(da_call.commitments.clone());
		// As we have already correctness of commitments against data, we can safely assume that the commitments are correct
		let rows_taken = da_call.commitments.len() / COMMITMENT_SIZE;

		// Update app_rows
		app_rows.push((da_call.id, rows_taken));
	}

	let app_lookup = match DataLookup::from_id_and_len_iter(app_rows.into_iter()) {
		Ok(lookup) => lookup,
		Err(_) => return HeaderExtension::get_faulty_header(data_root, version),
	};

	let original_rows = app_lookup.len();
	let padded_rows = original_rows.next_power_of_two();

	// We can reduce the header size further letting the verification clients to do this padding since anyway they're extending the commitments
	if padded_rows > original_rows {
		let (_, padded_row_commitment) =
			match kate::gridgen::core::get_pregenerated_row_and_commitment(max_columns) {
				Ok(result) => result,
				Err(e) => {
					log::error!("NODE_CRITICAL_ERROR_003 - A critical error has occured: {e:?}.");
					log::error!("NODE_CRITICAL_ERROR_003 - If you see this, please warn Avail team and raise an issue.");
					return HeaderExtension::get_faulty_header(data_root, version);
				},
			};
		commitment = commitment
			.into_iter()
			.chain(
				std::iter::repeat(padded_row_commitment)
					.take((padded_rows - original_rows) as usize)
					.flatten(),
			)
			.collect();
	}

	let commitment = kc::v3::KateCommitment::new(
		padded_rows.try_into().unwrap_or_default(),
		max_columns.try_into().unwrap_or_default(),
		data_root,
		commitment,
	);

	he::v4::HeaderExtension {
		app_lookup,
		commitment,
	}
	.into()
}

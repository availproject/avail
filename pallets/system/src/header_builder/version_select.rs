use super::MIN_WIDTH;
use crate::limits::BlockLength;

use avail_base::metrics::avail::HeaderExtensionBuilderMetrics as Metrics;
use avail_core::{
	app_extrinsic::AppExtrinsic,
	header::{extension as he, HeaderExtension},
	kate_commitment as kc, HeaderVersion,
};
use kate::{
	couscous::multiproof_params,
	gridgen::{AsBytes, EvaluationGrid},
	pmp::m1_blst::M1NoPrecomp,
	Seed,
};

use sp_core::H256;
use sp_runtime::SaturatedConversion;
use std::{sync::OnceLock, time::Instant, vec::Vec};

static PMP: OnceLock<M1NoPrecomp> = OnceLock::new();

pub fn build_extension(
	submitted: Vec<AppExtrinsic>,
	data_root: H256,
	block_length: BlockLength,
	_block_number: u32,
	seed: Seed,
	version: HeaderVersion,
) -> HeaderExtension {
	let build_extension_start = Instant::now();

	// couscous has pp for degree upto 1024
	let pmp = PMP.get_or_init(multiproof_params);

	let timer = Instant::now();
	let grid = EvaluationGrid::from_extrinsics(
		submitted,
		MIN_WIDTH,
		block_length.cols.0.saturated_into(), // even if we run on a u16 target this is fine
		block_length.rows.0.saturated_into(),
		seed,
	)
	.expect("Grid construction cannot fail");

	// Evaluation Grid Build Time Metrics
	Metrics::observe_evaluation_grid_build_time(timer.elapsed());

	let timer = Instant::now();
	let commitment = grid
		.make_polynomial_grid()
		.expect("Make polynomials cannot fail")
		.extended_commitments(pmp, 2)
		.expect("Extended commitments cannot fail")
		.iter()
		.flat_map(|c| c.to_bytes().expect("Commitment serialization cannot fail"))
		.collect::<Vec<u8>>();

	// Commitment Build Time Metrics
	Metrics::observe_commitment_build_time(timer.elapsed());

	// Note that this uses the original dims, _not the extended ones_
	let rows = grid.dims().rows().get();
	let cols = grid.dims().cols().get();

	// Grid Metrics
	Metrics::observe_grid_rows(rows as f64);
	Metrics::observe_grid_cols(cols as f64);

	let app_lookup = grid.lookup().clone();

	let extension = match version {
		HeaderVersion::V3 => {
			let commitment = kc::v3::KateCommitment::new(rows, cols, data_root, commitment);
			he::v3::HeaderExtension {
				app_lookup,
				commitment,
			}
			.into()
		},
	};

	// Total Execution Time Metrics
	Metrics::observe_total_execution_time(build_extension_start.elapsed());
	extension
}

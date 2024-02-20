use avail_core_v1800::kate_commitment::v1::KateCommitment; 
use kate_v1800::{couscous::multiproof_params, pmp::m1_blst::M1NoPrecomp ,kate::gridgen::EvaluationGrid};

use std::{cell::OnceLock, time::Instant };
use avail_base::metrics::avail::HeaderExtensionBuilderMetrics as Metrics;

const MIN_WIDTH: usize = 4;
// couscous has pp for degree upto 1024
static PMP: OnceLock<M1NoPrecomp> = OnceLock::new();

pub fn build_extension(
	app_extrinsics: &[AppExtrinsic],
	data_root: H256,
	block_length: BlockLength,
	_block_number: u32,
	seed: Seed,
) -> HeaderExtension {

	let build_extension_start = std::time::Instant::now();
	let pmp = PMP.get_or_init(multiproof_params)

	let timer = Instant::now();
	let grid = EvaluationGrid::from_extrinsics(
		app_extrinsics.to_vec(),
		MIN_WIDTH,
		block_length.cols.0.saturated_into(), // even if we run on a u16 target this is fine
		block_length.rows.0.saturated_into(),
		seed,
	)
	.expect("V1 Grid construction cannot fail");

	// Evaluation Grid Build Time Metrics
	Metrics::observe_evaluation_grid_build_time(timer.elapsed());

	let timer = Instant::now();
	use kate::gridgen::AsBytes;
	let commitment = grid
		.make_polynomial_grid()
		.expect("V1 Make polynomials cannot fail")
		.extended_commitments(&pmp, 2)
		.expect("V1 Extended commitments cannot fail")
		.iter()
		.flat_map(|c| c.to_bytes().expect("V1 Commitment serialization cannot fail"))
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
	let commitment = KateCommitment { rows, cols, commitment, data_root };

	// Total Execution Time Metrics
	Metrics::observe_total_execution_time( build_extension_start.elapsed());

	v1::HeaderExtension { app_lookup, commitment }.into()
}

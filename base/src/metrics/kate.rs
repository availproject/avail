use substrate_prometheus_endpoint::{Histogram, PrometheusError, Registry};

use crate::metrics::linear_histogram;

pub struct KateMetrics {
	pub extend_block_time: Histogram,
	pub preparation_block_time: Histogram,
	pub commitment_time: Histogram,
	pub proof_build_time: Histogram,
	pub proof_cells: Histogram,
}

impl KateMetrics {
	pub fn new(section: &str, registry: &Registry) -> Result<Self, PrometheusError> {
		let extend_block_time = linear_histogram(
			registry,
			format!("avail_{section}_extend_block_time"),
			"Duration in microseconds of the block extension process".to_owned(),
			100.0,
			100.0,
			20,
		)?;
		let preparation_block_time = linear_histogram(
			registry,
			format!("avail_{section}_preparation_block_time"),
			"Duration in microseconds of the whole block preparation process".to_owned(),
			100.0,
			100.0,
			20,
		)?;
		let commitment_time = linear_histogram(
			registry,
			format!("avail_{section}_commitment_time"),
			"Duration in microseconds of the kate commitment".to_owned(),
			1000.0,
			250.0,
			16,
		)?;
		let proof_build_time = linear_histogram(
			registry,
			format!("avail_{section}_proof_build_time"),
			"Duration in microseconds of the kate proof".to_owned(),
			1000.0,
			250.0,
			16,
		)?;
		let proof_cells = linear_histogram(
			registry,
			format!("avail_{section}_proof_cells"),
			"Number of cells per kate proof".to_owned(),
			1000.0,
			250.0,
			16,
		)?;

		Ok(Self {
			extend_block_time,
			preparation_block_time,
			commitment_time,
			proof_build_time,
			proof_cells,
		})
	}
}

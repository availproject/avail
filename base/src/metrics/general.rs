use substrate_prometheus_endpoint::{Histogram, PrometheusError, Registry};

use crate::metrics::exp_histogram;

pub struct GeneralMetrics {
	pub block_dims_rows: Histogram,
	pub block_dims_cols: Histogram,
	pub block_len: Histogram,
}

impl GeneralMetrics {
	pub fn new(registry: &Registry) -> Result<Self, PrometheusError> {
		let block_dims_rows = exp_histogram(
			registry,
			"avail_block_dims_rows",
			"Block dimension row",
			32.0,
			2.0,
			5,
		)?;
		let block_dims_cols = exp_histogram(
			registry,
			"avail_block_dims_cols",
			"Block dimension col",
			32.0,
			2.0,
			5,
		)?;
		// @TODO Define buckets properly
		let block_len = exp_histogram(registry, "avail_block_len", "Block length", 0.001, 4.0, 9)?;

		Ok(Self {
			block_dims_cols,
			block_dims_rows,
			block_len,
		})
	}
}

use kate::{metrics::Metrics, BlockDimensions};
use sp_std::time::Duration;

use crate::metrics::AVAIL_METRICS;

/// Helper macro to implement methods from `MetricAdapter`.
macro_rules! may_observe {
		($($section: tt, $metric:tt, $observation:expr),+) => {
			if let Some(avail) = AVAIL_METRICS.get() {
				observe!(avail, $($section, $metric, $observation),+)
			}
		};
	}

/// Variadic macro to support multiple pairs of `metric` and `observation`
macro_rules! observe {
		($avail: tt, $section:tt, $metric:tt, $observation: expr) => {
			$avail.$section.$metric.observe($observation as f64)
		};

		($avail: tt, $section:tt, $metric:tt, $observation: expr, $($s:tt, $m:tt, $o: expr),+) => {{
			$avail.$section.$metric.observe($observation as f64);
			observe!($avail, $($s, $m, $o),+)
		}};
	}

/// Adapter which implements `Metrics` using `AVAIL_METRIC` signleton.
pub struct MetricAdapter {}

impl Metrics for MetricAdapter {
	fn extended_block_time(&self, elapsed: Duration) {
		may_observe!(kate, extend_block_time, elapsed.as_micros());
	}

	fn preparation_block_time(&self, elapsed: Duration) {
		may_observe!(kate, preparation_block_time, elapsed.as_micros());
	}

	fn commitment_build_time(&self, elapsed: Duration) {
		may_observe!(kate, commitment_time, elapsed.as_micros());
	}

	fn proof_build_time(&self, elapsed: Duration, cells: u32) {
		#[rustfmt::skip]
			may_observe!(
				kate, proof_build_time, elapsed.as_micros(),
				kate, proof_cells, cells
			);
	}

	fn block_dims_and_size(&self, block_dims: BlockDimensions, block_len: u32) {
		#[rustfmt::skip]
			may_observe!(
				general, block_dims_rows, block_dims.rows().0,
				general, block_dims_cols, block_dims.cols().0,
				general, block_len, block_len
			);
	}
}

pub struct RPCMetricAdapter {}

impl Metrics for RPCMetricAdapter {
	fn extended_block_time(&self, elapsed: Duration) {
		may_observe!(rpc, extend_block_time, elapsed.as_micros());
	}

	fn preparation_block_time(&self, elapsed: Duration) {
		may_observe!(rpc, preparation_block_time, elapsed.as_micros());
	}

	fn commitment_build_time(&self, elapsed: Duration) {
		may_observe!(rpc, commitment_time, elapsed.as_micros());
	}

	fn proof_build_time(&self, elapsed: Duration, cells: u32) {
		#[rustfmt::skip]
			may_observe!(
				rpc, proof_build_time, elapsed.as_micros(),
				rpc, proof_cells, cells
			);
	}

	fn block_dims_and_size(&self, block_dims: BlockDimensions, block_len: u32) {
		#[rustfmt::skip]
			may_observe!(
				general, block_dims_rows, block_dims.rows().0,
				general, block_dims_cols, block_dims.cols().0,
				general, block_len, block_len
			);
	}
}

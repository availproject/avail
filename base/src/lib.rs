#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
pub mod metrics {
	use log;
	use once_cell::sync::OnceCell;
	use substrate_prometheus_endpoint::{
		exponential_buckets, register, Histogram, HistogramOpts, Opts, PrometheusError, Registry,
	};

	const LOG_TARGET: &str = "avail::base::metrics";
	pub static AVAIL_METRICS: OnceCell<AvailMetrics> = OnceCell::new();

	/// Avail metrics.
	pub struct AvailMetrics {
		pub kate_extend_block_time: Histogram,
		pub kate_preparation_block_time: Histogram,
		pub kate_commitment_time: Histogram,
		pub kate_proof_build_time: Histogram,
		pub kate_proof_cells: Histogram,

		pub block_dims_rows: Histogram,
		pub block_dims_cols: Histogram,
		pub block_len: Histogram,
	}

	impl AvailMetrics {
		/// Creates and registries Avail Metrics.
		pub fn new(registry: &Registry) -> Result<Self, PrometheusError> {
			let kate_extend_block_time = linear_histogram(
				registry,
				"avail_kate_extend_block_time",
				"Duration in microseconds of the block extension process",
				100.0,
				100.0,
				20,
			)?;
			let kate_preparation_block_time = linear_histogram(
				registry,
				"avail_kate_preparation_block_time",
				"Duration in microseconds of the whole block preparation process",
				100.0,
				100.0,
				20,
			)?;
			let kate_commitment_time = linear_histogram(
				registry,
				"avail_kate_commitment_time",
				"Duration in microseconds of the kate commitment",
				1000.0,
				250.0,
				16,
			)?;
			let kate_proof_build_time = linear_histogram(
				registry,
				"avail_kate_proof_build_time",
				"Duration in microseconds of the kate proof",
				1000.0,
				250.0,
				16,
			)?;
			let kate_proof_cells = linear_histogram(
				registry,
				"avail_kate_proof_cells",
				"Number of cells per kate proof",
				1000.0,
				250.0,
				16,
			)?;

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
			let block_len =
				exp_histogram(registry, "avail_block_len", "Block length", 0.001, 4.0, 9)?;

			log::info!(
				target: LOG_TARGET,
				"Prometheus metrics extended with avail metrics"
			);

			Ok(Self {
				kate_extend_block_time,
				kate_preparation_block_time,
				kate_commitment_time,
				kate_proof_build_time,
				kate_proof_cells,
				block_dims_rows,
				block_dims_cols,
				block_len,
			})
		}
	}

	/// Creates an histogram using exponential buckets
	fn exp_histogram(
		registry: &Registry,
		name: &str,
		help: &str,
		start: f64,
		width: f64,
		count: usize,
	) -> Result<Histogram, PrometheusError> {
		let histogram = Histogram::with_opts(HistogramOpts {
			common_opts: Opts::new(name, help),
			buckets: exponential_buckets(start, width, count)?,
		})?;
		register(histogram.clone(), registry)?;
		log::trace!(
			target: LOG_TARGET,
			"Added exponential metric `{0}` to prometheus",
			name
		);

		Ok(histogram)
	}

	/// Creates an histogram using linear buckets
	fn linear_histogram(
		registry: &Registry,
		name: &str,
		help: &str,
		start: f64,
		width: f64,
		count: usize,
	) -> Result<Histogram, PrometheusError> {
		let histogram = Histogram::with_opts(HistogramOpts {
			common_opts: Opts::new(name, help),
			buckets: linear_buckets(start, width, count),
		})?;
		register(histogram.clone(), registry)?;
		log::trace!(
			target: LOG_TARGET,
			"Added linear metric `{0}` to prometheus",
			name
		);

		Ok(histogram)
	}

	// NOTE: `linear_buckets` exists on `prometheus` library but it is not exported by
	// `prometheus-endpoint` O_o!
	fn linear_buckets(start: f64, width: f64, count: usize) -> Vec<f64> {
		(0..count)
			.map(|step| start + width * (step as f64))
			.collect()
	}
}

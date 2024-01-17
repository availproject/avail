use core::time::Duration;

use substrate_prometheus_endpoint::{Histogram, PrometheusError, Registry};

use crate::metrics::LOG_TARGET;

use super::{custom_histogram, AVAIL_METRICS};

/// Avail metrics.
pub struct AvailMetrics {
	pub import_block: ImportBlockMetrics,
	pub header_extension: HeaderExtensionBuilderMetrics,
	pub kate_rpc: KateRpcMetrics,
}

impl AvailMetrics {
	/// Creates and registries Avail Metrics.
	pub fn new(registry: &Registry) -> Result<Self, PrometheusError> {
		let import_block = ImportBlockMetrics::new(registry)?;
		let header_extension = HeaderExtensionBuilderMetrics::new(registry)?;
		let kate_rpc = KateRpcMetrics::new(registry)?;

		log::info!(
			target: LOG_TARGET,
			"Prometheus metrics extended with avail metrics"
		);

		Ok(Self {
			import_block,
			header_extension,
			kate_rpc,
		})
	}
}

pub struct HeaderExtensionBuilderMetrics {
	pub total_execution_time: Histogram,
	pub evaluation_grid_build_time: Histogram,
	pub commitment_build_time: Histogram,
	pub grid_rows: Histogram,
	pub grid_cols: Histogram,
}

impl HeaderExtensionBuilderMetrics {
	pub fn new(registry: &Registry) -> Result<Self, PrometheusError> {
		let buckets = [
			25000.0, 100000.0, 250000.0, 500000.0, //  25ms, 100ms, 250ms, 500ms
			1000000.0, 2000000.0, 3000000.0, 4000000.0, // 1s, 2s, 3s, 4s
			5000000.0, 7500000.0, 10000000.0, // 5s, 7.5s, 10s
		];
		let total_execution_time = custom_histogram(
			registry,
			"avail_header_extension_builder_total_execution_time",
			"Header Extension Builder - Total Execution Time in microseconds",
			buckets.to_vec(),
		)?;

		let buckets = [
			150.0, 500.0, 1000.0, 50000.0, 100000.0, // 0.15ms, 0.5ms, 1ms, 50ms, 100ms
			250000.0, 500000.0, 1000000.0, 1500000.0, // 250ms, 500ms, 1s, 1.5s
			2000000.0, 3000000.0, 4000000.0, 5000000.0, // 2s, 3s, 4s, 5s
		];
		let evaluation_grid_build_time = custom_histogram(
			registry,
			"avail_header_extension_builder_evaluation_grid_build_time",
			"Header Extension Builder - Evaluation Grid Build Time in microseconds",
			buckets.to_vec(),
		)?;

		let buckets = [
			25000.0, 100000.0, 250000.0, 500000.0, //  25ms, 100ms, 250ms, 500ms
			1000000.0, 2000000.0, 3000000.0, 4000000.0, // 1s, 2s, 3s, 4s
			5000000.0, 7500000.0, 10000000.0, // 5s, 7.5s, 10s
		];
		let commitment_build_time = custom_histogram(
			registry,
			"avail_header_extension_builder_commitment_build_time",
			"Header Extension Builder - Commitment Build Time in microseconds",
			buckets.to_vec(),
		)?;

		let buckets = [4.0, 6.0, 8.0, 12.0, 16.0, 32.0, 64.0, 128.0, 256.0, 512.0];
		let grid_rows = custom_histogram(
			registry,
			"avail_header_extension_builder_grid_rows",
			"Header Extension Builder - Grid Rows",
			buckets.to_vec(),
		)?;
		let grid_cols = custom_histogram(
			registry,
			"avail_header_extension_builder_grid_cols",
			"Header Extension Builder - Grid Columns",
			buckets.to_vec(),
		)?;

		Ok(Self {
			total_execution_time,
			evaluation_grid_build_time,
			commitment_build_time,
			grid_rows,
			grid_cols,
		})
	}

	pub fn observe_total_execution_time(duration: Duration) {
		if let Some(metrics) = AVAIL_METRICS.get() {
			metrics
				.header_extension
				.total_execution_time
				.observe(duration.as_micros() as f64);
		}
	}

	pub fn observe_evaluation_grid_build_time(duration: Duration) {
		if let Some(metrics) = AVAIL_METRICS.get() {
			metrics
				.header_extension
				.evaluation_grid_build_time
				.observe(duration.as_micros() as f64);
		}
	}

	pub fn observe_commitment_build_time(duration: Duration) {
		if let Some(metrics) = AVAIL_METRICS.get() {
			metrics
				.header_extension
				.commitment_build_time
				.observe(duration.as_micros() as f64);
		}
	}

	pub fn observe_grid_rows(value: f64) {
		if let Some(metrics) = AVAIL_METRICS.get() {
			metrics.header_extension.grid_rows.observe(value);
		}
	}

	pub fn observe_grid_cols(value: f64) {
		if let Some(metrics) = AVAIL_METRICS.get() {
			metrics.header_extension.grid_cols.observe(value);
		}
	}
}

pub struct KateRpcMetrics {
	pub query_rows_execution_time: Histogram,
	pub query_app_data_execution_time: Histogram,
	pub query_proof_execution_time: Histogram,
	pub query_block_length_execution_time: Histogram,
	pub query_data_proof_execution_time: Histogram,
	pub query_data_proof_v2_execution_time: Histogram,
}

impl KateRpcMetrics {
	pub fn new(registry: &Registry) -> Result<Self, PrometheusError> {
		let buckets = [
			1000.0, 5000.0, 10000.0, 25000.0, //  1ms, 5ms, 10ms, 25ms
			50000.0, 75000.0, 100000.0, 150000.0, // 50ms, 75ms, 100ms, 150ms
			200000.0, 300000.0, 400000.0, 500000.0, // 200ms, 300ms, 400ms, 500ms
		];
		let query_rows_execution_time = custom_histogram(
			registry,
			"avail_kate_rpc_query_rows_execution_time",
			"Kate RPC - Query Rows Time in microseconds",
			buckets.to_vec(),
		)?;

		let buckets = [
			1000.0, 10000.0, 25000.0, 50000.0, // 1ms, 10ms, 25ms, 50ms
			75000.0, 100000.0, 150000.0, 200000.0, // 75ms, 100ms, 150ms, 200ms
			300000.0, 500000.0, // 300ms, 500ms
		];
		let query_app_data_execution_time = custom_histogram(
			registry,
			"avail_kate_rpc_query_app_data_execution_time",
			"Kate RPC - Query App Data Time in microseconds",
			buckets.to_vec(),
		)?;

		let buckets = [
			100000.0, 250000.0, 500000.0, 1000000.0, //  100ms, 250ms, 500ms, 1s
			2500000.0, 5000000.0, 7500000.0, 10000000.0, // 2.5s, 5s, 7.5s, 10s
			12500000.0, 15000000.0, 17500000.0, // 12.5s, 15s, 17.5s
		];
		let query_proof_execution_time = custom_histogram(
			registry,
			"avail_kate_rpc_query_proof_execution_time",
			"Kate RPC - Query Proof Time in microseconds",
			buckets.to_vec(),
		)?;

		let buckets = [
			100.0, 200.0, 300.0, 400.0, 500.0, // 0.10ms, 0.20ms, 0.30ms, 0.40ms, 0.50ms,
			750.0, 1000.0, 1250.0, 2500.0, // 0.75ms, 1.0ms, 1.25ms, 2.5ms
			5000.0, 7500.0, 10000.0, // 5ms, 7.5ms, 10ms
		];
		let query_block_length_execution_time = custom_histogram(
			registry,
			"avail_kate_rpc_query_block_length_execution_time",
			"Kate RPC - Query Block Length Time in microseconds",
			buckets.to_vec(),
		)?;

		let buckets = [
			100.0, 250.0, 500.0, 1000.0, 2500.0, // 0.10ms, 0.25ms, 0.5ms, 1ms, 2.5ms,
			5000.0, 7500.0, 10000.0, 25000.0, // 5ms, 7.5ms, 10ms, 25ms
			50000.0, // 50ms
		];
		let query_data_proof_execution_time = custom_histogram(
			registry,
			"avail_kate_rpc_query_data_proof_execution_time",
			"Kate RPC - Query Data Proof Time in microseconds",
			buckets.to_vec(),
		)?;

		let buckets = [
			100.0, 250.0, 500.0, 1000.0, 2500.0, // 0.10ms, 0.25ms, 0.5ms, 1ms, 2.5ms,
			5000.0, 7500.0, 10000.0, 25000.0, // 5ms, 7.5ms, 10ms, 25ms
			50000.0, // 50ms
		];
		let query_data_proof_v2_execution_time = custom_histogram(
			registry,
			"avail_kate_rpc_query_data_proof_v2_execution_time",
			"Kate RPC - Query Data Proof V2 Time in microseconds",
			buckets.to_vec(),
		)?;

		Ok(Self {
			query_rows_execution_time,
			query_app_data_execution_time,
			query_proof_execution_time,
			query_block_length_execution_time,
			query_data_proof_execution_time,
			query_data_proof_v2_execution_time,
		})
	}

	pub fn observe_query_rows_execution_time(duration: Duration) {
		if let Some(metrics) = AVAIL_METRICS.get() {
			metrics
				.kate_rpc
				.query_rows_execution_time
				.observe(duration.as_micros() as f64);
		}
	}

	pub fn observe_query_app_data_execution_time(duration: Duration) {
		if let Some(metrics) = AVAIL_METRICS.get() {
			metrics
				.kate_rpc
				.query_app_data_execution_time
				.observe(duration.as_micros() as f64);
		}
	}

	pub fn observe_query_proof_execution_time(duration: Duration) {
		if let Some(metrics) = AVAIL_METRICS.get() {
			metrics
				.kate_rpc
				.query_proof_execution_time
				.observe(duration.as_micros() as f64);
		}
	}

	pub fn observe_query_block_length_execution_time(duration: Duration) {
		if let Some(metrics) = AVAIL_METRICS.get() {
			metrics
				.kate_rpc
				.query_block_length_execution_time
				.observe(duration.as_micros() as f64);
		}
	}

	pub fn observe_query_data_proof_execution_time(duration: Duration) {
		if let Some(metrics) = AVAIL_METRICS.get() {
			metrics
				.kate_rpc
				.query_data_proof_execution_time
				.observe(duration.as_micros() as f64);
		}
	}

	pub fn observe_query_data_proof_v2_execution_time(duration: Duration) {
		if let Some(metrics) = AVAIL_METRICS.get() {
			metrics
				.kate_rpc
				.query_data_proof_execution_time
				.observe(duration.as_micros() as f64);
		}
	}
}

pub struct ImportBlockMetrics {
	pub total_execution_time: Histogram,
}
impl ImportBlockMetrics {
	pub fn new(registry: &Registry) -> Result<Self, PrometheusError> {
		let buckets = [
			500.0, 1000.0, 50000.0, 100000.0, 250000.0, //  0.5ms, 1ms, 50ms, 100ms, 250ms,
			500000.0, 1000000.0, 2000000.0, 3000000.0, // 500ms, 1s, 2s,  3s
			4000000.0, 5000000.0, 7500000.0, 10000000.0, // 4s, 5s, 7.5s, 10s
		];

		let total_execution_time = custom_histogram(
			registry,
			"avail_import_block_total_execution_time",
			"Import Block - Total Execution Time in microseconds",
			buckets.to_vec(),
		)?;

		Ok(Self {
			total_execution_time,
		})
	}

	pub fn observe_total_execution_time(duration: Duration) {
		if let Some(metrics) = AVAIL_METRICS.get() {
			metrics
				.import_block
				.total_execution_time
				.observe(duration.as_micros() as f64);
		}
	}
}

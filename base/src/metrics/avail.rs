use substrate_prometheus_endpoint::{PrometheusError, Registry};

use crate::metrics::{general::GeneralMetrics, kate::KateMetrics, LOG_TARGET};

/// Avail metrics.
pub struct AvailMetrics {
	pub kate: KateMetrics,
	pub general: GeneralMetrics,
	pub rpc: KateMetrics,
}

impl AvailMetrics {
	/// Creates and registries Avail Metrics.
	pub fn new(registry: &Registry) -> Result<Self, PrometheusError> {
		let kate = KateMetrics::new("kate", registry)?;
		let general = GeneralMetrics::new(registry)?;
		let rpc = KateMetrics::new("rpc", registry)?;

		log::info!(
			target: LOG_TARGET,
			"Prometheus metrics extended with avail metrics"
		);

		Ok(Self { kate, general, rpc })
	}
}

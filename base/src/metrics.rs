use once_cell::sync::OnceCell;
use substrate_prometheus_endpoint::{
	exponential_buckets, register, Histogram, HistogramOpts, Opts, PrometheusError, Registry,
};

const LOG_TARGET: &str = "avail::base::metrics";
pub static AVAIL_METRICS: OnceCell<AvailMetrics> = OnceCell::new();

pub mod avail;
pub use avail::AvailMetrics;
use sp_std::fmt::Display;

/// Creates an histogram using exponential buckets
#[allow(dead_code)]
fn exp_histogram<S: Into<String> + Display + Clone>(
	registry: &Registry,
	name: S,
	help: S,
	start: f64,
	width: f64,
	count: usize,
) -> Result<Histogram, PrometheusError> {
	let histogram = Histogram::with_opts(HistogramOpts {
		common_opts: Opts::new(name.clone(), help),
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
#[allow(dead_code)]
fn linear_histogram<S: Into<String> + Display + Clone>(
	registry: &Registry,
	name: S,
	help: S,
	start: f64,
	width: f64,
	count: usize,
) -> Result<Histogram, PrometheusError> {
	let histogram = Histogram::with_opts(HistogramOpts {
		common_opts: Opts::new(name.clone(), help),
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
#[allow(dead_code)]
fn linear_buckets(start: f64, width: f64, count: usize) -> Vec<f64> {
	(0..count)
		.map(|step| start + width * (step as f64))
		.collect()
}

fn custom_histogram(
	registry: &Registry,
	name: &str,
	help: &str,
	buckets: Vec<f64>,
) -> Result<Histogram, PrometheusError> {
	let histogram = Histogram::with_opts(HistogramOpts {
		common_opts: Opts::new(name.clone(), help),
		buckets,
	})?;
	register(histogram.clone(), registry)?;
	log::trace!(
		target: LOG_TARGET,
		"Added linear metric `{0}` to prometheus",
		name
	);

	Ok(histogram)
}

use crate::BlockDimensions;
use sp_std::time::Duration;

/// Trait for measurements during the header built process.
pub trait Metrics {
	fn extended_block_time(&self, elapsed: Duration);
	fn preparation_block_time(&self, elapsed: Duration);
	fn commitment_build_time(&self, elapsed: Duration);
	fn proof_build_time(&self, elapsed: Duration, cells: u32);
	fn block_dims_and_size(&self, block_dims: &BlockDimensions, block_len: u32);
}

/// Adapter to ignore any measurements.
/// It should be used for testing environments.
#[derive(Clone, Copy, Debug)]
pub struct IgnoreMetrics {}

impl Metrics for IgnoreMetrics {
	fn extended_block_time(&self, _: Duration) {}
	fn preparation_block_time(&self, _: Duration) {}
	fn commitment_build_time(&self, _: Duration) {}
	fn proof_build_time(&self, _: Duration, _: u32) {}
	fn block_dims_and_size(&self, _: &BlockDimensions, _: u32) {}
}

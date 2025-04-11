use sc_telemetry::log;
use std::time::{Duration, Instant};

use super::generate_duration_stats;
pub struct Logger {
	block_fetch: Vec<Duration>,
	timer: Instant,
	timer_interval: Duration,
	name: String,
}

impl Logger {
	pub fn new(name: String, logging_interval: u64) -> Self {
		Self {
			block_fetch: Default::default(),
			timer: Instant::now(),
			timer_interval: Duration::from_secs(logging_interval),
			name,
		}
	}

	pub fn add_block_fetch(&mut self, duration: Duration) {
		self.block_fetch.push(duration);
	}

	pub fn log_stats(&mut self) {
		if self.timer.elapsed() < self.timer_interval {
			return;
		}

		let mut message = String::new();

		if !self.block_fetch.is_empty() {
			let (count, total, min, median, max) = generate_duration_stats(&mut self.block_fetch);
			message = std::format!("Block fetch count: {}, Total Duration: {:.02?}, Min Duration: {:.02?}, Median Duration: {:.02?}, Max Duration: {:.02?}. ", count, total, min, median, max);
		}

		if !message.is_empty() {
			log::info!("👾 {}: {}", self.name, message,);
		}

		self.block_fetch.clear();
		self.block_fetch.shrink_to(25_000);

		self.timer = Instant::now();
	}

	pub fn log_sync_time(&self, duration: Duration) {
		log::info!("👾 {}: Sync time duration: {:.02?}", self.name, duration);
	}

	pub fn log_index_old_blocks_time(&self, duration: Duration) {
		log::info!(
			"👾 {}: Index old blocks duration: {:.02?}",
			self.name,
			duration
		);
	}

	pub fn log_error(&self, message: String) {
		log::warn!("👾 {}: {}", self.name, message);
	}

	pub fn log(&self, message: String) {
		log::info!("👾 {}: {}", self.name, message);
	}
}

use std::time::{Duration, Instant};

use sc_telemetry::log;

use super::generate_duration_stats;

pub struct DatabaseLogging {
	pub rpc_calls: Vec<Duration>,
	pub new_blocks: Vec<Duration>,
	pub timer: Instant,
	pub timer_interval: Duration,
}

impl DatabaseLogging {
	pub fn new(logging_interval: u64) -> Self {
		Self {
			timer: Instant::now(),
			rpc_calls: Default::default(),
			new_blocks: Default::default(),
			timer_interval: Duration::from_secs(logging_interval),
		}
	}
	pub fn add_block(&mut self, duration: Duration) {
		self.new_blocks.push(duration);
	}

	pub fn add_rpc_call(&mut self, duration: Duration) {
		self.rpc_calls.push(duration);
	}

	pub fn log_stats(&mut self) -> bool {
		if self.timer.elapsed() < self.timer_interval {
			return false;
		}

		let mut message = String::new();

		if !self.rpc_calls.is_empty() {
			let (count, total, min, median, max) = generate_duration_stats(&mut self.rpc_calls);
			message = std::format!("RPC call count: {}, Total Duration: {:.02?}, Min Duration: {:.02?}, Median Duration: {:.02?}, Max Duration: {:.02?}. ", count, total, min, median, max);
		}

		if !self.new_blocks.is_empty() {
			let (count, total, min, median, max) = generate_duration_stats(&mut self.new_blocks);
			message = std::format!("{}Block received count: {}, Total Duration: {:.02?}, Min Duration: {:.02?}, Median Duration: {:.02?}, Max Duration: {:.02?}. ", message, count, total, min, median, max)
		}

		if !message.is_empty() {
			log::info!("👾 Database: {}", message,);
		}

		self.rpc_calls.clear();
		self.rpc_calls.shrink_to(25_000);
		self.new_blocks.clear();
		self.new_blocks.shrink_to(25_000);

		self.timer = Instant::now();

		true
	}

	pub fn log_resize(&self, duration: Duration) {
		log::info!("👾 Database: Resize duration {:.02?}", duration);
	}
}

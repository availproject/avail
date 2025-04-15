use crate::workers::Timer;
use sc_telemetry::log;
use std::time::Duration;

pub struct DatabaseLogging {
	blocks_count: u32,
	rpc_calls_count: u32,
	resize_count: u32,
	total_time: Duration,
	resize_time: Duration,
	pub timer: Timer,
}

impl DatabaseLogging {
	pub fn new(logging_interval: u64) -> Self {
		Self {
			blocks_count: 0,
			rpc_calls_count: 0,
			resize_count: 0,
			total_time: Duration::default(),
			resize_time: Duration::default(),
			timer: Timer::new(logging_interval),
		}
	}

	pub fn increment_block(&mut self) {
		self.blocks_count += 1;
	}

	pub fn increment_rpc_call(&mut self) {
		self.rpc_calls_count += 1;
	}

	pub fn increment_total_time(&mut self, value: Duration) {
		self.total_time += value;
	}

	pub fn increment_resize_time(&mut self, value: Duration) {
		self.resize_time += value;
		self.resize_count += 1;
	}

	pub fn log(&mut self) {
		if !self.timer.expired() {
			return;
		}

		let message = std::format!(
			"👾 Database: Total Duration: {} ms, Blocks Received Count: {}, RPC Calls Count: {}, Resize Total Duration: {} ms, Resize Count: {}",
			self.total_time.as_millis(),
			self.blocks_count,
			self.rpc_calls_count,
			self.resize_time.as_millis(),
			self.resize_count,
		);

		log::info!("👾 Database: {}", message,);

		self.blocks_count = 0;
		self.rpc_calls_count = 0;
		self.resize_count = 0;
		self.resize_time = Duration::default();
		self.total_time = Duration::default();

		self.timer.restart();
	}
}

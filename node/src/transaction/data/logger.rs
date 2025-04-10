use std::{ops::AddAssign, time::Duration};

use sc_telemetry::log;

#[derive(Default)]
pub(crate) struct Logger {
	tx_hash: Duration,
	encoded_call: Duration,
	events: Duration,
	total: Duration,
}

impl Logger {
	/* 	pub fn new_tx_hash(&mut self, value: Duration) {
		self.tx_hash.add_assign(value);
	}

	pub fn new_encoded_call(&mut self, value: Duration) {
		self.encoded_call.add_assign(value);
	} */

	pub fn new_events(&mut self, value: Duration) {
		self.events.add_assign(value);
	}

	pub fn new_total(&mut self, value: Duration) {
		self.total.add_assign(value);
	}

	pub fn log(&mut self) {
		log::info!(
			"🐖 Total Duration: {:.02?}. Tx Hash: {:.02?}, Encoded Call: {:.02?}, Events: {:.02?}",
			self.total,
			self.tx_hash,
			self.encoded_call,
			self.events
		);

		self.tx_hash = Duration::default();
		self.encoded_call = Duration::default();
		self.events = Duration::default();
		self.total = Duration::default();
	}
}

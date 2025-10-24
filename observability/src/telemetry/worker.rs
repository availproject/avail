use crate::telemetry::{Receiver, Sender};
use sc_telemetry::{TelemetryHandle, SUBSTRATE_INFO};
use std::thread::JoinHandle;

pub struct Worker {
	pub handle: Option<TelemetryHandle>,
	pub messages: Receiver,
}

impl Worker {
	pub fn new(handle: Option<TelemetryHandle>) -> (Self, Sender) {
		let (tx, rx) = tokio::sync::mpsc::channel(1000);
		let s = Self {
			handle,
			messages: rx,
		};
		(s, tx)
	}

	pub fn spawn_background_task(self) -> JoinHandle<()> {
		std::thread::spawn(|| Worker::task(self))
	}

	fn task(mut self) {
		let Some(handle) = self.handle else { return };

		while let Some(message) = self.messages.blocking_recv() {
			handle.send_telemetry(SUBSTRATE_INFO, message.build());
		}
	}
}

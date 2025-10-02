mod worker;

pub use worker::Worker;

use serde::Serialize;

pub type Sender = tokio::sync::mpsc::Sender<TelemetryMessage>;
pub type Receiver = tokio::sync::mpsc::Receiver<TelemetryMessage>;

#[derive(Debug, Clone)]
pub struct TelemetryMessage {
	// Example: `block.metrics`. This is
	msg: String,
	// Data that will be send to the telemetry server
	data: serde_json::Map<String, serde_json::Value>,
}

impl TelemetryMessage {
	pub fn new(msg: impl Into<String>) -> TelemetryMessage {
		Self {
			msg: msg.into(),
			data: Default::default(),
		}
	}

	pub fn push(
		&mut self,
		field_name: impl Into<String>,
		field_data: impl Serialize,
	) -> Result<(), serde_json::Error> {
		let field_name: String = field_name.into();
		let field_data = serde_json::to_value(field_data)?;
		self.data.insert(field_name, field_data);
		Ok(())
	}

	pub fn build(mut self) -> serde_json::Map<String, serde_json::Value> {
		self.data
			.insert("msg".into(), serde_json::Value::String(self.msg));
		self.data
	}
}

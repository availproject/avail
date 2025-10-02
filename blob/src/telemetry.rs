use avail_telemetry::TelemetryMessage;
use sp_core::H256;

#[derive(Debug, Clone)]
pub struct BlobReceived {
	pub size: usize,
	pub hash: H256,
	// Milliseconds past UNIX EPOCH
	pub timestamp: String,
}

impl From<BlobReceived> for TelemetryMessage {
	fn from(value: BlobReceived) -> Self {
		let mut msg = TelemetryMessage::new("blob.received");
		msg.push("size", value.size).unwrap();
		msg.push("hash", value.hash).unwrap();
		msg.push("timestamp", value.timestamp).unwrap();
		msg
	}
}

#[derive(Debug, Clone)]
pub struct BlobAddedToPool {
	pub size: usize,
	pub hash: H256,
	// Milliseconds past UNIX EPOCH
	pub timestamp: String,
}

impl From<BlobAddedToPool> for TelemetryMessage {
	fn from(value: BlobAddedToPool) -> Self {
		let mut msg = TelemetryMessage::new("blob.addedToPool");
		msg.push("size", value.size).unwrap();
		msg.push("hash", value.hash).unwrap();
		msg.push("timestamp", value.timestamp).unwrap();
		msg
	}
}

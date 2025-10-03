use avail_telemetry::TelemetryMessage;
use sp_core::H256;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

#[derive(Clone)]
pub struct TelemetryOperator {
	channel: Option<avail_telemetry::Sender>,
}

impl TelemetryOperator {
	pub fn new(channel: Option<avail_telemetry::Sender>) -> Self {
		Self { channel }
	}

	pub fn blob_received(&self, size: usize, hash: H256) {
		let Some(channel) = self.channel.as_ref() else {
			return;
		};

		let timestamp = SystemTime::now()
			.duration_since(UNIX_EPOCH)
			.unwrap()
			.as_millis();

		let msg = BlobReceived {
			size,
			hash,
			timestamp: std::format!("{}", timestamp),
		};
		_ = channel.try_send(msg.into());
	}

	pub fn blob_added_to_pool(&self, size: usize, hash: H256) {
		let Some(channel) = self.channel.as_ref() else {
			return;
		};

		let timestamp = SystemTime::now()
			.duration_since(UNIX_EPOCH)
			.unwrap()
			.as_millis();

		let msg = BlobReceived {
			size,
			hash,
			timestamp: std::format!("{}", timestamp),
		};
		_ = channel.try_send(msg.into());
	}

	pub fn blob_compression(&self, org_size: usize, new_size: usize, hash: H256) {
		let Some(channel) = self.channel.as_ref() else {
			return;
		};

		let msg = BlobCompression {
			org_size,
			new_size,
			hash,
		};
		_ = channel.try_send(msg.into());
	}
}

#[derive(Debug, Clone)]
struct BlobReceived {
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
struct BlobAddedToPool {
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

#[derive(Debug, Clone)]
struct BlobCompression {
	pub org_size: usize,
	pub new_size: usize,
	pub hash: H256,
}

impl From<BlobCompression> for TelemetryMessage {
	fn from(value: BlobCompression) -> Self {
		let mut msg = TelemetryMessage::new("blob.compression");
		msg.push("org_size", value.org_size).unwrap();
		msg.push("new_size", value.new_size).unwrap();
		msg.push("hash", value.hash).unwrap();
		msg
	}
}

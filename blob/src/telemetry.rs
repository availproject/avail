use crate::utils::get_current_timestamp_ms;
use avail_observability::telemetry::send_telemetry;
use avail_observability::telemetry::TelemetryMessage;
use sp_core::H256;
use std::time::Duration;

pub fn blob_received(size: usize, hash: H256) {
	let timestamp = get_current_timestamp_ms();

	let msg = BlobReceived {
		size,
		hash,
		timestamp,
	};

	send_telemetry(msg.into());
}

pub fn blob_added_to_pool(size: usize, hash: H256) {
	let timestamp = get_current_timestamp_ms();

	let msg = BlobAddedToPool {
		size,
		hash,
		timestamp,
	};

	send_telemetry(msg.into());
}

pub fn blob_compression(org_size: usize, new_size: usize, hash: H256, duration: Duration) {
	let msg = BlobCompression {
		org_size,
		new_size,
		hash,
		duration: duration.as_millis(),
	};

	send_telemetry(msg.into());
}

pub fn blob_poly_grid(hash: H256, start: u128, end: u128) {
	let msg = BlobPolyGrid { hash, start, end };

	send_telemetry(msg.into());
}

pub fn blob_commitment(hash: H256, start: u128, end: u128, queue_size: usize) {
	let msg = BlobCommitment {
		hash,
		start,
		end,
		queue_size,
	};

	send_telemetry(msg.into());
}

pub fn blob_request(
	size: usize,
	hash: H256,
	start: u128,
	end: u128,
	from: String,
	to: String,
	success: bool,
) {
	let msg = BlobRequest {
		size,
		hash,
		start,
		end,
		from,
		to,
		success,
	};

	send_telemetry(msg.into());
}

pub fn blob_dropped(hash: Option<H256>, queue_full: bool) {
	let msg = BlobDropped { hash, queue_full };
	send_telemetry(msg.into());
}

#[derive(Debug, Clone)]
struct BlobReceived {
	pub size: usize,
	pub hash: H256,
	// Milliseconds past UNIX EPOCH
	pub timestamp: u128,
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
	pub timestamp: u128,
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
	// Duration in ms
	pub duration: u128,
}
impl From<BlobCompression> for TelemetryMessage {
	fn from(value: BlobCompression) -> Self {
		let mut msg = TelemetryMessage::new("blob.compression");
		msg.push("org_size", value.org_size).unwrap();
		msg.push("new_size", value.new_size).unwrap();
		msg.push("hash", value.hash).unwrap();
		msg.push("duration", value.duration).unwrap();
		msg
	}
}

#[derive(Debug, Clone)]
struct BlobPolyGrid {
	pub hash: H256,
	// Milliseconds past UNIX EPOCH
	pub start: u128,
	// Milliseconds past UNIX EPOCH
	pub end: u128,
}
impl From<BlobPolyGrid> for TelemetryMessage {
	fn from(value: BlobPolyGrid) -> Self {
		let mut msg = TelemetryMessage::new("blob.polygrid");
		msg.push("hash", value.hash).unwrap();
		msg.push("start", value.start).unwrap();
		msg.push("end", value.end).unwrap();
		msg
	}
}

#[derive(Debug, Clone)]
struct BlobCommitment {
	pub hash: H256,
	// Milliseconds past UNIX EPOCH
	pub start: u128,
	// Milliseconds past UNIX EPOCH
	pub end: u128,
	pub queue_size: usize,
}
impl From<BlobCommitment> for TelemetryMessage {
	fn from(value: BlobCommitment) -> Self {
		let mut msg = TelemetryMessage::new("blob.commitment");
		msg.push("hash", value.hash).unwrap();
		msg.push("start", value.start).unwrap();
		msg.push("end", value.end).unwrap();
		msg.push("queue_size", value.queue_size).unwrap();
		msg
	}
}

#[derive(Debug, Clone)]
struct BlobRequest {
	pub size: usize,
	pub hash: H256,
	pub start: u128,
	pub end: u128,
	pub from: String,
	pub to: String,
	pub success: bool,
}
impl From<BlobRequest> for TelemetryMessage {
	fn from(value: BlobRequest) -> Self {
		let mut msg = TelemetryMessage::new("blob.request");
		msg.push("size", value.size).unwrap();
		msg.push("hash", value.hash).unwrap();
		msg.push("start", value.start).unwrap();
		msg.push("end", value.end).unwrap();
		msg.push("from", value.from).unwrap();
		msg.push("to", value.to).unwrap();
		msg.push("success", value.success).unwrap();
		msg
	}
}

#[derive(Debug, Clone)]
struct BlobDropped {
	pub hash: Option<H256>,
	pub queue_full: bool,
}
impl From<BlobDropped> for TelemetryMessage {
	fn from(value: BlobDropped) -> Self {
		let mut msg = TelemetryMessage::new("blob.dropped");
		msg.push("hash", value.hash).unwrap();
		msg.push("queue_full", value.queue_full).unwrap();
		msg
	}
}

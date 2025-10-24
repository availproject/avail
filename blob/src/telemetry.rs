use crate::utils::get_current_timestamp_ms;
use avail_observability::telemetry::send_telemetry;
use avail_observability::telemetry::TelemetryMessage;
use sp_core::H256;
use std::time::Duration;

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

pub struct BlobSubmission {
	pub hash: H256,
	pub size: Option<u64>,
	// Milliseconds past UNIX EPOCH
	pub submission_tracked: Option<u128>,
	// Milliseconds past UNIX EPOCH
	pub added_to_pool_timestamp: Option<u128>,
	// In bytes
	pub compression_size: Option<u128>,
	// In ms
	pub compression_duration: Option<u128>,
	// Milliseconds past UNIX EPOCH
	pub poly_grid_build_start_timestamp: Option<u128>,
	// Milliseconds past UNIX EPOCH
	pub poly_grid_build_end_timestamp: Option<u128>,
	// 0 means it was full
	pub queue_capacity: Option<u64>,
	// At what point in time did we measure the queue capacity
	pub queue_capacity_timestamp: Option<u128>,
	// Milliseconds past UNIX EPOCH
	pub commitment_grid_build_start_timestamp: Option<u128>,
	// Milliseconds past UNIX EPOCH
	pub commitment_grid_build_end_timestamp: Option<u128>,
}
impl From<BlobSubmission> for TelemetryMessage {
	fn from(value: BlobSubmission) -> Self {
		let mut msg = TelemetryMessage::new("blob.submission");
		msg.push("hash", value.hash).unwrap();

		if let Some(size) = value.size {
			msg.push("size", size).unwrap();
		}
		if let Some(submission_tracked) = value.submission_tracked {
			msg.push("submission_tracked", submission_tracked).unwrap();
		}
		if let Some(added_to_pool_timestamp) = value.added_to_pool_timestamp {
			msg.push("added_to_pool_timestamp", added_to_pool_timestamp)
				.unwrap();
		}
		if let Some(compression_size) = value.compression_size {
			msg.push("compression_size", compression_size).unwrap();
		}
		if let Some(compression_duration) = value.compression_duration {
			msg.push("compression_duration", compression_duration)
				.unwrap();
		}
		if let Some(poly_grid_build_start_timestamp) = value.poly_grid_build_start_timestamp {
			msg.push(
				"poly_grid_build_start_timestamp",
				poly_grid_build_start_timestamp,
			)
			.unwrap();
		}
		if let Some(poly_grid_build_end_timestamp) = value.poly_grid_build_end_timestamp {
			msg.push(
				"poly_grid_build_end_timestamp",
				poly_grid_build_end_timestamp,
			)
			.unwrap();
		}
		if let Some(queue_capacity) = value.queue_capacity {
			msg.push("queue_capacity", queue_capacity).unwrap();
		}
		if let Some(queue_capacity_timestamp) = value.queue_capacity_timestamp {
			msg.push("queue_capacity_timestamp", queue_capacity_timestamp)
				.unwrap();
		}
		if let Some(commitment_grid_build_start_timestamp) =
			value.commitment_grid_build_start_timestamp
		{
			msg.push(
				"commitment_grid_build_start_timestamp",
				commitment_grid_build_start_timestamp,
			)
			.unwrap();
		}
		if let Some(commitment_grid_build_end_timestamp) = value.commitment_grid_build_end_timestamp
		{
			msg.push(
				"commitment_grid_build_end_timestamp",
				commitment_grid_build_end_timestamp,
			)
			.unwrap();
		}

		msg
	}
}

impl BlobSubmission {
	pub fn new(hash: H256) -> Self {
		Self {
			hash,
			size: None,
			submission_tracked: None,
			added_to_pool_timestamp: None,
			compression_size: None,
			compression_duration: None,
			poly_grid_build_start_timestamp: None,
			poly_grid_build_end_timestamp: None,
			queue_capacity: None,
			queue_capacity_timestamp: None,
			commitment_grid_build_start_timestamp: None,
			commitment_grid_build_end_timestamp: None,
		}
	}

	pub fn submission_tracked(hash: H256, size: usize) {
		let timestamp = get_current_timestamp_ms();

		let mut msg = BlobSubmission::new(hash);
		msg.size = Some(size as u64);
		msg.submission_tracked = Some(timestamp);

		send_telemetry(msg.into());
	}

	pub fn added_to_pool(hash: H256) {
		let timestamp = get_current_timestamp_ms();

		let mut msg = BlobSubmission::new(hash);
		msg.added_to_pool_timestamp = Some(timestamp);

		send_telemetry(msg.into());
	}

	pub fn compression(hash: H256, size: usize, duration: Duration) {
		let mut msg = BlobSubmission::new(hash);
		msg.compression_size = Some(size as u128);
		msg.compression_duration = Some(duration.as_millis());

		send_telemetry(msg.into());
	}

	pub fn build_poly_grid(hash: H256, start: u128, end: u128) {
		let mut msg = BlobSubmission::new(hash);
		msg.poly_grid_build_start_timestamp = Some(start);
		msg.poly_grid_build_end_timestamp = Some(end);

		send_telemetry(msg.into());
	}

	pub fn build_commitment(hash: H256, start: u128, end: u128) {
		let mut msg = BlobSubmission::new(hash);
		msg.commitment_grid_build_start_timestamp = Some(start);
		msg.commitment_grid_build_end_timestamp = Some(end);

		send_telemetry(msg.into());
	}

	pub fn queue_capacity(hash: H256, capacity: usize) {
		let timestamp = get_current_timestamp_ms();

		let mut msg = BlobSubmission::new(hash);
		msg.queue_capacity = Some(capacity as u64);
		msg.queue_capacity_timestamp = Some(timestamp);

		send_telemetry(msg.into());
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

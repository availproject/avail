use std::io;

pub struct DaSamplingProtocolSpec {
	pub protocol_name: sc_network::types::ProtocolName,
	pub max_request_size: u64,
	pub max_response_size: u64,
	pub request_timeout: std::time::Duration,
	pub inbound_queue: usize,
}

/// DA sampling protocol errors
#[derive(Debug, thiserror::Error)]
pub enum SamplingError {
	/// Failed to send response back to requester (server-side)
	#[error("Failed to send DA sampling response to peer")]
	SendResponse,

	/// Generic request-level failure with context
	#[error("DA sampling request failed: {reason}")]
	RequestFailure { reason: String },

	/// Failed to decode a sampling response
	#[error("Failed to decode DA sampling response: {0}")]
	ResponseDecode(#[from] prost::DecodeError),

	/// Failed to encode request or response
	#[error("Failed to encode DA sampling message: {0}")]
	Encode(#[from] prost::EncodeError),

	/// Underlying I/O error (network, channel, etc.)
	#[error("I/O error during DA sampling: {0}")]
	Io(#[from] io::Error),

	/// Runtime API failure (block/blob lookup, etc.)
	#[error("Runtime API error during DA sampling: {0}")]
	Api(#[from] sp_api::ApiError),

	/// Cryptographic verification failed
	#[error("DA sampling verification failed (invalid proof or corrupted data)")]
	VerificationFailed,

	/// Request timed out waiting for peer response
	#[error("DA sampling request timed out waiting for peer response")]
	Timeout,

	/// No peers (or owners) available to serve the request
	#[error("No eligible peers available for DA sampling")]
	NoPeersAvailable,

	/// Cell length is invalid (expected 16 bytes for B128)
	#[error("Invalid cell length: expected 16 bytes, got {actual}")]
	InvalidCellLength { actual: usize },
}

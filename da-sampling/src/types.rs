use sp_core::H256;
use std::io;

#[derive(Clone, Debug)]
pub struct SamplingContext {
	pub block_hash: H256,
	pub blob_hash: H256,
	pub cell_indices: Vec<u32>,
}

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
	// #[error(transparent)]
	// Client(#[from] sp_blockchain::Error),
	#[error("Failed to send response.")]
	SendResponse,

	#[error("Request failed: {0}")]
	RequestFailure(String),

	#[error("Response decode error: {0}")]
	ResponseDecode(#[from] prost::DecodeError),

	#[error(transparent)]
	Encode(#[from] prost::EncodeError),

	#[error(transparent)]
	Io(#[from] io::Error),

	#[error(transparent)]
	Api(#[from] sp_api::ApiError),

	#[error("Verification failed for blob")]
	VerificationFailed,

	#[error("Operation timed out")]
	Timeout,

	#[error("No peers available")]
	NoPeersAvailable,

	#[error("Invalid cell length")]
	InvalidCellLength,
}

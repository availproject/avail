use avail_fri::SamplingProof;
use sc_chain_spec::ChainSpec;
use sc_network::types::ProtocolName;
pub mod client;
pub mod codec;
pub mod schema;
pub mod server;
pub mod types;

pub use schema::v1::da_sampling::{CellProof, DaSamplingRequest, DaSamplingResponse};
pub const DA_SAMPLING_PROTOCOL: &str = "/avail/da-sampling/1";

use std::time::Duration;

use crate::types::{DaSamplingProtocolSpec, SamplingError};

pub const MAX_PACKET_SIZE: u64 = 16 * 1024 * 1024;
pub const MAX_REQUEST_QUEUE: usize = 100;
pub const REQUEST_TIMEOUT: Duration = Duration::from_secs(30);

fn get_protocol_name<Hash: AsRef<[u8]>>(
	genesis_hash: Hash,
	chain_spec: &Box<dyn ChainSpec>,
) -> ProtocolName {
	let genesis_hash = genesis_hash.as_ref();
	let genesis_hash_hex = const_hex::encode(genesis_hash);
	let chain_prefix = match chain_spec.fork_id() {
		Some(fork_id) => format!("/{}/{}", genesis_hash_hex, fork_id),
		None => format!("/{}", genesis_hash_hex),
	};
	format!("{}{}", chain_prefix, DA_SAMPLING_PROTOCOL).into()
}

pub fn protocol_spec(
	genesis_hash: &[u8],
	chain_spec: &Box<dyn ChainSpec>,
) -> DaSamplingProtocolSpec {
	DaSamplingProtocolSpec {
		protocol_name: get_protocol_name(genesis_hash, chain_spec),
		max_request_size: MAX_PACKET_SIZE,
		max_response_size: MAX_PACKET_SIZE,
		request_timeout: REQUEST_TIMEOUT,
		inbound_queue: MAX_REQUEST_QUEUE,
	}
}

pub fn cellproof_to_samplingproof(p: CellProof) -> Result<SamplingProof, SamplingError> {
	if p.cell.is_empty() || !p.cell.len().is_multiple_of(16) {
		return Err(SamplingError::InvalidCellLength {
			actual: p.cell.len(),
		});
	}

	Ok(SamplingProof {
		index: p.index,
		cell: p.cell,
		proof: p.proof,
	})
}

pub fn response_to_samplingproofs(
	proofs: Vec<CellProof>,
) -> Result<Vec<SamplingProof>, SamplingError> {
	proofs.into_iter().map(cellproof_to_samplingproof).collect()
}

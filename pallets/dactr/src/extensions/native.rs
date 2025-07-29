use da_commitment::build_da_commitments;
use frame_system::limits::BlockLength;
use kate::Seed;
use sp_runtime_interface::runtime_interface;
use sp_std::vec::Vec;
use sp_runtime::SaturatedConversion;

pub type DaCommitments = Vec<u8>;

/// Hosted function to build the DA commitments.
#[runtime_interface]
pub trait HostedCommitmentBuilder {
	fn build_da_commitments(data: Vec<u8>, block_length: BlockLength, seed: Seed) -> DaCommitments {
		let cols: usize = block_length.cols.0.saturated_into();
		let rows: usize = block_length.rows.0.saturated_into();
		build_da_commitments::build_da_commitments(data, cols, rows, seed)
	}
}

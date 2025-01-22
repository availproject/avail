use avail_core::DaCommitments;
use frame_system::limits::BlockLength;
use kate::Seed;
use sp_runtime_interface::runtime_interface;
use sp_std::vec::Vec;
pub mod build_da_commitments;

/// Hosted function to build the DA commitments.
#[runtime_interface]
pub trait HostedCommitmentBuilder {
	fn build_da_commitments(data: Vec<u8>, block_length: BlockLength, seed: Seed) -> DaCommitments {
		build_da_commitments::build_da_commitments(data, block_length, seed)
	}
}

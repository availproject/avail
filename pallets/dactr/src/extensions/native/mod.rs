use frame_system::limits::BlockLength;
use kate::Seed;
use sp_runtime_interface::runtime_interface;
use sp_std::vec::Vec;
pub mod build_da_commitments;

pub type DaCommitments = Vec<u8>;

/// Hosted function to build the DA commitments.
#[runtime_interface]
pub trait HostedCommitmentBuilder {
	fn build_da_commitments(data: Vec<u8>, block_length: BlockLength, seed: Seed) -> DaCommitments {
		build_da_commitments::build_da_commitments(data, block_length, seed)
	}

	fn verify_multiproof(
		data: Vec<u8>,
		block_length: BlockLength,
		seed: Seed,
		commitments: Vec<u8>,
		proof: &[u8; 48],
	) -> bool {
		build_da_commitments::verify_multiproof(data, block_length, seed, commitments, proof)
	}
}

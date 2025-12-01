use kate::Seed;
use sp_runtime::SaturatedConversion;
use sp_runtime_interface::runtime_interface;
use sp_std::vec::Vec;

pub type DaCommitments = Vec<u8>;

/// Hosted function to build the DA commitments.
#[runtime_interface]
pub trait HostedCommitmentBuilder {
	fn build_da_commitments(data: &[u8], cols: u32, rows: u32, seed: Seed) -> DaCommitments {
		let cols: usize = cols.saturated_into();
		let rows: usize = rows.saturated_into();
		da_commitment::build_kzg_commitments::build_da_commitments(data, cols, rows, seed)
	}
}

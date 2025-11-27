use kate::Seed;
use sp_runtime::SaturatedConversion;
use sp_runtime_interface::{
	pass_by::{AllocateAndReturnFatPointer, PassFatPointerAndDecode, PassFatPointerAndRead},
	runtime_interface,
};
use sp_std::vec::Vec;

pub type DaCommitments = AllocateAndReturnFatPointer<Vec<u8>>;

/// Hosted function to build the DA commitments.
#[runtime_interface]
pub trait HostedCommitmentBuilder {
	fn build_da_commitments(
		data: PassFatPointerAndRead<&[u8]>,
		cols: u32,
		rows: u32,
		seed: PassFatPointerAndDecode<Seed>,
	) -> DaCommitments {
		let cols: usize = cols.saturated_into();
		let rows: usize = rows.saturated_into();
		#[cfg(feature = "std")]
		{
			da_commitment::build_da_commitments::build_da_commitments(data, cols, rows, seed)
		}
		#[cfg(not(feature = "std"))]
		{
			// one should never reach here
			Vec::new()
		}
	}
}

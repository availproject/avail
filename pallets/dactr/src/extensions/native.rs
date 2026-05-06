use avail_core::FriParamsVersion;
use sp_runtime_interface::{
	pass_by::{AllocateAndReturnFatPointer, PassFatPointerAndDecode, PassFatPointerAndRead},
	runtime_interface,
};
use sp_std::vec::Vec;

pub type DaCommitments = AllocateAndReturnFatPointer<Vec<u8>>;

/// Hosted function to build the DA commitments.
#[runtime_interface]
pub trait HostedCommitmentBuilder {
	fn build_fri_commitments(
		data: PassFatPointerAndRead<&[u8]>,
		params_version: PassFatPointerAndDecode<FriParamsVersion>,
	) -> DaCommitments {
		da_commitment::build_fri_commitments::build_fri_da_commitment(data, params_version)
	}
}

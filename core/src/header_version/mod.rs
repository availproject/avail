use codec::{Decode, Encode};
use scale_info::TypeInfo;
#[cfg(feature = "runtime")]
use sp_runtime_interface::pass_by::PassByCodec;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "runtime", derive(PassByCodec))]
pub enum HeaderVersion {
	V1,
	V2, // Current one
	V3, // To be used after removing padding_tail_value
}

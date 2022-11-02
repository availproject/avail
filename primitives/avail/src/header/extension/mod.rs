use codec::{Decode, Encode};
#[cfg(feature = "std")]
use parity_util_mem::{MallocSizeOf, MallocSizeOfOps};
use scale_info::TypeInfo;
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_core::{RuntimeDebug, H256};
use sp_runtime_interface::pass_by::PassByCodec;

pub mod v1;

#[cfg(feature = "header-backward-compatibility-test")]
pub mod v_test;

/// Header extension data.
#[derive(PartialEq, Eq, Clone, RuntimeDebug, TypeInfo, Encode, Decode, PassByCodec)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum HeaderExtension {
	V1(v1::HeaderExtension),
	#[cfg(feature = "header-backward-compatibility-test")]
	VTest(v_test::HeaderExtension),
}

/// It forwards the call to the inner version of the header. Any invalid version will return the
/// default value or execute an empty block.
macro_rules! forward_to_version {
	($self:ident, $function:ident) => {{
		match $self {
			HeaderExtension::V1(header) => header.$function(),
			#[cfg(feature = "header-backward-compatibility-test")]
			HeaderExtension::VTest(header) => header.$function(),
		}
	}};

	($self:ident, $function:ident, $arg:expr) => {{
		match $self {
			HeaderExtension::V1(header) => header.$function($arg),
			#[cfg(feature = "header-backward-compatibility-test")]
			HeaderExtension::VTest(header) => header.$function($arg),
		}
	}};
}

impl HeaderExtension {
	pub fn data_root(&self) -> H256 { forward_to_version!(self, data_root) }
}

impl Default for HeaderExtension {
	fn default() -> Self { v1::HeaderExtension::default().into() }
}

#[cfg(feature = "std")]
impl MallocSizeOf for HeaderExtension {
	fn size_of(&self, ops: &mut MallocSizeOfOps) -> usize {
		forward_to_version!(self, size_of, ops)
	}
}

impl From<v1::HeaderExtension> for HeaderExtension {
	#[inline]
	fn from(ext: v1::HeaderExtension) -> Self { Self::V1(ext) }
}

#[cfg(feature = "header-backward-compatibility-test")]
impl From<v_test::HeaderExtension> for HeaderExtension {
	#[inline]
	fn from(ext: v_test::HeaderExtension) -> Self { Self::VTest(ext) }
}

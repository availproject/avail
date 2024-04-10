use codec::{Decode, Encode};
use scale_info::TypeInfo;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use sp_core::{RuntimeDebug, H256};
#[cfg(feature = "runtime")]
use sp_runtime_interface::pass_by::PassByCodec;

use crate::{DataLookup, HeaderVersion};

pub mod v3;

/// Header extension data.
#[derive(PartialEq, Eq, Clone, RuntimeDebug, TypeInfo, Encode, Decode)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "runtime", derive(PassByCodec))]
#[repr(u8)]
pub enum HeaderExtension {
	V3(v3::HeaderExtension) = 2,
}

/// It forwards the call to the inner version of the header. Any invalid version will return the
/// default value or execute an empty block.
macro_rules! forward_to_version {
	($self:ident, $function:ident) => {{
		match $self {
			HeaderExtension::V3(ext) => ext.$function(),
		}
	}};

	($self:ident, $function:ident, $arg:expr) => {{
		match $self {
			HeaderExtension::V3(ext) => ext.$function($arg),
		}
	}};
}

impl HeaderExtension {
	pub fn data_root(&self) -> H256 {
		forward_to_version!(self, data_root)
	}

	pub fn app_lookup(&self) -> &DataLookup {
		forward_to_version!(self, app_lookup)
	}

	pub fn rows(&self) -> u16 {
		forward_to_version!(self, rows)
	}

	pub fn cols(&self) -> u16 {
		forward_to_version!(self, cols)
	}

	pub fn get_empty_header(data_root: H256, version: HeaderVersion) -> HeaderExtension {
		match version {
			HeaderVersion::V3 => v3::HeaderExtension::get_empty_header(data_root).into(),
		}
	}

	pub fn get_faulty_header(data_root: H256, version: HeaderVersion) -> HeaderExtension {
		match version {
			HeaderVersion::V3 => v3::HeaderExtension::get_faulty_header(data_root).into(),
		}
	}

	pub fn get_header_version(&self) -> HeaderVersion {
		match self {
			HeaderExtension::V3(_) => HeaderVersion::V3,
		}
	}
}

impl Default for HeaderExtension {
	fn default() -> Self {
		v3::HeaderExtension::default().into()
	}
}

impl From<v3::HeaderExtension> for HeaderExtension {
	#[inline]
	fn from(ext: v3::HeaderExtension) -> Self {
		Self::V3(ext)
	}
}

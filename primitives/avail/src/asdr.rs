use codec::{Decode, Encode, MaxEncodedLen};
use derive_more::{Add, Deref, Display, From, Into};
use frame_support::RuntimeDebug;
#[cfg(feature = "std")]
use parity_util_mem::{MallocSizeOf, MallocSizeOfOps};
use scale_info::TypeInfo;
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_runtime::traits::Zero;
use sp_std::vec::Vec;

mod data_lookup;
pub use data_lookup::*;

mod get_app_id;
pub use get_app_id::*;

mod app_unchecked_extrinsic;
pub use app_unchecked_extrinsic::*;

#[derive(
	Clone,
	Copy,
	PartialEq,
	Eq,
	PartialOrd,
	Ord,
	Add,
	From,
	Deref,
	TypeInfo,
	RuntimeDebug,
	Encode,
	Decode,
	Display,
	Into,
	Default,
	MaxEncodedLen,
)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct AppId(#[codec(compact)] pub u32);

#[cfg(feature = "std")]
impl MallocSizeOf for AppId {
	fn size_of(&self, ops: &mut MallocSizeOfOps) -> usize {
		self.0.size_of(ops)
	}
}

impl Zero for AppId {
	fn zero() -> Self {
		0u32.into()
	}

	fn is_zero(&self) -> bool {
		self.0 == 0u32
	}

	fn set_zero(&mut self) {
		self.0 = 0u32;
	}
}

/// Raw Extrinsic with application id.
#[derive(Clone, TypeInfo, RuntimeDebug, Default, Encode, Decode)]
pub struct AppExtrinsic {
	pub app_id: AppId,
	pub data: Vec<u8>,
}

impl From<Vec<u8>> for AppExtrinsic {
	#[inline]
	fn from(data: Vec<u8>) -> Self {
		Self {
			data,
			app_id: <_>::default(),
		}
	}
}

impl GetAppId for AppExtrinsic {
	fn app_id(&self) -> AppId {
		self.app_id
	}
}

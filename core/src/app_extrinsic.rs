use crate::{traits::GetAppId, AppId};

use codec::{Decode, Encode};
use derive_more::Constructor;
use scale_info::TypeInfo;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use sp_core::RuntimeDebug;
use sp_std::vec::Vec;

/// Raw Extrinsic with application id.
/// TODO:
/// - [ ] Support `opaque` as ref?
#[derive(Clone, TypeInfo, Default, Encode, Decode, RuntimeDebug, Constructor)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AppExtrinsic {
	pub app_id: AppId,
	#[cfg_attr(feature = "serde", serde(skip))]
	pub tx_idx: u32,
	#[cfg_attr(feature = "serde", serde(with = "hex", alias = "data"))]
	pub opaque: Vec<u8>,
}

impl GetAppId for AppExtrinsic {
	fn app_id(&self) -> AppId {
		self.app_id
	}
}

impl From<Vec<u8>> for AppExtrinsic {
	fn from(opaque: Vec<u8>) -> Self {
		Self::new(AppId::default(), 0, opaque)
	}
}

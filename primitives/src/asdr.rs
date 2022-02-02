use codec::{Decode, Encode};
use frame_support::RuntimeDebug;
use scale_info::TypeInfo;
use sp_std::vec::Vec;

mod data_lookup;
pub use data_lookup::*;

mod get_app_id;
pub use get_app_id::*;

mod app_unchecked_extrinsic;
pub use app_unchecked_extrinsic::*;

pub type AppId = u32;

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

impl GetAppId<AppId> for AppExtrinsic {
	fn app_id(&self) -> AppId { self.app_id }
}

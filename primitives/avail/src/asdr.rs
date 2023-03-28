use codec::Encode;
pub use da_types::{AppExtrinsic, AppId, DataLookup, DataLookupIndexItem};
use sp_runtime::traits::SignedExtension;

mod get_app_id;
pub use get_app_id::*;

mod app_unchecked_extrinsic;
pub use app_unchecked_extrinsic::*;

impl<A: Encode, C: Encode, S: Encode, E> From<AppUncheckedExtrinsic<A, C, S, E>> for AppExtrinsic
where
	E: SignedExtension + GetAppId,
{
	fn from(app_ext: AppUncheckedExtrinsic<A, C, S, E>) -> Self {
		Self {
			app_id: app_ext.app_id(),
			data: app_ext.encode(),
		}
	}
}

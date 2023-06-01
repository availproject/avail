use codec::Encode;
pub use da_types::{AppExtrinsic, AppId, DataLookup, DataLookupIndexItem, GetAppId};
use sp_runtime::traits::SignedExtension;

mod app_unchecked_extrinsic;
pub use app_unchecked_extrinsic::*;

impl<A, C, S, E> From<&AppUncheckedExtrinsic<A, C, S, E>> for AppExtrinsic
where
	A: Encode,
	C: Encode,
	S: Encode,
	E: SignedExtension + GetAppId,
{
	fn from(app_ext: &AppUncheckedExtrinsic<A, C, S, E>) -> Self {
		Self {
			app_id: app_ext.app_id(),
			data: app_ext.encode(),
		}
	}
}

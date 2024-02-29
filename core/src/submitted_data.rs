use crate::{traits::GetAppId, AppId};

use derive_more::Constructor;

#[derive(Constructor, Debug, Clone, Copy)]
pub struct SubmittedData<'a> {
	pub id: AppId,
	pub data: &'a [u8],
}

impl<'a> GetAppId for SubmittedData<'a> {
	fn app_id(&self) -> AppId {
		self.id
	}
}

impl<'a> From<&'a [u8]> for SubmittedData<'a> {
	fn from(opaque: &'a [u8]) -> Self {
		Self::new(AppId(0), opaque)
	}
}

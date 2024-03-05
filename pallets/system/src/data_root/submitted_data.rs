use avail_core::{traits::GetAppId, AppId};

use codec::{Decode, Encode};
use derive_more::Constructor;
use sp_std::vec::Vec;

#[derive(Constructor, Debug)]
pub struct SubmittedDataRef<'a> {
	pub id: AppId,
	pub data: &'a [u8],
}

impl SubmittedDataRef<'_> {
	pub fn to_owned(&self) -> SubmittedData {
		SubmittedData::new(self.id, self.data.to_vec())
	}
}

impl<'a> GetAppId for SubmittedDataRef<'a> {
	fn app_id(&self) -> AppId {
		self.id
	}
}

impl<'a> From<&'a [u8]> for SubmittedDataRef<'a> {
	fn from(opaque: &'a [u8]) -> Self {
		Self::new(AppId(0), opaque)
	}
}

#[derive(Debug, Constructor, Encode, Decode)]
pub struct SubmittedData {
	pub id: AppId,
	pub data: Vec<u8>,
}

impl SubmittedData {
	pub fn to_ref(&self) -> SubmittedDataRef<'_> {
		SubmittedDataRef::new(self.id, &self.data)
	}
}

impl GetAppId for SubmittedData {
	fn app_id(&self) -> AppId {
		self.id
	}
}

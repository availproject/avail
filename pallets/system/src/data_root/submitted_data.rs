use avail_core::{traits::GetAppId, AppId};

use codec::{Decode, Encode};
use derive_more::Constructor;
use sp_std::vec::Vec;

#[derive(Constructor, Debug)]
pub struct SubmittedDataRef<'a> {
	pub id: AppId,
	pub tx_index: u32,
	pub data: &'a [u8],
}

impl SubmittedDataRef<'_> {
	pub fn to_owned(&self) -> SubmittedData {
		SubmittedData::new(self.id, self.tx_index, self.data.to_vec())
	}
}

impl<'a> GetAppId for SubmittedDataRef<'a> {
	fn app_id(&self) -> AppId {
		self.id
	}
}

#[derive(Debug, Constructor, Encode, Decode)]
pub struct SubmittedData {
	pub id: AppId,
	pub tx_index: u32,
	pub data: Vec<u8>,
}

impl SubmittedData {
	pub fn to_ref(&self) -> SubmittedDataRef<'_> {
		SubmittedDataRef::new(self.id, self.tx_index, &self.data)
	}
}

impl GetAppId for SubmittedData {
	fn app_id(&self) -> AppId {
		self.id
	}
}

use avail_core::{traits::GetAppId, AppId};

use codec::{Decode, Encode};
use derive_more::Constructor;
use sp_std::vec::Vec;

#[derive(Debug, Constructor, Encode, Decode, PartialEq, Eq)]
pub struct SubmittedData {
	pub id: AppId,
	pub tx_index: u32,
	pub data: Vec<u8>,
}

impl GetAppId for SubmittedData {
	fn app_id(&self) -> AppId {
		self.id
	}
}

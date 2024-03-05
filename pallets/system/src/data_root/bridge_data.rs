use avail_core::data_proof_v2::{AddressedMessage, AddressedMessageRef};

use codec::{Decode, Encode};
use derive_more::Constructor;

#[derive(Constructor, Debug)]
pub struct BridgedDataRef<'a> {
	pub tx_index: u32,
	pub addr_msg: AddressedMessageRef<'a>,
}

impl<'a> BridgedDataRef<'a> {
	pub fn to_owned(&self) -> BridgedData {
		BridgedData::new(self.tx_index, self.addr_msg.to_owned())
	}
}

#[derive(Constructor, Debug, Encode, Decode, Clone)]
pub struct BridgedData {
	pub tx_index: u32,
	pub addr_msg: AddressedMessage,
}

impl BridgedData {
	pub fn to_ref(&self) -> BridgedDataRef<'_> {
		BridgedDataRef::new(self.tx_index, self.addr_msg.to_ref())
	}
}

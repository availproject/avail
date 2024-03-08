use avail_core::data_proof_v2::AddressedMessage;

use codec::{Decode, Encode};
use derive_more::Constructor;

#[derive(Constructor, Debug, Encode, Decode, Clone)]
pub struct BridgedData {
	pub tx_index: u32,
	pub addr_msg: AddressedMessage,
}

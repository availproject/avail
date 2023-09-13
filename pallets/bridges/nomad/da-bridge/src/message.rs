use core::convert::TryFrom;

use nomad_core::{TypedMessage, TypedMessageVariant};
use sp_core::H256;
use sp_std::vec::Vec;

/// DA Bridge message 1-byte type tags. Note that the invalid 0 variant
/// is to maintain parity with our existing Solidity contracts.
#[repr(u8)]
#[derive(Clone, Debug, PartialEq)]
pub enum DABridgeMessageTypes {
	Invalid = 0,
	DataRootMessage = 1,
}

#[derive(PartialEq)]
/// Errors during the creation from `extrinsics`.
pub enum TryFromError {
	/// Unknown DABridgeMessage variant
	Unknown,
}

impl TryFrom<u8> for DABridgeMessageTypes {
	type Error = TryFromError;

	fn try_from(num: u8) -> Result<Self, Self::Error> {
		match num {
			0 => Ok(Self::Invalid),
			1 => Ok(Self::DataRootMessage),
			_ => Err(TryFromError::Unknown),
		}
	}
}

/// Data root message variant. Contains a block number and its corresponding
/// data root.
#[derive(Clone, Debug, PartialEq)]
pub struct DataRootMessage {
	/// Block number
	pub block_number: u32,
	/// Data root
	pub data_root: H256,
}

impl TypedMessageVariant for DataRootMessage {
	const MESSAGE_TYPE: u8 = DABridgeMessageTypes::DataRootMessage as u8;

	fn encode(&self) -> Vec<u8> {
		let block_number_bytes = self.block_number.to_be_bytes();
		let data_root_bytes = self.data_root.as_bytes();

		let mut buf: Vec<u8> = Vec::with_capacity(self.len());

		buf.push(Self::MESSAGE_TYPE);
		buf.extend_from_slice(block_number_bytes.as_ref());
		buf.extend_from_slice(data_root_bytes);

		buf
	}
}

/// Enum of DABridge message types
pub enum DABridgeMessages {
	DataRootMessage(DataRootMessage),
}

impl From<DataRootMessage> for DABridgeMessages {
	fn from(data_root_msg: DataRootMessage) -> Self {
		Self::DataRootMessage(data_root_msg)
	}
}

impl TypedMessage for DABridgeMessages {
	type MessageEnum = DABridgeMessageTypes;

	fn encode(&self) -> Vec<u8> {
		match self {
			Self::DataRootMessage(msg) => msg.encode(),
		}
	}
}

#[cfg(test)]
mod test {
	use core::convert::TryInto;

	use frame_support::{parameter_types, BoundedVec};
	use nomad_core::{NomadMessage, NON_BODY_LENGTH};

	use super::*;

	const DATA_ROOT_MSG_LEN: usize = 1 + 4 + 32;

	parameter_types! {
		const MaxMessageBodyBytes: u32 = 1024;
	}

	#[test]
	fn it_creates_data_root_msg() {
		let message = DataRootMessage {
			block_number: 5,
			data_root: H256::repeat_byte(1),
		};

		assert_eq!(message.len(), DATA_ROOT_MSG_LEN);

		let body: DABridgeMessages = message.into();
		let bounded: BoundedVec<u8, MaxMessageBodyBytes> = body.encode().try_into().unwrap();

		let nomad_msg = NomadMessage {
			origin: 0,
			sender: Default::default(),
			nonce: 0,
			destination: 0,
			recipient: Default::default(),
			body: bounded,
		};

		let nomad_msg_vec = nomad_msg.to_vec();
		assert_eq!(nomad_msg_vec.len(), NON_BODY_LENGTH + DATA_ROOT_MSG_LEN);
	}
}

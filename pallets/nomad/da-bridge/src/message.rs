use nomad_core::TypedMessage;
use primitive_types::H256;
use sp_std::vec::Vec;

#[repr(u8)]
#[derive(Clone, Debug, PartialEq)]
pub enum DABridgeMessageTypes {
	Invalid = 0,
	DataRootMessage = 1,
}

impl From<u8> for DABridgeMessageTypes {
	fn from(num: u8) -> Self {
		match num {
			0 => Self::Invalid,
			1 => Self::DataRootMessage,
			_ => panic!("Invalid u8 for DABridgeMessage enum!"),
		}
	}
}

pub struct DABridgeMessage(Vec<u8>);

impl AsRef<[u8]> for DABridgeMessage {
	fn as_ref(&self) -> &[u8] { self.0.as_ref() }
}

impl TypedMessage for DABridgeMessage {
	type MessageEnum = DABridgeMessageTypes;
}

impl DABridgeMessage {
	/* Extrinsic Root: message containing an extrinsic root and its
	 * corresponding block number
	 * type (1 byte) || block number (4 bytes) || ext root (32 bytes)
	 */

	/// Format extrinsic root message with block number and root. Internally
	/// checks that passed in generics for block number and extrinsic root are
	/// expected lengths.
	pub fn format_data_root_message(
		block_number: impl Into<u32>,
		ext_root: impl Into<H256>,
	) -> Self {
		let mut buf: Vec<u8> = Vec::new();

		let block_number: u32 = block_number.into();
		let block_number_bytes = block_number.to_be_bytes();

		let ext_root: H256 = ext_root.into();
		let ext_root_bytes = ext_root.as_bytes();

		buf.push(DABridgeMessageTypes::DataRootMessage as u8);
		buf.extend(block_number_bytes.to_vec());
		buf.extend(ext_root_bytes.to_vec());
		Self(buf)
	}
}

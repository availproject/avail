use nomad_core::TypedMessage;
use primitive_types::H256;
use sp_runtime::DispatchError;
use sp_std::vec::Vec;

const EXTRINSIC_ROOT_MESSAGE_LEN: usize = 1 + 4 + 32;

#[repr(u8)]
#[derive(Clone, Debug, PartialEq)]
pub enum DABridgeMessageTypes {
	Invalid = 0,
	ExtrinsicsRootMessage = 1,
}

impl From<u8> for DABridgeMessageTypes {
	fn from(num: u8) -> Self {
		match num {
			0 => Self::Invalid,
			1 => Self::ExtrinsicsRootMessage,
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
	pub fn format_extrinsics_root_message<B: AsRef<[u8]>, E: AsRef<[u8]>>(
		block_number: B,
		ext_root: E,
	) -> Self {
		let mut buf: Vec<u8> = Vec::new();

		// TODO: is asserting sizes a valid pattern?

		// Assert block number is 4 byte u32
		let block_number_bytes = block_number.as_ref();
		assert!(block_number_bytes.len() == 4);

		// Assert extrinsic root is 32 byte hash (H256)
		let ext_root_bytes = ext_root.as_ref();
		assert!(ext_root_bytes.len() == 32);

		buf.push(DABridgeMessageTypes::ExtrinsicsRootMessage as u8);
		buf.extend(block_number_bytes.to_vec());
		buf.extend(ext_root_bytes.to_vec());
		Self(buf)
	}

	/// Check type and length of extrinsic root message is valid
	pub fn is_valid_extrinsics_root_message(&self) -> bool {
		self.message_type() == DABridgeMessageTypes::ExtrinsicsRootMessage
			&& self.as_ref().len() == EXTRINSIC_ROOT_MESSAGE_LEN
	}
}

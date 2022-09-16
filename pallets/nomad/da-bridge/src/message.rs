use core::convert::TryInto;

use frame_support::BoundedVec;
use nomad_core::TypedMessage;
use primitive_types::H256;
use sp_std::vec::Vec;

use crate::{Config, Error};

const DATA_ROOT_MESSAGE_LEN: usize = 1 + 4 + 32;

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

pub struct DABridgeMessage<T: Config>(pub BoundedVec<u8, T::MaxMessageBodyBytes>);

impl<T: Config> AsRef<BoundedVec<u8, T::MaxMessageBodyBytes>> for DABridgeMessage<T> {
	fn as_ref(&self) -> &BoundedVec<u8, T::MaxMessageBodyBytes> { &self.0 }
}

impl<T: Config> TypedMessage<T::MaxMessageBodyBytes> for DABridgeMessage<T> {
	type MessageEnum = DABridgeMessageTypes;
}

impl<T: Config> DABridgeMessage<T> {
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
	) -> Result<Self, Error<T>> {
		let mut buf: Vec<u8> = Vec::with_capacity(DATA_ROOT_MESSAGE_LEN);

		let block_number: u32 = block_number.into();
		let block_number_bytes = block_number.to_be_bytes();

		let ext_root: H256 = ext_root.into();
		let ext_root_bytes = ext_root.as_bytes();

		buf.push(DABridgeMessageTypes::DataRootMessage as u8);
		buf.extend_from_slice(block_number_bytes.as_ref());
		buf.extend_from_slice(ext_root_bytes);

		let bounded: BoundedVec<u8, T::MaxMessageBodyBytes> = buf
			.try_into()
			.or(Err(Error::<T>::DABridgeMessageExceedsMaxMessageSize))?;
		Ok(Self(bounded))
	}
}

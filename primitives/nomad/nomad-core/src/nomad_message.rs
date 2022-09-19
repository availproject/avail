use alloc::vec::Vec;

use frame_support::pallet_prelude::*;
use sp_core::H256;
use sp_runtime::traits::{Hash, Keccak256};

pub const NON_BODY_LENGTH: usize = 4 + 32 + 4 + 4 + 32;

/// A full Nomad message
#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo)]
pub struct NomadMessage<S: frame_support::traits::Get<u32>> {
	/// 4   SLIP-44 ID
	pub origin: u32,
	/// 32  Address in home convention
	pub sender: H256,
	/// 4   Count of all previous messages to destination
	pub nonce: u32,
	/// 4   SLIP-44 ID
	pub destination: u32,
	/// 32  Address in destination convention
	pub recipient: H256,
	/// 0+  Message contents
	pub body: BoundedVec<u8, S>,
}

impl<S: frame_support::traits::Get<u32>> NomadMessage<S> {
	/// Serialize to a vec
	pub fn to_vec(&self) -> Vec<u8> {
		let size = NON_BODY_LENGTH + (S::get() as usize);
		let body: &[u8] = self.body.as_ref();

		let mut buf = Vec::<u8>::with_capacity(size);

		buf.extend_from_slice(&self.origin.to_be_bytes());
		buf.extend_from_slice(&self.sender.as_ref());
		buf.extend_from_slice(&self.nonce.to_be_bytes());
		buf.extend_from_slice(&self.destination.to_be_bytes());
		buf.extend_from_slice(&self.recipient.as_ref());
		buf.extend_from_slice(body);

		buf
	}

	/// Get hash of message
	pub fn hash(&self) -> H256 { Keccak256::hash(&self.to_vec()).into() }
}

#[cfg(test)]
mod tests {
	use core::convert::TryInto;

	use frame_support::{parameter_types, BoundedVec};

	use super::NON_BODY_LENGTH;
	use crate::NomadMessage;

	parameter_types! {
		const MaxBodyLen :u32 = 1024;
	}

	#[test]
	fn formats_message_to_vec() {
		let body = [1u8; 32];
		let bounded: BoundedVec<u8, MaxBodyLen> = body.to_vec().try_into().unwrap();

		let message = NomadMessage {
			origin: 0,
			sender: Default::default(),
			nonce: 0,
			destination: 0,
			recipient: Default::default(),
			body: bounded,
		};

		assert_eq!(message.to_vec().len(), NON_BODY_LENGTH + 32);
	}
}

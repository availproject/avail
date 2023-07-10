use frame_support::{pallet_prelude::*, traits::Get};
use sp_core::H256;
use sp_std::{mem::size_of, vec::Vec};

/// Size of `NomadMessage` fields except `body`.
pub const NON_BODY_LENGTH: usize = 3 * size_of::<u32>() + 2 * size_of::<H256>();

/// A full Nomad message
#[derive(Clone, Encode, Decode, PartialEq, Eq, RuntimeDebug, TypeInfo)]
pub struct NomadMessage<S: Get<u32>> {
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

impl<S: Get<u32>> NomadMessage<S> {
	/// Serialize to a vec
	pub fn to_vec(&self) -> Vec<u8> {
		let size = NON_BODY_LENGTH + self.body.len();
		let mut buf = Vec::<u8>::with_capacity(size);

		buf.extend_from_slice(&self.origin.to_be_bytes());
		buf.extend_from_slice(self.sender.as_ref());
		buf.extend_from_slice(&self.nonce.to_be_bytes());
		buf.extend_from_slice(&self.destination.to_be_bytes());
		buf.extend_from_slice(self.recipient.as_ref());
		buf.extend_from_slice(&self.body);

		buf
	}

	/// Get hash of message
	pub fn hash(&self) -> H256 {
		keccak256_concat!(
			&self.origin.to_be_bytes(),
			&self.sender,
			&self.nonce.to_be_bytes(),
			&self.destination.to_be_bytes(),
			&self.recipient,
			&self.body
		)
	}
}

#[cfg(test)]
mod tests {
	use core::convert::TryInto;

	use frame_support::{parameter_types, BoundedVec};
	use sp_std::mem::size_of_val;

	use super::{NomadMessage, NON_BODY_LENGTH};

	parameter_types! {
		const MaxBodyLen :u32 = 1024;
	}

	/// Double checks that constant `NON_BODY_LENGTH` will be synchronized with actual
	#[test]
	fn non_body_lenght_synch_with_nomad_message_type() {
		let m = NomadMessage::<MaxBodyLen> {
			origin: 0,
			sender: Default::default(),
			nonce: 0,
			destination: 0,
			recipient: Default::default(),
			body: Default::default(),
		};

		let actual_non_body_len = size_of_val(&m.origin)
			+ size_of_val(&m.sender)
			+ size_of_val(&m.nonce)
			+ size_of_val(&m.destination)
			+ size_of_val(&m.recipient);

		assert_eq!(actual_non_body_len, NON_BODY_LENGTH);
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

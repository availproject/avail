use alloc::vec::Vec;

use frame_support::pallet_prelude::*;
use sp_core::H256;
use sp_runtime::traits::{Hash, Keccak256};

const NON_BODY_LENGTH: usize = 4 + 32 + 4 + 4 + 32;

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
		let mut buf = Vec::<u8>::with_capacity(size);

		buf.extend_from_slice(&self.origin.to_be_bytes());
		buf.extend_from_slice(&self.sender.as_ref());
		buf.extend_from_slice(&self.nonce.to_be_bytes());
		buf.extend_from_slice(&self.destination.to_be_bytes());
		buf.extend_from_slice(&self.recipient.as_ref());
		buf.extend_from_slice(&self.body);

		buf
	}

	/// Get hash of message
	pub fn hash(&self) -> H256 { Keccak256::hash(&self.to_vec()).into() }
}

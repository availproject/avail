use alloc::vec::Vec;

use frame_support::pallet_prelude::*;
use primitive_types::H256;
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

use crate::utils::keccak256;

/// A full Nomad message
#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct NomadMessage {
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
	pub body: Vec<u8>,
}

impl NomadMessage {
	/// Serialize to a vec
	pub fn to_vec(&self) -> Vec<u8> {
		let mut buf = Vec::<u8>::new();
		buf.extend_from_slice(&self.origin.to_be_bytes());
		buf.extend_from_slice(&self.sender.as_ref());
		buf.extend_from_slice(&self.nonce.to_be_bytes());
		buf.extend_from_slice(&self.destination.to_be_bytes());
		buf.extend_from_slice(&self.recipient.as_ref());
		buf.extend_from_slice(&self.body);

		buf
	}

	/// Get hash of message
	pub fn hash(&self) -> H256 { keccak256(self.to_vec()).into() }
}

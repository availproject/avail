use super::BoundedData;

use ethabi::{encode, Token};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use sp_core::H256;

/// Possible types of Messages allowed by Avail to bridge to other chains.
#[derive(Debug, Default, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub enum MessageType {
	ArbitraryMessage,
	#[default]
	FungibleToken,
}

impl From<MessageType> for Vec<u8> {
	fn from(msg_type: MessageType) -> Self {
		match msg_type {
			MessageType::ArbitraryMessage => vec![0x01],
			MessageType::FungibleToken => vec![0x02],
		}
	}
}

/// Message type used to bridge between Avail & other chains
#[derive(Debug, Default, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct Message {
	pub message_type: MessageType,
	pub from: H256,
	pub to: H256,
	pub origin_domain: u32,
	pub destination_domain: u32,
	pub data: BoundedData,
	pub id: u64, // a global nonce that is incremented with each leaf
}

impl Message {
	pub fn abi_encode(self) -> Vec<u8> {
		encode(&[Token::Tuple(vec![
			Token::FixedBytes(self.message_type.into()),
			Token::FixedBytes(self.from.to_fixed_bytes().to_vec()),
			Token::FixedBytes(self.to.to_fixed_bytes().to_vec()),
			Token::Uint(ethabi::Uint::from(self.origin_domain)),
			Token::Uint(ethabi::Uint::from(self.destination_domain)),
			Token::Bytes(self.data.into()),
			Token::Uint(ethabi::Uint::from(self.id)),
		])])
	}
}

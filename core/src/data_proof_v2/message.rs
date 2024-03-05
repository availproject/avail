use super::{tx_uid_deconstruct, AddressedMessageRef, BoundedData, MessageRef};

use codec::{Decode, Encode};
use derive_more::{Constructor, From};
use scale_info::TypeInfo;
use sp_core::H256;
use sp_std::{vec, vec::Vec};

use ethabi_decode::{encode, Token, U256};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Encode, Decode, PartialEq, Eq, TypeInfo)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub enum MessageType {
	MTData,
	MTFungibleToken,
}

/// Possible types of Messages allowed by Avail to bridge to other chains.
#[derive(Debug, Clone, Encode, Decode, PartialEq, Eq, From, TypeInfo)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub enum Message {
	Data(BoundedData),
	FungibleToken {
		asset_id: H256,
		#[codec(compact)]
		amount: u128,
	},
}

impl Message {
	pub fn selector_abi_encode(&self) -> Vec<u8> {
		match self {
			Message::Data(..) => vec![0x01],
			Message::FungibleToken { .. } => vec![0x02],
		}
	}

	pub fn to_ref(&self) -> MessageRef<'_> {
		match self {
			Message::Data(data) => MessageRef::Data(data.as_ref()),
			Message::FungibleToken { asset_id, amount } => MessageRef::FungibleToken {
				asset_id: *asset_id,
				amount: *amount,
			},
		}
	}

	pub fn r#type(&self) -> MessageType {
		match self {
			Message::Data(..) => MessageType::MTData,
			Message::FungibleToken { .. } => MessageType::MTFungibleToken,
		}
	}

	pub fn is_empty(&self) -> bool {
		match self {
			Message::Data(data) => data.is_empty(),
			Message::FungibleToken { .. } => false,
		}
	}
}

/// Message type used to bridge between Avail & other chains
#[derive(Debug, Clone, Encode, Decode, PartialEq, Eq, Constructor, TypeInfo)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct AddressedMessage {
	pub message: Message,
	pub from: H256,
	pub to: H256,
	#[codec(compact)]
	pub origin_domain: u32,
	#[codec(compact)]
	pub destination_domain: u32,
	/// Unique identifier for the message
	#[codec(compact)]
	pub id: u64,
}

impl AddressedMessage {
	fn abi_data(&self) -> Vec<u8> {
		match &self.message {
			Message::Data(data) => data.clone().into_inner(),
			Message::FungibleToken { asset_id, amount } => {
				let data = [
					Token::FixedBytes(asset_id.encode()),
					Token::Uint(U256::from(*amount)),
				];
				encode(&data)
			},
		}
	}

	pub fn abi_encode(&self) -> Vec<u8> {
		let data = self.abi_data();
		encode(&[Token::Tuple(vec![
			Token::FixedBytes(self.message.selector_abi_encode()),
			Token::FixedBytes(self.from.to_fixed_bytes().to_vec()),
			Token::FixedBytes(self.to.to_fixed_bytes().to_vec()),
			Token::Uint(U256::from(self.origin_domain)),
			Token::Uint(U256::from(self.destination_domain)),
			Token::Bytes(data),
			Token::Uint(U256::from(self.id)),
		])])
	}

	pub fn to_ref(&self) -> AddressedMessageRef<'_> {
		let (block, tx_index) = tx_uid_deconstruct(self.id);

		AddressedMessageRef::new(
			self.message.to_ref(),
			self.from,
			self.to,
			self.destination_domain,
			block,
			tx_index,
		)
	}
}

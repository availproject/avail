use super::{AddressedMessage, BoundedData, Message};

use codec::Encode;
use ethabi_decode::{encode, Token, U256};
use sp_core::H256;
use sp_std::{vec, vec::Vec};

#[derive(Debug, Clone, Copy)]
pub enum MessageRef<'a> {
	Data(&'a [u8]),
	FungibleToken { asset_id: H256, amount: u128 },
}

impl MessageRef<'_> {
	pub fn is_empty(&self) -> bool {
		match self {
			MessageRef::Data(data) => data.is_empty(),
			MessageRef::FungibleToken { .. } => false,
		}
	}
}

impl<'a> From<MessageRef<'a>> for Vec<u8> {
	fn from(msg: MessageRef<'a>) -> Self {
		match msg {
			MessageRef::Data(..) => vec![0x01],
			MessageRef::FungibleToken { .. } => vec![0x02],
		}
	}
}

impl<'a> From<&'a Message> for MessageRef<'a> {
	fn from(m: &'a Message) -> Self {
		match m {
			Message::Data(data) => MessageRef::Data(data.as_ref()),
			Message::FungibleToken { asset_id, amount } => MessageRef::FungibleToken {
				asset_id: *asset_id,
				amount: *amount,
			},
		}
	}
}

impl<'a> From<MessageRef<'a>> for Message {
	fn from(m: MessageRef<'a>) -> Self {
		match m {
			MessageRef::Data(data) => Message::Data(BoundedData::truncate_from(data.to_vec())),
			MessageRef::FungibleToken { asset_id, amount } => {
				Message::FungibleToken { asset_id, amount }
			},
		}
	}
}

#[derive(Debug, Clone, Copy)]
pub struct AddressedMessageRef<'a> {
	pub message: MessageRef<'a>,
	pub from: H256,
	pub to: H256,
	pub domain: u32,
	pub block: u32,
	pub tx_index: u32,
}

impl<'a> AddressedMessageRef<'a> {
	pub fn new(
		message: &'a Message,
		from: H256,
		to: H256,
		domain: u32,
		block: u32,
		tx_index: u32,
	) -> Self {
		let message = MessageRef::from(message);
		Self {
			message,
			from,
			to,
			domain,
			block,
			tx_index,
		}
	}

	fn abi_data(&self) -> Vec<u8> {
		match &self.message {
			MessageRef::Data(data) => data.to_vec(),
			MessageRef::FungibleToken { asset_id, amount } => {
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
			Token::FixedBytes(self.message.into()),
			Token::FixedBytes(self.from.to_fixed_bytes().to_vec()),
			Token::FixedBytes(self.to.to_fixed_bytes().to_vec()),
			Token::Uint(U256::from(1u32)),
			Token::Uint(U256::from(self.domain)),
			Token::Bytes(data),
			Token::Uint(U256::from(self.id())),
		])])
	}

	pub fn id(&self) -> u64 {
		let mut buf = [0u8; 8];
		buf[..4].copy_from_slice(&self.block.to_be_bytes());
		buf[4..].copy_from_slice(&self.tx_index.to_be_bytes());
		u64::from_be_bytes(buf)
	}

	pub fn is_empty(&self) -> bool {
		self.message.is_empty()
	}
}

const SLICE_ERR: &str = "Valid slice .qed";

impl<'a> From<&'a AddressedMessage> for AddressedMessageRef<'a> {
	fn from(m: &'a AddressedMessage) -> Self {
		let id: [u8; 8] = m.id.to_be_bytes();
		let block = u32::from_be_bytes(id[..4].try_into().expect(SLICE_ERR));
		let tx_index = u32::from_be_bytes(id[4..].try_into().expect(SLICE_ERR));

		Self {
			message: (&m.message).into(),
			from: m.from,
			to: m.to,
			domain: m.destination_domain,
			block,
			tx_index,
		}
	}
}

impl<'a> From<AddressedMessageRef<'a>> for AddressedMessage {
	fn from(m: AddressedMessageRef<'a>) -> Self {
		let mut id_bytes = [0u8; 8];
		id_bytes[..4].copy_from_slice(&m.block.to_be_bytes());
		id_bytes[4..].copy_from_slice(&m.tx_index.to_be_bytes());
		let id = u64::from_be_bytes(id_bytes);

		Self {
			message: m.message.into(),
			from: m.from,
			to: m.to,
			origin_domain: 1,
			destination_domain: m.domain,
			id,
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::data_proof_v2::{AddressedMessage, BoundedData};
	use rand::random;
	use test_case::test_case;

	fn data(data: Vec<u8>) -> AddressedMessage {
		AddressedMessage {
			message: BoundedData::truncate_from(data).into(),
			from: H256(random()),
			to: H256(random()),
			origin_domain: 1,
			destination_domain: random(),
			id: random(),
		}
	}

	fn fungible(asset_id: H256, amount: u128) -> AddressedMessage {
		AddressedMessage {
			message: Message::FungibleToken { asset_id, amount },
			from: H256(random()),
			to: H256(random()),
			origin_domain: 1,
			destination_domain: random(),
			id: random(),
		}
	}

	#[test_case( &data(vec![]); "arbitray no data")]
	#[test_case( &data(b"Some data".to_vec()); "arbitray data")]
	#[test_case( &fungible(H256::zero(), 0); "fungible 0x0 zero")]
	#[test_case( &fungible(H256::zero(), random()); "fungible 0x0 rand")]
	#[test_case( &fungible(H256::zero(), u128::MAX); "fungible 0x0 max")]
	#[test_case( &fungible(H256(random()), 0); "fungible rand zero")]
	#[test_case( &fungible(H256(random()), random()); "fungible rand rand")]
	#[test_case( &fungible(H256(random()), u128::MAX); "fungible rand max")]
	fn check_ref_abi_encode(m: &AddressedMessage) {
		// `Message` -> `MessageRef`, check `abi_encode` is the same
		let m_encoded = m.clone().abi_encode();
		let m_ref = AddressedMessageRef::from(m);
		let m_ref_encoded = m_ref.abi_encode();
		assert_eq!(m_encoded, m_ref_encoded);

		// `MessageRef` -> `Message`, check `abi_encode` is the same
		let new_m = AddressedMessage::from(m_ref);
		let new_m_encoded = new_m.abi_encode();
		assert_eq!(m_encoded, new_m_encoded);
	}
}

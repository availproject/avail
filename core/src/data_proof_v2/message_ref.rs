use super::{BoundedData, Message, MessageType};

use ethabi::{encode, Token};
use sp_core::H256;
use sp_std::{vec, vec::Vec};

#[derive(Debug, Clone, Copy)]
pub enum MessageTypeRef<'a> {
	ArbitraryMessage(&'a [u8]),
	FungibleToken { asset_id: H256, value: u128 },
}

impl<'a> From<MessageTypeRef<'a>> for Vec<u8> {
	fn from(msg: MessageTypeRef<'a>) -> Self {
		match msg {
			MessageTypeRef::ArbitraryMessage(..) => vec![0x01],
			MessageTypeRef::FungibleToken { .. } => vec![0x02],
		}
	}
}

#[derive(Debug, Clone, Copy)]
pub enum MessageRefError {
	ArbitraryWithoutData,
	FungibleWithoutAsset,
}

#[derive(Debug, Clone, Copy)]
pub struct MessageRef<'a> {
	pub message: MessageTypeRef<'a>,
	pub from: H256,
	pub to: H256,
	pub domain: u32,
	pub block: u32,
	pub tx_index: u32,
}

impl<'a> MessageRef<'a> {
	pub fn new<F>(
		m_type: MessageType,
		from: F,
		to: H256,
		domain: u32,
		value: Option<u128>,
		asset_id: Option<H256>,
		data: Option<&'a [u8]>,
		block: u32,
		tx_index: u32,
	) -> Result<Self, MessageRefError>
	where
		H256: From<F>,
	{
		let message = match m_type {
			MessageType::ArbitraryMessage => {
				MessageTypeRef::ArbitraryMessage(data.ok_or(MessageRefError::ArbitraryWithoutData)?)
			},
			MessageType::FungibleToken => {
				let asset_id = asset_id.ok_or(MessageRefError::FungibleWithoutAsset)?;
				let value = value.unwrap_or_default();
				MessageTypeRef::FungibleToken { asset_id, value }
			},
		};

		Ok(Self {
			message,
			from: from.into(),
			to,
			domain,
			block,
			tx_index,
		})
	}

	pub fn abi_encode(&self) -> Vec<u8> {
		encode(&[Token::Tuple(vec![
			Token::FixedBytes(self.message.clone().into()),
			Token::FixedBytes(self.from.to_fixed_bytes().to_vec()),
			Token::FixedBytes(self.to.to_fixed_bytes().to_vec()),
			Token::Uint(ethabi::Uint::from(1u32)),
			Token::Uint(ethabi::Uint::from(self.domain)),
			Token::Bytes(self.data()),
			Token::Uint(ethabi::Uint::from(self.id())),
		])])
	}

	fn id(&self) -> u64 {
		let mut buf = [0u8; 8];
		buf[..4].copy_from_slice(&self.block.to_be_bytes());
		buf[4..].copy_from_slice(&self.tx_index.to_be_bytes());
		u64::from_be_bytes(buf)
	}

	fn data(&self) -> Vec<u8> {
		match self.message {
			MessageTypeRef::ArbitraryMessage(data) => data.to_vec(),
			MessageTypeRef::FungibleToken { asset_id, value } => {
				let mut buf = [0u8; { 32 + 16 }];
				buf[..32].copy_from_slice(asset_id.as_bytes());
				buf[32..].copy_from_slice(&value.to_be_bytes());
				buf.to_vec()
			},
		}
	}
}

const SLICE_ERR: &str = "Valid slice .qed";

impl<'a> From<&'a Message> for MessageRef<'a> {
	fn from(m: &'a Message) -> Self {
		let id: [u8; 8] = m.id.to_be_bytes();
		let block = u32::from_be_bytes(id[..4].try_into().expect(SLICE_ERR));
		let tx_index = u32::from_be_bytes(id[4..].try_into().expect(SLICE_ERR));

		let message = match m.message_type {
			MessageType::ArbitraryMessage => MessageTypeRef::ArbitraryMessage(&m.data),
			MessageType::FungibleToken => {
				if m.data.len() >= 48 {
					let asset_id = H256::from_slice(&m.data[..32]);
					let value_bytes = <[u8; 16]>::try_from(&m.data[32..48]).unwrap_or_default();
					let value = u128::from_be_bytes(value_bytes);
					MessageTypeRef::FungibleToken { asset_id, value }
				} else {
					MessageTypeRef::FungibleToken {
						asset_id: H256::zero(),
						value: 0,
					}
				}
			},
		};

		Self {
			message,
			from: m.from,
			to: m.to,
			domain: m.destination_domain,
			block,
			tx_index,
		}
	}
}

impl<'a> From<MessageRef<'a>> for Message {
	fn from(m: MessageRef<'a>) -> Self {
		let (data, message_type) = match m.message {
			MessageTypeRef::ArbitraryMessage(data) => {
				(data.to_vec(), MessageType::ArbitraryMessage)
			},
			MessageTypeRef::FungibleToken { asset_id, value } => {
				let value_bytes = value.to_be_bytes();
				let mut data = asset_id.as_bytes().to_vec();
				data.extend_from_slice(&value_bytes);
				(data, MessageType::FungibleToken)
			},
		};

		let mut id_bytes = [0u8; 8];
		id_bytes[..4].copy_from_slice(&m.block.to_be_bytes());
		id_bytes[4..].copy_from_slice(&m.tx_index.to_be_bytes());
		let id = u64::from_be_bytes(id_bytes);

		Message {
			message_type,
			from: m.from,
			to: m.to,
			origin_domain: 1,
			destination_domain: m.domain,
			data: BoundedData::truncate_from(data),
			id,
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::data_proof_v2::{BoundedData, Message};
	use rand::random;
	use test_case::test_case;

	fn arbitrary(data: Vec<u8>) -> Message {
		Message {
			message_type: MessageType::ArbitraryMessage,
			from: H256(random()),
			to: H256(random()),
			origin_domain: 1,
			destination_domain: random(),
			data: BoundedData::truncate_from(data),
			id: random(),
		}
	}

	fn fungible(asset_id: H256, value: u128) -> Message {
		let mut data = asset_id.as_fixed_bytes().to_vec();
		data.extend_from_slice(&value.to_be_bytes());

		Message {
			message_type: MessageType::FungibleToken,
			from: H256(random()),
			to: H256(random()),
			origin_domain: 1,
			destination_domain: random(),
			data: BoundedData::truncate_from(data),
			id: random(),
		}
	}

	#[test_case( arbitrary(vec![]); "arbitray no data")]
	#[test_case( arbitrary(b"Some data".to_vec()); "arbitray data")]
	#[test_case( fungible(H256::zero(), 0); "fungible 0x0 zero")]
	#[test_case( fungible(H256::zero(), random()); "fungible 0x0 rand")]
	#[test_case( fungible(H256::zero(), u128::MAX); "fungible 0x0 max")]
	#[test_case( fungible(H256(random()), 0); "fungible rand zero")]
	#[test_case( fungible(H256(random()), random()); "fungible rand rand")]
	#[test_case( fungible(H256(random()), u128::MAX); "fungible rand max")]
	fn check_ref_abi_encode(m: Message) {
		// `Message` -> `MessageRef`, check `abi_encode` is the same
		let m_encoded = m.clone().abi_encode();
		let m_ref = MessageRef::from(&m);
		let m_ref_encoded = m_ref.abi_encode();
		assert_eq!(m_encoded, m_ref_encoded);

		// `MessageRef` -> `Message`, check `abi_encode` is the same
		let new_m = Message::from(m_ref);
		let new_m_encoded = new_m.abi_encode();
		assert_eq!(m_encoded, new_m_encoded);
	}
}

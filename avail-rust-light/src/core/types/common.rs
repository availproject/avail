use parity_scale_codec::Encode;
use serde::{Deserialize, Serialize};

/*
	This object guarantees that an already encoded data is not encoded twice.
*/
#[derive(Debug, Clone, Serialize)]
pub struct AlreadyEncoded(pub Vec<u8>);
impl AlreadyEncoded {
	pub fn to_hex_string(&self) -> String {
		std::format!("0x{}", hex::encode(self.0.as_slice()))
	}
}
impl Encode for AlreadyEncoded {
	fn size_hint(&self) -> usize {
		0
	}

	fn encode(&self) -> Vec<u8> {
		self.0.clone()
	}
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct H256(pub [u8; 32]);
impl H256 {
	pub fn to_hex_string(&self) -> String {
		std::format!("0x{}", hex::encode(self.0))
	}

	pub fn from_hex_string(mut s: &str) -> Result<Self, ()> {
		if s.starts_with("0x") {
			s = &s[2..];
		}

		if s.len() != 64 {
			return Err(());
		}

		let block_hash = hex::decode(s).unwrap();

		if block_hash.len() != 32 {
			return Err(());
		}

		let block_hash = TryInto::<[u8; 32]>::try_into(block_hash).map_err(|_| ())?;

		Ok(H256(block_hash))
	}
}
impl Encode for H256 {
	fn size_hint(&self) -> usize {
		self.0.size_hint()
	}

	fn encode_to<T: parity_scale_codec::Output + ?Sized>(&self, dest: &mut T) {
		self.0.encode_to(dest);
	}
}

use super::error::CoreError;
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

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct H256(pub [u8; 32]);
impl H256 {
	pub fn to_hex_string(&self) -> String {
		std::format!("0x{}", hex::encode(self.0))
	}

	pub fn from_hex_string(mut s: &str) -> Result<Self, CoreError> {
		if s.starts_with("0x") {
			s = &s[2..];
		}

		if s.len() != 64 {
			let msg = std::format!(
				"Failed to convert string to H256. Expected 64 bytes got {}. Input string: {}",
				s.len(),
				s
			);
			return Err(CoreError::ConversionError(msg));
		}

		let block_hash = hex::decode(s).map_err(|e| CoreError::FromHexError(e))?;

		if block_hash.len() != 32 {
			let msg = std::format!(
				"Failed to convert string to H256. Expected 32 bytes for decoded value got {}. Decoded string: {:?}",
				block_hash.len(),
				block_hash
			);
			return Err(CoreError::ConversionError(msg));
		}

		let block_hash = TryInto::<[u8; 32]>::try_into(block_hash);
		match block_hash {
			Ok(v) => Ok(H256(v)),
			Err(e) => {
				let msg = std::format!("Failed to covert decoded string to H256. Input {:?}", e);
				Err(CoreError::ConversionError(msg))
			},
		}
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
// When we receive block hash from RPC calls it's always in Hex like String format.
// Example: "0x26aa394eea5630e07c48ae0c9558cef780d41e5e16056765bc8461851072c9d7"
//
impl<'de> Deserialize<'de> for H256 {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		let buf = String::deserialize(deserializer)?;
		Ok(H256::from_hex_string(&buf).unwrap())
	}
}

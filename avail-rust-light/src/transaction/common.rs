use parity_scale_codec::Encode;
use serde::Serialize;

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

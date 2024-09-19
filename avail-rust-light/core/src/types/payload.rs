use super::{
	payload_fields::{Additional, Call, Extra},
	AlreadyEncoded,
};
use crate::crypto::{blake2_256, Keypair, Signature};
use parity_scale_codec::Encode;

#[derive(Debug, Clone)]
pub struct UnsignedPayload {
	call: Call,
	extra: Extra,
	additional: Additional,
}
impl UnsignedPayload {
	pub fn new(call: Call, extra: Extra, additional: Additional) -> Self {
		Self {
			call,
			extra,
			additional,
		}
	}

	pub fn encode(self) -> UnsignedEncodedPayload {
		UnsignedEncodedPayload::new(
			AlreadyEncoded(self.call.encode()),
			AlreadyEncoded(self.extra.encode()),
			AlreadyEncoded(self.additional.encode()),
		)
	}
}

#[derive(Debug, Clone)]
pub struct UnsignedEncodedPayload {
	pub call: AlreadyEncoded,
	pub extra: AlreadyEncoded,
	pub additional: AlreadyEncoded,
}
impl UnsignedEncodedPayload {
	pub fn new(call: AlreadyEncoded, extra: AlreadyEncoded, additional: AlreadyEncoded) -> Self {
		Self {
			call,
			extra,
			additional,
		}
	}

	pub fn sign(&self, signer: &Keypair) -> Signature {
		let mut bytes: Vec<u8> = Vec::with_capacity(
			self.call.size_hint() + self.extra.size_hint() + self.additional.size_hint(),
		);

		self.call.encode_to(&mut bytes);
		self.extra.encode_to(&mut bytes);
		self.additional.encode_to(&mut bytes);

		if bytes.len() > 256 {
			let blake = blake2_256(&bytes);
			signer.sign(blake.as_ref())
		} else {
			signer.sign(&bytes)
		}
	}
}

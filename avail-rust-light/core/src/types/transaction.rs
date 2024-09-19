use super::{
	multi::{MultiAddress, MultiSignature},
	AlreadyEncoded, H256,
};
use crate::crypto::{blake2_256, AccountId, Signature};
use parity_scale_codec::{Compact, Encode};

#[derive(Debug, Clone)]
pub struct OpaqueTransaction {
	pub data: AlreadyEncoded,
}
impl OpaqueTransaction {
	pub fn new(
		payload_extra: &AlreadyEncoded,
		payload_call: &AlreadyEncoded,
		account_id: AccountId,
		signature: Signature,
	) -> Self {
		let mut encoded_inner: Vec<u8> = Vec::new();

		// "is signed" + transaction protocol version (4)
		(0b10000000 + 4u8).encode_to(&mut encoded_inner);

		// Attach Address from Signer
		MultiAddress::Id(account_id).encode_to(&mut encoded_inner);

		// Attach Signature
		MultiSignature::Sr25519(signature.0).encode_to(&mut encoded_inner);

		// Attach Extra
		payload_extra.encode_to(&mut encoded_inner);

		// Attach Data
		payload_call.encode_to(&mut encoded_inner);

		// now, prefix byte length:
		let len = Compact(
			u32::try_from(encoded_inner.len()).expect("extrinsic size expected to be <4GB"),
		);
		let mut encoded = Vec::new();
		len.encode_to(&mut encoded);
		encoded.extend(encoded_inner);

		Self::new_raw(AlreadyEncoded(encoded))
	}

	pub fn new_raw(data: AlreadyEncoded) -> Self {
		Self { data }
	}

	pub fn to_hex_string(&self) -> String {
		self.data.to_hex_string()
	}

	pub fn get_hash(&self) -> H256 {
		H256(blake2_256(&self.data.0))
	}
}

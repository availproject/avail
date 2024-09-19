use crate::crypto::AccountId;
use parity_scale_codec::{Compact, Encode};

#[repr(u8)]
#[derive(Debug, Clone)]
pub enum MultiAddress {
	/// It's an account ID (pubkey).
	Id(AccountId) = 0,
	/// It's an account index.
	Index(Compact<u32>) = 1,
	/// It's some arbitrary raw bytes.
	Raw(Vec<u8>) = 2,
	/// It's a 32 byte representation.
	Address32([u8; 32]) = 3,
	/// Its a 20 byte representation.
	Address20([u8; 20]) = 4,
}

impl Encode for MultiAddress {
	fn size_hint(&self) -> usize {
		let size = match self {
			MultiAddress::Id(x) => x.size_hint(),
			MultiAddress::Index(x) => x.size_hint(),
			MultiAddress::Raw(x) => x.size_hint(),
			MultiAddress::Address32(x) => x.size_hint(),
			MultiAddress::Address20(x) => x.size_hint(),
		};

		size + 1
	}

	fn encode_to<T: parity_scale_codec::Output + ?Sized>(&self, dest: &mut T) {
		match self {
			MultiAddress::Id(_) => 0u8.encode_to(dest),
			MultiAddress::Index(_) => 1u8.encode_to(dest),
			MultiAddress::Raw(_) => 2u8.encode_to(dest),
			MultiAddress::Address32(_) => 3u8.encode_to(dest),
			MultiAddress::Address20(_) => 4u8.encode_to(dest),
		}

		match self {
			MultiAddress::Id(x) => x.encode_to(dest),
			MultiAddress::Index(x) => x.encode_to(dest),
			MultiAddress::Raw(x) => x.encode_to(dest),
			MultiAddress::Address32(x) => x.encode_to(dest),
			MultiAddress::Address20(x) => x.encode_to(dest),
		}
	}
}

#[repr(u8)]
#[derive(Debug, Clone)]
pub enum MultiSignature {
	/// An Ed25519 signature.
	Ed25519([u8; 64]) = 0,
	/// An Sr25519 signature.
	Sr25519([u8; 64]) = 1,
	/// An ECDSA/SECP256k1 signature (a 512-bit value, plus 8 bits for recovery ID).
	Ecdsa([u8; 65]) = 2,
}

impl Encode for MultiSignature {
	fn size_hint(&self) -> usize {
		let size = match self {
			MultiSignature::Ed25519(x) => x.size_hint(),
			MultiSignature::Sr25519(x) => x.size_hint(),
			MultiSignature::Ecdsa(x) => x.size_hint(),
		};

		size + 1
	}

	fn encode_to<T: parity_scale_codec::Output + ?Sized>(&self, dest: &mut T) {
		match self {
			MultiSignature::Ed25519(_) => 0u8.encode_to(dest),
			MultiSignature::Sr25519(_) => 1u8.encode_to(dest),
			MultiSignature::Ecdsa(_) => 2u8.encode_to(dest),
		}

		match self {
			MultiSignature::Ed25519(x) => x.encode_to(dest),
			MultiSignature::Sr25519(x) => x.encode_to(dest),
			MultiSignature::Ecdsa(x) => x.encode_to(dest),
		}
	}
}

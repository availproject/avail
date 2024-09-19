use parity_scale_codec::Encode;

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

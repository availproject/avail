mod account;
mod derive_junction;
mod secret_uri;
mod sr25519;
mod ss58;

pub use account::AccountId;
pub use secret_uri::SecretUri;
pub use sr25519::{Keypair, PublicKey, Signature};
pub use ss58::Ss58Codec;

#[inline(always)]
fn blake2<const N: usize>(data: &[u8]) -> [u8; N] {
	blake2b_simd::Params::new()
		.hash_length(N)
		.hash(data)
		.as_bytes()
		.try_into()
		.expect("slice is always the necessary length")
}

/// Do a Blake2 256-bit hash and return result.
pub fn blake2_256(data: &[u8]) -> [u8; 32] {
	blake2(data)
}

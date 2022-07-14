//! Various utilities for manipulating Ethereum related dat
use sp_core::H256;
use tiny_keccak::{Hasher, Keccak};

const PREFIX: &str = "\x19Ethereum Signed Message:\n";

/// Hash a message according to EIP-191.
///
/// The data is a UTF-8 encoded string and will enveloped as follows:
/// `"\x19Ethereum Signed Message:\n" + message.length + message` and hashed
/// using keccak256.
pub fn hash_message<S>(message: S) -> H256
where
	S: AsRef<[u8]>,
{
	let message = message.as_ref();

	let mut eth_message = format!("{}{}", PREFIX, message.len()).into_bytes();
	eth_message.extend_from_slice(message);

	keccak256(&eth_message).into()
}

/// Compute the Keccak-256 hash of input bytes.
// TODO: Add Solidity Keccak256 packing support
pub fn keccak256<S>(bytes: S) -> [u8; 32]
where
	S: AsRef<[u8]>,
{
	let mut output = [0u8; 32];
	let mut hasher = Keccak::v256();
	hasher.update(bytes.as_ref());
	hasher.finalize(&mut output);
	output
}

//! Various utilities for manipulating Ethereum related data
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
	let eth_message = format!("{}{}", PREFIX, message.len()).into_bytes();

	let mut output = [0u8; 32];
	let mut hasher = Keccak::v256();
	hasher.update(eth_message.as_ref());
	hasher.update(message);
	hasher.finalize(&mut output);
	output.into()
}

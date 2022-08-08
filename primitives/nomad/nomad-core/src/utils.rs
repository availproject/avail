use sp_core::H256;
use tiny_keccak::{Hasher, Keccak};

/// Computes hash of home domain concatenated with "NOMAD"
pub fn home_domain_hash(home_domain: u32) -> H256 {
	let mut output = [0u8; 32];
	let mut hasher = Keccak::v256();
	hasher.update(home_domain.to_be_bytes().as_ref());
	hasher.update("NOMAD".as_bytes());
	hasher.finalize(&mut output);
	output.into()
}

/// Compute the Keccak-256 hash of input bytes.
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

/// Destination and destination-specific nonce combined in single field (
/// (destination << 32) & nonce)
pub fn destination_and_nonce(destination: u32, nonce: u32) -> u64 {
	assert!(destination < u32::MAX);
	assert!(nonce < u32::MAX);
	((destination as u64) << 32) | nonce as u64
}

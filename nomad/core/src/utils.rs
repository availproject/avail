use sp_core::H256;

const NOMAD_SUFFIX: &[u8] = b"NOMAD";
const ETH_PREFIX: &[u8] = b"\x19Ethereum Signed Message:\n32";

/// Computes hash of home domain concatenated with "NOMAD"
pub fn home_domain_hash(home_domain: u32) -> H256 {
	keccak256_concat!(home_domain.to_be_bytes(), NOMAD_SUFFIX)
}

/// Hash a message according to EIP-191 with the ethereum signed message prefix.
pub fn to_eth_signed_message_hash(hash: &H256) -> H256 {
	keccak256_concat!(ETH_PREFIX, hash)
}

/// Destination and destination-specific nonce combined in single field (
/// (destination << 32) & nonce)
pub fn destination_and_nonce(destination: u32, nonce: u32) -> u64 {
	((destination as u64) << 32) | nonce as u64
}

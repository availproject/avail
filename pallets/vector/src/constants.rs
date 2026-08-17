use hex_literal::hex;
use sp_core::H256;

// Vector initial configuration
pub const BROADCASTER_DOMAIN: u32 = 2;
pub const BROADCASTER: H256 = H256::zero();
pub const SLOTS_PER_PERIOD: u64 = 8192;
pub const FINALITY_THRESHOLD: u16 = 342;
pub const PERIOD: u64 = 1151;
pub const GENESIS_VALIDATOR_ROOT: H256 = H256(hex!(
	"4b363db94e286120d76eb905340fdd4e54bfe9f06bf33ff6cf5ad27f511bfe95"
));
pub const GENESIS_TIME: u64 = 1606824023;
pub const SECONDS_PER_SLOT: u64 = 12;
pub const SOURCE_CHAIN_ID: u64 = 1;

/// Trusted Ethereum mainnet anchor used to bootstrap the SP1 light client.
///
/// These values are also pinned by the committed SP1 proof fixture: the first update
/// starts from this slot and committee, and proves the next finalized header.
pub const SP1_HEAD: u64 = 14_823_232;
pub const SP1_HEADER: H256 = H256(hex!(
	"13336665133b7e26ea5d56b3d3fc3d46eec523881be65d5fb8dcaf4e478f3bad"
));
pub const SP1_SYNC_COMMITTEE_HASH: H256 = H256(hex!(
	"f8ee980d80cd1e3e48033ff8bb0a594cca36772a7f3e457513d3ae1bfc74e130"
));

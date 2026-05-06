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

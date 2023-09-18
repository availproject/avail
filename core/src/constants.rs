use core::num::NonZeroU32;
use sp_arithmetic::Perbill;
use static_assertions::const_assert;

pub mod well_known_keys {
	/// Public params used to generate Kate commitment
	pub const KATE_PUBLIC_PARAMS: &[u8] = b":kate_public_params:";
}

/// We allow `Normal` extrinsics to fill up the block up to 85%, the rest can be used
/// by  Operational  extrinsics.
pub const NORMAL_DISPATCH_RATIO: Perbill = Perbill::from_percent(85);

const_assert!(BLOCK_CHUNK_SIZE.get() > 0);
pub const BLOCK_CHUNK_SIZE: NonZeroU32 = unsafe { NonZeroU32::new_unchecked(32) };

/// Money matters.
pub mod currency {

	pub type Balance = u128;

	/// AVL has 18 decimal positions.
	pub const AVL: Balance = 1_000_000_000_000_000_000;

	/// Cents of AVL has 16 decimal positions (100 Cents = $1)
	/// 1 DOLLARS = 10_000_000_000_000_000
	pub const CENTS: Balance = AVL / 100;

	/// Millicent of AVL has 13 decimal positions( 100 mCents = 1 cent).
	pub const MILLICENTS: Balance = CENTS / 1_000;
}

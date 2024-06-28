use core::num::NonZeroU32;
use sp_arithmetic::Perbill;
use static_assertions::{const_assert, const_assert_eq};

/// We allow `Normal` extrinsics to fill up the block up to 85%, the rest can be used
/// by  Operational  extrinsics.
pub const NORMAL_DISPATCH_RATIO: Perbill = Perbill::from_percent(85);

/// We allow `Normal` data submissions to fill up the matrix up to 100%, there is no
/// Operational or mandatory extrinsic that goes in the matrix.
pub const DA_DISPATCH_RATIO: Perbill = Perbill::from_percent(100);

const_assert!(BLOCK_CHUNK_SIZE.get() > 0);
pub const BLOCK_CHUNK_SIZE: NonZeroU32 = unsafe { NonZeroU32::new_unchecked(32) };

/// Money matters.
pub mod currency {

	pub type Balance = u128;

	/// `AVAIL` has 18 decimal positions.
	pub const AVAIL: Balance = 1_000_000_000_000_000_000;

	/// Cents of AVAIL has 16 decimal positions (100 Cents = 1)
	pub const CENTS: Balance = AVAIL / 100;

	/// Millicent of AVAIL has 13 decimal positions( 1000 mCents = 1 cent).
	pub const MILLICENTS: Balance = CENTS / 1_000;

	/// `MILLI_AVAIL` has 15 decimal positions
	pub const MILLI_AVAIL: Balance = AVAIL / 1_000;

	/// `MICRO_AVAIL` has 12 decimal positions
	pub const MICRO_AVAIL: Balance = MILLI_AVAIL / 1_000;

	/// `NANO_AVAIL` has 9 decimal positions
	pub const NANO_AVAIL: Balance = MICRO_AVAIL / 1_000;

	/// `PICO_AVAIL` has 6 decimal positions
	pub const PICO_AVAIL: Balance = NANO_AVAIL / 1_000;
}

pub mod kate {
	use super::*;
	pub const EXTENSION_FACTOR: u32 = 2;
	pub const COMMITMENT_SIZE: usize = 48;
	pub const DATA_CHUNK_SIZE: usize = 31;
	pub const CHUNK_SIZE: usize = 32;

	const_assert_eq!(DATA_CHUNK_SIZE, CHUNK_SIZE - 1);
	const_assert_eq!(CHUNK_SIZE, BLOCK_CHUNK_SIZE.get() as usize);
}

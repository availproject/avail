#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for `pallet_fusion`.
pub trait WeightInfo {
	fn create_currency() -> Weight;
}

/// Weights for `pallet_fusion` using the Avail node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	fn create_currency() -> Weight {
		Weight::from_parts(10_000, 0)
	}
}

// For backwards compatibility and tests.
impl WeightInfo for () {
	fn create_currency() -> Weight {
		Weight::from_parts(10_000, 0)
	}
}
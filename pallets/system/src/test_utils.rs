use codec::Decode;
use frame_support::traits::Randomness;
use sp_runtime::traits::TrailingZeroInput;

use crate::*;

/// Provides an implementation of [`frame_support::traits::Randomness`] that should only be used in
/// tests!
pub struct TestRandomness<T>(sp_std::marker::PhantomData<T>);

impl<Output, T> Randomness<Output, BlockNumberFor<T>> for TestRandomness<T>
where
	Output: Decode + Default,
	T: Config,
{
	fn random(subject: &[u8]) -> (Output, BlockNumberFor<T>) {
		(
			Output::decode(&mut TrailingZeroInput::new(subject)).unwrap_or_default(),
			crate::Pallet::<T>::block_number(),
		)
	}
}

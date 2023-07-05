use frame_support::traits::Randomness;

/// Provides an implementation of [`frame_support::traits::Randomness`] that should only be used in
/// on Benchmarks!
pub struct BenchRandomness<T>(sp_std::marker::PhantomData<T>);

impl<Output, T> Randomness<Output, T> for BenchRandomness<T>
where
	Output: codec::Decode + Default,
	T: Default,
{
	fn random(subject: &[u8]) -> (Output, T) {
		use sp_runtime::traits::TrailingZeroInput;

		(
			Output::decode(&mut TrailingZeroInput::new(subject)).unwrap_or_default(),
			T::default(),
		)
	}
}

#![cfg(feature = "runtime-benchmarks")]

use super::*;
use crate::Pallet;
use frame_benchmarking::{
	impl_benchmark_test_suite, v1::BenchmarkError, v2::*, whitelisted_caller,
};
use frame_system::RawOrigin;

#[benchmarks(
	where <T as Config>::RuntimeCall: From<frame_system::Call<T>>,
)]
mod benchmarks {
	use super::*;

	#[benchmark]
	fn claim_era_fusion_reward() -> Result<(), BenchmarkError> {
		let caller = whitelisted_caller::<T::AccountId>();
		let origin = RawOrigin::Signed(caller);

		#[extrinsic_call]
		_(origin, 0);

		Ok(())
	}

	impl_benchmark_test_suite!(Pallet, crate::mock::new_test_ext(), crate::mock::Test);
}

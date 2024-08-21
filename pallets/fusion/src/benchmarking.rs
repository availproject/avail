#![cfg(feature = "runtime-benchmarks")]

use super::*;
use crate::Pallet;
use frame_benchmarking::{
	impl_benchmark_test_suite, v1::BenchmarkError, v2::*, whitelisted_caller,
};
use frame_support::storage::bounded_vec::BoundedVec;
use frame_system::RawOrigin;

#[benchmarks(
	where <T as Config>::RuntimeCall: From<frame_system::Call<T>>,
)]
mod benchmarks {
	use super::*;

	#[benchmark]
	fn set_fusion_pool() -> Result<(), BenchmarkError> {
		let caller = whitelisted_caller::<T::AccountId>();
		let origin = RawOrigin::Signed(caller.clone());

		#[extrinsic_call]
		_(origin, caller, BoundedVec::default(), BoundedVec::default());

		Ok(())
	}

	impl_benchmark_test_suite!(Pallet, crate::mock::new_test_ext(), crate::mock::Test);
}

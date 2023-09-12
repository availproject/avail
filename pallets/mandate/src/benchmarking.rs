#![cfg(feature = "runtime-benchmarks")]

use super::*;
use crate::Pallet;
use frame_benchmarking::{impl_benchmark_test_suite, v1::BenchmarkError, v2::*};
use frame_system::RawOrigin;

#[benchmarks(
	where <T as Config>::RuntimeCall: From<frame_system::Call<T>>,
)]
mod benchmarks {
	use super::*;

	#[benchmark]
	fn mandate() -> Result<(), BenchmarkError> {
		let call: <T as Config>::RuntimeCall = frame_system::Call::remark { remark: vec![] }.into();

		#[extrinsic_call]
		_(RawOrigin::Root, Box::new(call));

		Ok(())
	}

	impl_benchmark_test_suite!(Pallet, crate::mock::new_test_ext(), crate::mock::Test);
}

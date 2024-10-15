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
	fn kick_user() -> Result<(), BenchmarkError> {
		let origin = RawOrigin::Root;

		#[extrinsic_call]
		_(origin, EvmAddress::zero());

		Ok(())
	}

	impl_benchmark_test_suite!(Pallet, crate::mock::new_test_ext(), crate::mock::Test);
}

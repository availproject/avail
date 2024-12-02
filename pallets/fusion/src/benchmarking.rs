#![cfg(feature = "runtime-benchmarks")]

use super::*;
use crate::Pallet;
use frame_benchmarking::{impl_benchmark_test_suite, v1::BenchmarkError, v2::*};
use frame_system::RawOrigin;
use sp_core::H160;

#[benchmarks(
	where <T as Config>::RuntimeCall: From<frame_system::Call<T>>,
)]
mod benchmarks {
	use super::*;

	#[benchmark]
	fn set_slash_destination() -> Result<(), BenchmarkError> {
		let origin = RawOrigin::Root;

		#[extrinsic_call]
		_(origin, Some(FusionAddress::new_evm(H160::zero())), None);

		Ok(())
	}

	impl_benchmark_test_suite!(Pallet, crate::mock::new_test_ext(), crate::mock::Test);
}

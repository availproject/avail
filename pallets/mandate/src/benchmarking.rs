#![cfg(feature = "runtime-benchmarks")]

use frame_benchmarking::benchmarks;
use frame_system::RawOrigin;
use sp_std::prelude::*;

use super::*;
#[allow(unused)]
use crate::Pallet as Mandate;

benchmarks! {
	where_clause { where <T as Config>::RuntimeCall: From<frame_system::Call<T>> }
	mandate {
		let call: <T as Config>::RuntimeCall = frame_system::Call::remark { remark: vec![] }.into();
	}: _(RawOrigin::Root, Box::new(call))

	impl_benchmark_test_suite!(Mandate, crate::mock::new_test_ext(), crate::mock::Test);
}

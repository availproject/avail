#![cfg(feature = "runtime-benchmarks")]

use super::*;
use crate::Pallet;
use frame_benchmarking::{
	account as benchmark_account, impl_benchmark_test_suite, v1::BenchmarkError, v2::*,
};
use frame_system::RawOrigin;

pub fn get_account<T: Config>(name: &'static str) -> T::AccountId {
	let account: T::AccountId = benchmark_account(name, 0, 0);
	account
}

#[benchmarks(
	where <T as Config>::RuntimeCall: From<frame_system::Call<T>>,
)]
mod benchmarks {
	use super::*;

	#[benchmark]
	fn wrap() -> Result<(), BenchmarkError> {
		let caller = whitelisted_caller::<T::AccountId>();
		let origin = RawOrigin::Signed(caller.clone());
		let call: <T as Config>::RuntimeCall = frame_system::Call::remark { remark: vec![] }.into();

		#[extrinsic_call]
		_(origin, Box::new(call));

		Ok(())
	}

	#[benchmark]
	fn set_proxy_account() -> Result<(), BenchmarkError> {
		let alice: T::AccountId = get_account::<T>("ALICE");

		#[extrinsic_call]
		_(RawOrigin::Root, Some(alice));

		Ok(())
	}

	impl_benchmark_test_suite!(Pallet, crate::mock::new_test_ext(), crate::mock::Test);
}

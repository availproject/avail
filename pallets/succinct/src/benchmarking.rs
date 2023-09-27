// #![cfg(feature = "runtime-benchmarks")]
//
// use super::*;
// use crate::Pallet;
// use frame_benchmarking::{impl_benchmark_test_suite, v1::BenchmarkError, v2::*};
// use frame_system::RawOrigin;
// use hex_literal::hex;
//
// #[cfg(feature = "runtime-benchmarks")]
// use frame_benchmarking::{benchmarks, whitelisted_caller};
// #[cfg(feature = "runtime-benchmarks")]
// use frame_support::{assert_ok, traits::Get as _, BoundedVec};
// #[cfg(feature = "runtime-benchmarks")]
// use sp_core::{H160, H256, U256};
// #[cfg(feature = "runtime-benchmarks")]
// use sp_std::{iter::repeat, vec::Vec};
//
// #[cfg(feature = "runtime-benchmarks")]
// use crate::*;
//
//
// #[cfg(feature = "runtime-benchmarks")]
// benchmarks! {
// 	where_clause { where [u8; 32]: From<<T as frame_system::Config>::AccountId> }
//
// 	set_updater {
//
// 		let new_updater = H256(hex!("d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d")).into();
//
// 	let origin = RawOrigin::Root;
// 		// let signed_update = expected_signed_update();
// 	}: _(origin, new_updater)
// 	verify {
// 		assert!(SuccinctCfg::<T>::get().updater == new_updater);
// 	}
//
// }

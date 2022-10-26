#![cfg(feature = "runtime-benchmarks")]

use frame_benchmarking::{benchmarks, whitelisted_caller};
use frame_support::traits::Get;
use frame_system::RawOrigin;
use sp_std::{fmt::Debug, iter::repeat, vec::Vec};

use crate::*;

/// Generates a bounded container of `len` elements.
fn generate_bounded<B: TryFrom<Vec<u8>>>(len: u32) -> B
where
	B: TryFrom<Vec<u8>>,
	<B as TryFrom<Vec<u8>>>::Error: Debug,
{
	let raw = repeat(b'X').take(len as usize).collect::<Vec<_>>();
	B::try_from(raw).expect("Bounded fixed by `len` parameter .qed")
}

benchmarks! {
	create_application_key {
		let max_key_len = T::MaxAppKeyLength::get();

		let key = generate_bounded::<AppKeyFor<T>>(max_key_len);
		let key_verify = key.clone();

		let caller = whitelisted_caller::<T::AccountId>();
		let origin = RawOrigin::Signed(caller.clone());

	}: create_application_key(origin, key)
	verify {
		let info = Pallet::<T>::application_key(&key_verify);
		assert_eq!( info, Some(AppKeyInfoFor::<T> { owner: caller, id: 3.into()}));
	}

	submit_data {
		let i in 0..T::MaxAppDataLength::get();

		let data = generate_bounded::<AppDataFor<T>>(i);
		let origin = RawOrigin::Signed(whitelisted_caller::<T::AccountId>());

	}: submit_data(origin, data)

	submit_block_length_proposal {
		let rows = T::MaxBlockRows::get();
		let cols = T::MaxBlockCols::get();
		let origin = RawOrigin::Signed(whitelisted_caller::<T::AccountId>());
	}: submit_block_length_proposal(origin, rows, cols)
}

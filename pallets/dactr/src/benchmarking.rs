#![cfg(feature = "runtime-benchmarks")]

use codec::{Decode, Encode};
use da_primitives::{asdr::AppUncheckedExtrinsic, OpaqueExtrinsic};
use frame_benchmarking::{benchmarks, whitelisted_caller};
use frame_support::traits::Get;
use frame_system::{submitted_data, RawOrigin};
use sp_std::{
	fmt::Debug,
	iter::{once, repeat},
	vec::Vec,
};

use crate::{pallet::Call as DACall, *};

type RuntimeCallOf<T> = <T as frame_system::Config>::RuntimeCall;
type SignedExtensionUnused<T> = (
	frame_system::CheckNonZeroSender<T>,
	frame_system::CheckSpecVersion<T>,
	frame_system::CheckTxVersion<T>,
	frame_system::CheckGenesis<T>,
	frame_system::CheckEra<T>,
	frame_system::CheckNonce<T>,
	frame_system::CheckWeight<T>,
);

fn repeat_bytes(byte: u8, len: u32) -> Vec<u8> {
	repeat(byte).take(len as usize).collect::<Vec<_>>()
}

/// Generates a bounded container of `len` elements.
fn generate_bounded<B: TryFrom<Vec<u8>>>(len: u32) -> B
where
	B: TryFrom<Vec<u8>>,
	<B as TryFrom<Vec<u8>>>::Error: Debug,
{
	let raw = repeat_bytes(b'X', len);
	B::try_from(raw).expect("Bounded fixed by `len` parameter .qed")
}

fn submit_data_ext<T: Config>(data: AppDataFor<T>) -> OpaqueExtrinsic
where
	T: Config + Send + Sync,
	RuntimeCallOf<T>: From<DACall<T>>,
{
	let call = DACall::submit_data::<T> { data };
	let runtime_call: <T as frame_system::Config>::RuntimeCall = call.into();
	let unchecked_extrinsic =
		AppUncheckedExtrinsic::<(), RuntimeCallOf<T>, (), SignedExtensionUnused<T>>::new_unsigned(
			runtime_call,
		);
	let encoded_call = unchecked_extrinsic.encode();

	OpaqueExtrinsic::decode(&mut encoded_call.as_slice())
		.expect("Unchecked is always decoded as opaque .qed")
}

benchmarks! {
	where_clause {
		where <T as frame_system::Config>::RuntimeCall: From<DACall<T>>,
			  T: Send + Sync
	}

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

	data_root {
		let i in 0..T::MaxAppDataLength::get();

		let data = generate_bounded::<AppDataFor<T>>(i);
		let opaque = submit_data_ext::<T>(data);

	}:{
		let _data_root = submitted_data::extrinsics_root::<T::SubmittedDataExtractor, _>(once(&opaque));
	}

	/*
	commitment_builder{

		let seed = [0u8;32];
		let col:u32 = 512;
		let row:u32 = 256;
		let siz:usize = col as usize*row as usize * (32usize -2);
		let data = vec![0u8;siz];
		let tx = AppExtrinsic::from(data.to_vec());
		let txs = [tx];

	}par_build_commitments((row as usize).into(), (col as usize).into(), 32usize, &txs, seed)

	submit_block_length_proposal {
		let rows = T::MaxBlockRows::get();
		let cols = T::MaxBlockCols::get();
		let origin = RawOrigin::Root;
	}: submit_block_length_proposal(origin, rows, cols)
	*/
}

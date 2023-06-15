#![cfg(feature = "runtime-benchmarks")]

use codec::{Decode, Encode};
use da_primitives::{
	asdr::{AppExtrinsic, AppUncheckedExtrinsic},
	BlockLengthColumns, BlockLengthRows, OpaqueExtrinsic, BLOCK_CHUNK_SIZE,
};
use frame_benchmarking::{benchmarks, vec, whitelisted_caller};
use frame_support::{log::info, traits::Get};
use frame_system::{submitted_data, RawOrigin};
#[cfg(feature = "std")]
use kate::com::par_build_commitments;
use kate::metrics::IgnoreMetrics;
use scale_info::{StaticTypeInfo, TypeInfo};
use sp_runtime::{
	traits::{DispatchInfoOf, Dispatchable, SignedExtension},
	transaction_validity::{TransactionValidity, TransactionValidityError},
};
use sp_std::{
	fmt::Debug,
	iter::{once, repeat},
	vec::Vec,
};

use crate::{pallet::Call as DACall, *};

type RuntimeCallOf<T> = <T as frame_system::Config>::RuntimeCall;

#[derive(PartialEq, Eq, Clone, Debug, Encode, Decode, TypeInfo)]
pub struct SignedExtensionUnused<
	T: frame_system::Config + Send + Sync + pallet::Config + Debug + StaticTypeInfo,
>(
	pub  (
		frame_system::CheckNonZeroSender<T>,
		frame_system::CheckSpecVersion<T>,
		frame_system::CheckTxVersion<T>,
		frame_system::CheckGenesis<T>,
		frame_system::CheckEra<T>,
		frame_system::CheckNonce<T>,
		frame_system::CheckWeight<T>,
	),
);

impl<T: frame_system::Config + Send + Sync + pallet::Config + Debug + StaticTypeInfo>
	SignedExtension for SignedExtensionUnused<T>
where
	T: frame_system::Config + Send + Sync + pallet::Config + Debug + StaticTypeInfo,
	RuntimeCallOf<T>: Dispatchable<RuntimeOrigin = T::RuntimeOrigin> + From<DACall<T>>,
{
	type AccountId = T::AccountId;
	type AdditionalSigned = ();
	type Call = RuntimeCallOf<T>;
	type Pre = ();

	const IDENTIFIER: &'static str = "SignedExtensionUnused";

	fn additional_signed(&self) -> Result<Self::AdditionalSigned, TransactionValidityError> {
		Ok(())
	}

	fn pre_dispatch(
		self,
		_who: &Self::AccountId,
		_call: &Self::Call,
		_info: &DispatchInfoOf<Self::Call>,
		_len: usize,
	) -> Result<(), TransactionValidityError> {
		Ok(())
	}

	fn validate(
		&self,
		_who: &Self::AccountId,
		_call: &Self::Call,
		_info: &DispatchInfoOf<Self::Call>,
		_len: usize,
	) -> TransactionValidity {
		Ok(Default::default())
	}
}

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

fn submit_data_ext<
	T: frame_system::Config + Send + Sync + pallet::Config + Debug + StaticTypeInfo,
>(
	data: AppDataFor<T>,
) -> OpaqueExtrinsic
where
	T: frame_system::Config + Send + Sync + pallet::Config + Debug + StaticTypeInfo,
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
			  T: Send + Sync + Debug + StaticTypeInfo
	}

	// create_application_key {
	// 	let max_key_len = T::MaxAppKeyLength::get();

	// 	let key = generate_bounded::<AppKeyFor<T>>(max_key_len);
	// 	let key_verify = key.clone();

	// 	let caller = whitelisted_caller::<T::AccountId>();
	// 	let origin = RawOrigin::Signed(caller.clone());

	// }: create_application_key(origin, key)
	// verify {
	// 	let info = Pallet::<T>::application_key(&key_verify);
	// 	assert_eq!( info, Some(AppKeyInfoFor::<T> { owner: caller, id: 3.into()}));
	// }

	// submit_block_length_proposal {
	// 	let rows = T::MaxBlockRows::get().0;
	// 	let cols = T::MaxBlockCols::get().0;
	// 	let origin = RawOrigin::Root;
	// }: submit_block_length_proposal(origin, rows, cols)

	// submit_data {
	// 	let i in 1..T::MaxAppDataLength::get();

	// 	let data = generate_bounded::<AppDataFor<T>>(i);
	// 	let origin = RawOrigin::Signed(whitelisted_caller::<T::AccountId>());

	// }: submit_data(origin, data)

	// data_root {
	// 	let i in 0..T::MaxAppDataLength::get();

	// 	let data = generate_bounded::<AppDataFor<T>>(i);
	// 	let opaque = submit_data_ext::<T>(data);

	// }:{
	// 	let _data_root =submitted_data::extrinsics_root::<T::SubmittedDataExtractor, _>(once(&opaque));
	// }

	commitment_builder{
		let seed = [0u8;32];
		let i:u32 = 512;// in 32..128;//T::MaxBlockRows::get().0;
		let j:u32 = 256;// in 32..128;//T::MaxBlockCols::get().0;
		// let k in 0..100;//i*j*(32-2);
		let k in 0..T::MaxAppDataLength::get();
		let l in 0..128;

		info!("i: {} - j: {}", i, j);
		info!("k: {} - l: {}", k, l);

		let row = BlockLengthRows(i);
		let col = BlockLengthColumns(j);
		// let data = vec![0u8; k as usize];
		let data:Vec<u8> = generate_bounded::<AppDataFor<T>>(k).to_vec();
		let txs = vec![AppExtrinsic::from(data.to_vec()); l.try_into().unwrap()];
		// let tx = AppExtrinsic::from(data.to_vec());
		info!("Launching extrinsic");
	}: {
		#[cfg(feature = "std")]
		let _commitment = par_build_commitments(row, col, BLOCK_CHUNK_SIZE, &txs, seed, &IgnoreMetrics {});
	}

}

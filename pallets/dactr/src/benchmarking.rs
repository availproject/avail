#![cfg(feature = "runtime-benchmarks")]

use avail_core::{
	asdr::AppUncheckedExtrinsic, AppExtrinsic, BlockLengthColumns, BlockLengthRows,
	OpaqueExtrinsic, BLOCK_CHUNK_SIZE, NORMAL_DISPATCH_RATIO,
};
use codec::{Decode, Encode};
use frame_benchmarking::{benchmarks, vec, whitelisted_caller};
use frame_support::{log::info, traits::Get};
use frame_system::{
	header_builder::hosted_header_builder, limits::BlockLength, submitted_data, RawOrigin,
};
use scale_info::{StaticTypeInfo, TypeInfo};
use sp_core::H256;
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

fn prev_power_of_two(n: u32) -> u32 {
	if n.is_power_of_two() {
		n
	} else {
		(n.next_power_of_two()) / 2
	}
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

fn commitment_parameters<T: frame_system::Config + pallet::Config>(
	rows: u32,
	cols: u32,
) -> (Vec<AppExtrinsic>, H256, BlockLength, u32, [u8; 32])
where
	T: frame_system::Config + pallet::Config,
{
	let seed = [0u8; 32];
	let root = H256::zero();
	let block_number: u32 = 0;
	let data_length = T::MaxAppDataLength::get();

	let rows = BlockLengthRows(prev_power_of_two(rows));
	let cols = BlockLengthColumns(cols);

	let mut nb_tx = 4; // Value set depending on MaxAppDataLength (512 kb) to reach 2 mb
	let max_tx: u32 =
		rows.0 * cols.0 * (BLOCK_CHUNK_SIZE.get().checked_sub(2).unwrap()) / data_length;
	if nb_tx > max_tx {
		nb_tx = max_tx;
	}

	let block_length =
		BlockLength::with_normal_ratio(rows, cols, BLOCK_CHUNK_SIZE, NORMAL_DISPATCH_RATIO)
			.unwrap();
	let data: Vec<u8> = generate_bounded::<AppDataFor<T>>(data_length).to_vec();
	let txs = vec![AppExtrinsic::from(data.to_vec()); nb_tx as usize];

	info!("Launching extrinsic with:");
	info!(
		"rows: {} - cols: {} - DataLength: {} - Nb Txs: {}",
		rows.0, cols.0, data_length, nb_tx
	);

	(txs, root, block_length, block_number, seed)
}

benchmarks! {
	where_clause {
		where <T as frame_system::Config>::RuntimeCall: From<DACall<T>>,
			  T: Send + Sync + Debug + StaticTypeInfo
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
		assert_eq!( info, Some(AppKeyInfoFor::<T> { owner: caller, id: AppId(3)}));
	}

	submit_block_length_proposal {
		let rows = T::MaxBlockRows::get().0;
		let cols = T::MaxBlockCols::get().0;
		let origin = RawOrigin::Root;
	}: submit_block_length_proposal(origin, rows, cols)

	submit_data {
		let i in 1..T::MaxAppDataLength::get();

		let data = generate_bounded::<AppDataFor<T>>(i);
		let origin = RawOrigin::Signed(whitelisted_caller::<T::AccountId>());

	}: submit_data(origin, data)

	data_root {
		let i in 0..T::MaxAppDataLength::get();

		let data = generate_bounded::<AppDataFor<T>>(i);
		let opaque = submit_data_ext::<T>(data);

	}:{
		let _data_root =submitted_data::extrinsics_root::<T::SubmittedDataExtractor, _>(once(&opaque));
	}

	// This benchmark is not directly used by extrinsic.
	// It is mostly used to check that the weight is lower or approximately equal the `data_root` benchmark
	data_root_batch {
		let i in 0..(2 * 1024 * 1024);

		let max_tx_size = T::MaxAppDataLength::get();
		let nb_full_tx = i / max_tx_size;
		let remaining_size = i % max_tx_size;
		let mut calls = Vec::with_capacity(nb_full_tx as usize + 1usize);

		// Create the full-sized transactions
		for _ in 0..nb_full_tx {
			let data = generate_bounded::<AppDataFor<T>>(max_tx_size);
			let opaque = submit_data_ext::<T>(data);
			calls.push(opaque);
		}

		// If there is a remaining size, create one more transaction
		if remaining_size > 0 {
			let data = generate_bounded::<AppDataFor<T>>(remaining_size);
			let opaque = submit_data_ext::<T>(data);
			calls.push(opaque);
		}
	}:{
		// TODO: Here the `.iter()` is also accounted in the benchmark. It's unwanted
		// and should be avoidable when we'll use the new benchmarking syntax.
		let _data_root = submitted_data::extrinsics_root::<T::SubmittedDataExtractor, _>(calls.iter());
	}

	commitment_builder_32{
		let i in 32..T::MaxBlockRows::get().0;
		let (txs, root, block_length, block_number, seed) = commitment_parameters::<T>(i, 32);
	}: {
		let _ = hosted_header_builder::build(txs, root, block_length, block_number, seed);
	}

	commitment_builder_64{
		let i in 32..T::MaxBlockRows::get().0;
		let (txs, root, block_length, block_number, seed) = commitment_parameters::<T>(i, 64);
	}: {
		let _ = hosted_header_builder::build(txs, root, block_length, block_number, seed);
	}

	commitment_builder_128{
		let i in 32..T::MaxBlockRows::get().0;
		let (txs, root, block_length, block_number, seed) = commitment_parameters::<T>(i, 128);
	}: {
		let _ = hosted_header_builder::build(txs, root, block_length, block_number, seed);
	}

	commitment_builder_256{
		let i in 32..T::MaxBlockRows::get().0;
		let (txs, root, block_length, block_number, seed) = commitment_parameters::<T>(i, 256);
	}: {
		let _ = hosted_header_builder::build(txs, root, block_length, block_number, seed);
	}
}

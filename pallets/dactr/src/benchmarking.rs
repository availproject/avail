#![cfg(feature = "runtime-benchmarks")]

use super::*;
use crate::Pallet;
use avail_base::HeaderExtensionBuilderData;
use avail_core::{
	asdr::AppUncheckedExtrinsic, AppExtrinsic, BlockLengthColumns, BlockLengthRows,
	BLOCK_CHUNK_SIZE,
};
use codec::{Decode, Encode};
use frame_benchmarking::{
	impl_benchmark_test_suite, v1::BenchmarkError, v2::*, whitelisted_caller,
};
use frame_support::traits::Get;
use frame_system::{
	limits::BlockLength, native::hosted_header_builder::hosted_header_builder, RawOrigin,
};
use kate::Seed;
use scale_info::{StaticTypeInfo, TypeInfo};
use sp_core::H256;
use sp_runtime::{
	traits::{Bounded, DispatchInfoOf, Dispatchable, SignedExtension},
	transaction_validity::{TransactionValidity, TransactionValidityError},
	Perbill,
};
use sp_std::{fmt::Debug, iter::repeat, vec, vec::Vec};

use crate::pallet::Call as DACall;

type RuntimeCallOf<T> = <T as frame_system::Config>::RuntimeCall;

fn assert_last_event<T: Config>(generic_event: <T as Config>::RuntimeEvent) {
	frame_system::Pallet::<T>::assert_last_event(generic_event.into());
}

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
) -> Vec<u8>
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

	unchecked_extrinsic.encode()
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
		BlockLength::with_normal_ratio(rows, cols, BLOCK_CHUNK_SIZE, DA_DISPATCH_RATIO_PERBILL)
			.unwrap();
	let data: Vec<u8> = generate_bounded::<AppDataFor<T>>(data_length).to_vec();
	let txs = vec![AppExtrinsic::from(data.to_vec()); nb_tx as usize];

	(txs, root, block_length, block_number, seed)
}

#[benchmarks(
	where <T as frame_system::Config>::RuntimeCall: From<DACall<T>>, T: Send + Sync + Debug + StaticTypeInfo
)]
mod benchmarks {
	use super::*;

	#[benchmark]
	fn create_application_key() -> Result<(), BenchmarkError> {
		let caller = whitelisted_caller::<T::AccountId>();
		let origin = RawOrigin::Signed(caller.clone());
		let max_key_len = T::MaxAppKeyLength::get();
		let key = generate_bounded::<AppKeyFor<T>>(max_key_len);
		let key_verify = key.clone();

		#[extrinsic_call]
		_(origin, key);

		let info = Pallet::<T>::application_key(key_verify);
		assert_eq!(
			info,
			Some(AppKeyInfoFor::<T> {
				owner: caller,
				id: AppId(10)
			})
		);

		Ok(())
	}

	#[benchmark]
	fn submit_block_length_proposal() -> Result<(), BenchmarkError> {
		let origin = RawOrigin::Root;
		let rows = T::MaxBlockRows::get().0;
		let cols = T::MaxBlockCols::get().0;

		#[extrinsic_call]
		_(origin, rows, cols);

		Ok(())
	}

	#[benchmark]
	fn submit_data(i: Linear<1, { T::MaxAppDataLength::get() }>) -> Result<(), BenchmarkError> {
		let caller = whitelisted_caller::<T::AccountId>();
		let origin = RawOrigin::Signed(caller.clone());
		let data = generate_bounded::<AppDataFor<T>>(i);
		let data_hash = H256(keccak_256(&data));

		#[extrinsic_call]
		_(origin, data);

		assert_last_event::<T>(
			Event::DataSubmitted {
				who: caller,
				data_hash,
			}
			.into(),
		);
		Ok(())
	}

	#[benchmark]
	fn set_application_key() -> Result<(), BenchmarkError> {
		let origin = RawOrigin::Root;
		let max_key_len = T::MaxAppKeyLength::get();
		let old_key = AppKeyFor::<T>::try_from(b"Avail".to_vec()).unwrap();
		let new_key = generate_bounded::<AppKeyFor<T>>(max_key_len);
		let key_verify = new_key.clone();

		#[extrinsic_call]
		_(origin, old_key, new_key);

		let _info = Pallet::<T>::application_key(key_verify).unwrap();
		Ok(())
	}

	#[benchmark]
	fn data_root(i: Linear<0, { T::MaxAppDataLength::get() }>) -> Result<(), BenchmarkError> {
		let data = generate_bounded::<AppDataFor<T>>(i);
		let opaque = submit_data_ext::<T>(data);

		#[block]
		{
			HeaderExtensionBuilderData::from_raw_extrinsics::<T::HeaderExtensionDataFilter>(
				1u32,
				&vec![opaque],
				1024,
				4069,
			)
			.data_root();
		}

		Ok(())
	}

	// This benchmark is not directly used by extrinsic.
	// It is mostly used to check that the weight is lower or approximately equal the `data_root` benchmark
	#[benchmark]
	fn data_root_batch(i: Linear<0, { 2 * 1024 * 1024 }>) -> Result<(), BenchmarkError> {
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

		#[block]
		{
			HeaderExtensionBuilderData::from_raw_extrinsics::<T::HeaderExtensionDataFilter>(
				1u32, &calls, 1024, 4069,
			)
			.data_root();
		}

		Ok(())
	}

	#[benchmark]
	fn set_submit_data_fee_modifier() -> Result<(), BenchmarkError> {
		let origin = RawOrigin::Root;
		let value = DispatchFeeModifier {
			weight_maximum_fee: Some(100),
			weight_fee_divider: Some(99),
			weight_fee_multiplier: Some(98),
		};

		#[extrinsic_call]
		_(origin, value);

		Ok(())
	}

	#[benchmark]
	fn submit_blob_metadata(s: Linear<1, { 31 * 1024 * 1024 }>) -> Result<(), BenchmarkError> {
		let caller = whitelisted_caller::<T::AccountId>();
		let origin = RawOrigin::Signed(caller.clone());

		let blob_hash = H256::repeat_byte((s + 1) as u8);

		let block_length = frame_system::Pallet::<T>::block_length();
		let data = vec![0u8; s as usize];
		let commitment = crate::extensions::native::hosted_commitment_builder::build_da_commitments(
			&data,
			block_length.cols.0,
			block_length.rows.0,
			Seed::default(),
		);
		debug_assert!(!commitment.is_empty());

		#[extrinsic_call]
		_(origin, blob_hash, s.into(), commitment);

		assert_last_event::<T>(
			Event::SubmitBlobMetadataRequest {
				who: caller,
				blob_hash,
			}
			.into(),
		);

		Ok(())
	}

	#[benchmark]
	fn submit_blob_txs_summary(n: Linear<1, 1_000>) -> Result<(), BenchmarkError> {
		let origin = RawOrigin::None;

		let summaries: Vec<crate::BlobTxSummaryRuntime> = (0..n)
			.map(|i| crate::BlobTxSummaryRuntime {
				hash: H256::repeat_byte((i + 1) as u8),
				tx_index: i as u32,
				success: i % 2 == 0,
				reason: if i % 3 == 0 {
					Some("bench".into())
				} else {
					None
				},
				ownership: Vec::new(),
			})
			.collect();

		let total_blob_size: u64 = (n as u64) * 1024;
		let nb_blobs: u32 = n as u32;

		#[extrinsic_call]
		_(origin, total_blob_size, nb_blobs, summaries);

		Ok(())
	}

	#[benchmark]
	fn set_blob_runtime_parameters() -> Result<(), BenchmarkError> {
		let origin = RawOrigin::Root;

		let max_blob_size = Some(10 * 1024 * 1024);
		let min_blob_holder_percentage = Some(Perbill::from_percent(5));
		let min_blob_holder_count = Some(3);
		let blob_ttl = Some(2_000);
		let temp_blob_ttl = Some(60);
		let min_tx_validity = Some(10);
		let max_tx_validity = Some(120);
		let max_retry = Some(5);
		let max_block_size = Some(1 * 1024 * 1024 * 1024);
		let max_total_old_submission_size = Some(2 * 1024 * 1024);
		let disable_old_da_submission = Some(true);
		let vouch_threshold = Some(1);

		#[extrinsic_call]
		_(
			origin,
			max_blob_size,
			min_blob_holder_percentage,
			min_blob_holder_count,
			blob_ttl,
			temp_blob_ttl,
			min_tx_validity,
			max_tx_validity,
			max_retry,
			max_block_size,
			max_total_old_submission_size,
			disable_old_da_submission,
			vouch_threshold,
		);

		let expected = crate::BlobRuntimeParameters {
			max_blob_size: max_blob_size.unwrap(),
			min_blob_holder_percentage: min_blob_holder_percentage.unwrap(),
			min_blob_holder_count: min_blob_holder_count.unwrap(),
			blob_ttl: blob_ttl.unwrap(),
			temp_blob_ttl: temp_blob_ttl.unwrap(),
			min_transaction_validity: min_tx_validity.unwrap(),
			max_transaction_validity: max_tx_validity.unwrap(),
			max_blob_retry_before_discarding: max_retry.unwrap(),
			max_block_size: max_block_size.unwrap(),
			max_total_old_submission_size: max_total_old_submission_size.unwrap(),
			disable_old_da_submission: disable_old_da_submission.unwrap(),
			vouch_threshold: vouch_threshold.unwrap(),
		};
		assert_last_event::<T>(
			Event::SubmitBlobRuntimeParametersSet {
				new_params: expected,
			}
			.into(),
		);

		Ok(())
	}

	#[benchmark]
	fn register_blob_offence() -> Result<(), BenchmarkError> {
		use sp_core::sr25519::{Public, Signature};
		use sp_runtime::AccountId32;

		let origin = RawOrigin::None;

		let offence_key = crate::OffenceKey {
			kind: crate::BlobOffenceKind::SummaryNbBlobMismatch,
			block_hash: H256::zero(),
			blob_hash: None,
			missing_validator: None,
		};

		let validator = AccountId32::new([1; 32]);
		let validator_account_id = T::AccountId::decode(&mut &validator.encode()[..]).unwrap();
		let block_author = validator.clone();
		let voucher = crate::ValidatorVoucher {
			validator: validator.clone(),
			key: Public::from_h256(H256::zero()),
			session_index: 0,
			signature: Signature::from_raw([0u8; 64]),
			block_author,
		};

		T::Currency::make_free_balance_be(&validator_account_id, BalanceOf::<T>::max_value());

		#[extrinsic_call]
		_(origin, offence_key.clone(), voucher.clone());

		assert_last_event::<T>(
			Event::BlobOffenceReported {
				who: validator_account_id,
				offence_key,
				voucher,
				added: true,
			}
			.into(),
		);

		Ok(())
	}

	#[benchmark]
	fn clear_blob_offence_records() -> Result<(), BenchmarkError> {
		let origin = RawOrigin::Root;

		for _ in 0..10 {
			let key = crate::OffenceKey {
				kind: crate::BlobOffenceKind::SummaryNbBlobMismatch,
				block_hash: H256::zero(),
				blob_hash: Some(H256::zero()),
				missing_validator: None,
			};
			let record = crate::OffenceRecord::<T>::new(
				key.kind.clone(),
				key.block_hash,
				key.blob_hash.clone(),
				key.missing_validator.clone(),
			);
			crate::BlobOffenceRecords::<T>::insert(key, record);
		}

		// Sanity check: ensure something is stored
		assert!(!crate::BlobOffenceRecords::<T>::iter()
			.collect::<Vec<_>>()
			.is_empty());

		#[extrinsic_call]
		_(origin);

		// Verify everything has been cleared
		assert!(crate::BlobOffenceRecords::<T>::iter()
			.collect::<Vec<_>>()
			.is_empty());

		Ok(())
	}

	#[benchmark(extra)]
	fn commitment_builder_64(
		i: Linear<32, { T::MaxBlockRows::get().0 }>,
	) -> Result<(), BenchmarkError> {
		let (txs, root, block_length, block_number, seed) = commitment_parameters::<T>(i, 64);

		#[block]
		{
			hosted_header_builder::build(txs, root, block_length, block_number, seed);
		}

		Ok(())
	}

	#[benchmark(extra)]
	fn commitment_builder_128(
		i: Linear<32, { T::MaxBlockRows::get().0 }>,
	) -> Result<(), BenchmarkError> {
		let (txs, root, block_length, block_number, seed) = commitment_parameters::<T>(i, 128);

		#[block]
		{
			hosted_header_builder::build(txs, root, block_length, block_number, seed);
		}

		Ok(())
	}

	#[benchmark(extra)]
	fn commitment_builder_256(
		i: Linear<32, { T::MaxBlockRows::get().0 }>,
	) -> Result<(), BenchmarkError> {
		let (txs, root, block_length, block_number, seed) = commitment_parameters::<T>(i, 256);

		#[block]
		{
			hosted_header_builder::build(txs, root, block_length, block_number, seed);
		}

		Ok(())
	}

	impl_benchmark_test_suite!(Pallet, crate::mock::new_benchmark_ext(), crate::mock::Test);
}

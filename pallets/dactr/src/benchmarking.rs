#![cfg(feature = "runtime-benchmarks")]

use super::*;
use crate::Pallet;
use frame_benchmarking::{v1::BenchmarkError, v2::*, whitelisted_caller};
use frame_support::traits::Get;
use frame_system::{Config as SystemConfig, RawOrigin};
use sp_core::H256;
use sp_runtime::{traits::Bounded, Perbill};
use sp_std::{fmt::Debug, iter::repeat, vec, vec::Vec};

use crate::pallet::Call as DACall;

fn assert_last_event<T: Config>(generic_event: <T as SystemConfig>::RuntimeEvent) {
	frame_system::Pallet::<T>::assert_last_event(generic_event.into());
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

#[benchmarks(
	where <T as frame_system::Config>::RuntimeCall: From<DACall<T>>, T: Send + Sync + Debug
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
	fn submit_blob_metadata(s: Linear<1, { 31 * 1024 * 1024 }>) -> Result<(), BenchmarkError> {
		let caller = whitelisted_caller::<T::AccountId>();
		let origin = RawOrigin::Signed(caller.clone());

		let blob_hash = H256::repeat_byte((s + 1) as u8);

		let data = vec![0u8; s as usize];
		let app_id = AppId(2);
		let commitment =
			crate::extensions::native::hosted_commitment_builder::build_fri_commitments(
				&data,
				crate::Pallet::<T>::fri_params_version(),
			);
		debug_assert!(!commitment.is_empty());

		#[extrinsic_call]
		_(
			origin,
			app_id,
			blob_hash,
			s.into(),
			commitment,
			[0u8; 32],
			[0u8; 16],
		);

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

	impl_benchmark_test_suite!(Pallet, crate::mock::new_benchmark_ext(), crate::mock::Test);
}

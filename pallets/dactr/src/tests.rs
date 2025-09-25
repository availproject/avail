use avail_core::{BlockLengthColumns, BlockLengthRows, BLOCK_CHUNK_SIZE};
use frame_support::{assert_noop, assert_ok, error::BadOrigin};
use frame_system::{limits::BlockLength, RawOrigin};
use sp_core::H256;

use crate::config_preludes::{
	MaxAppDataLength, MaxBlockCols, MaxBlockRows, MinBlockCols, MinBlockRows,
};
use crate::{
	mock::{new_test_ext, DataAvailability, RuntimeEvent, RuntimeOrigin, System, Test},
	AppDataFor, AppKeyFor, AppKeyInfoFor, Event, DA_DISPATCH_RATIO_PERBILL,
};

type Error = crate::Error<Test>;

const ALICE: u64 = 1;

mod create_application_key {
	use super::*;

	#[test]
	fn create_application_key() {
		new_test_ext().execute_with(|| {
			let alice: RuntimeOrigin = RawOrigin::Signed(ALICE).into();
			let new_id = DataAvailability::peek_next_application_id();
			let new_key = AppKeyFor::<Test>::try_from(b"New App".to_vec()).unwrap();

			assert_eq!(DataAvailability::application_key(&new_key), None);
			assert_ok!(DataAvailability::create_application_key(
				alice,
				new_key.clone()
			));
			assert_eq!(
				DataAvailability::application_key(&new_key),
				Some(AppKeyInfoFor::<Test> {
					id: new_id,
					owner: ALICE
				})
			);

			let event = RuntimeEvent::DataAvailability(Event::ApplicationKeyCreated {
				key: new_key,
				owner: ALICE,
				id: new_id,
			});
			System::assert_last_event(event);
		})
	}

	#[test]
	fn app_key_cannot_be_empty() {
		new_test_ext().execute_with(|| {
			let alice: RuntimeOrigin = RawOrigin::Signed(ALICE).into();
			let new_key = AppKeyFor::<Test>::try_from(vec![]).unwrap();

			let err = DataAvailability::create_application_key(alice, new_key);
			assert_noop!(err, Error::AppKeyCannotBeEmpty);
		})
	}

	#[test]
	fn app_key_already_exists() {
		new_test_ext().execute_with(|| {
			let alice: RuntimeOrigin = RawOrigin::Signed(ALICE).into();
			let new_key = AppKeyFor::<Test>::try_from(b"New App".to_vec()).unwrap();

			assert_ok!(DataAvailability::create_application_key(
				alice.clone(),
				new_key.clone()
			));

			let err = DataAvailability::create_application_key(alice, new_key);
			assert_noop!(err, Error::AppKeyAlreadyExists);
		})
	}
}

mod submit_data {
	use super::*;

	#[test]
	fn submit_data() {
		new_test_ext().execute_with(|| {
			let alice: RuntimeOrigin = RawOrigin::Signed(ALICE).into();
			let max_app_key_length: usize = MaxAppDataLength::get().try_into().unwrap();
			let data = AppDataFor::<Test>::try_from(vec![b'X'; max_app_key_length]).unwrap();
			let data_hash = H256(sp_io::hashing::keccak_256(&data));

			assert_ok!(DataAvailability::submit_data(alice, data));

			let event = RuntimeEvent::DataAvailability(Event::DataSubmitted {
				who: ALICE,
				data_hash,
			});
			System::assert_last_event(event);
		})
	}

	#[test]
	fn data_cannot_be_empty() {
		new_test_ext().execute_with(|| {
			let alice: RuntimeOrigin = RawOrigin::Signed(ALICE).into();
			let data = AppDataFor::<Test>::try_from(vec![]).unwrap();

			let err = DataAvailability::submit_data(alice, data);
			assert_noop!(err, Error::DataCannotBeEmpty);
		})
	}

	#[test]
	fn submit_data_too_long() {
		new_test_ext().execute_with(|| {
			// This test could be removed since we use a bounded vec, but due to criticity of this extrinsic, it does not hurt to have it.
			let max_app_key_length: usize = MaxAppDataLength::get().try_into().unwrap();
			let err = AppDataFor::<Test>::try_from(vec![b'X'; max_app_key_length + 1]);
			assert!(err.is_err());
		})
	}
}

mod submit_block_length_proposal {
	use super::*;

	#[test]
	fn submit_block_length_proposal() {
		new_test_ext().execute_with(|| {
			let root: RuntimeOrigin = RawOrigin::Root.into();
			let rows = BlockLengthRows(128);
			let cols = BlockLengthColumns(128);

			assert_ok!(DataAvailability::submit_block_length_proposal(
				root, rows.0, cols.0
			));

			let dynamic_block_length = System::block_length();
			let new_block_length = BlockLength::with_normal_ratio(
				rows,
				cols,
				BLOCK_CHUNK_SIZE,
				DA_DISPATCH_RATIO_PERBILL,
			)
			.unwrap();
			assert_eq!(dynamic_block_length, new_block_length);

			let event =
				RuntimeEvent::DataAvailability(Event::BlockLengthProposalSubmitted { rows, cols });
			System::assert_last_event(event);
		})
	}

	#[test]
	fn submit_block_length_proposal_min() {
		new_test_ext().execute_with(|| {
			let root: RuntimeOrigin = RawOrigin::Root.into();
			let rows = MinBlockRows::get();
			let cols = MinBlockCols::get();

			assert_ok!(DataAvailability::submit_block_length_proposal(
				root, rows.0, cols.0
			));

			let dynamic_block_length = System::block_length();
			let new_block_length = BlockLength::with_normal_ratio(
				rows,
				cols,
				BLOCK_CHUNK_SIZE,
				DA_DISPATCH_RATIO_PERBILL,
			)
			.unwrap();
			assert_eq!(dynamic_block_length, new_block_length);

			let event =
				RuntimeEvent::DataAvailability(Event::BlockLengthProposalSubmitted { rows, cols });
			System::assert_last_event(event);
		})
	}

	#[test]
	fn submit_block_length_proposal_max() {
		new_test_ext().execute_with(|| {
			let root: RuntimeOrigin = RawOrigin::Root.into();
			let rows = MaxBlockRows::get();
			let cols = MaxBlockCols::get();

			assert_ok!(DataAvailability::submit_block_length_proposal(
				root, rows.0, cols.0
			));

			let dynamic_block_length = System::block_length();
			let new_block_length = BlockLength::with_normal_ratio(
				rows,
				cols,
				BLOCK_CHUNK_SIZE,
				DA_DISPATCH_RATIO_PERBILL,
			)
			.unwrap();
			assert_eq!(dynamic_block_length, new_block_length);

			let event =
				RuntimeEvent::DataAvailability(Event::BlockLengthProposalSubmitted { rows, cols });
			System::assert_last_event(event);
		})
	}

	#[test]
	fn bad_origin() {
		new_test_ext().execute_with(|| {
			let alice: RuntimeOrigin = RawOrigin::Signed(ALICE).into();
			let rows = MaxBlockRows::get();
			let cols = MaxBlockCols::get();

			let err = DataAvailability::submit_block_length_proposal(alice, rows.0, cols.0);
			assert_noop!(err, BadOrigin);
		})
	}

	#[test]
	fn block_dimensions_out_of_bounds() {
		new_test_ext().execute_with(|| {
			let root: RuntimeOrigin = RawOrigin::Root.into();
			let rows = MaxBlockRows::get();
			let cols = MaxBlockCols::get();

			let err =
				DataAvailability::submit_block_length_proposal(root.clone(), rows.0 + 1, cols.0);
			assert_noop!(err, Error::BlockDimensionsOutOfBounds);

			let err = DataAvailability::submit_block_length_proposal(root, rows.0, cols.0 + 1);
			assert_noop!(err, Error::BlockDimensionsOutOfBounds);
		})
	}

	#[test]
	fn block_dimensions_too_small() {
		new_test_ext().execute_with(|| {
			let root: RuntimeOrigin = RawOrigin::Root.into();
			let rows = MinBlockRows::get();
			let cols = MinBlockCols::get();

			let err =
				DataAvailability::submit_block_length_proposal(root.clone(), rows.0 - 1, cols.0);
			assert_noop!(err, Error::BlockDimensionsTooSmall);

			let err = DataAvailability::submit_block_length_proposal(root, rows.0, cols.0 - 1);
			assert_noop!(err, Error::BlockDimensionsTooSmall);
		})
	}

	#[test]
	fn not_power_of_two() {
		new_test_ext().execute_with(|| {
			let root: RuntimeOrigin = RawOrigin::Root.into();

			let err = DataAvailability::submit_block_length_proposal(root.clone(), 118, 128);
			assert_noop!(err, Error::NotPowerOfTwo);

			let err = DataAvailability::submit_block_length_proposal(root.clone(), 128, 118);
			assert_noop!(err, Error::NotPowerOfTwo);

			let err = DataAvailability::submit_block_length_proposal(root.clone(), 111, 111);
			assert_noop!(err, Error::NotPowerOfTwo);
		})
	}
}

mod set_application_key {
	use super::*;

	#[test]
	fn set_application_key() {
		new_test_ext().execute_with(|| {
			let root: RuntimeOrigin = RawOrigin::Root.into();

			let old_key = AppKeyFor::<Test>::try_from(b"Avail".to_vec()).unwrap();
			let new_key = AppKeyFor::<Test>::try_from(b"Avail Let's goo".to_vec()).unwrap();

			let old_info = DataAvailability::application_key(&old_key);

			assert_ok!(DataAvailability::set_application_key(
				root,
				old_key.clone(),
				new_key.clone(),
			));

			assert_eq!(DataAvailability::application_key(&new_key), old_info);

			let event =
				RuntimeEvent::DataAvailability(Event::ApplicationKeySet { old_key, new_key });
			System::assert_last_event(event);
		})
	}

	#[test]
	fn app_key_cannot_be_empty() {
		new_test_ext().execute_with(|| {
			let root: RuntimeOrigin = RawOrigin::Root.into();

			let old_key = AppKeyFor::<Test>::try_from(b"".to_vec()).unwrap();
			let new_key = AppKeyFor::<Test>::try_from(b"Avail Let's goo".to_vec()).unwrap();

			let err = DataAvailability::set_application_key(root.clone(), old_key, new_key);
			assert_noop!(err, Error::AppKeyCannotBeEmpty);

			let old_key = AppKeyFor::<Test>::try_from(b"Avail Let's goo".to_vec()).unwrap();
			let new_key = AppKeyFor::<Test>::try_from(b"".to_vec()).unwrap();

			let err = DataAvailability::set_application_key(root, old_key, new_key);
			assert_noop!(err, Error::AppKeyCannotBeEmpty);
		})
	}

	#[test]
	fn app_key_already_exists() {
		new_test_ext().execute_with(|| {
			let root: RuntimeOrigin = RawOrigin::Root.into();

			let old_key = AppKeyFor::<Test>::try_from(b"Avail".to_vec()).unwrap();
			let new_key = AppKeyFor::<Test>::try_from(b"Reserved-1".to_vec()).unwrap();

			let err = DataAvailability::set_application_key(root, old_key, new_key);
			assert_noop!(err, Error::AppKeyAlreadyExists);
		})
	}

	#[test]
	fn unknown_app_key() {
		new_test_ext().execute_with(|| {
			let root: RuntimeOrigin = RawOrigin::Root.into();

			let old_key = AppKeyFor::<Test>::try_from(b"NotExisting".to_vec()).unwrap();
			let new_key = AppKeyFor::<Test>::try_from(b"Hello".to_vec()).unwrap();

			let err = DataAvailability::set_application_key(root, old_key, new_key);
			assert_noop!(err, Error::UnknownAppKey);
		})
	}
}

mod set_submit_data_fee_modifier {
	use super::*;
	use crate::SubmitDataFeeModifier;
	use frame_support::dispatch::DispatchFeeModifier;

	#[test]
	fn default_value() {
		new_test_ext().execute_with(|| {
			let value = SubmitDataFeeModifier::<Test>::get();
			assert_eq!(value.weight_maximum_fee, None);
			assert_eq!(value.weight_fee_divider, None);
			assert_eq!(value.weight_fee_multiplier, None);
		})
	}

	#[test]
	fn only_sudo_can_call_this() {
		new_test_ext().execute_with(|| {
			let alice: RuntimeOrigin = RawOrigin::Signed(ALICE).into();
			let value = SubmitDataFeeModifier::<Test>::get();
			assert!(DataAvailability::set_submit_data_fee_modifier(alice, value).is_err());
		})
	}

	#[test]
	fn set_submit_data_fee_modifier() {
		new_test_ext().execute_with(|| {
			let root: RuntimeOrigin = RawOrigin::Root.into();

			let old_value = SubmitDataFeeModifier::<Test>::get();
			let new_value = DispatchFeeModifier {
				weight_maximum_fee: Some(100),
				weight_fee_divider: Some(100),
				weight_fee_multiplier: Some(100),
			};

			assert_ne!(old_value, new_value);

			assert_ok!(DataAvailability::set_submit_data_fee_modifier(
				root, new_value
			));
			assert_eq!(new_value, SubmitDataFeeModifier::<Test>::get());
		})
	}
}

mod submit_blob_metadata {
	use super::*;

	#[test]
	fn submit_blob_metadata() {
		new_test_ext().execute_with(|| {
			let alice: RuntimeOrigin = RawOrigin::Signed(ALICE).into();
			let blob_hash = H256::random();
			let size: u64 = 1;
			let commitment = vec![1u8];

			assert_ok!(DataAvailability::submit_blob_metadata(
				alice, blob_hash, size, commitment
			));

			let event = RuntimeEvent::DataAvailability(Event::SubmitBlobMetadataRequest {
				who: ALICE,
				blob_hash,
			});
			System::assert_last_event(event);
		})
	}

	#[test]
	fn commitment_cannot_be_empty() {
		new_test_ext().execute_with(|| {
			let alice: RuntimeOrigin = RawOrigin::Signed(ALICE).into();
			let blob_hash = H256::random();
			let size: u64 = 1;
			let commitment = Vec::<u8>::new();

			let err = DataAvailability::submit_blob_metadata(alice, blob_hash, size, commitment);
			assert_noop!(err, Error::CommitmentCannotBeEmpty);
		})
	}

	#[test]
	fn data_cannot_be_empty_1() {
		new_test_ext().execute_with(|| {
			let alice: RuntimeOrigin = RawOrigin::Signed(ALICE).into();
			let blob_hash = H256::random();
			let size: u64 = 0;
			let commitment = vec![1u8];

			let err = DataAvailability::submit_blob_metadata(alice, blob_hash, size, commitment);
			assert_noop!(err, Error::DataCannotBeEmpty);
		})
	}

	#[test]
	fn data_cannot_be_empty_2() {
		new_test_ext().execute_with(|| {
			let alice: RuntimeOrigin = RawOrigin::Signed(ALICE).into();
			let blob_hash = H256::zero();
			let size: u64 = 1;
			let commitment = vec![1u8];

			let err = DataAvailability::submit_blob_metadata(alice, blob_hash, size, commitment);
			assert_noop!(err, Error::DataCannotBeEmpty);
		})
	}
}

mod submit_blob_txs_summary {
	use super::*;

	#[test]
	fn submit_blob_txs_summary() {
		new_test_ext().execute_with(|| {
			let none: RuntimeOrigin = RawOrigin::None.into();

			let s1 = crate::pallet::BlobTxSummaryRuntime {
				hash: H256::random(),
				tx_index: 0,
				success: true,
				reason: None,
				ownership: Vec::new(),
			};
			let s2 = crate::pallet::BlobTxSummaryRuntime {
				hash: H256::random(),
				tx_index: 1,
				success: false,
				reason: Some("example".into()),
				ownership: Vec::new(),
			};

			let total_blob_size: u64 = (2 * H256::random().0.len()) as u64;
			let nb_blobs: u32 = 2;

			assert_ok!(DataAvailability::submit_blob_txs_summary(
				none,
				total_blob_size,
				nb_blobs,
				vec![s1, s2],
			));
		})
	}
}

mod set_blob_runtime_parameters {
	use crate::BlobRuntimeParams;
	use sp_runtime::Perbill;

	use super::*;

	#[test]
	fn set_blob_runtime_parameters() {
		new_test_ext().execute_with(|| {
			let before = BlobRuntimeParams::<Test>::get();

			let root: RuntimeOrigin = RawOrigin::Root.into();

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

			assert_ok!(DataAvailability::set_blob_runtime_parameters(
				root,
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
			));

			let after = BlobRuntimeParams::<Test>::get();
			let expected = crate::pallet::BlobRuntimeParameters {
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
			};

			assert_ne!(before, after);
			assert_eq!(after, expected);

			let event = RuntimeEvent::DataAvailability(Event::SubmitBlobRuntimeParametersSet {
				new_params: expected,
			});
			System::assert_last_event(event);
		})
	}

	#[test]
	fn set_blob_runtime_parameters_noop() {
		new_test_ext().execute_with(|| {
			let root: RuntimeOrigin = RawOrigin::Root.into();

			let before = BlobRuntimeParams::<Test>::get();

			assert_ok!(DataAvailability::set_blob_runtime_parameters(
				root, None, None, None, None, None, None, None, None, None, None, None
			));

			let after = BlobRuntimeParams::<Test>::get();
			assert_eq!(after, before);

			let event = RuntimeEvent::DataAvailability(Event::SubmitBlobRuntimeParametersSet {
				new_params: after,
			});
			System::assert_last_event(event);
		})
	}

	#[test]
	fn set_blob_runtime_parameters_errors() {
		new_test_ext().execute_with(|| {
			let baseline = BlobRuntimeParams::<Test>::get();

			let assert_unchanged = || {
				let now = BlobRuntimeParams::<Test>::get();
				assert_eq!(now, baseline);
			};

			{
				let root: RuntimeOrigin = RawOrigin::Root.into();
				let err = DataAvailability::set_blob_runtime_parameters(
					root,
					Some(32 * 1024 * 1024 + 1),
					None,
					None,
					None,
					None,
					None,
					None,
					None,
					None,
					None,
					None,
				);
				assert_noop!(err, Error::BlobSizeTooLarge);
				assert_unchanged();
			}

			{
				let root: RuntimeOrigin = RawOrigin::Root.into();
				let err = DataAvailability::set_blob_runtime_parameters(
					root,
					None,
					Some(Perbill::from_percent(0)),
					None,
					None,
					None,
					None,
					None,
					None,
					None,
					None,
					None,
				);
				assert_noop!(err, Error::MinBlobHolderPercentageInvalid);
				assert_unchanged();
			}

			{
				let root: RuntimeOrigin = RawOrigin::Root.into();
				let err = DataAvailability::set_blob_runtime_parameters(
					root,
					None,
					None,
					Some(0),
					None,
					None,
					None,
					None,
					None,
					None,
					None,
					None,
				);
				assert_noop!(err, Error::MinBlobHolderCountInvalid);
				assert_unchanged();
			}

			{
				let root: RuntimeOrigin = RawOrigin::Root.into();
				let err = DataAvailability::set_blob_runtime_parameters(
					root,
					None,
					None,
					None,
					Some(1_000),
					None,
					None,
					None,
					None,
					None,
					None,
					None,
				);
				assert_noop!(err, Error::BlobTtlTooShort);
				assert_unchanged();
			}

			{
				let root: RuntimeOrigin = RawOrigin::Root.into();
				let err = DataAvailability::set_blob_runtime_parameters(
					root,
					None,
					None,
					None,
					None,
					Some(10),
					None,
					None,
					None,
					None,
					None,
					None,
				);
				assert_noop!(err, Error::TempBlobTtlTooShort);
				assert_unchanged();
			}

			{
				let root: RuntimeOrigin = RawOrigin::Root.into();
				let err = DataAvailability::set_blob_runtime_parameters(
					root,
					None,
					None,
					None,
					None,
					None,
					Some(3),
					None,
					None,
					None,
					None,
					None,
				);
				assert_noop!(err, Error::MinTransactionValidityTooLow);
				assert_unchanged();
			}

			{
				let root: RuntimeOrigin = RawOrigin::Root.into();
				let err = DataAvailability::set_blob_runtime_parameters(
					root,
					None,
					None,
					None,
					None,
					None,
					None,
					Some(150),
					None,
					None,
					None,
					None,
				);
				assert_noop!(err, Error::MaxTransactionValidityTooHigh);
				assert_unchanged();
			}

			{
				let root: RuntimeOrigin = RawOrigin::Root.into();
				let err = DataAvailability::set_blob_runtime_parameters(
					root,
					None,
					None,
					None,
					None,
					None,
					None,
					None,
					Some(2),
					None,
					None,
					None,
				);
				assert_noop!(err, Error::MaxBlobRetryTooLow);
				assert_unchanged();
			}

			{
				let root: RuntimeOrigin = RawOrigin::Root.into();
				let err = DataAvailability::set_blob_runtime_parameters(
					root,
					None,
					None,
					None,
					None,
					None,
					None,
					None,
					None,
					Some(10 * 1024 * 1024 * 1024),
					None,
					None,
				);
				assert_noop!(err, Error::MaxBlockSizeTooLarge);
				assert_unchanged();
			}

			{
				let root: RuntimeOrigin = RawOrigin::Root.into();
				let err = DataAvailability::set_blob_runtime_parameters(
					root,
					None,
					None,
					None,
					None,
					None,
					None,
					None,
					None,
					None,
					Some(5 * 1024 * 1024),
					None,
				);
				assert_noop!(err, Error::MaxOldSubmissionTooLarge);
				assert_unchanged();
			}
		})
	}
}

mod set_submit_blob_metadata_fee_modifier {
	use super::*;
	use crate::SubmitBlobMetadataFeeModifier;
	use frame_support::dispatch::DispatchFeeModifier;

	#[test]
	fn default_value() {
		new_test_ext().execute_with(|| {
			let value = SubmitBlobMetadataFeeModifier::<Test>::get();
			assert_eq!(value.weight_maximum_fee, None);
			assert_eq!(value.weight_fee_divider, None);
			assert_eq!(value.weight_fee_multiplier, None);
		})
	}

	#[test]
	fn only_sudo_can_call_this() {
		new_test_ext().execute_with(|| {
			let alice: RuntimeOrigin = RawOrigin::Signed(ALICE).into();
			let value = SubmitBlobMetadataFeeModifier::<Test>::get();
			assert!(DataAvailability::set_submit_blob_metadata_fee_modifier(alice, value).is_err());
		})
	}

	#[test]
	fn set_submit_blob_metadata_fee_modifier() {
		new_test_ext().execute_with(|| {
			let root: RuntimeOrigin = RawOrigin::Root.into();

			let old_value = SubmitBlobMetadataFeeModifier::<Test>::get();
			let new_value = DispatchFeeModifier {
				weight_maximum_fee: Some(100),
				weight_fee_divider: Some(100),
				weight_fee_multiplier: Some(100),
			};

			assert_ne!(old_value, new_value);

			assert_ok!(DataAvailability::set_submit_blob_metadata_fee_modifier(
				root, new_value
			));
			assert_eq!(new_value, SubmitBlobMetadataFeeModifier::<Test>::get());
		})
	}
}

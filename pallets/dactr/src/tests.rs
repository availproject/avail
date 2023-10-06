use avail_core::{BlockLengthColumns, BlockLengthRows, BLOCK_CHUNK_SIZE, NORMAL_DISPATCH_RATIO};
use frame_support::{assert_noop, assert_ok, error::BadOrigin};
use frame_system::{limits::BlockLength, RawOrigin};
use sp_core::H256;

use crate::{
	mock::{
		new_test_ext, DataAvailability, MaxAppDataLength, MaxBlockCols, MaxBlockRows, MinBlockCols,
		MinBlockRows, RuntimeEvent, RuntimeOrigin, System, Test,
	},
	AppDataFor, AppKeyFor, AppKeyInfoFor, Event,
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
			let data_hash = H256(sp_io::hashing::blake2_256(&data));

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
			let new_block_length =
				BlockLength::with_normal_ratio(rows, cols, BLOCK_CHUNK_SIZE, NORMAL_DISPATCH_RATIO)
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
			let new_block_length =
				BlockLength::with_normal_ratio(rows, cols, BLOCK_CHUNK_SIZE, NORMAL_DISPATCH_RATIO)
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
			let new_block_length =
				BlockLength::with_normal_ratio(rows, cols, BLOCK_CHUNK_SIZE, NORMAL_DISPATCH_RATIO)
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
}

use std::fs;
use frame_support::{assert_err, assert_ok};
use hex_literal::hex;
use primitive_types::{H256, U256};
use sp_core::crypto::AccountId32;
use crate::mock::{new_test_ext, Bridge,RuntimeEvent, RuntimeOrigin, System, Test, ROTATE_FUNCTION_ID, STEP_FUNCTION_ID};
use crate::{ConfigurationStorage,Error, Event, ExecutionStateRoots, FunctionInputs, Head, Headers, SyncCommitteeHashes, Updater};
use crate::state::Configuration;
const TEST_SENDER_VEC: [u8; 32] =
	hex!("d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d");
const TEST_SENDER_ACCOUNT: AccountId32 = AccountId32::new(TEST_SENDER_VEC);

#[test]
fn test_fulfill_step_call() {
	new_test_ext().execute_with(|| {
		Updater::<Test>::set(H256(TEST_SENDER_VEC));

		// These inputs, encoded in CBOR format, would be passed in via the operator
		let inputs: Vec<u8> = fs::read("./examples/step_call.cbor").unwrap();

		ConfigurationStorage::<Test>::set(Configuration {
			slots_per_period: 8192,
			finality_threshold: 461,
		});

		let result = Bridge::fulfill_call(
			RuntimeOrigin::signed(TEST_SENDER_ACCOUNT),
			STEP_FUNCTION_ID, // TODO: replace with working h256::zero
			inputs.clone(),
		);

		assert_ok!(result);

		let parsed_inputs: FunctionInputs = serde_cbor::from_slice(&inputs).unwrap();

		let finalized_slot = parsed_inputs.finality_update.finalized_header.slot.as_u64();
		// ensure that event is fired
		let expected_event = RuntimeEvent::Bridge(Event::HeadUpdated {
			slot: finalized_slot,
			finalization_root: H256(hex!(
				"a6e3468985f31ca58e34fe0a40a72f4bbc08d4d00a0933d28b07ddb95d1faf95"
			)),
			execution_state_root: H256(hex!(
				"6518be340ee1bad6c6c6bef6ea3e99ecebc142e196b7edd56b3a5e513d0c6392"
			)),
		});

		let header = Headers::<Test>::get(finalized_slot);
		let head = Head::<Test>::get();
		let ex_state_root = ExecutionStateRoots::<Test>::get(finalized_slot);

		assert_eq!(
			header,
			H256(hex!(
				"a6e3468985f31ca58e34fe0a40a72f4bbc08d4d00a0933d28b07ddb95d1faf95"
			))
		);
		assert_eq!(
			ex_state_root,
			H256(hex!(
			"6518be340ee1bad6c6c6bef6ea3e99ecebc142e196b7edd56b3a5e513d0c6392"
			))
		);
		assert_eq!(head, finalized_slot);
		assert_eq!(expected_event, System::events()[0].event);
	});
}

#[test]
fn test_fulfill_step_call_slot_behind_head() {
	new_test_ext().execute_with(|| {
		Updater::<Test>::set(H256(TEST_SENDER_VEC));
		let inputs: Vec<u8> = fs::read("./examples/step_call.cbor").unwrap();

		// move head forward
		Head::<Test>::set(9678877);

		ConfigurationStorage::<Test>::set(Configuration {
			slots_per_period: 8192,
			finality_threshold: 461,
		});

		let result = Bridge::fulfill_call(
			RuntimeOrigin::signed(TEST_SENDER_ACCOUNT),
			STEP_FUNCTION_ID,
			inputs,
		);

		assert_err!(result, Error::<Test>::SlotBehindHead);
	});
}

#[test]
fn test_fulfill_rotate_call() {
	new_test_ext().execute_with(|| {
		Updater::<Test>::set(H256(TEST_SENDER_VEC));

		// These inputs, encoded in CBOR format, would be passed in via the operator
		let inputs: Vec<u8> = fs::read("./examples/rotate_call.cbor").unwrap();

		ConfigurationStorage::<Test>::set(Configuration {
			slots_per_period: 8192,
			finality_threshold: 342,
		});

		let result = Bridge::fulfill_call(
			RuntimeOrigin::signed(TEST_SENDER_ACCOUNT),
			ROTATE_FUNCTION_ID, // TODO: replace with working h256::zero
			inputs,
		);

		assert_ok!(result);
		// ensure that event is fired
		let expected_hash = U256::from_dec_str(
			"78004113044439342907882478475913997887515213797155324584820998418219758944903",
		)
			.unwrap();

		let current_period = 1178;
		let expected_event = RuntimeEvent::Bridge(Event::SyncCommitteeUpdated {
			period: current_period + 1,
			root: expected_hash,
		});

		let poseidon = SyncCommitteeHashes::<Test>::get(current_period + 1);

		assert_eq!(expected_event, System::events()[1].event);
		assert_eq!(poseidon, expected_hash);
	});
}
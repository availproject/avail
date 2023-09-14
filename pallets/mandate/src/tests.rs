use frame_support::{assert_noop, assert_ok};
use frame_system::RawOrigin;

use crate::{
	mock::{new_test_ext, Mandate, RuntimeCall, RuntimeOrigin, System, TechnicalCommittee},
	*,
};

const ALICE: u64 = 1u64;
const BOB: u64 = 2u64;
const DAVID: u64 = 3u64;

#[test]
fn mandate_can_be_called_with_sudo() {
	new_test_ext().execute_with(|| {
		// Goal: Adding new members to TC via Mandate call.

		// Checking. Making sure that we actually add new members.
		let old_members = TechnicalCommittee::members();
		let new_members = [ALICE, BOB, DAVID].to_vec();
		assert_ne!(new_members, old_members);

		// Setting up our call. `old_count` is not really important in this case.
		let privileged_call = pallet_collective::Call::set_members {
			new_members: new_members.clone(),
			prime: None,
			old_count: 0,
		};
		let call = Box::new(RuntimeCall::TechnicalCommittee(privileged_call));

		// The Call
		let o = RuntimeOrigin::from(RawOrigin::Root);
		assert_ok!(Mandate::mandate(o, call));

		// Checking that the storage has changed
		assert_eq!(TechnicalCommittee::members(), new_members);

		// Checking Events
		System::assert_last_event(Event::RootOp { result: Ok(()) }.into());
	});
}

#[test]
fn mandate_can_be_called_with_technical_committee() {
	new_test_ext().execute_with(|| {
		// Goal: Adding new members to TC via Mandate call.

		// Making Alice the only members of TC
		let sudo = RuntimeOrigin::from(RawOrigin::Root);
		assert_ok!(TechnicalCommittee::set_members(
			sudo,
			[ALICE].to_vec(),
			None,
			0
		));

		// Create Proposal
		let new_members = [ALICE, BOB].to_vec();
		let alice_o = RuntimeOrigin::from(RawOrigin::Signed(ALICE));
		let privileged_call = pallet_collective::Call::set_members {
			new_members: new_members.clone(),
			prime: None,
			old_count: 0,
		};
		let inner_call = Box::new(RuntimeCall::TechnicalCommittee(privileged_call));
		let outer_call = RuntimeCall::Mandate(crate::pallet::Call::mandate { call: inner_call });
		let proposal = Box::new(outer_call);
		let length_bound = proposal.encoded_size() as u32 + 1;

		// The Call. Since TC has only one member, this proposal will be immediately executed.
		let res = TechnicalCommittee::propose(alice_o, 1, proposal, length_bound);
		assert_ok!(res);

		// Checking that the storage has changed
		assert_eq!(TechnicalCommittee::members(), new_members);

		// Checking Events
		System::assert_has_event(Event::RootOp { result: Ok(()) }.into());
	});
}

#[test]
fn mandate_can_not_be_called_with_normal_signed_origins() {
	new_test_ext().execute_with(|| {
		// Goal: Adding new members to TC via Mandate call.

		// Checking. Making sure that we actually add new members.
		let old_members = TechnicalCommittee::members();
		let new_members = [ALICE, BOB, DAVID].to_vec();
		assert_ne!(new_members, old_members);

		// Setting up our call. `old_count` is not really important in this case.
		let privileged_call = pallet_collective::Call::set_members {
			new_members: new_members.clone(),
			prime: None,
			old_count: 0,
		};
		let call = Box::new(RuntimeCall::TechnicalCommittee(privileged_call));

		// The Call
		let o = RuntimeOrigin::from(RawOrigin::Signed(ALICE));
		assert_noop!(
			Mandate::mandate(o, call),
			sp_runtime::DispatchError::BadOrigin
		);
	});
}

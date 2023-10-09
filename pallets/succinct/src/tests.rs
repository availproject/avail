// use hex_literal::hex;
// use sp_runtime::testing::H256;
// use crate::Call::set_updater;
// use crate::mock::new_test_ext;
//
// #[test]
// fn test_set_updater() {
//     new_test_ext().execute_with(|| {
//
//         // Goal: Set updater.
//
//         let new_updater = H256(hex!("d54593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d"));
//
//         set_updater { updater: new_updater }
//
//
//         // Checking. Making sure that we actually add new members.
//         // let old_members = TechnicalCommittee::members();
//         // let new_members = [ALICE, BOB, DAVID].to_vec();
//         // assert_ne!(new_members, old_members);
//         //
//         // // Setting up our call. `old_count` is not really important in this case.
//         // let privileged_call = pallet_collective::Call::set_members {
//         //     new_members: new_members.clone(),
//         //     prime: None,
//         //     old_count: 0,
//         // };
//         // let call = Box::new(RuntimeCall::TechnicalCommittee(privileged_call));
//         //
//         // // The Call
//         // let o = RuntimeOrigin::from(RawOrigin::Root);
//         // assert_ok!(Mandate::mandate(o, call));
//         //
//         // // Checking that the storage has changed
//         // assert_eq!(TechnicalCommittee::members(), new_members);
//         //
//         // // Checking Events
//         // System::assert_last_event(Event::RootOp { result: Ok(()) }.into());
//     });
// }

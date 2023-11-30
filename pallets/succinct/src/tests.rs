use frame_support::dispatch::RawOrigin;
use frame_support::{assert_err, assert_ok};
use hex_literal::hex;
use sp_runtime::testing::H256;
use sp_runtime::DispatchError::BadOrigin;

use crate::mock::{new_test_ext, Bridge, RuntimeOrigin, Test};
use crate::StateStorage;

#[test]
fn test_set_updater() {
	new_test_ext().execute_with(|| {
		// Goal: Set updater - bad origin.
		let new_updater = H256(hex!(
			"d54593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d"
		));
		let old_updater = H256(hex!(
			"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d"
		));

		let before = StateStorage::<Test>::get();
		assert_eq!(before.updater, old_updater);
		let bad_origin = RuntimeOrigin::from(RawOrigin::None);
		let bad_result = Bridge::set_updater(bad_origin, new_updater);
		assert_err!(bad_result, BadOrigin);
		assert_eq!(before.updater, old_updater);

		// Goal: Set updater - success.
		let root_origin = RuntimeOrigin::from(RawOrigin::Root);
		let success = Bridge::set_updater(root_origin, new_updater);
		assert_ok!(success);
		let after = StateStorage::<Test>::get();
		assert_eq!(after.updater, new_updater);
	});
}

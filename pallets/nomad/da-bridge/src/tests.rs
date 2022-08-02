use frame_support::assert_ok;
use sp_core::H160;

use crate::mock::*;

#[test]
fn it_sets_updater() {
	new_test_ext().execute_with(|| {
		// Default updater is zero bytes
		assert_eq!(UpdaterManager::get_updater(), H160::zero());

		// Set to 1 and check new state
		let new_updater = H160::repeat_byte(1);
		assert_ok!(UpdaterManager::set_updater(new_updater));
		assert_eq!(UpdaterManager::get_updater(), new_updater);

		let expected = vec![crate::Event::NewUpdater {
			old_updater: H160::zero(),
			new_updater,
		}];
		assert_eq!(events(), expected);
	});
}

#[test]
fn it_slashes_updater() {
	new_test_ext().execute_with(|| {
		// Slash updater
		let reporter = 1u64;
		UpdaterManager::slash_updater(reporter);

		let expected = vec![crate::Event::FakeSlashed { reporter }];
		assert_eq!(events(), expected);
	});
}

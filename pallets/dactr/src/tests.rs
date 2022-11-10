use frame_support::{assert_noop, assert_ok};
use frame_system::RawOrigin;

use crate::{
	mock::{new_test_ext, DataAvailability, Event, System, Test},
	AppKeyFor, AppKeyInfoFor,
	Event::ApplicationKeyCreated,
};
type Error = crate::Error<Test>;

#[test]
fn create_application_keys() {
	new_test_ext().execute_with(|| {
		let new_key = AppKeyFor::<Test>::try_from(b"New App".to_vec()).unwrap();
		assert_eq!(DataAvailability::application_key(&new_key), None);
		assert_ok!(DataAvailability::create_application_key(
			RawOrigin::Signed(3).into(),
			new_key.clone()
		));

		assert_eq!(
			System::events()[0].event,
			Event::DataAvailability(ApplicationKeyCreated {
				key: new_key.clone(),
				owner: 3,
				id: 3.into()
			})
		);
		assert_noop!(
			DataAvailability::create_application_key(RawOrigin::Signed(2).into(), new_key.clone()),
			Error::AppKeyAlreadyExists
		);
		assert_eq!(
			DataAvailability::application_key(&new_key),
			Some(AppKeyInfoFor::<Test> {
				id: 3.into(),
				owner: 3
			})
		);
	})
}

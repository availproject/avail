// use frame_support::assert_ok;
// use frame_system::RawOrigin;

// use crate::{
// 	mock::{new_test_ext, Fusion, RuntimeCall, RuntimeOrigin, System},
// 	*,
// };

use crate::mock::new_test_ext;

// const ALICE: u64 = 1u64;
// const BOB: u64 = 2u64;
// const DAVID: u64 = 3u64;

#[test]
fn random_test() {
	new_test_ext().execute_with(|| {
		assert_eq!(true, true);
	});
}

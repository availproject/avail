use hex_literal::hex;
use sp_runtime::testing::H256;
use crate::Call::set_updater;
use crate::mock::new_test_ext;
use crate::mock::Test;

#[test]
fn test_set_updater() {
    new_test_ext().execute_with(|| {

        // Goal: Set updater.

        let new_updater = H256(hex!("d54593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d"));

        set_updater::<Test> { updater: new_updater }
    });
}

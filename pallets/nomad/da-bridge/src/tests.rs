use frame_support::{assert_err, assert_ok};
#[cfg(feature = "testing")]
use nomad_base::testing::*;
use nomad_core::{destination_and_nonce, NomadMessage, NomadState};
use once_cell::sync::Lazy;
use sp_core::H256;
use sp_runtime::AccountId32;

use crate::{mock::*, Error};

const TEST_REMOTE_DOMAIN: u32 = 2222;
const TEST_SENDER_VEC: [u8; 32] = [2u8; 32];
static TEST_SENDER_BYTES: Lazy<H256> = Lazy::new(|| H256::from(TEST_SENDER_VEC));
static TEST_SENDER_ACCOUNT: Lazy<AccountId32> = Lazy::new(|| AccountId32::new(TEST_SENDER_VEC));
static TEST_RECIPIENT: Lazy<H256> = Lazy::new(|| H256::repeat_byte(3));

#[test]
#[cfg(feature = "testing")]
fn it_dispatches_valid_ext_root() {
	ExtBuilder::default()
		.with_base(*TEST_NOMAD_BASE)
		.build()
		.execute_with(|| {})
}

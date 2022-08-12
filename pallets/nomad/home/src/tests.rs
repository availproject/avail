use frame_support::{assert_err, assert_ok};
use merkle::Merkle;
#[cfg(feature = "testing")]
use nomad_base::testing::*;
use nomad_core::{destination_and_nonce, NomadMessage, NomadState};
use once_cell::sync::Lazy;
use primitive_types::H256;
use sp_runtime::AccountId32;

use crate::{mock::*, Error};

const TEST_REMOTE_DOMAIN: u32 = 2222;
const TEST_SENDER_VEC: [u8; 32] = [2u8; 32];
static TEST_SENDER_BYTES: Lazy<H256> = Lazy::new(|| H256::from(TEST_SENDER_VEC));
static TEST_SENDER_ACCOUNT: Lazy<AccountId32> = Lazy::new(|| AccountId32::new(TEST_SENDER_VEC));
static TEST_RECIPIENT: Lazy<H256> = Lazy::new(|| H256::repeat_byte(3));

// TODO: test governance router can set updater for base and UM

#[test]
#[cfg(feature = "testing")]
fn it_dispatches_message() {
	ExtBuilder::default()
		.with_base(*TEST_NOMAD_BASE)
		.build()
		.execute_with(|| {
			// Fetch expected values
			let nonce = Home::get_nonce(TEST_REMOTE_DOMAIN);
			let destination_and_nonce = destination_and_nonce(TEST_REMOTE_DOMAIN, nonce);
			let leaf_index = Home::tree().count();
			let body = [1u8; 8].to_vec();
			let committed_root = Home::base().committed_root();

			// Format expected message
			let message = NomadMessage {
				origin: TEST_LOCAL_DOMAIN,
				sender: *TEST_SENDER_BYTES,
				nonce,
				destination: TEST_REMOTE_DOMAIN,
				recipient: *TEST_RECIPIENT,
				body: body.clone(),
			};
			let message_hash = message.hash();

			// Dispatch message
			let origin = Origin::signed((*TEST_SENDER_ACCOUNT).clone());
			assert_ok!(Home::dispatch(
				origin,
				TEST_REMOTE_DOMAIN,
				*TEST_RECIPIENT,
				body
			));

			// Tree count incremented
			assert!(Home::tree().count() == 1);

			// Mappings have root and index
			let current_root = Home::tree().root();
			assert!(Home::index_to_root(leaf_index).unwrap() == current_root);
			assert!(Home::root_to_index(current_root).unwrap() == leaf_index);

			let expected = vec![crate::Event::Dispatch {
				message_hash,
				leaf_index,
				destination_and_nonce,
				committed_root,
				message: message.to_vec(),
			}];
			assert_eq!(events(), expected);
		})
}

#[test]
#[cfg(feature = "testing")]
fn it_rejects_big_message() {
	ExtBuilder::default()
		.with_base(*TEST_NOMAD_BASE)
		.build()
		.execute_with(|| {
			let body = [1u8; 5_001].to_vec();

			// Dispatch message too large
			let origin = Origin::signed((*TEST_SENDER_ACCOUNT).clone());
			assert_err!(
				Home::dispatch(origin, TEST_REMOTE_DOMAIN, *TEST_RECIPIENT, body),
				Error::<Test>::MessageTooLarge
			);
		})
}

#[test]
#[cfg(feature = "testing")]
fn it_catches_improper_update() {
	ExtBuilder::default()
		.with_base(*TEST_NOMAD_BASE)
		.build()
		.execute_with(|| {
			let committed_root = Home::base().committed_root();

			// Sign improper update
			let fake_root = H256::repeat_byte(9);
			let improper_signed = TEST_UPDATER.sign_update(committed_root, fake_root);

			let origin = Origin::signed(TEST_SENDER_ACCOUNT.clone());
			assert_ok!(Home::improper_update(origin, improper_signed.clone()));
			assert!(Home::state() == NomadState::Failed);

			let expected = vec![
				crate::Event::UpdaterSlashed {
					updater: TEST_UPDATER.address(),
					reporter: TEST_SENDER_ACCOUNT.clone(),
				},
				crate::Event::ImproperUpdate {
					previous_root: committed_root,
					new_root: fake_root,
					signature: improper_signed.signature.to_vec(),
				},
			];
			assert_eq!(events(), expected);
		})
}

#[test]
#[cfg(feature = "testing")]
fn it_dispatches_message_and_accepts_update() {
	ExtBuilder::default()
		.with_base(*TEST_NOMAD_BASE)
		.build()
		.execute_with(move || {
			let committed_root = Home::base().committed_root();

			let body = [1u8; 8].to_vec();
			// Dispatch message
			let origin = Origin::signed((*TEST_SENDER_ACCOUNT).clone());
			assert_ok!(Home::dispatch(
				origin.clone(),
				TEST_REMOTE_DOMAIN,
				*TEST_RECIPIENT,
				body.clone()
			));

			// Dispatch another message
			assert_ok!(Home::dispatch(
				origin.clone(),
				TEST_REMOTE_DOMAIN,
				*TEST_RECIPIENT,
				body
			));

			// Get updater signature
			let new_root = Home::root();
			let signed_update = TEST_UPDATER.sign_update(committed_root, new_root);

			// Submit signed update
			assert_ok!(Home::update(origin, signed_update.clone()));

			let expected_update_event = crate::Event::Update {
				home_domain: TEST_LOCAL_DOMAIN,
				previous_root: committed_root,
				new_root,
				signature: signed_update.signature.to_vec(),
			};
			assert!(events().contains(&expected_update_event));
		})
}

#[test]
#[cfg(feature = "testing")]
fn it_rejects_invalid_signature() {
	ExtBuilder::default()
		.with_base(*TEST_NOMAD_BASE)
		.build()
		.execute_with(move || {
			let committed_root = Home::base().committed_root();

			let body = [1u8; 8].to_vec();
			// Dispatch message
			let origin = Origin::signed((*TEST_SENDER_ACCOUNT).clone());
			assert_ok!(Home::dispatch(
				origin.clone(),
				TEST_REMOTE_DOMAIN,
				*TEST_RECIPIENT,
				body.clone()
			));

			// Get fake updater signature
			let new_root = Home::root();
			let signed_update = FAKE_UPDATER.sign_update(committed_root, new_root);

			// Assert err returned from submitting signed update
			assert_err!(
				Home::update(origin, signed_update.clone()),
				Error::<Test>::InvalidUpdaterSignature
			);
		})
}

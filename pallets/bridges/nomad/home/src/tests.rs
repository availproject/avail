use std::convert::TryInto;

use frame_benchmarking::whitelisted_caller;
use frame_support::{assert_err, assert_ok, pallet_prelude::Get, BoundedVec};
use nomad_base::testing::*;
use nomad_core::{destination_and_nonce, NomadMessage, NomadState};
use nomad_merkle::Merkle;
use sp_core::H256;
use sp_runtime::{AccountId32, DispatchResult};
use test_case::test_case;

use crate::{
	common_tests_and_benches::expected_longest_tree_signed_update, mock::*, Config, Error,
};

const TEST_REMOTE_DOMAIN: u32 = 2222;
const TEST_SENDER_VEC: [u8; 32] = [2u8; 32];
const TEST_SENDER_BYTES: H256 = H256(TEST_SENDER_VEC);
const TEST_SENDER_ACCOUNT: AccountId32 = AccountId32::new(TEST_SENDER_VEC);
const TEST_RECIPIENT: H256 = H256::repeat_byte(3);

// TODO: test governance router can set updater for base and UM

#[test]
fn it_dispatches_message() {
	ExtBuilder::default()
		.with_base(*TEST_NOMAD_BASE)
		.build()
		.execute_with(|| {
			// Fetch expected values
			let nonce = Home::nonces(TEST_REMOTE_DOMAIN);
			let destination_and_nonce = destination_and_nonce(TEST_REMOTE_DOMAIN, nonce);
			let leaf_index = Home::tree().count();
			let body: BoundedVec<u8, _> = [1u8; 8].to_vec().try_into().unwrap();
			let committed_root = Home::base().committed_root;

			// Format expected message
			let message = NomadMessage {
				origin: TEST_LOCAL_DOMAIN,
				sender: TEST_SENDER_BYTES,
				nonce,
				destination: TEST_REMOTE_DOMAIN,
				recipient: TEST_RECIPIENT,
				body: body.clone(),
			};
			let message_hash = message.hash();

			// Dispatch message
			let origin = RuntimeOrigin::signed(TEST_SENDER_ACCOUNT);
			assert_ok!(Home::dispatch(
				origin,
				TEST_REMOTE_DOMAIN,
				TEST_RECIPIENT,
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
fn it_rejects_big_message() {
	use core::convert::TryFrom;

	ExtBuilder::default()
		.with_base(*TEST_NOMAD_BASE)
		.build()
		.execute_with(|| {
			let bounded_res = BoundedVec::<u8, <Test as Config>::MaxMessageBodyBytes>::try_from(
				[1u8; 5001].to_vec(),
			);
			assert!(bounded_res.is_err());
		})
}

#[test]
fn it_catches_improper_update() {
	ExtBuilder::default()
		.with_base(*TEST_NOMAD_BASE)
		.build()
		.execute_with(|| {
			let committed_root = Home::base().committed_root;

			// Sign improper update
			let fake_root = H256::repeat_byte(9);
			let improper_signed = TEST_UPDATER.sign_update(committed_root, fake_root);

			let origin = RuntimeOrigin::signed(TEST_SENDER_ACCOUNT);
			assert_ok!(Home::improper_update(origin, improper_signed.clone()));
			assert!(Home::base().state == NomadState::Failed);

			let expected = vec![
				crate::Event::UpdaterSlashed {
					updater: TEST_UPDATER.address(),
					reporter: TEST_SENDER_ACCOUNT,
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

/// Dispatch a random message and returns the new `root`.
fn dispatch_random_message(origin: RuntimeOrigin) -> H256 {
	let body = [1u8; 8].to_vec().try_into().unwrap();
	assert_ok!(Home::dispatch(
		origin,
		TEST_REMOTE_DOMAIN,
		TEST_RECIPIENT,
		body
	));

	Home::tree().root()
}

#[test_case( 1, 0 => Err(Error::<Test>::MaxIndexWitnessExhausted.into()) ; "Invalid max index witness")]
#[test_case( 2, 1 => Err(Error::<Test>::MaxIndexWitnessExhausted.into()) ; "Max index witness exhausted")]
#[test_case( 2, 2 => Ok(()); "Valid update")]
fn it_update_max_index_witness(dispatch_messages: usize, max_index: u32) -> DispatchResult {
	ExtBuilder::default()
		.with_base(*TEST_NOMAD_BASE)
		.build()
		.execute_with(move || {
			let committed_root = Home::base().committed_root;

			// Dispatch `dispatch_messages` messages and get the latest root.
			let sender = RuntimeOrigin::signed(TEST_SENDER_ACCOUNT);
			let last_root = (0..)
				.take(dispatch_messages)
				.map(|_| dispatch_random_message(sender.clone()))
				.last()
				.unwrap_or(committed_root);

			let signed_update = TEST_UPDATER.sign_update(committed_root, last_root);
			Home::update(sender, signed_update, max_index)
		})
}

#[test]
fn it_dispatches_messages_and_accepts_updates() {
	ExtBuilder::default()
		.with_base(*TEST_NOMAD_BASE)
		.build()
		.execute_with(move || {
			let committed_root = Home::base().committed_root;

			let origin = RuntimeOrigin::signed(TEST_SENDER_ACCOUNT);

			// Dispatch 2 messages.
			let roots = (0..2)
				.map(|_| dispatch_random_message(origin.clone()))
				.collect::<Vec<_>>();
			let last_root = *roots.last().unwrap();

			// Get updater signature
			let signed_update = TEST_UPDATER.sign_update(committed_root, last_root);

			// Submit signed update
			assert_ok!(Home::update(origin.clone(), signed_update.clone(), 3));

			let expected_update_event = crate::Event::Update {
				home_domain: TEST_LOCAL_DOMAIN,
				previous_root: committed_root,
				new_root: last_root,
				signature: signed_update.signature.to_vec(),
			};
			assert!(events().contains(&expected_update_event));

			// Assert mappings are cleared out up to signed_update.new_root()
			assert_eq!(Home::base().committed_root, last_root);
			for (idx, root) in roots.into_iter().enumerate() {
				assert_eq!(Home::index_to_root(idx as u32), None);
				assert_eq!(Home::root_to_index(root), None);
			}

			// Dispatch third message
			let committed_root = Home::base().committed_root;
			let root_after_third_msg = dispatch_random_message(origin.clone());

			// Get updater signature
			let signed_update = TEST_UPDATER.sign_update(committed_root, root_after_third_msg);

			// Submit signed update
			assert_ok!(Home::update(origin, signed_update.clone(), 3));

			let expected_update_event = crate::Event::Update {
				home_domain: TEST_LOCAL_DOMAIN,
				previous_root: committed_root,
				new_root: root_after_third_msg,
				signature: signed_update.signature.to_vec(),
			};
			assert!(events().contains(&expected_update_event));

			// Assert mappings are cleared out up to signed_update.new_root()
			assert!(Home::index_to_root(2).is_none());
			assert!(Home::root_to_index(root_after_third_msg).is_none());
			assert!(Home::base().committed_root == root_after_third_msg);
		})
}

#[test]
fn it_rejects_invalid_signature() {
	ExtBuilder::default()
		.with_base(*TEST_NOMAD_BASE)
		.build()
		.execute_with(move || {
			let committed_root = Home::base().committed_root;

			let body: BoundedVec<u8, _> = [1u8; 8].to_vec().try_into().unwrap();
			// Dispatch message
			let origin = RuntimeOrigin::signed(TEST_SENDER_ACCOUNT);
			assert_ok!(Home::dispatch(
				origin.clone(),
				TEST_REMOTE_DOMAIN,
				TEST_RECIPIENT,
				body
			));

			// Get fake updater signature
			let new_root = Home::tree().root();
			let signed_update = FAKE_UPDATER.sign_update(committed_root, new_root);

			// Assert err returned from submitting signed update
			assert_err!(
				Home::update(origin, signed_update, 10),
				Error::<Test>::InvalidUpdaterSignature
			);
		})
}

#[test]
fn it_longest_tree() {
	ExtBuilder::default()
		.with_base(*TEST_NOMAD_BASE)
		.build()
		.execute_with(move || {
			use nomad_merkle::TREE_DEPTH;

			let committed_root = Home::base().committed_root;

			let body_len = <<Test as Config>::MaxMessageBodyBytes as Get<u32>>::get() as usize;
			let body: BoundedVec<u8, _> = sp_std::iter::repeat(3u8)
				.take(body_len)
				.collect::<Vec<_>>()
				.try_into()
				.unwrap();

			let origin = RuntimeOrigin::signed(whitelisted_caller::<AccountId32>());

			for _ in 0..TREE_DEPTH {
				assert_ok!(Home::dispatch(
					origin.clone(),
					1111,
					TEST_RECIPIENT,
					body.clone()
				));
			}

			let new_root = Home::tree().root();
			let signed_update = TEST_UPDATER.sign_update(committed_root, new_root);
			let exp_signed_update = expected_longest_tree_signed_update();
			assert_eq!(new_root, exp_signed_update.update.new_root);
			assert_eq!(signed_update, exp_signed_update);

			assert_ok!(Home::update(origin, signed_update, TREE_DEPTH as u32));
		})
}

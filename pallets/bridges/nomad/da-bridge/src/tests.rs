use avail_core::header::{Header, HeaderExtension};
use frame_support::assert_ok;
use frame_system::pallet_prelude::BlockNumberFor;
use hex_literal::hex;
use nomad_base::testing::*;
use nomad_merkle::Merkle;
use sp_core::H256;
use sp_runtime::{testing::Digest, traits::BlakeTwo256, AccountId32};

use crate::mock::*;

const TEST_SENDER_VEC: [u8; 32] = [2u8; 32];
const TEST_SENDER_ACCOUNT: AccountId32 = AccountId32::new(TEST_SENDER_VEC);
const DESTINATION_DOMAIN: u32 = 1000;

#[test]
fn it_accepts_valid_extrinsic_root() {
	ExtBuilder::default()
		.with_base(*TEST_NOMAD_BASE)
		.build()
		.execute_with(|| {
			fill_block_hash_mapping_up_to_n(9);

			// Create extrinsics root for block 10
			let extension = HeaderExtension::default();

			// Create block header for block 10
			let header = Header::<BlockNumberFor<Test>, BlakeTwo256> {
				parent_hash: [1u8; 32].into(),
				number: 10_u32,
				state_root: [2u8; 32].into(),
				extrinsics_root: hex!(
					"03170a2e7597b7b7e3d84c05391d139a62b157e78786d8c082f29dcf4c111314"
				)
				.into(),
				digest: Digest { logs: vec![] },
				extension,
			};

			// Insert 10th block's hash into block number --> hash mapping so
			// submitting 10th block's header is accepted by pallet
			frame_system::BlockHash::<Test>::insert::<u32, <Test as frame_system::Config>::Hash>(
				10u32,
				header.hash(),
			);

			// Get home's current merkle root pre-enqueue
			let root_pre = Home::tree().root();
			let nonce_pre = Home::nonces(DESTINATION_DOMAIN);

			// Enqueue extrinsic root
			assert_ok!(DABridge::try_dispatch_data_root(
				RuntimeOrigin::signed(TEST_SENDER_ACCOUNT),
				DESTINATION_DOMAIN,
				H256::zero(),
				Box::new(header)
			));

			// Get home's merkle root post-enqueue
			let root_post = Home::tree().root();
			let nonce_post = Home::nonces(DESTINATION_DOMAIN);

			// Ensure home's merkle root changed after enqueueing message
			assert_ne!(root_pre, root_post);
			assert_eq!(nonce_pre + 1, nonce_post);
		})
}

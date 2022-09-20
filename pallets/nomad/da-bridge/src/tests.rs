use da_primitives::{Header, KateCommitment};
use frame_support::assert_ok;
use frame_system::Config;
use hex_literal::hex;
use merkle::Merkle;
use nomad_base::testing::*;
use once_cell::sync::Lazy;
use primitive_types::H256;
use sp_runtime::{testing::Digest, traits::BlakeTwo256, AccountId32};

use crate::mock::*;

const TEST_SENDER_VEC: [u8; 32] = [2u8; 32];
static TEST_SENDER_ACCOUNT: Lazy<AccountId32> = Lazy::new(|| AccountId32::new(TEST_SENDER_VEC));

#[test]
fn it_accepts_valid_extrinsic_root() {
	ExtBuilder::default()
		.with_base(*TEST_NOMAD_BASE)
		.build()
		.execute_with(|| {
			fill_block_hash_mapping_up_to_n(9);

			// Create extrinsics root for block 10
			let extrinsics_root = KateCommitment {
				hash: hex!("03170a2e7597b7b7e3d84c05391d139a62b157e78786d8c082f29dcf4c111314")
					.into(),
				..Default::default()
			};

			// Create block header for block 10
			let header = Header::<<Test as Config>::BlockNumber, BlakeTwo256> {
				parent_hash: [1u8; 32].into(),
				number: 10 as u32,
				state_root: [2u8; 32].into(),
				extrinsics_root,
				digest: Digest { logs: vec![] },
				app_data_lookup: Default::default(),
			};

			// Insert 10th block's hash into block number --> hash mapping so
			// submitting 10th block's header is accepted by pallet
			frame_system::BlockHash::<Test>::insert::<u32, <Test as frame_system::Config>::Hash>(
				10u32.into(),
				header.hash(),
			);

			// Get home's current merkle root pre-enqueue
			let root_pre = Home::tree().root();

			// Enqueue extrinsic root
			let origin = Origin::signed((*TEST_SENDER_ACCOUNT).clone());
			assert_ok!(DABridge::try_dispatch_data_root(
				origin,
				1000,
				H256::zero(),
				header
			));

			// Get home's merkle root post-enqueue
			let root_post = Home::tree().root();

			// Ensure home's merkle root changed after enqueueing message
			assert_ne!(root_pre, root_post);
		})
}

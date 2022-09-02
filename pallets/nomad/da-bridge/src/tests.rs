use da_primitives::{Header, KateCommitment};
use frame_support::{assert_err, assert_ok};
use frame_system::Config;
use hex_literal::hex;
use merkle::Merkle;
#[cfg(feature = "testing")]
use nomad_base::testing::*;
use nomad_core::{destination_and_nonce, NomadMessage, NomadState};
use once_cell::sync::Lazy;
use primitive_types::H256;
use sp_runtime::{testing::Digest, traits::BlakeTwo256, AccountId32};

use crate::{mock::*, pallet::FinalizedBlockNumberToBlockHash, Error};

const TEST_REMOTE_DOMAIN: u32 = 2222;
const TEST_SENDER_VEC: [u8; 32] = [2u8; 32];
static TEST_SENDER_BYTES: Lazy<H256> = Lazy::new(|| H256::from(TEST_SENDER_VEC));
static TEST_SENDER_ACCOUNT: Lazy<AccountId32> = Lazy::new(|| AccountId32::new(TEST_SENDER_VEC));
static TEST_RECIPIENT: Lazy<H256> = Lazy::new(|| H256::repeat_byte(3));

#[test]
#[cfg(feature = "testing")]
fn it_fills_block_hash_mapping() {
	ExtBuilder::default()
		.with_base(*TEST_NOMAD_BASE)
		.build()
		.execute_with(|| {
			// Fill mapping for blocks 0 to 10
			fill_block_hash_mapping_up_to_n(10);

			// Ensure there are exactly blocks 0-10 filled and block 11 has no
			// mapping yet
			for i in 0..=10 as u8 {
				assert_eq!(
					DABridge::finalized_block_number_to_block_hash(i as u32).unwrap(),
					H256::repeat_byte(i)
				);
			}
			assert!(DABridge::finalized_block_number_to_block_hash(11).is_none());
		})
}

#[test]
#[cfg(feature = "testing")]
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
				commitment: Default::default(),
				rows: Default::default(),
				cols: Default::default(),
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

			// Insert block header's hash into finalized mapping
			FinalizedBlockNumberToBlockHash::<Test>::insert(10, header.hash());

			// Get home's current merkle root pre-enqueue
			let root_pre = Home::tree().root();

			// Enqueue extrinsic root
			let origin = Origin::signed((*TEST_SENDER_ACCOUNT).clone());
			assert_ok!(DABridge::try_enqueue_extrinsics_root(
				origin,
				1000,
				H256::zero(),
				header
			));

			// Get home's merkle root post-enqueue
			let root_post = Home::tree().root();

			// Ensure home's merkle root changed after enqueueing message
			assert_ne!(root_pre, root_post);

			// Ensure all previous mappings from blocks 0 to 10 are cleared
			for i in 0..=10 {
				assert!(DABridge::finalized_block_number_to_block_hash(i).is_none());
			}
		})
}

#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::pallet_prelude::*;
use nomad_core::{home_domain_hash, to_eth_signed_message_hash, NomadState, SignedUpdate, Update};
use primitive_types::{H160, H256};
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use signature::SignatureError;

#[cfg(feature = "testing")]
pub mod testing;

#[derive(Clone, Copy, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct NomadBase {
	pub state: NomadState,
	pub local_domain: u32,
	pub committed_root: H256,
	pub updater: H160,
}

impl Default for NomadBase {
	fn default() -> Self {
		Self {
			state: NomadState::Active,
			local_domain: Default::default(),
			committed_root: Default::default(),
			updater: Default::default(),
		}
	}
}

pub enum NomadBaseError {
	FailedInitialization,
}

impl NomadBase {
	pub fn new(local_domain: u32, committed_root: H256, updater: H160) -> Self {
		Self {
			state: Default::default(),
			local_domain,
			committed_root,
			updater,
		}
	}

	pub fn state(&self) -> NomadState { self.state }

	pub fn local_domain(&self) -> u32 { self.local_domain }

	pub fn committed_root(&self) -> H256 { self.committed_root }

	pub fn updater(&self) -> H160 { self.updater }

	pub fn home_domain_hash(&self) -> H256 { home_domain_hash(self.local_domain()) }

	pub fn set_state(&mut self, new_state: NomadState) { self.state = new_state }

	pub fn set_updater(&mut self, new_updater: H160) { self.updater = new_updater }

	pub fn set_committed_root(&mut self, new_committed_root: H256) {
		self.committed_root = new_committed_root;
	}

	pub fn is_updater_signature(
		&self,
		signed_update: &SignedUpdate,
	) -> Result<bool, SignatureError> {
		let supposed_signing_hash = Update {
			home_domain: self.local_domain,
			previous_root: signed_update.previous_root(),
			new_root: signed_update.new_root(),
		}
		.signing_hash();

		let digest = to_eth_signed_message_hash(&supposed_signing_hash);
		signed_update
			.signature
			.recover(digest)
			.map(|a| a == self.updater)
	}
}

#[cfg(test)]
mod test {
	use nomad_core::test_utils::Updater;

	use super::*;
	#[cfg(feature = "testing")]
	use crate::testing::{FAKE_UPDATER, TEST_NOMAD_BASE, TEST_UPDATER, TEST_UPDATER_PRIVKEY};

	#[test]
	#[cfg(feature = "testing")]
	fn it_accepts_valid_signature() {
		let valid_signed = TEST_UPDATER.sign_update(H256::repeat_byte(0), H256::repeat_byte(1));
		assert!(
			TEST_NOMAD_BASE.is_updater_signature(&valid_signed).unwrap(),
			"should have passed on valid signature"
		);
	}

	#[test]
	#[cfg(feature = "testing")]
	fn it_rejects_invalid_signature() {
		let invalid_signed = FAKE_UPDATER.sign_update(H256::repeat_byte(0), H256::repeat_byte(1));
		assert!(
			!TEST_NOMAD_BASE
				.is_updater_signature(&invalid_signed)
				.unwrap(),
			"should have failed on invalid signature"
		);

		let other_updater = Updater::new(9999, TEST_UPDATER_PRIVKEY.parse().unwrap());
		let wrong_domain_signed =
			other_updater.sign_update(H256::repeat_byte(0), H256::repeat_byte(1));
		assert!(
			!TEST_NOMAD_BASE
				.is_updater_signature(&wrong_domain_signed)
				.unwrap(),
			"should have failed on invalid signature"
		);
	}
}

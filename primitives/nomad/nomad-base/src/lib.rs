#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::pallet_prelude::*;
use nomad_core::{home_domain_hash, NomadState, SignedUpdate};
use sp_core::{H160, H256};

#[cfg(feature = "testing")]
pub mod testing;

#[derive(Clone, Copy, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct NomadBase {
	pub state: NomadState,
	pub local_domain: u32,
	pub updater: H160,
}

impl Default for NomadBase {
	fn default() -> Self {
		Self {
			state: NomadState::Active,
			local_domain: Default::default(),
			updater: Default::default(),
		}
	}
}

pub enum NomadBaseError {
	FailedInitialization,
}

impl NomadBase {
	pub fn new(local_domain: u32, updater: H160) -> Self {
		Self { local_domain, updater, state: Default::default() }
	}

	pub fn state(&self) -> NomadState {
		self.state
	}

	pub fn local_domain(&self) -> u32 {
		self.local_domain
	}

	pub fn updater(&self) -> H160 {
		self.updater
	}

	pub fn home_domain_hash(&self) -> H256 {
		home_domain_hash(self.local_domain())
	}

	pub fn set_state(&mut self, new_state: NomadState) {
		self.state = new_state
	}

	pub fn set_updater(&mut self, new_updater: H160) {
		self.updater = new_updater
	}

	pub fn is_updater_signature(&self, signed_update: &SignedUpdate) -> bool {
		signed_update.verify(self.updater).is_ok()
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[cfg(feature = "testing")]
	use crate::testing::{TEST_LOCAL_DOMAIN, TEST_NOMAD_BASE, TEST_UPDATER};
	use ethers_signers::LocalWallet;
	use nomad_core::test_utils::Updater;

	#[test]
	#[cfg(feature = "testing")]
	fn it_accepts_valid_signature() {
		let valid_signed = TEST_UPDATER.sign_update(H256::repeat_byte(1));
		assert!(TEST_NOMAD_BASE.is_updater_signature(&valid_signed));
	}

	#[test]
	#[cfg(feature = "testing")]
	fn it_rejects_invalid_signature() {
		let random_signer: LocalWallet =
			"2222222222222222222222222222222222222222222222222222222222222222"
				.parse()
				.unwrap();
		let random_updater = Updater::new(TEST_LOCAL_DOMAIN, random_signer);
		let invalid_signed = random_updater.sign_update(H256::repeat_byte(1));

		assert!(!TEST_NOMAD_BASE.is_updater_signature(&invalid_signed));
	}
}

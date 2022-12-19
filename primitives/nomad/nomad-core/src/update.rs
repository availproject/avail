use frame_support::pallet_prelude::*;
use nomad_signature::{hash_message, Signature, SignatureError};
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_core::{H160, H256};

use crate::utils::home_domain_hash;

/// Nomad update
#[derive(Clone, Encode, Decode, PartialEq, Eq, RuntimeDebug, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct Update {
	/// The home chain
	pub home_domain: u32,
	/// The previous root
	pub previous_root: H256,
	/// The new root
	pub new_root: H256,
}

impl Update {
	/// Get signing hash for update
	pub fn signing_hash(&self) -> H256 {
		keccak256_concat!(
			home_domain_hash(self.home_domain),
			self.previous_root,
			self.new_root
		)
	}

	fn prepended_hash(&self) -> H256 {
		hash_message(self.signing_hash())
	}
}

/// A Signed Nomad Update
#[derive(Clone, Encode, Decode, PartialEq, Eq, RuntimeDebug, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct SignedUpdate {
	/// The update
	pub update: Update,
	/// The signature
	pub signature: Signature,
}

impl SignedUpdate {
	pub fn previous_root(&self) -> H256 {
		self.update.previous_root
	}

	pub fn new_root(&self) -> H256 {
		self.update.new_root
	}

	/// Recover the Ethereum address of the signer
	pub fn recover(&self) -> Result<H160, SignatureError> {
		self.signature.recover(self.update.prepended_hash())
	}

	/// Check whether a message was signed by a specific address
	pub fn verify(&self, signer: H160) -> Result<(), SignatureError> {
		self.signature.verify(self.update.prepended_hash(), signer)
	}
}

#[cfg(test)]
mod tests {
	use super::H256;
	use crate::test_utils::Updater;

	pub const TEST_UPDATER_PRIVKEY: &str =
		"1111111111111111111111111111111111111111111111111111111111111111";

	#[test]
	fn recover_valid_update() {
		use ethers_signers::{LocalWallet, Signer};

		let wallet: LocalWallet = TEST_UPDATER_PRIVKEY.parse().unwrap();
		println!("Wallet address: {:x}", wallet.address());

		let updater = Updater::new(1000, TEST_UPDATER_PRIVKEY.parse().unwrap());
		let signed_update = updater.sign_update(H256::repeat_byte(0), H256::repeat_byte(1));

		let recovered = signed_update.recover().expect("!recover");
		println!("Recovered address: {:x}", recovered);

		signed_update.verify(updater.address()).expect("!sig");
	}
}

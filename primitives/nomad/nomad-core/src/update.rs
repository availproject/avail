use frame_support::pallet_prelude::*;
use primitive_types::{H160, H256};
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use signature::{hash_message, Signature, SignatureError};
use tiny_keccak::{Hasher, Keccak};

use crate::utils::home_domain_hash;

/// Nomad update
#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo)]
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
		let mut output = [0u8; 32];
		let mut hasher = Keccak::v256();
		hasher.update(home_domain_hash(self.home_domain).as_ref());
		hasher.update(self.previous_root.as_ref());
		hasher.update(self.new_root.as_ref());
		hasher.finalize(&mut output);
		output.into()
	}

	fn prepended_hash(&self) -> H256 { hash_message(self.signing_hash()) }
}

/// A Signed Nomad Update
#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct SignedUpdate {
	/// The update
	pub update: Update,
	/// The signature
	pub signature: Signature,
}

impl SignedUpdate {
	/// Recover the Ethereum address of the signer
	pub fn recover(&self) -> Result<H160, SignatureError> {
		Ok(self.signature.recover(self.update.prepended_hash())?)
	}

	/// Check whether a message was signed by a specific address
	pub fn verify(&self, signer: H160) -> Result<(), SignatureError> {
		Ok(self
			.signature
			.verify(self.update.prepended_hash(), signer)?)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	#[cfg(feature = "testing")]
	use crate::test_utils::Updater;

	pub const TEST_UPDATER_PRIVKEY: &str =
		"1111111111111111111111111111111111111111111111111111111111111111";

	#[test]
	#[cfg(feature = "testing")]
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

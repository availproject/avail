use crate::utils::home_domain_hash;
use frame_support::pallet_prelude::*;
use sp_core::{H160, H256};
use signature::{hash_message, Signature, SignatureError};
use tiny_keccak::{Hasher, Keccak};

/// Nomad update
#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo)]
pub struct Update {
	/// The home chain
	pub home_domain: u32,
	/// The new root
	pub root: H256,
}

impl Update {
	/// Get signing hash for update
	pub fn signing_hash(&self) -> H256 {
		let mut output = [0u8; 32];
		let mut hasher = Keccak::v256();
		hasher.update(home_domain_hash(self.home_domain).as_ref());
		hasher.update(self.root.as_ref());
		hasher.finalize(&mut output);
		output.into()
	}

	fn prepended_hash(&self) -> H256 {
		hash_message(self.signing_hash())
	}
}

/// A Signed Nomad Update
#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo)]
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
		Ok(self.signature.verify(self.update.prepended_hash(), signer)?)
	}
}

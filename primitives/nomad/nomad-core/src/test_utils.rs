use ethers_core::utils::hash_message;
use ethers_signers::{LocalWallet, Signer};
use signature::Signature;
use sp_core::{H160, H256};

use crate::{
	update_v2::{SignedUpdateV2, UpdateV2},
	SignedUpdate, Update,
};

#[derive(Debug, Clone)]
pub struct Updater {
	pub domain: u32,
	signer: LocalWallet,
}

impl Updater {
	pub fn new(domain: u32, signer: LocalWallet) -> Self { Self { domain, signer } }

	pub fn address(&self) -> H160 { self.signer.address().0.into() }

	fn sign_message_without_eip_155<M: Send + Sync + AsRef<[u8]>>(&self, message: M) -> Signature {
		// Had to reimplement hash and signing to remove async-ness for
		// substrate testing
		let message = message.as_ref();
		let message_hash = hash_message(message);
		let mut signature = self.signer.sign_hash(message_hash);

		signature.v = 28 - (signature.v % 2);
		signature.into()
	}

	pub fn sign_update(&self, previous_root: H256, new_root: H256) -> SignedUpdate {
		let update = Update {
			home_domain: self.domain,
			previous_root,
			new_root,
		};
		let signature = self.sign_message_without_eip_155(update.signing_hash());
		SignedUpdate { update, signature }
	}

	pub fn sign_update_v2(&self, root: H256) -> SignedUpdateV2 {
		let update = UpdateV2 {
			home_domain: self.domain,
			root,
		};
		let signature = self.sign_message_without_eip_155(update.signing_hash());
		SignedUpdateV2 { update, signature }
	}
}
